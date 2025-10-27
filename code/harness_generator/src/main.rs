mod type_converters;

use std::{borrow::BorrowMut, collections::{HashMap, HashSet}, env, error::Error, fmt::format, fs::{self, File}, io::{BufReader, Write}, path::PathBuf, process::{self, Command, Output}, sync::{Arc, Mutex}};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use petgraph::{dot::{Config, Dot}, graph::*, Direction};
use petgraph::visit::{Dfs, EdgeRef};
use std::{mem, cmp};
use type_converters::*;
use std::path::Path;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use rand::seq::SliceRandom;
use itertools::Itertools;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::index;
use std::time::Instant;
use toml::Value;
use walkdir::WalkDir;
use regex::Regex;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use syn::{parse_file, parse_str, spanned::Spanned, Item, Signature, visit::Visit, Macro};
use proc_macro2::LineColumn;
use tokio::time::{sleep, Duration};
use chrono::Local;

/*
PARAMETERS:
    - SURF_DISABLE_CMPLX
    - SURF_DISABLE_CLOSURES
    - SURF_DISABLE_GENERICS
    - SURF_DISABLE_URAPI_CONSTR
    - SURF_ENABLE_NON_COMPILABLE_REPORT
    - SURF_DISABLE_UNSAFE_TRAITS
    - SURF_DISABLE_TARGET_BUILD
    - SURF_DISABLE_REMOVE_LAST_TARGET
    - SURF_ENABLE_LLMS
    - SURF_DISABLE_LLM_DOCUMENTATION
    - SURF_TRIM_LLM_DOCUMENTATION_FENCES
    - SURF_ENABLE_VERBOSE
    - SURF_SKIP_OPTION
    - SURF_ENABLE_LLM_ONLY
    - SURF_ENABLE_OPTIMIZED_TREE_GEN
    - SURF_ENABLE_FEATURES
    - SURF_EXTRA_DEPS
    - SURF_DISABLE_TARGET_FLAG
    - SURF_ENABLE_LINE_COVERAGE
*/

const DEFAULT_LLM_BACKEND: &str = "deepseek/deepseek-r1";
const MAX_PROMPT_ATTEMPTS: usize = 6;
const MAX_CONN_ATEMPTS: usize = 10;
const CONN_DELAY: u64 = 60; //seconds
const MAX_MACRO_FAIL_RATE: f64 = 0.2;
const STR_SIZE: usize = 16;
const VECTOR_SIZE: usize = 32;
const CARGO_JOBS: usize = 4;
const RAYON_THREADS: usize = 20;
const MAX_PATH: usize = 5;
const MAX_STRUCT_CONSTRUCTORS: usize = 4;
const MAX_ENUM_CONSTRUCTORS: usize = 1;
//const MAX_TARGETS_TO_CHECK: Option<usize> = None;
const MAX_TARGETS_TO_CHECK: Option<usize> = Some(50);
const MAX_TARGETS_TO_GEN: usize = 4;
const SEED: u64 = 17832827937830847112;

lazy_static! {
    pub static ref ROOT_CRATE_NAME: String = {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            eprintln!("[Help]: cargo run 'crate_identifier'.");
            process::exit(1);
        }
        String::from(&args[1])
    };
    pub static ref SURF_COMPLEX_TYPES_TO_CONSTRUCTORS: HashMap<String, HashSet<String>> = surf_load_cmplx_tys_file(&ROOT_CRATE_NAME);
    pub static ref SURF_MACRO_URAPIS: HashMap<String, HashSet<String>> = surf_load_macro_urapis_file(&ROOT_CRATE_NAME);
    pub static ref SURF_CONSTRUCTORS: HashMap<String, SurfConstructorData> = surf_load_constructors_file(&ROOT_CRATE_NAME);
    pub static ref SURF_TRAIT_FNS: HashMap<String, SurfTraitFnData> = surf_load_trait_fns_file(&ROOT_CRATE_NAME);
    pub static ref SURF_ENUMS: HashMap<String, HashMap::<String, Vec<SurfFnArg>>> = surf_load_enums_file(&ROOT_CRATE_NAME);
    pub static ref SURF_URAPIS: HashMap<String, SurfURAPI> = surf_load_urapis_file(&ROOT_CRATE_NAME);
    pub static ref SURF_TRAITS: HashMap<String, SurfTraitData> = surf_load_traits_file(&ROOT_CRATE_NAME);
    pub static ref SURF_USED_DEPS: HashMap<String, SurfDepType> = surf_load_deps_file(&ROOT_CRATE_NAME);
    pub static ref SURF_WORKING_PATH: String = surf_load_working_path();
    pub static ref GLOBAL_DATA_PATH: String = surf_load_global_data_path();
    pub static ref TARGET_CRATE_NAME: String = load_target_crate_name();
    // URAPIDefId -> UrapiTargets
    pub static ref TARGETS_MAP: Arc<Mutex<HashMap<String, UrapiTargets>>> = Arc::new(Mutex::new(HashMap::<String, UrapiTargets>::new()));
    // StableURAPIDefId -> FuzzableURAPI
    pub static ref FUZZING_TARGETS_MAP: Arc<Mutex<HashMap<String, FuzzableURAPI>>> = Arc::new(Mutex::new(HashMap::<String, FuzzableURAPI>::new()));
    pub static ref LLM_COVERED_URAPIS: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::<String>::new()));
    #[derive(Debug)]
    pub static ref SKIP_OPTION: Option<SkipOption> = try_get_skip_option();
    pub static ref CUSTOM_URAPIS: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::<String>::new()));
    pub static ref STABLE_URAPIS: HashSet<String> = get_stable_urapi_def_ids_set();
    pub static ref REDUNDANT_TARGETS: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::<String>::new()));
    pub static ref TARGETED_URAPI_MISS_TARGETS: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::<String>::new()));
    pub static ref LLM_BACKEND: String = get_llm_backend();
}

fn clean_documentation_fence_lines(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed == "```" || trimmed == "```rust" {
                None // remove the entire line
            } else if line.contains("```rust") {
                Some(line.replacen("```rust", "", 1)) // remove just the first occurrence
            } else {
                Some(line.to_string())
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn surf_find_rs_files_recursively(dir: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut rs_files = Vec::new();
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    rs_files.push(entry.path().to_path_buf());
                }
            }
        }
    }
    Ok(rs_files)
}

/// Collects only the *.rs files directly in `dir` (no deeper subdirectories).
fn surf_find_rs_files_top_level(dir: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut rs_files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "rs" {
                    rs_files.push(path);
                }
            }
        }
    }
    Ok(rs_files)
}

fn load_target_crate_name() -> String {
    match env::var("SURF_WORKING_PATH"){
        Ok(targeting_crate_path) => {
            let targeting_toml_path = format!("{targeting_crate_path}/Cargo.toml");
            if let Ok(toml_contents) = fs::read_to_string(&targeting_toml_path){
                if let Ok(toml_entries) = toml_contents.parse::<Value>(){
                    match toml_entries.get("package").and_then(|pkg| pkg.get("name")){
                        Some(crate_name) => {
                            match crate_name.as_str(){
                                Some(crate_name_str) => {return crate_name_str.to_string()},
                                _ => panic!("Unknown crate name."),
                            }
                        },
                        _ => panic!("Crate name not found."),
                    }
                }
            }
        },
        _ => panic!("The env variable 'SURF_WORKING_PATH' is not set!"),
    }
    panic!("The env variable 'SURF_WORKING_PATH' is not set!")
}

fn get_source_code(library_dir: &str) -> String {
    let src_dir = PathBuf::from(library_dir).join("src");
    let source_code_files = if src_dir.is_dir() {
        surf_find_rs_files_recursively(&src_dir).unwrap()
    }
    else {
        surf_find_rs_files_top_level(&PathBuf::from(&library_dir)).unwrap()
    };

    let source_code_str = if let Ok(_) = env::var("SURF_DISABLE_LLM_DOCUMENTATION"){
        String::from("")
    }
    else{
        source_code_files
            .iter()
            .map(|file| fs::read_to_string(file))
            .collect::<Result<Vec<String>, _>>()
            .expect("Could not create the file list")
            .join("\n")
    };
    source_code_str
}

fn parse_response_to_hashset(response: &str) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    // Try to parse as JSON array
    let parsed: Vec<String> = serde_json::from_str(response)
        .or_else(|_| {
            // Fallback: look for Rust vec syntax
            let cleaned = response
                .trim_matches(|c| c == '`' || c == '\n' || c == ' ')
                .replace("\\\"", "\"");
            serde_json::from_str(&cleaned)
        })?;

    Ok(parsed.into_iter().collect())
}

fn get_template_harness(crate_name: &str) -> String {
    format!(r##"
#[macro_use]
extern crate afl;    

use {crate_name}::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{{Deref, DerefMut, Index, IndexMut}};
fn main() {{
    fuzz_nohook!(|data: &[u8]| {{
        if data.len() < 105 {{ return; }} // Ensure minimum length for all examples
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        // Unsigned integers
        let t_0 = _to_u8(GLOBAL_DATA, 0);    // 1 byte (index 0)
        let t_1 = _to_u16(GLOBAL_DATA, 1);   // 2 bytes (index 1-2)
        let t_2 = _to_u32(GLOBAL_DATA, 3);   // 4 bytes (index 3-6)
        let t_3 = _to_u64(GLOBAL_DATA, 7);   // 8 bytes (index 7-14)
        let t_4 = _to_u128(GLOBAL_DATA, 15); // 16 bytes (index 15-30)
        let t_5 = _to_usize(GLOBAL_DATA, 31); // 8 bytes (index 31-38)

        // Signed integers
        let t_6 = _to_i8(GLOBAL_DATA, 39);   // 1 byte (index 39)
        let t_7 = _to_i16(GLOBAL_DATA, 40);  // 2 bytes (index 40-41)
        let t_8 = _to_i32(GLOBAL_DATA, 42);  // 4 bytes (index 42-45)
        let t_9 = _to_i64(GLOBAL_DATA, 46);  // 8 bytes (index 46-53)
        let t_10 = _to_i128(GLOBAL_DATA, 54); // 16 bytes (index 54-69)
        let t_11 = _to_isize(GLOBAL_DATA, 70); // 8 bytes (index 70-77)

        // Floating points
        let t_12 = _to_f32(GLOBAL_DATA, 78); // 4 bytes (index 78-81)
        let t_13 = _to_f64(GLOBAL_DATA, 82); // 8 bytes (index 82-89)

        // Characters and bool
        let t_14 = _to_char(GLOBAL_DATA, 90); // 4 bytes (index 90-93)
        let t_15 = _to_bool(GLOBAL_DATA, 94); // 1 byte (index 94)

        // &str and Strings
        let t_16 = _to_str(GLOBAL_DATA, 95, 105); // 10 bytes (index 95-104)
        let t_17 = String::from(t_16);

        // Option/Result handling
        let opt_t0 = Some(t_0);
        let unwrapped_opt = _unwrap_option(opt_t0);
        
        let res_t1: Result<u16, ()> = Ok(t_1);
        let unwrapped_res = _unwrap_result(res_t1);
    }});
}}

// All converter functions below...
fn _to_u8(data:&[u8], index:usize)->u8 {{
    data[index]
}}

fn _to_u16(data:&[u8], index:usize)->u16 {{
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}}

fn _to_u32(data:&[u8], index:usize)->u32 {{
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}}

fn _to_u64(data:&[u8], index:usize)->u64 {{
    let data0 = _to_u32(data, index) as u64;
    let data1 = _to_u32(data, index+4) as u64;
    data0 << 32 | data1
}}

fn _to_u128(data:&[u8], index:usize)->u128 {{
    let data0 = _to_u64(data, index) as u128;
    let data1 = _to_u64(data, index+8) as u128;
    data0 << 64 | data1
}}

fn _to_usize(data:&[u8], index:usize)->usize {{
    _to_u64(data, index) as usize
}}

fn _to_i8(data:&[u8], index:usize)->i8 {{    
    data[index] as i8
}}

fn _to_i16(data:&[u8], index:usize)->i16 {{
    let data0 = _to_i8(data, index) as i16;
    let data1 = _to_i8(data, index+1) as i16;
    data0 << 8 | data1
}}

fn _to_i32(data:&[u8], index:usize)->i32 {{
    let data0 = _to_i16(data, index) as i32;
    let data1 = _to_i16(data, index+2) as i32;
    data0 << 16 | data1
}}

fn _to_i64(data:&[u8], index:usize)->i64 {{
    let data0 = _to_i32(data, index) as i64;
    let data1 = _to_i32(data, index+4) as i64;
    data0 << 32 | data1
}}

fn _to_i128(data:&[u8], index:usize)->i128 {{
    let data0 = _to_i64(data, index) as i128;
    let data1 = _to_i64(data, index+8) as i128;
    data0 << 64 | data1
}}

fn _to_isize(data:&[u8], index:usize)->isize {{
    _to_i64(data, index) as isize
}}

fn _to_f32(data:&[u8], index: usize) -> f32 {{
    let data_slice = &data[index..index+4];
    use std::convert::TryInto;
    let data_array:[u8;4] = data_slice.try_into().expect("slice with incorrect length");
    f32::from_le_bytes(data_array)
}}

fn _to_f64(data:&[u8], index: usize) -> f64 {{
    let data_slice = &data[index..index+8];
    use std::convert::TryInto;
    let data_array:[u8;8] = data_slice.try_into().expect("slice with incorrect length");
    f64::from_le_bytes(data_array)
}}

fn _to_char(data:&[u8], index: usize)->char {{
    let char_value = _to_u32(data,index);
    match char::from_u32(char_value) {{
        Some(c)=>c,
        None=>{{
            std::process::exit(0);
        }}
    }}
}}

fn _to_bool(data:&[u8], index: usize)->bool {{
    let bool_value = _to_u8(data, index);
    if bool_value %2 == 0 {{
        true
    }} else {{
        false
    }}
}}

fn _to_str(data:&[u8], start_index: usize, end_index: usize)->&str {{
    let data_slice = &data[start_index..end_index];
    use std::str;
    match str::from_utf8(data_slice) {{
        Ok(s)=>s,
        Err(_)=>{{
            std::process::exit(0);
        }}
    }}
}}

fn _unwrap_option<T>(opt: Option<T>) -> T {{
    match opt {{
        Some(_t) => _t,
        None => {{
            std::process::exit(0);
        }}
    }}
}}

fn _unwrap_result<T, E>(_res: std::result::Result<T, E>) -> T {{
    match _res {{
        Ok(_t) => _t,
        Err(_) => {{
            std::process::exit(0);
        }},
    }}
}}
"##)
}

async fn perform_llm_cfg() -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    
    let api_key = std::env::var("OPENROUTER_API_KEY")
            .expect("OPENROUTER_API_KEY must be set");

    let client = OpenAIClient::builder()
            .with_endpoint("https://openrouter.ai/api/v1")
            .with_api_key(api_key)
            .build()?;
    
    let library_source_code = get_source_code(&SURF_WORKING_PATH);

    let system_content = "You are a security-focused Rust code analyzer.\n\
                                Perform control flow analysis to find all public functions that can reach unsafe code.\n\
                                Exclude public functions that are itself defined as unsafe.\n\
                                But include functions that are defined as safe and encapsulate unsafe code or can reach unsafe code by calling other functions.\n\
                                Exclude functions that are not part of the library's API but are for testing purposes.\n\
                                Return ONLY a valid Rust Vec<String> containing full function names. No explanations, no formatting.".to_string();
    
    let dynamic_prompt = format!("Analyze this Rust code and list all public functions that can reach unsafe code:\n{}", library_source_code);
    // println!("{}", dynamic_prompt);
    // process::exit(1);
    println!("[LLM-CFG]: Performing LLM CFG Analysis...");
    let mut conn_attempts = 0;
    let llm_urapis = loop { // connection attempts
        conn_attempts += 1;
        match get_llm_urapis(&client, Some(system_content.clone()), dynamic_prompt.clone()).await {
            Ok(llm_urapis) => {
                // We got a successful value, break out of the loop
                break llm_urapis;
            }
            Err(e) => {
                if conn_attempts >= MAX_CONN_ATEMPTS {
                    // Give up if we've hit the max
                    println!("[LLM-CFG]: Failed after {conn_attempts} attempts");
                    return Err(e.into()); 
                }

                println!("[LLM-CFG]: {e}.");
                println!("[LLM-CFG]: Sleeping {CONN_DELAY} seconds, then retrying...");
                sleep(Duration::from_secs(CONN_DELAY)).await;
            }
        }
    };
    Ok(llm_urapis)
}

fn create_llm_only_directories(llm_urapis: &HashSet<String>) -> HashMap<String, String>{
    
    // Create templates directory
    let fuzz_llm_only_templates_dir = format!("{}/deepSURF/fuzz/llm-only/templates/", &SURF_WORKING_PATH.clone());
    let path = Path::new(&fuzz_llm_only_templates_dir);
    if !path.exists(){
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {fuzz_llm_only_templates_dir}"));
    }
    
    // Update the Map: LLM-URAPI -> Template Name
    let mut llm_urapis_to_templates = HashMap::<String, String>::new();
    let mut llm_urapi_index = 0;
    println!("[LLM-CFG]: Creating Templates...");
    for llm_urapi in llm_urapis.iter(){
        
        // Create template name
        let template_name = format!("llm_only_target_{}", llm_urapi_index);
        let template_code = get_template_harness(&TARGET_CRATE_NAME.to_string().replace("-", "_"));
        llm_urapis_to_templates.insert(llm_urapi.clone(), template_name.clone());
        
        // Create template dir
        let fuzz_llm_only_template_dir = format!("{}/deepSURF/fuzz/llm-only/templates/{}", &SURF_WORKING_PATH.clone(), template_name);
        let path = Path::new(&fuzz_llm_only_template_dir);
        if !path.exists(){
            fs::create_dir_all(path).expect(&format!("Failed to create folder: {fuzz_llm_only_template_dir}"));
        }

        // Create template toml
        let toml_contents = surf_get_toml_contents(&template_name, "", None).0;
        let working_path = &SURF_WORKING_PATH.clone();
        let template_toml_path = format!("{}/Cargo.toml", fuzz_llm_only_template_dir);
        let mut file = File::create(template_toml_path).unwrap();
        file.write_all(toml_contents.as_bytes()).expect("Unable to write to Cargo.toml file");
        file.flush().expect("Unable to flush to Cargo.toml file");

        // Create template source dir
        let template_src_dir_path = format!("{}/src", fuzz_llm_only_template_dir);
        let path = Path::new(&template_src_dir_path);
        if !path.exists() {
            fs::create_dir_all(path).expect(&format!("Failed to create folder: {template_src_dir_path}"));
        }
        
        // Create template source code file
        let template_src_path = format!("{}/src/{}.rs", fuzz_llm_only_template_dir, template_name);
        let mut file = File::create(template_src_path).unwrap();
        file.write_all(template_code.as_bytes()).expect("Unable to write to fuzz_target file");
        file.flush().expect("Unable to flush to fuzz_target file");

        llm_urapi_index += 1;
    }
    llm_urapis_to_templates
}

async fn improve_targets_llm_only(llm_urapis_to_templates: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    //process::exit(1);
    let mut fuzzing_targets_map_lock = FUZZING_TARGETS_MAP.lock().unwrap();
    let mut covered_urapis = HashSet::<String>::new();
    
    // Create compilable and non_compilable llm-only generation fuzz target dirs if they do not exist
    let fuzz_llm_only_comp_dir = format!("{}/deepSURF/fuzz/llm-only/compilable/", &SURF_WORKING_PATH.clone());
    let path = Path::new(&fuzz_llm_only_comp_dir);
    if !path.exists(){
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {fuzz_llm_only_comp_dir}"));
    }
    let fuzz_llm_only_non_comp_dir = format!("{}/deepSURF/fuzz/llm-only/non_compilable/", &SURF_WORKING_PATH.clone());
    let path = Path::new(&fuzz_llm_only_non_comp_dir);
    if !path.exists(){
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {fuzz_llm_only_non_comp_dir}"));
    }
    let fuzz_llm_only_non_comp_dir = PathBuf::from(fuzz_llm_only_non_comp_dir);


    let local_time = Local::now();
    println!("[{}]: Starting LLM-only fuzz target generation...", local_time.format("%Y-%m-%d %H:%M:%S"));
    println!("-----------------------------------------------------------------------------------------------------------------------------------");
    
    for (llm_urapi, template_name) in llm_urapis_to_templates.iter(){
        // Get template paths
        let template_paths = surf_load_llm_only_paths(llm_urapi.clone(), &template_name, &SURF_WORKING_PATH);

        // Read the template harness file
        let harness_content = fs::read_to_string(&template_paths.template_harness).expect("Unable to read `template_harness` file.");

        // Read library's source code (documentation)
        let documentation = match env::var("SURF_DISABLE_LLM_DOCUMENTATION"){
            Ok(_) => String::from(""),
            _ => match fs::read_to_string(template_paths.documentation){
                Ok(markdown_content) => {
                    match env::var("SURF_TRIM_LLM_DOCUMENTATION_FENCES"){
                        Ok(_) => clean_documentation_fence_lines(&markdown_content),
                        _ => markdown_content,
                    }
                    
                },
                _ => String::from(""),
            }
        };
        let api_key = std::env::var("OPENROUTER_API_KEY")
            .expect("OPENROUTER_API_KEY must be set");
    
        let client = OpenAIClient::builder()
            .with_endpoint("https://openrouter.ai/api/v1")
            .with_api_key(api_key)
            .build()?;

        let mut static_prompt = surf_get_init_system_prompt_llm_only();
        let mut dynamic_prompt = surf_build_init_dynamic_prompt_llm_only(
            &llm_urapi,
            &harness_content,
            &documentation,
        );
        // println!("----> {}", static_prompt);
        // println!("----> {}", dynamic_prompt);


        let mut attempt: usize = 1;
        println!("[{llm_urapi}]: attempting llm-improvement...");
        loop{ // improvement attempts
            let mut conn_attempts = 0;
            let improved_harness = loop { // connection attempts
                conn_attempts += 1;
                match surf_get_improved_harness(&client, Some(static_prompt.clone()), dynamic_prompt.clone()).await {
                    Ok(improved_harness) => {
                        // We got a successful value, break out of the loop
                        break improved_harness;
                    }
                    Err(e) => {
                        if conn_attempts >= MAX_CONN_ATEMPTS {
                            // Give up if we've hit the max
                            println!("[{llm_urapi}]: Failed after {conn_attempts} attempts");
                            LLM_COVERED_URAPIS.lock().unwrap().extend(covered_urapis);
                            return Err(e.into()); 
                        }

                        println!("[{llm_urapi}]: {e}.");
                        println!("[{llm_urapi}]: Sleeping {CONN_DELAY} seconds, then retrying...");
                        sleep(Duration::from_secs(CONN_DELAY)).await;
                    }
                }
            };

            if !has_main_function(&improved_harness){
                println!("[{llm_urapi}]: Could not locate main function. Repeating the last prompt...");
                println!("[{llm_urapi}]: checking target {attempt}/{MAX_PROMPT_ATTEMPTS} --> Err!");
                if attempt >= MAX_PROMPT_ATTEMPTS{
                    println!("[{llm_urapi}]: Harness improvement failed after {MAX_PROMPT_ATTEMPTS} attempts.");
                    break;
                }
                attempt+=1;
                continue;
            }
    
            let cur_fuzz_target_dir = &template_paths.template_dir;            
            let improved_target_name = format!("{}_{}",
                                                        cur_fuzz_target_dir.file_name().unwrap().to_str().unwrap(),
                                                        match LLM_BACKEND.split("/").last(){
                                                            Some(model_name) => format!("{model_name}_turn{attempt}"),
                                                            _ => format!("openrouter_turn{attempt}"),
                                                        },
            );
            let improved_target_dir = surf_create_improved_llm_only_target_dirs(cur_fuzz_target_dir, &improved_target_name)?;
            let improved_harness_path = surf_find_harness_file(&improved_target_dir)?;
            fs::write(&improved_harness_path, &improved_harness)?;
            //println!("{}", improved_harness);
            match surf_run_llm_fb_cargo_check(&improved_target_dir) {
                Ok(called_apis) => {

                    // Do some validity checks here
                    if !contains_fuzz_macros(&improved_harness){
                        println!("[{llm_urapi}]: could not find fuzz_nohook! macro.");
                        println!("[{llm_urapi}]: checking target {attempt}/{MAX_PROMPT_ATTEMPTS} --> Err!");
                        surf_remove_target_dir(&improved_target_dir);
                        surf_move_dir_into(&improved_target_dir, &fuzz_llm_only_non_comp_dir)?;
                        break;
                    }

                    println!("[{llm_urapi}]: checking target {attempt}/{MAX_PROMPT_ATTEMPTS} --> Ok!");

                    for called_api_stable_def_id in called_apis{
                        // Check if is a URAPI
                        if STABLE_URAPIS.contains(&called_api_stable_def_id){
                            covered_urapis.insert(called_api_stable_def_id.clone());
                            // Since is a URAPI now check if there is already a target for it
                            
                            if let Some(fuzzing_urapi) = fuzzing_targets_map_lock.get_mut(&called_api_stable_def_id){
                                fuzzing_urapi.fuzz_targets_names.insert(improved_target_name.clone());
                            }
                            else{
                                fuzzing_targets_map_lock.insert(called_api_stable_def_id.clone(),
                                                                FuzzableURAPI{
                                                                                urapi_def_id: llm_urapi.clone(),
                                                                                is_macro_expanded: false,
                                                                                is_llm_improved: true,
                                                                                fuzz_targets_names: {
                                                                                                        let mut fuzz_targets_names = HashSet::<String>::new();
                                                                                                        fuzz_targets_names.insert(improved_target_name.clone());
                                                                                                        fuzz_targets_names
                                                                                },
                                                                }
                                );
                            }
                        }
                    }
                    
                    if let Some(fuzzer_input_size) = surf_extract_number_and_double(&improved_harness){
                        //println!("---New fuzzer's length: {fuzzer_input_size}");
                        surf_replace_fuzzer_input_len(&improved_target_dir, fuzzer_input_size);
                        println!("[{llm_urapi}]_non_asan: compiling target {} --> {}!",
                                                    template_name,
                                                    if surf_cargo_build_fuzz_target_non_asan(&improved_target_dir) {"Ok"} else {"Err"}
                        );
                        println!("[{llm_urapi}]_asan: compiling target {} --> {}!",
                                                    template_name,
                                                    if surf_cargo_build_fuzz_target_asan(&improved_target_dir) {"Ok"} else {"Err"}
                        );
                        surf_remove_target_dir(&improved_target_dir);
                    }
                    else{
                        println!("[{llm_urapi}]: Unable to extract new fuzzer's length. Try doing it manually.");
                    }
                    break
                },
                Err(output) => {
                    println!("[{llm_urapi}]: checking target {attempt}/{MAX_PROMPT_ATTEMPTS} --> Err!");
                    
                    // Move the llm_improved fuzz target to non_compilable
                    surf_remove_target_dir(&improved_target_dir);
                    surf_move_dir_into(&improved_target_dir, &fuzz_llm_only_non_comp_dir)?;

                    //println!("{:?}", output);
                    let error_message = String::from_utf8_lossy(&output.stderr).to_string();

                    static_prompt = surf_get_retry_system_prompt_llm_only();

                    dynamic_prompt = surf_build_retry_prompt_llm_only(
                        &improved_harness,
                        &error_message,
                        &documentation
                    );
                },
            }
            if attempt >= MAX_PROMPT_ATTEMPTS{
                println!("[{llm_urapi}]: Harness improvement failed after {MAX_PROMPT_ATTEMPTS} attempts.");
                break;
            }
            attempt+=1;
        }
    }

    //println!("{}", harness_content);
    println!("Fuzz target LLM improvement is complete!");
    println!("-----------------------------------------------------------------------------------------------------------------------------------");
    LLM_COVERED_URAPIS.lock().unwrap().extend(covered_urapis);
    Ok(())
}

fn main(){
    if let Ok(_) = env::var("SURF_ENABLE_LLM_ONLY"){
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        // 1. Perform the LLM CFG Analysis first
        let llm_urapis = match rt.block_on(perform_llm_cfg()){
            Ok(llm_urapis) => llm_urapis,
            Err(err) => {
                println!("Future Err: {:?}", err);
                process::exit(-1);
            },
        };
        println!("URAPIs of LLM CFG Analysis: {:#?}", llm_urapis);
        
        //2. Generate template directories under deepSURF/fuzz/llm-only/
        let llm_urapis_to_templates = create_llm_only_directories(&llm_urapis);
        let deepsurf_total_gen_time = Instant::now();
        // 3. Run similar logic to improve targets
        if let Err(err) = rt.block_on(improve_targets_llm_only(&llm_urapis_to_templates)){
            println!("Future Err: {:?}", err);
        }
        let deepsurf_total_gen_time = deepsurf_total_gen_time.elapsed();
        calculate_llm_only_stats(deepsurf_total_gen_time);
    }
    else{
        let surf_total_gen_time = generate_targets();
        select_fuzzing_targets();
        calculate_stats(surf_total_gen_time);
        if let Ok(_) = env::var("SURF_ENABLE_LLMS"){
            get_custom_logic_urapis();
            // println!("Custom URAPI Fuzz Targets: {:?}", surf_get_custom_urapis_fuzz_targets_count());
            let rt = tokio::runtime::Runtime::new().unwrap();
            let deepsurf_total_gen_time = Instant::now();
            if let Err(err) = rt.block_on(improve_targets()){
                println!("Future Err: {:?}", err);
            }
            let deepsurf_total_gen_time = deepsurf_total_gen_time.elapsed();
            calculate_llm_int_stats(deepsurf_total_gen_time);
        }
    }
}

fn generate_targets() -> Duration{
    
    let local_time = Local::now();
    println!("[{}]: Starting fuzz target generation...", local_time.format("%Y-%m-%d %H:%M:%S"));
    println!("-----------------------------------------------------------------------------------------------------------------------------------");
    surf_create_fuzz_and_replay_dirs();
    ThreadPoolBuilder::new().stack_size(64 * 1024 * 1024).num_threads(RAYON_THREADS).build_global().unwrap();
    let total_gen_time = Instant::now();
    SURF_URAPIS.par_iter().for_each(|(urapi_def_id, surf_urapi)| {
        {
            let start_time = Instant::now();
            let mut urapi_dep_tree = SurfTree::new();

            // Build dependency tree
            surf_build_dep_tree(urapi_def_id, surf_urapi, &mut urapi_dep_tree);
            //println!("{:?}", Dot::with_config(&urapi_dep_tree.tree, &[Config::EdgeNoLabel]));
            if let Ok(_) = env::var("SURF_ENABLE_VERBOSE"){
                println!("[{urapi_def_id}]: building dependency tree -> Ok!");
            }

            // Build fuzz trees
            let urapi_fuzz_trees = {
                if let Ok(_) = env::var("SURF_ENABLE_OPTIMIZED_TREE_GEN"){
                    surf_build_optz_fuzz_trees(urapi_def_id, urapi_dep_tree, SEED)
                }
                else{
                    let mut rng = StdRng::seed_from_u64(SEED);
                    let mut urapi_fuzz_trees = surf_build_fuzz_trees(urapi_def_id, urapi_dep_tree);
                    urapi_fuzz_trees.shuffle(&mut rng);
                    urapi_fuzz_trees
                }
            };
            if let Ok(_) = env::var("SURF_ENABLE_VERBOSE"){
                println!("[{urapi_def_id}]: building '{}' fuzz trees -> Ok!", urapi_fuzz_trees.len());
            }
            
            urapi_fuzz_trees.iter().enumerate().for_each(|(fuzz_tree_index, fuzz_tree)| {
                {
                    let mut targets_map_lock = TARGETS_MAP.lock().unwrap();
                    let urapi_targets =  targets_map_lock.entry(urapi_def_id.clone()).or_insert(UrapiTargets::new());
                    if let Some(targets_to_check_limit) = MAX_TARGETS_TO_CHECK{
                        if urapi_targets.checked_targets >= targets_to_check_limit{
                            return;
                        }
                    }
                    
                    if urapi_targets.target_pairs.len() >= MAX_TARGETS_TO_GEN{
                        return;
                    }
                }
                // Try w/o substituting implicit generics
                let (mut cargo_check_status, mut target_pair) = surf_check_harness_with_mode(urapi_def_id, fuzz_tree, &HarnessGenerationMode::NoSubstitution);

                // Try w/ substituting implicit generics (aggressive approach)
                if !cargo_check_status{
                    (cargo_check_status, target_pair) = surf_check_harness_with_mode(urapi_def_id, fuzz_tree, &HarnessGenerationMode::SubstituteImplicitGens);
                }
                
                // Update the shared map 
                {
                    let mut targets_map_lock = TARGETS_MAP.lock().unwrap();
                    let urapi_targets =  targets_map_lock.get_mut(urapi_def_id).unwrap();
                    if cargo_check_status{
                        let duration = start_time.elapsed();
                        target_pair.fuzz_target.time_to_gen = Some(duration);
                        if urapi_targets.target_pairs.len() < MAX_TARGETS_TO_GEN{
                            urapi_targets.target_pairs.push(target_pair.clone());
                            if let Ok(_) = env::var("SURF_ENABLE_VERBOSE"){
                                println!("[{urapi_def_id}]: checking target {}/{MAX_TARGETS_TO_GEN} --> Ok!", urapi_targets.target_pairs.len());
                            }
                        }
                    }
                    urapi_targets.checked_targets += 1;
                }

                // Build the harness or remove the non-compilable target
                if cargo_check_status{
                    if let Ok(_) = env::var("SURF_DISABLE_TARGET_BUILD"){
                        // Skip building, only checking here
                    }
                    else{
                        let cargo_build_status = surf_compile_harnesses(&target_pair);
                        if cargo_build_status{
                            if let Ok(_) = env::var("SURF_ENABLE_VERBOSE"){
                                println!("[{urapi_def_id}]: compiling target {:?} --> Ok!", target_pair.fuzz_target.fuzz_target_id.target_name);
                            }
                        }
                        else{
                            if let Ok(_) = env::var("SURF_ENABLE_VERBOSE"){
                                println!("[{urapi_def_id}]: compiling target {:?} --> Error!", target_pair.fuzz_target.fuzz_target_id.target_name);
                            }
                        }
                    }
                }
                else{
                    if let Ok(_) = env::var("SURF_DISABLE_REMOVE_LAST_TARGET"){
                        let mut min_limit = urapi_fuzz_trees.len() - 1;
                        if let Some(targets_to_check_limit) = MAX_TARGETS_TO_CHECK {
                            min_limit = cmp::min(min_limit, targets_to_check_limit-1);
                        }

                        if fuzz_tree_index == min_limit{
                            // Last target should be maintained
                            {
                                let mut targets_map_lock = TARGETS_MAP.lock().unwrap();
                                let urapi_targets =  targets_map_lock.get_mut(urapi_def_id).unwrap();
                                urapi_targets.target_pairs.push(target_pair.clone());
                                let fuzz_target_dir = PathBuf::from(&SURF_WORKING_PATH.clone()).join(format!("deepSURF/fuzz/no_llm/compilable/{}", &target_pair.fuzz_target.fuzz_target_id.target_name));
                                surf_remove_target_dir(&fuzz_target_dir);
                            }
                        }
                        else{
                            surf_remove_uncompilable(&target_pair);
                        }
                    }
                    else{
                        surf_remove_uncompilable(&target_pair);
                    }
                    
                }
            });
        }
    });
    
    // Output Elapsed Time
    println!("Fuzz target generation is complete!");
    println!("-----------------------------------------------------------------------------------------------------------------------------------");
    let total_gen_time = total_gen_time.elapsed();
    println!("Total Elapsed Time: {:?}", total_gen_time);
    println!("-----------------------------------------------------------------------------------------------------------------------------------");
    total_gen_time
}

// This will be called before the LLM Improvement
fn select_fuzzing_targets(){
    let targets_map_lock = TARGETS_MAP.lock().unwrap();
    let mut fuzzing_targets_map_lock = FUZZING_TARGETS_MAP.lock().unwrap();
    // Handle the non-macro-expanded URAPIs first
    for (urapi_def_id, urapi_targets) in targets_map_lock.iter(){
        let stable_urapi_def_id = SURF_URAPIS.get(urapi_def_id).unwrap().def_path_str.clone();
        if SURF_MACRO_URAPIS.contains_key(&stable_urapi_def_id){
            continue;
        }
        let mut fuzz_targets_names = HashSet::<String>::new();
        for urapi_target in urapi_targets.target_pairs.iter(){
            if urapi_target.compilable{
                let urapi_target_name = &urapi_target.fuzz_target.fuzz_target_id.target_name;
                let urapi_target_harness = &urapi_target.fuzz_target.fuzz_target_harness.harness;
                if let Ok(unsafety) = has_unsafe_code(urapi_target_harness){
                    if !unsafety{
                        fuzz_targets_names.insert(urapi_target_name.clone());
                    }
                }
            }
        }
        if !fuzz_targets_names.is_empty(){
            fuzzing_targets_map_lock.entry(stable_urapi_def_id).or_insert(FuzzableURAPI{
                                                                                            urapi_def_id: urapi_def_id.clone(),
                                                                                            is_macro_expanded: false,
                                                                                            is_llm_improved: false,
                                                                                            fuzz_targets_names,
                                                                                        });
        }
    }

    // Handle the macro-expanded URAPIs here
    // Select the one variant that offers the most fuzzable targets
    for (macro_expanded_urapi_stable_def_id, variants) in SURF_MACRO_URAPIS.iter(){
        let mut max_complilable_fuzz_targets = 0;
        let mut variant_with_max_compilable_targets_opt = None;
        let mut fuzz_targets_names_of_max_variant = HashSet::<String>::new();
        for variant_def_id in variants{
            let mut fuzz_targets_names = HashSet::<String>::new();
            let mut variant_compilable_fuzz_targets = 0;
            if let Some(urapi_targets) = targets_map_lock.get(variant_def_id){
                for urapi_target in urapi_targets.target_pairs.iter(){
                    if urapi_target.compilable{
                        let urapi_target_name = &urapi_target.fuzz_target.fuzz_target_id.target_name;
                        let urapi_target_harness = &urapi_target.fuzz_target.fuzz_target_harness.harness;
                        if let Ok(unsafety) = has_unsafe_code(urapi_target_harness){
                            if !unsafety{
                                fuzz_targets_names.insert(urapi_target_name.clone());
                                variant_compilable_fuzz_targets += 1;
                            }
                        }
                    }
                }
            }
            if variant_compilable_fuzz_targets > max_complilable_fuzz_targets{
                max_complilable_fuzz_targets = variant_compilable_fuzz_targets;
                variant_with_max_compilable_targets_opt = Some(variant_def_id);
                fuzz_targets_names_of_max_variant = fuzz_targets_names;
            }
        }
        if let Some(variant_with_max_compilable_targets) = variant_with_max_compilable_targets_opt{
            fuzzing_targets_map_lock.entry(macro_expanded_urapi_stable_def_id.clone()).or_insert(FuzzableURAPI{
                                                                                                                urapi_def_id: variant_with_max_compilable_targets.clone(),
                                                                                                                is_macro_expanded: true,
                                                                                                                is_llm_improved: false,
                                                                                                                fuzz_targets_names: fuzz_targets_names_of_max_variant,
                                                                                                            });
        }
    }
}

fn surf_check_harness_with_mode(
    urapi_def_id: &String,
    fuzz_tree: &FuzzTree,
    mode: &HarnessGenerationMode,
) -> (bool, TargetPair)
{
    let (fuzz_harness, replay_harness) = surf_get_fuzz_harness(urapi_def_id, fuzz_tree, &mode);
    let fuzz_target_name = surf_get_current_fuzz_target_name(&TARGET_CRATE_NAME, fuzz_tree.id);
    let replay_target_name = surf_get_current_replay_target_name(&TARGET_CRATE_NAME, fuzz_tree.id);

    let fuzz_target_id = TargetId::new(urapi_def_id.to_string(), fuzz_target_name.clone());
    let replay_target_id = TargetId::new(urapi_def_id.to_string(), replay_target_name.clone());

    let fuzz_target = FuzzTarget::new(fuzz_target_id, fuzz_harness);
    let replay_target = ReplayTarget::new(replay_target_id, replay_harness);
    let mut target_pair = TargetPair::new(fuzz_target, replay_target);

    let cargo_check_status = surf_check_harness(&target_pair);
    target_pair.compilable = cargo_check_status;
    (cargo_check_status, target_pair)
}

fn has_impl_block(harness: &str) -> bool {
    let syntax_tree = parse_str::<syn::File>(harness);
    
    if let Ok(file) = syntax_tree {
        file.items.iter().any(|item| matches!(item, Item::Impl(_)))
    } else {
        false
    }
}

fn has_custom_fn(harness: &str) -> bool {
    let syntax_tree = parse_str::<syn::File>(harness);
    let Ok(file) = syntax_tree else { return false };

    file.items.iter().any(|item| {
        match item {
            Item::Fn(fn_item) => check_signature(&fn_item.sig),
            _ => false
        }
    })
}

struct MacroVisitor {
    found_fuzz_macro: bool,
}

impl MacroVisitor {
    fn new() -> Self {
        Self {
            found_fuzz_macro: false,
        }
    }
}

impl<'ast> Visit<'ast> for MacroVisitor {
    fn visit_macro(&mut self, mac: &'ast Macro) {
        // Check if the macro path matches fuzz_nohook!
        // Allow only fuzz_nohook! for now
        let path = &mac.path;
        if path.is_ident("fuzz_nohook") {
            self.found_fuzz_macro = true;
        }
        
        // Continue traversing the AST
        syn::visit::visit_macro(self, mac);
    }
}

pub fn contains_fuzz_macros(source: &str) -> bool {
    let Ok(ast) = syn::parse_file(source) else {
        return false; // Invalid Rust code
    };

    let mut visitor = MacroVisitor::new();
    visitor.visit_file(&ast);
    visitor.found_fuzz_macro
}

struct UnsafeVisitor {
    has_unsafe: bool,
}

impl<'ast> Visit<'ast> for UnsafeVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        if node.sig.unsafety.is_some() {
            self.has_unsafe = true;
        }
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_expr_unsafe(&mut self, _node: &'ast syn::ExprUnsafe) {
        self.has_unsafe = true;
        syn::visit::visit_expr_unsafe(self, _node);
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        if node.unsafety.is_some() {
            self.has_unsafe = true;
        }
        syn::visit::visit_item_trait(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        if node.unsafety.is_some() {
            self.has_unsafe = true;
        }
        syn::visit::visit_item_impl(self, node);
    }

    // Updated method name and parameter type
    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        if node.sig.unsafety.is_some() {
            self.has_unsafe = true;
        }
        syn::visit::visit_impl_item_fn(self, node);
    }
}

pub fn has_unsafe_code(input: &str) -> Result<bool, syn::Error> {
    let syntax_tree = syn::parse_file(input)?;
    let mut visitor = UnsafeVisitor { has_unsafe: false };
    visitor.visit_file(&syntax_tree);
    Ok(visitor.has_unsafe)
}

fn check_signature(sig: &Signature) -> bool {
    sig.ident.to_string().starts_with("_custom_fn")
}

fn has_main_function(src: &str) -> bool {
    let cleaned = src
        .lines()
        .filter(|line| !line.trim_start().starts_with("//")) // ignore single-line comments
        .collect::<Vec<_>>()
        .join("\n");

    // Look for `fn main` with optional attributes or modifiers
    let pattern = regex::Regex::new(r"\bfn\s+main\s*\(").unwrap();
    pattern.is_match(&cleaned)
}

fn get_custom_logic_urapis(){
    for (urapi_def_id, urapi_targets) in TARGETS_MAP.lock().unwrap().iter(){
        let total_pairs = urapi_targets.target_pairs.len();
        for (index, target_pair) in urapi_targets.target_pairs.iter().enumerate() {
            let is_last_pair = index == total_pairs - 1;
            if target_pair.compilable{
                let fuzz_target_harness = &target_pair.fuzz_target.fuzz_target_harness.harness;
                if has_impl_block(fuzz_target_harness) || has_custom_fn(fuzz_target_harness){
                    CUSTOM_URAPIS.lock().unwrap().insert(SURF_URAPIS.get(urapi_def_id).unwrap().def_path_str.clone());
                }
                break;
            }
            else{
                if is_last_pair {
                    // The last target should be maintained for LLM even if is non-compilable:
                    if let Ok(_) = env::var("SURF_DISABLE_REMOVE_LAST_TARGET"){
                        let fuzz_target_harness = &target_pair.fuzz_target.fuzz_target_harness.harness;
                        if has_impl_block(fuzz_target_harness) || has_custom_fn(fuzz_target_harness){
                            CUSTOM_URAPIS.lock().unwrap().insert(SURF_URAPIS.get(urapi_def_id).unwrap().def_path_str.clone());
                        }
                    }
                    
                }
            }
        }
    }
    
}

fn surf_get_custom_urapis_fuzz_targets_count() -> usize{
    let fuzzing_targets_map_lock = FUZZING_TARGETS_MAP.lock().unwrap();
    let custom_urapis_lock = CUSTOM_URAPIS.lock().unwrap();
    let mut fuzz_targets_count = 0;
    for stable_custom_urapi_def_id in custom_urapis_lock.iter(){
        if fuzzing_targets_map_lock.contains_key(stable_custom_urapi_def_id){
            fuzz_targets_count+= fuzzing_targets_map_lock.get(stable_custom_urapi_def_id).unwrap().fuzz_targets_names.len();
        }
    }
    fuzz_targets_count
}

async fn improve_targets() -> Result<(), Box<dyn std::error::Error>> {
    //process::exit(1);
    let targets_map_lock = TARGETS_MAP.lock().unwrap();
    let mut fuzzing_targets_map_lock = FUZZING_TARGETS_MAP.lock().unwrap();
    let mut macro_improvement_attempts_map = HashMap::<String, usize>::new();
    let mut covered_urapis = HashSet::<String>::new();
    
    // Create compilable and non_compilable llm-improved dirs if they do not exist
    let fuzz_llm_comp_dir = format!("{}/deepSURF/fuzz/llm/compilable/", &SURF_WORKING_PATH.clone());
    let path = Path::new(&fuzz_llm_comp_dir);
    if !path.exists(){
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {fuzz_llm_comp_dir}"));
    }
    let fuzz_llm_non_comp_dir = format!("{}/deepSURF/fuzz/llm/non_compilable/", &SURF_WORKING_PATH.clone());
    let path = Path::new(&fuzz_llm_non_comp_dir);
    if !path.exists(){
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {fuzz_llm_non_comp_dir}"));
    }
    let fuzz_llm_non_comp_dir = PathBuf::from(fuzz_llm_non_comp_dir);

    let local_time = Local::now();
    println!("[{}]: Starting fuzz target LLM improvement...", local_time.format("%Y-%m-%d %H:%M:%S"));
    println!("Selected Skip Option: {:?}", SKIP_OPTION.unwrap());
    println!("-----------------------------------------------------------------------------------------------------------------------------------");
    for (urapi_def_id, urapi_targets) in targets_map_lock.iter(){
        let stable_urapi_def_id = &SURF_URAPIS.get(urapi_def_id).unwrap().def_path_str;

        // SKIP logic here
        match SKIP_OPTION.unwrap(){
            SkipOption::Skip => {
                if let Some(surf_urapi) = SURF_URAPIS.get(urapi_def_id){
                    if covered_urapis.contains(&surf_urapi.def_path_str){
                        println!("[{urapi_def_id}]: Skipping Covered URAPI...");
                        continue;
                    }
                }
            }
            SkipOption::CondSkip => {
                if let Some(surf_urapi) = SURF_URAPIS.get(urapi_def_id){
                    if covered_urapis.contains(&surf_urapi.def_path_str){
                        if !CUSTOM_URAPIS.lock().unwrap().contains(&surf_urapi.def_path_str){
                            println!("[{urapi_def_id}]: Skipping Covered URAPI...");
                            continue;
                        }
                    }
                }
            }
            SkipOption::NoSkip => {},
        }

        // Check if is a macro-expanded URAPI variant that has been already improved
        if SURF_MACRO_URAPIS.contains_key(stable_urapi_def_id){
            if fuzzing_targets_map_lock.contains_key(stable_urapi_def_id){
                if fuzzing_targets_map_lock.get(stable_urapi_def_id).unwrap().is_llm_improved{
                    println!("[{urapi_def_id}]: Skipping Variant of Already LLM-Improved Macro-Expanded URAPI...");
                    continue;
                }
            }
            let macro_variants_count = SURF_MACRO_URAPIS.get(stable_urapi_def_id).unwrap().len();
            let macro_improvement_attempts_count = macro_improvement_attempts_map.entry(stable_urapi_def_id.clone()).or_insert(0);
            let macro_improvement_fail_rate = (*macro_improvement_attempts_count as f64)/(macro_variants_count as f64);
            if macro_improvement_fail_rate > MAX_MACRO_FAIL_RATE{
                println!("[{urapi_def_id}]: Skipping Macro Variant Due To Reached Failure Threshold...");
                continue;
            }
        }

        // Get fuzz target paths
        let fuzz_target_id = urapi_targets.target_pairs.get(0).unwrap().fuzz_target.fuzz_target_id.target_name.clone();
        let fuzz_target_paths = surf_load_paths(urapi_def_id.clone(), &fuzz_target_id, &SURF_WORKING_PATH);
        // Read sample harness file
        let harness_content = fs::read_to_string(&fuzz_target_paths.sample_harness).expect("Unable to read `sample_harness` file.");
        
        // Read static analysis metadata
        let cmplx_types_to_cons_str = fs::read_to_string(&fuzz_target_paths.static_analysis.cmplx_types_to_cons).expect("Unable to read `cmplx_types_to_cons` file.");
        let analyzed_cons_str = fs::read_to_string(&fuzz_target_paths.static_analysis.analyzed_cons).expect("Unable to read `analyzed_cons` file.");
        let analyzed_urapis_str = fs::read_to_string(&fuzz_target_paths.static_analysis.analyzed_urapis).expect("Unable to read `analyzed_urapis` file.");
        let analysis_combined = format!("{cmplx_types_to_cons_str}\n\n{analyzed_cons_str}\n\n{analyzed_urapis_str}");

        // Read library's source code (documentation)
        
        let documentation = match env::var("SURF_DISABLE_LLM_DOCUMENTATION"){
            Ok(_) => String::from(""),
            _ => match fs::read_to_string(fuzz_target_paths.documentation){
                Ok(markdown_content) => {
                    match env::var("SURF_TRIM_LLM_DOCUMENTATION_FENCES"){
                        Ok(_) => clean_documentation_fence_lines(&markdown_content),
                        _ => markdown_content,
                    }
                    
                },
                _ => String::from(""),
            }
        };

        let api_key = std::env::var("OPENROUTER_API_KEY")
            .expect("OPENROUTER_API_KEY must be set");
        
        let client = OpenAIClient::builder()
            .with_endpoint("https://openrouter.ai/api/v1")
            .with_api_key(api_key)
            .build()?;
        
        let mut static_prompt = surf_get_init_system_prompt();
        let mut dynamic_prompt = surf_build_init_dynamic_prompt(
            &fuzz_target_paths.target_urapi,
            &harness_content,
            &analysis_combined,
            &documentation,
        );

        // println!("----> {}", static_prompt);
        // println!("----> {}", dynamic_prompt);
        //std::process::exit(0);

        let mut attempt: usize = 1;
        println!("[{urapi_def_id}]: attempting llm-improvement...");
        loop{ // improvement attempts
            let mut conn_attempts = 0;
            let improved_harness = loop { // connection attempts
                conn_attempts += 1;
                match surf_get_improved_harness(&client, Some(static_prompt.clone()), dynamic_prompt.clone()).await {
                    Ok(improved_harness) => {
                        // We got a successful value, break out of the loop
                        break improved_harness;
                    }
                    Err(e) => {
                        if conn_attempts >= MAX_CONN_ATEMPTS {
                            // Give up if we've hit the max
                            println!("[{urapi_def_id}]: Failed after {conn_attempts} attempts");
                            LLM_COVERED_URAPIS.lock().unwrap().extend(covered_urapis);
                            return Err(e.into()); 
                        }

                        println!("[{urapi_def_id}]: {e}.");
                        println!("[{urapi_def_id}]: Sleeping {CONN_DELAY} seconds, then retrying...");
                        sleep(Duration::from_secs(CONN_DELAY)).await;
                    }
                }
            };

            if !has_main_function(&improved_harness){
                println!("[{urapi_def_id}]: Could not locate main function. Repeating the last prompt...");
                println!("[{urapi_def_id}]: checking target {attempt}/{MAX_PROMPT_ATTEMPTS} --> Err!");
                if attempt >= MAX_PROMPT_ATTEMPTS{
                    println!("[{urapi_def_id}]: Harness improvement failed after {MAX_PROMPT_ATTEMPTS} attempts.");
                    break;
                }
                attempt+=1;
                continue;
            }

            let cur_fuzz_target_dir = &fuzz_target_paths.target_dir;            
            let improved_target_name = format!("{}_{}",
                                                        cur_fuzz_target_dir.file_name().unwrap().to_str().unwrap(),
                                                        match LLM_BACKEND.split("/").last(){
                                                            Some(model_name) => format!("{model_name}_turn{attempt}"),
                                                            _ => format!("openrouter_turn{attempt}"),
                                                        },
            );
            let improved_target_dir = surf_create_improved_target_dirs(cur_fuzz_target_dir, &improved_target_name)?;
            let improved_harness_path = surf_find_harness_file(&improved_target_dir)?;
            fs::write(&improved_harness_path, &improved_harness)?;
            //println!("{}", improved_harness);
            match surf_run_llm_fb_cargo_check(&improved_target_dir) {
                Ok(called_apis) => {

                    // Do some validity checks here
                    if !contains_fuzz_macros(&improved_harness){
                        println!("[{urapi_def_id}]: could not find fuzz_nohook! macro.");
                        println!("[{urapi_def_id}]: checking target {attempt}/{MAX_PROMPT_ATTEMPTS} --> Err!");
                        surf_remove_target_dir(&improved_target_dir);
                        surf_move_dir_into(&improved_target_dir, &fuzz_llm_non_comp_dir)?;
                        break;
                    }

                    println!("[{urapi_def_id}]: checking target {attempt}/{MAX_PROMPT_ATTEMPTS} --> Ok!");

                    // Actually check here what new you really covered.
                    if !called_apis.contains(stable_urapi_def_id){
                        TARGETED_URAPI_MISS_TARGETS.lock().unwrap().insert(improved_target_name.clone());
                    }

                    let mut covered_new_urapi = false;
                    for called_api_stable_def_id in called_apis{
                        // Check if is a URAPI
                        if STABLE_URAPIS.contains(&called_api_stable_def_id){
                            covered_urapis.insert(called_api_stable_def_id.clone());
                            // Since is a URAPI now check if there is already a target for it
                            if let Some(fuzzing_urapi) = fuzzing_targets_map_lock.get_mut(&called_api_stable_def_id){
                                // Check if there is already an LLM-improved target
                                if fuzzing_urapi.is_llm_improved{
                                    fuzzing_urapi.fuzz_targets_names.insert(improved_target_name.clone());
                                }
                                else{
                                    // Replace the old fuzz target from SURF 
                                    fuzzing_urapi.is_llm_improved = true;
                                    if let SkipOption::CondSkip = SKIP_OPTION.unwrap(){
                                        if !CUSTOM_URAPIS.lock().unwrap().contains(&called_api_stable_def_id){
                                            fuzzing_urapi.fuzz_targets_names = HashSet::<String>::new();
                                        }
                                    }
                                    else{
                                        fuzzing_urapi.fuzz_targets_names = HashSet::<String>::new();
                                    }
                                    fuzzing_urapi.fuzz_targets_names.insert(improved_target_name.clone());
                                    covered_new_urapi = true;
                                }
                            }
                            else{
                                fuzzing_targets_map_lock.insert(called_api_stable_def_id.clone(),
                                                                FuzzableURAPI{
                                                                                urapi_def_id: urapi_def_id.clone(),
                                                                                is_macro_expanded: SURF_MACRO_URAPIS.contains_key(&called_api_stable_def_id),
                                                                                is_llm_improved: true,
                                                                                fuzz_targets_names: {
                                                                                                        let mut fuzz_targets_names = HashSet::<String>::new();
                                                                                                        fuzz_targets_names.insert(improved_target_name.clone());
                                                                                                        fuzz_targets_names
                                                                                },
                                                                }
                                );
                                covered_new_urapi = true;
                            }
                        }
                    }
                    // If is macro-expanded URAPI that eventually did not get covered (although the target compiled) then update the penalty
                    if SURF_MACRO_URAPIS.contains_key(stable_urapi_def_id) && !covered_urapis.contains(stable_urapi_def_id){
                        let macro_variants_count = SURF_MACRO_URAPIS.get(stable_urapi_def_id).unwrap().len();
                        let macro_improvement_attempts_count = macro_improvement_attempts_map.get_mut(stable_urapi_def_id).unwrap();
                        *macro_improvement_attempts_count += 1;
                        let macro_improvement_fail_rate = (*macro_improvement_attempts_count as f64)/(macro_variants_count as f64);
                        println!("[{urapi_def_id}]: Macro-Expanded URAPI `{stable_urapi_def_id}` failure rate: {:.2}", macro_improvement_fail_rate*100.0);
                    }

                    if !covered_new_urapi{
                        if let SkipOption::CondSkip = SKIP_OPTION.unwrap(){
                            if !CUSTOM_URAPIS.lock().unwrap().contains(stable_urapi_def_id){
                                REDUNDANT_TARGETS.lock().unwrap().insert(improved_target_name);
                            }
                            else{
                                if TARGETED_URAPI_MISS_TARGETS.lock().unwrap().contains(stable_urapi_def_id){
                                    REDUNDANT_TARGETS.lock().unwrap().insert(improved_target_name);
                                }
                            }
                        }
                        else{
                            REDUNDANT_TARGETS.lock().unwrap().insert(improved_target_name);
                        }
                    }
                    
                    if let Some(fuzzer_input_size) = surf_extract_number_and_double(&improved_harness){
                        //println!("---New fuzzer's length: {fuzzer_input_size}");
                        surf_replace_fuzzer_input_len(&improved_target_dir, fuzzer_input_size);
                        println!("[{urapi_def_id}]_non_asan: compiling target {} --> {}!",
                                                    fuzz_target_id,
                                                    if surf_cargo_build_fuzz_target_non_asan(&improved_target_dir) {"Ok"} else {"Err"}
                        );
                        println!("[{urapi_def_id}]_asan: compiling target {} --> {}!",
                                                    fuzz_target_id,
                                                    if surf_cargo_build_fuzz_target_asan(&improved_target_dir) {"Ok"} else {"Err"}
                        );
                        surf_remove_target_dir(&improved_target_dir);
                    }
                    else{
                        println!("[{urapi_def_id}]: Unable to extract new fuzzer's length. Try doing it manually.");
                    }
                    break
                },
                Err(output) => {
                    println!("[{urapi_def_id}]: checking target {attempt}/{MAX_PROMPT_ATTEMPTS} --> Err!");
                    
                    // Move the llm_improved fuzz target to non_compilable
                    surf_remove_target_dir(&improved_target_dir);
                    surf_move_dir_into(&improved_target_dir, &fuzz_llm_non_comp_dir)?;

                    if attempt == MAX_PROMPT_ATTEMPTS {
                        // Check if the failed harness improvement corresponds to a Macro-Expanded URAPI
                        if SURF_MACRO_URAPIS.contains_key(stable_urapi_def_id){
                            let macro_variants_count = SURF_MACRO_URAPIS.get(stable_urapi_def_id).unwrap().len();
                            let macro_improvement_attempts_count = macro_improvement_attempts_map.get_mut(stable_urapi_def_id).unwrap();
                            *macro_improvement_attempts_count += 1;
                            let macro_improvement_fail_rate = (*macro_improvement_attempts_count as f64)/(macro_variants_count as f64);
                            println!("[{urapi_def_id}]: Macro-Expanded URAPI `{stable_urapi_def_id}` failure rate: {:.2}", macro_improvement_fail_rate*100.0);
                            break;
                        }   
                    }

                    //println!("---`cargo check` failed. Sending errors back to the model...");
                    //println!("{:?}", output);
                    let error_message = String::from_utf8_lossy(&output.stderr).to_string();
    
                    static_prompt = surf_get_retry_system_prompt();
    
                    dynamic_prompt = surf_build_retry_prompt(
                        &improved_harness,
                        &error_message,
                        &analysis_combined,
                        &documentation
                    );
                },
            }
            if attempt >= MAX_PROMPT_ATTEMPTS{
                println!("[{urapi_def_id}]: Harness improvement failed after {MAX_PROMPT_ATTEMPTS} attempts.");
                break;
            }
            attempt+=1;
        }

        //println!("{}", harness_content);
        //println!("");
    }
    println!("Fuzz target LLM improvement is complete!");
    println!("-----------------------------------------------------------------------------------------------------------------------------------");
    LLM_COVERED_URAPIS.lock().unwrap().extend(covered_urapis);
    Ok(())
}

fn calculate_stats(total_gen_time: Duration){
    let targets_map_lock = TARGETS_MAP.lock().unwrap();
    let fuzzing_targets_map_lock = FUZZING_TARGETS_MAP.lock().unwrap();
    surf_calculate_and_write_stats(&targets_map_lock, &fuzzing_targets_map_lock, total_gen_time);
    surf_write_fuzzing_targets(&fuzzing_targets_map_lock);
}

fn calculate_llm_int_stats(total_gen_time: Duration){
    let covered_urapis_lock = LLM_COVERED_URAPIS.lock().unwrap();
    let fuzzing_targets_map_lock = FUZZING_TARGETS_MAP.lock().unwrap();
    surf_calculate_and_write_llm_stats(&covered_urapis_lock, &fuzzing_targets_map_lock, total_gen_time);
    surf_write_fuzzing_targets_llm(&fuzzing_targets_map_lock);
}

fn calculate_llm_only_stats(total_gen_time: Duration){
    let covered_urapis_lock = LLM_COVERED_URAPIS.lock().unwrap();
    let fuzzing_targets_map_lock = FUZZING_TARGETS_MAP.lock().unwrap();
    surf_calculate_and_write_llm_only_stats(&covered_urapis_lock, &fuzzing_targets_map_lock, total_gen_time);
    surf_write_fuzzing_targets_llm_only(&fuzzing_targets_map_lock);
}


/* -------------------------------------------------------------------------
                        LLM FUNCTIONS
--------------------------------------------------------------------------*/

fn surf_get_init_system_prompt_llm_only() -> String {
    let mut prompt = String::new();

    // Role
    prompt.push_str(
        "You are a Rust code-generation assistant. \n\
        I define a URAPI, as a safe API of a Rust library that can reach unsafe code and potentially expose memory safety vulnerabilities.\n\
        I will provide you a template of an AFL++ Rust harness that describes how to convert fuzzer bytes to primitive Rust types.\n\
        The template has only the skeleton of the wanted harness, including functions for converting fuzzer bytes to Rust types and how to call them.\n\
        Your GOAL is to complete the template so you produce ONE new Rust harness that targets the given URAPI, in a complex way.\n\n\
        To achieve the goal, I will provide you the following:\n\
        (I) Descriptions of the INPUT files.\n\
        (II) Directions on WHAT TO DO and WHAT NOT TO DO to generate the harness.\n\
        (III) Description of the desired OUTPUT.\n\
        (IV) The targeting URAPI (given in the user prompt).\n\
        (V) INPUT files (given in the user prompt).\n\n"
    );

    // I) INPUT FILES DESCRIPTION
    prompt.push_str("(I) Descriptions of the INPUT files:\n");
    prompt.push_str(
        "\t1. A template of a Rust AFL++ fuzz harness including type converters and examples of how to call them.\n\
        \t2. Documentation of the input library.\n\n"
    );

    // II) DIRECTIONS
    prompt.push_str("(II) You MUST follow all the 14 directions bellow. They are all important. Feel free to do more if it furthers the goal:\n");
    prompt.push_str(
        "\t1. For structs and enums, use multiple constructors from those available:\n\
        \t a) Dynamically choose which constructor to call based on fuzzer input.\n\
        \t b) If possible select a set of diverse constructors from the provided ones.\n\
        \t2. Do NOT return the existing type conversion functions such as `_to_u8`, `_to_usize`, `_to_str` etc.:\n\
        \t a) They supposed to remain unchanged.\n\
        \t b) Do NOT include their definitions in the output I'll manually add them later before I compile the harness.\n\
        \t3. Custom function and method implementations:\n\
        \t a) For arguments that expect closures, generate custom functions with the appropriate signature and sustitute with pointers to these functions.\n\
        \t b) Custom implementations of functions and methods (including trait functions) should be driven by the fuzzer's input. That is, their return values should be derived from the fuzzer's input, appropriately crafted to match the return type of the function or method.\n\
        \t c) Based on fuzzer's input allow custom functions and methods (including trait functions) to panic (used to expose panic safety bugs).\n\
        \t4. Create more complex API call sequences:\n\
        \t a) Focus on invoking the target URAPI.\n\
        \t b) **Before** and **after** the URAPI call, invoke as many additional syntactically and semantically relevant URAPIs/APIs as possible.\n\
        \t c) Use static analysis and documentation to discover available URAPIs/APIs.\n\
        \t d) If feasible, call APIs/URAPIs multiple times based on fuzzer input.\n\
        \t e) A possible structure: an outer loop for the number of operations, an inner match to dispatch each operation (including the targeting URAPI).\n\
        \t5. The fuzzer's input is via `data`.\n\
        \t a) GLOBAL_DATA extends `data`'s lifetime and is used to access fuzzer's data from all the functions of the harness.\n\
        \t b) Do not change GLOBAL_DATA logic.\n\
        \t6. Incorporate the fuzzer's random input to steer harness logic:\n\
        \t a) Use it to build arguments and decide iteration counts.\n\
        \t7. Bounds checks and modulo usage:\n\
        \t a) Keep the initial check on `data.len()`. Set data.len() bounds to be sufficiently large to accommodate all harness-generated data types while avoiding unnecessary excess. Also keep in mind that `global_data.first_half.len() + global_data.second_half.len() = data.len()`.\n\
        \t b) Do NOT exceed `data.len()` (and similarly GLOBAL_DATA) when converting bytes.\n\
        \t c) Vector sizes, should be bounded by modulo 65.\n\
        \t d) Every other URAPI and API that gets numeric arguments, should get these arguments directly from the fuzzer's input, AVOIDING any bounds checking or modulo logic, even if that leads exceeding its length.
        \t8. Force reference dereference.\n\
        \t a) You must dereference any references, when possible.\n\
        \t b) References can be returned from APIs, URAPIs or can occur when indexing collections.\n\
        \t c) You can use `println!` to access the point-to data of a reference. In this case, you may have to add `#[derive(Debug)]` as needed.\n\
        \t d) Use the static analysis and documentation input I provide you to see what APIs/URAPIs return references.\n\
        \t9. Handling of **unsafe trait** impls:\n\
        \t a) Custom implementations of **unsafe traits** ARE NOT ALLOWED in the resulting harness.\n\
        \t b) However you are allowed to use types that implement unsafe traits and these usafe trait implementations are part of the library.\n\
        \t10. Handling generic arguments and associated trait types:\n\
        \t a) Generate custom types to substitute generic arguments and associated types e.g. `struct CustomType0(String)`.\n\
        \t11. Handling of **safe trait** impls:\n\
        \t a) If a generic argument requires implementation of safe traits, then implement these traits for the custom type that will substitute the generic argument.\n\
        \t b) The custom implementations of trait functions should follow the DIRECTION 3.\n\
        \t12. Defining new array types:\n\
        \t a) If new arrays are introduced, use lengths larger than 10.\n\
        \t13. The use of unsafe code IS PROHIBITED.\n\
        \t a) Do not call unsafe functions even if they are part of the library's API.\n\
        \t b) Do not insert unsafe blocks.\n\
        \t c) The resulting harness should maintain the attribute #![forbid(unsafe_code)] at the beginning.\n\
        \t14. Imports/Externs & unused variables:\n\
        \t a) Do NOT alter existing imports/externs from the sample harness.\n\
        \t b) Only import/extern additional traits if absolutely necessary.\n\
        \t c) Do NOT include comments, dead code, or unused variables.\n\n"
    );

    // III) OUTPUT DESCRIPTION
    prompt.push_str("(III) Directions for the OUTPUT:\n");
    prompt.push_str(
        "\t1. Return ONLY the resulting harness as compilable Rust source code:\n\
        \t a) Do NOT include type converters.\n\
        \t b) Do NOT return explanations of the changes you made.\n\
        \t c) Do NOT return markers of where the harness starts and ends.\n\
        \t d) Do NOT insert `#![no_main]` attribute.\n\
        \t e) Do NOT include comments.\n"
    );
    prompt
}

fn surf_get_init_system_prompt() -> String {
    let mut prompt = String::new();

    // Role
    prompt.push_str(
        "You are a Rust code-generation assistant. \n\
        I define a URAPI, as a safe API of a Rust library that can reach unsafe code and potentially expose memory safety vulnerabilities. \n\
        I will provide you a sample Rust harness that stresses a targeting URAPI and additional supportive input. \n\
        Your GOAL is to produce ONE new Rust harness that targets the same URAPI, but in a more complex way than the original harness. \n\n\
        To achieve the goal, I will provide you the following:\n\
        (I) Descriptions of the INPUT files.\n\
        (II) Directions on WHAT TO DO and WHAT NOT TO DO to improve the sample harness.\n\
        (III) Description of the desired OUTPUT.\n\
        (IV) The targeting URAPI (given in the user prompt).\n\
        (V) INPUT files (given in the user prompt).\n\n"
    );

    // I) INPUT FILES DESCRIPTION
    prompt.push_str("(I) Descriptions of the INPUT files:\n");
    prompt.push_str(
        "\t1. A sample Rust fuzz harness targeting a specific URAPI.\n\
        \t2. JSON files from a static analysis of the library that contains this URAPI:\n\
        \t a) A mapping from complex types (structs/enums) to their constructor APIs.\n\
        \t b) The constructor APIs (after argument analysis).\n\
        \t c) The URAPIs of the input library (after argument analysis).\n\
        \t3. Documentation of the input library.\n\n"
    );

    // II) DIRECTIONS
    prompt.push_str("(II) You MUST follow all the 14 directions bellow. They are all important. Feel free to do more if it furthers the goal:\n");
    prompt.push_str(
        "\t1. For structs and enums, use multiple constructors from those available:\n\
        \t a) Dynamically choose which constructor to call based on fuzzer input.\n\
        \t b) If possible select a set of diverse constructors from the provided ones.\n\
        \t2. Do NOT return the existing type conversion functions such as `_to_u8`, `_to_usize`, `_to_str` etc.:\n\
        \t a) They supposed to remain unchanged.\n\
        \t b) Do NOT include their definitions in the output I'll manually add them later before I compile the harness.\n\
        \t3. Preserve intentional panic logic:\n\
        \t a) Do NOT alter or remove code that deliberately causes panics (used to expose panic safety bugs).\n\
        \t b) Closures, custom functions and trait functions should maintain their panic logic.\n\
        \t4. Create more complex API call sequences:\n\
        \t a) Focus on invoking the target URAPI.\n\
        \t b) **Before** and **after** the URAPI call, invoke as many additional syntactically and semantically relevant URAPIs/APIs as possible.\n\
        \t c) Use static analysis and documentation to discover available URAPIs/APIs.\n\
        \t d) If feasible, call APIs/URAPIs multiple times based on fuzzer input.\n\
        \t e) A possible structure: an outer loop for the number of operations, an inner match to dispatch each operation (including the targeting URAPI).\n\
        \t5. The fuzzer's input is via `data`.\n\
        \t a) GLOBAL_DATA extends `data`'s lifetime and is used to access fuzzer's data from all the functions of the harness.\n\
        \t b) Do not change GLOBAL_DATA logic.\n\
        \t6. Incorporate the fuzzer's random input to steer harness logic:\n\
        \t a) Use it to build arguments and decide iteration counts.\n\
        \t7. Bounds checks and modulo usage:\n\
        \t a) Keep the initial check on `data.len()`. Set data.len() bounds to be sufficiently large to accommodate all harness-generated data types while avoiding unnecessary excess. Also keep in mind that `global_data.first_half.len() + global_data.second_half.len() = data.len()`.\n\
        \t b) Do NOT exceed `data.len()` (and similarly GLOBAL_DATA) when converting bytes.\n\
        \t c) Vector sizes, should be bounded by modulo 65.\n\
        \t d) Every other URAPI and API that gets numeric arguments, should get these arguments directly from the fuzzer's input, AVOIDING any bounds checking or modulo logic, even if that leads exceeding its length.
        \t8. Force reference dereference.\n\
        \t a) You must dereference any references, when possible.\n\
        \t b) References can be returned from APIs, URAPIs or can occur when indexing collections.\n\
        \t c) You can use `println!` to access the point-to data of a reference. In this case, you may have to add `#[derive(Debug)]` as needed.\n\
        \t d) Use the static analysis and documentation input I provide you to see what APIs/URAPIs return references.\n\
        \t9. Handling of **unsafe trait** impls:\n\
        \t a) Custom implementations of **unsafe traits** ARE NOT ALLOWED in the improved harness and you MUST REMOVE THEM.\n\
        \t b) Replace all custom types that come with custom unsafe trait implementations with library data types that already implement these unsafe traits.\n\
        \t c) Search in the documentation and static analysis output I provide, to find library data type candidates.\n\
        \t10. Handling of **safe trait** impls:\n\
        \t a) Preserve any existing custom implementations for **safe traits**.\n\
        \t b) The improved harness must still use custom trait implementations where provided (for safe traits).\n\
        \t11. Handling custom types:\n\
        \t a) If not implementing unsafe traits, prefer to use existing custom types; extend or wrap them if needed.\n\
        \t12. Defining new array types:\n\
        \t a) If new arrays are introduced, use lengths larger than 10.\n\
        \t13. The use of unsafe code IS PROHIBITED.\n\
        \t a) Do not call unsafe functions even if they are part of the library's API.\n\
        \t b) Do not insert unsafe blocks.\n\
        \t c) The improved harness should maintain the attribute #![forbid(unsafe_code)] at the beginning.\n\
        \t14. Imports/Externs & unused variables:\n\
        \t a) Do NOT alter existing imports/externs from the sample harness.\n\
        \t b) Only import/extern additional traits if absolutely necessary.\n\
        \t c) Do NOT include comments, dead code, or unused variables.\n\n"
    );

    // III) OUTPUT DESCRIPTION
    prompt.push_str("(III) Directions for the OUTPUT:\n");
    prompt.push_str(
        "\t1. Return ONLY the improved harness as compilable Rust source code:\n\
        \t a) Do NOT include type converters.\n\
        \t b) Do NOT return explanations of the changes you made.\n\
        \t c) Do NOT return markers of where the harness starts and ends.\n\
        \t d) Do NOT insert `#![no_main]` attribute.\n\
        \t e) Do NOT include comments.\n"
    );
    prompt
}

fn surf_build_init_dynamic_prompt_llm_only(
    target_urapi: &str,
    harness_content: &str,
    docs: &str,
) -> String {
    // IV/V) Targeting URAPI and INPUT files
    format!(
        "(IV) The targeting URAPI is '{target_urapi}'.\n\n\
        (V) INPUT files:\n\
        === HARNESS TEMPLATE START ===\n{harness_content}\n=== HARNESS TEMPLATE END ===\n\n\
        === DOCUMENTATION START ===\n{docs}\n=== DOCUMENTATION END ===\n\n\
        Now please generate the improved harness.\n"
    )
}


fn surf_build_init_dynamic_prompt(
    target_urapi: &str,
    harness_content: &str,
    analysis_jsons: &str,
    docs: &str,
) -> String {
    // IV/V) Targeting URAPI and INPUT files
    format!(
        "(IV) The targeting URAPI is '{target_urapi}'.\n\n\
        (V) INPUT files:\n\
        === SAMPLE HARNESS START ===\n{harness_content}\n=== SAMPLE HARNESS END ===\n\n\
        === STATIC ANALYSIS START ===\n{analysis_jsons}\n=== STATIC ANALYSIS END ===\n\n\
        === DOCUMENTATION START ===\n{docs}\n=== DOCUMENTATION END ===\n\n\
        Now please generate the improved harness.\n"
    )
}

async fn surf_get_improved_harness(
    client: &OpenAIClient, 
    system_prompt: Option<String>, 
    user_prompt: String
) -> Result<String, Box<dyn std::error::Error>> {

    let mut messages = Vec::new();
    
    if let Some(system_prompt) = system_prompt {
        let system_msg = chat_completion::ChatCompletionMessage{
            role: chat_completion::MessageRole::system,
            content: chat_completion::Content::Text(system_prompt),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        };
        messages.push(system_msg);
    }

    let user_msg = chat_completion::ChatCompletionMessage{
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(user_prompt),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    };
    messages.push(user_msg);

    let mut msg_req = ChatCompletionRequest::new(
        LLM_BACKEND.to_string(),
        messages,
    );
    msg_req = msg_req.max_tokens(20000);
    // process::exit(0);
    //println!("---Sending the request to Openrouter {LLM_BACKEND} model. This may take a few seconds...");
    let response = client.chat_completion(msg_req).await?;

    if let Some(choice) = response.choices.first() {
        if let Some(content) = &choice.message.content {
            return Ok(format!("{}\n{}\n\n{}", 
                                            "#![forbid(unsafe_code)]",
                                            replace_fuzz_macro(&surf_clean_leftover_converters(&surf_remove_forbid_unsafe(&surf_extract_rust_code(&surf_remove_harness_markers(&content))))), 
                                            surf_get_type_converters()
            ));
        }
    }
    
    Err("---Failed to get a valid harness from the model".into())
    
}

async fn get_llm_urapis(
    client: &OpenAIClient, 
    system_prompt: Option<String>, 
    user_prompt: String
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {

    let mut messages = Vec::new();
    
    if let Some(system_prompt) = system_prompt {
        let system_msg = chat_completion::ChatCompletionMessage{
            role: chat_completion::MessageRole::system,
            content: chat_completion::Content::Text(system_prompt),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        };
        messages.push(system_msg);
    }

    let user_msg = chat_completion::ChatCompletionMessage{
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(user_prompt),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    };
    messages.push(user_msg);

    let mut msg_req = ChatCompletionRequest::new(
        LLM_BACKEND.to_string(),
        messages,
    );
    msg_req = msg_req.max_tokens(20000);
    //println!("---Sending the request to Openrouter {LLM_BACKEND} model. This may take a few seconds...");
    let response = client.chat_completion(msg_req).await?;

    if let Some(choice) = response.choices.first() {
        if let Some(content) = &choice.message.content {
            return parse_response_to_hashset(content);
        }
    }
    
    Err("---Failed to get a hashet of URAPIs".into())
}


fn replace_fuzz_macro(input: &str) -> String {
    input.replace("fuzz!", "fuzz_nohook!")
}

fn surf_extract_rust_code(content: &str) -> String {
    let re = Regex::new(r"(?s).*?```rust\n(.*?)\n```").unwrap();
    
    if let Some(captures) = re.captures(content) {
        return captures.get(1).unwrap().as_str().trim().to_string();
    }
    
    content.to_string() // Return original content if no match is found
}

fn surf_remove_forbid_unsafe(source: &str) -> String {
    // Create regex pattern to match the attribute with any spacing
    let pattern = Regex::new(r"(?m)^\s*#!\[forbid\(\s*unsafe_code\s*\)\]\s*$").unwrap();
    
    // Replace all occurrences with empty string
    pattern.replace_all(source, "").to_string()
}

fn surf_remove_harness_markers(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    // Must have at least 2 lines to possibly have start & end markers
    if lines.len() >= 2 {
        let first_line = lines[0].trim();
        let last_line = lines[lines.len() - 1].trim();

        // Check if first & last lines are "=== ... ==="
        if first_line.starts_with("===") 
            && first_line.ends_with("===")
            && last_line.starts_with("===")
            && last_line.ends_with("===")
        {
            // Return everything between those lines
            let middle_lines = &lines[1..lines.len() - 1];
            return middle_lines.join("\n");
        }
    }

    // If we don't match, return the entire input
    input.to_string()
}

fn surf_get_type_converters() -> String{
    let mut coverters = Vec::new();
    coverters.push(_data_to_u8().to_string());
    coverters.push(_data_to_u16().to_string());
    coverters.push(_data_to_u32().to_string());
    coverters.push(_data_to_u64().to_string());
    coverters.push(_data_to_u128().to_string());
    coverters.push(_data_to_usize().to_string());
    coverters.push(_data_to_i8().to_string());
    coverters.push(_data_to_i16().to_string());
    coverters.push(_data_to_i32().to_string());
    coverters.push(_data_to_i64().to_string());
    coverters.push(_data_to_i128().to_string());
    coverters.push(_data_to_isize().to_string());
    coverters.push(_data_to_f32().to_string());
    coverters.push(_data_to_f64().to_string());
    coverters.push(_data_to_char().to_string());
    coverters.push(_data_to_bool().to_string());
    coverters.push(_data_to_str().to_string());
    coverters.push(_unwrap_option_function().to_string());
    coverters.push(_unwrap_result_function().to_string());
    coverters.join("\n")
}

fn surf_move_dir_into(src_dir: &PathBuf, dst_parent: &PathBuf) -> std::io::Result<PathBuf> {
    // Extract the directory's final name (e.g. "my_folder")
    let dir_name = src_dir.file_name().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Source directory has no file name")
    })?;

    // Construct the destination path ("/new/place/my_folder")
    let dst_path = dst_parent.join(dir_name);

    // Attempt to move (rename) the directory
    fs::rename(&src_dir, &dst_path)?;

    // Return the new location if successful
    Ok(dst_path)
}

static FUNCTION_NAMES: &[&str] = &[
    "_to_u8", "_to_u16", "_to_u32", "_to_u64", "_to_u128", "_to_usize",
    "_to_i8", "_to_i16", "_to_i32", "_to_i64", "_to_i128", "_to_isize",
    "_to_f32", "_to_f64", "_to_char", "_to_bool", "_to_str",
    "_unwrap_option", "_unwrap_result",
];

fn surf_clean_leftover_converters(harness: &str) -> String {
    // Try AST-based removal first for precise results
    match surf_remove_functions_via_ast(harness, FUNCTION_NAMES) {
        Ok(cleaned) => cleaned,
        Err(_) => {
            // Fallback to line-based removal if AST parsing fails
            surf_remove_functions_via_line_scan(harness, FUNCTION_NAMES)
        }
    }
}

// AST-based removal (works on valid Rust code)
fn surf_remove_functions_via_ast(source: &str, names: &[&str]) -> Result<String, syn::Error> {
    let line_offsets = surf_build_line_offsets(source);
    let ast = parse_file(source)?; // Proper error propagation
    
    let mut removals = Vec::new();
    for item in ast.items {
        if let Item::Fn(fn_item) = item {
            let ident = fn_item.sig.ident.to_string();
            if names.contains(&ident.as_str()) {
                let span = fn_item.span();
                let start = span.start();
                let end = span.end();
                
                let start_byte = surf_position_to_offset(&line_offsets, start);
                let end_byte = surf_position_to_offset(&line_offsets, end);
                
                removals.push((start_byte, end_byte));
            }
        }
    }

    // Remove in reverse order to preserve offsets
    removals.sort_by_key(|k| std::cmp::Reverse(k.0));
    
    let mut result = source.to_string();
    for (start, end) in removals {
        let end = end.min(result.len());
        let start = start.min(end);
        result.replace_range(start..end, "");
    }

    Ok(result.trim().to_string())
}

// Line-based removal (works on invalid Rust code)
fn surf_remove_functions_via_line_scan(source: &str, names: &[&str]) -> String {
    let mut lines: Vec<&str> = source.lines().collect();
    let mut to_remove = Vec::new();
    let names_set: std::collections::HashSet<_> = names.iter().copied().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with("fn ") {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let possible_name = parts[1];
                let name = possible_name.split(&['(', '<'][..]).next().unwrap_or("");
                if names_set.contains(name) {
                    // Found function to remove, now find its body end
                    let start_line = i;
                    let mut brace_balance = 0;
                    let mut in_body = false;
                    let mut end_line = start_line;

                    for j in start_line..lines.len() {
                        let line = lines[j];
                        for c in line.chars() {
                            if c == '{' {
                                brace_balance += 1;
                                in_body = true;
                            } else if c == '}' {
                                brace_balance -= 1;
                            }
                        }

                        if in_body && brace_balance == 0 {
                            end_line = j;
                            break;
                        }
                    }

                    to_remove.push((start_line, end_line));
                    i = end_line; // Skip processed lines
                }
            }
        }
        i += 1;
    }

    // Remove ranges in reverse order
    to_remove.sort_by_key(|&(s, _)| std::cmp::Reverse(s));
    for (start, end) in to_remove {
        if start <= end && end < lines.len() {
            lines.drain(start..=end);
        }
    }

    lines.join("\n")
}

// Existing helper functions unchanged
fn surf_build_line_offsets(source: &str) -> Vec<usize> {
    let mut offsets = vec![0];
    let mut pos = 0;
    
    for c in source.chars() {
        if c == '\n' {
            pos += 1;
            offsets.push(pos);
        } else if c == '\r' {
            if pos < source.len() && source.as_bytes()[pos + 1] == b'\n' {
                pos += 2;
                offsets.push(pos);
            } else {
                pos += 1;
                offsets.push(pos);
            }
        } else {
            pos += c.len_utf8();
        }
    }
    offsets
}

fn surf_position_to_offset(offsets: &[usize], pos: LineColumn) -> usize {
    let line = pos.line - 1;
    let col = pos.column;
    
    if line >= offsets.len() {
        return offsets.last().copied().unwrap_or(0);
    }
    
    offsets[line] + col
}

fn surf_create_improved_llm_only_target_dirs(fuzz_target_dir: &Path, improved_target_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let project_dir = PathBuf::from(&SURF_WORKING_PATH.clone());    
    
    let new_dir = project_dir.join(format!("deepSURF/fuzz/llm-only/compilable/{}", improved_target_name));
    if new_dir.exists() {
        fs::remove_dir_all(&new_dir)?;
    }

    fs::create_dir_all(&new_dir)?;
    fs_extra::dir::copy(
        fuzz_target_dir,
        &new_dir,
        &fs_extra::dir::CopyOptions::new().overwrite(true).content_only(true),
    )?;

    Ok(new_dir)
}

fn surf_create_improved_target_dirs(fuzz_target_dir: &Path, improved_target_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let project_dir = PathBuf::from(&SURF_WORKING_PATH.clone());    
    
    let new_dir = project_dir.join(format!("deepSURF/fuzz/llm/compilable/{}", improved_target_name));
    if new_dir.exists() {
        fs::remove_dir_all(&new_dir)?;
    }

    fs::create_dir_all(&new_dir)?;
    fs_extra::dir::copy(
        fuzz_target_dir,
        &new_dir,
        &fs_extra::dir::CopyOptions::new().overwrite(true).content_only(true),
    )?;

    Ok(new_dir)
}

fn surf_find_harness_file(project_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let src_dir = project_dir.join("src");
    if !src_dir.is_dir() {
        return Err("`src` directory not found in the project".into());
    }

    let mut harness_files = fs::read_dir(src_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map(|ext| ext == "rs").unwrap_or(false))
        .collect::<Vec<_>>();

    if harness_files.len() != 1 {
        return Err("Expected exactly one .rs file in the `src` directory".into());
    }

    Ok(harness_files.pop().unwrap().path())
}

fn surf_run_llm_fb_cargo_check(project_dir: &PathBuf) -> Result<HashSet<String>, Output> {
    let output = Command::new("cargo")
        .arg("+rustc_surf")
        .arg("afl")
        .arg("check")
        .env("RUSTFLAGS", "-Awarnings -Zub-checks=no")
        .env("SURF_RECORD_URAPIS", "1")
        .env("SURF_WORKING_PATH", project_dir)
        .current_dir(project_dir)
        .output()
        .expect("Failed to run `cargo check`");

    // 1) If cargo returns a non-zero exit code, treat that as an error immediately
    if !output.status.success() {
        //println!("HERE");
        return Err(output);
    }

    // 2) Convert stdout to a String so we can search for bracket markers
    let stdout_string = String::from_utf8_lossy(&output.stdout);

    const BEGIN_MARKER: &str = "---BEGIN-CALLED-URAPIS---";
    const END_MARKER: &str   = "---END-CALLED-URAPIS---";

    let mut collecting = false;
    let mut json_buffer = String::new();

    for line in stdout_string.lines() {
        let trimmed = line.trim();

        if trimmed == BEGIN_MARKER {
            collecting = true;
            continue;
        }
        if trimmed == END_MARKER {
            collecting = false;
            break;
        }
        if collecting {
            json_buffer.push_str(line);
        }
    }

    if json_buffer.is_empty() {
        return Ok(HashSet::new());
    }

    let parsed_set = match serde_json::from_str::<HashSet<String>>(&json_buffer) {
        Ok(s) => s,
        Err(_) => {
            return Err(output);
        }
    };
    Ok(parsed_set)
}

fn surf_extract_number_and_double(s: &str) -> Option<u32> {
    // Create a regex pattern that matches:
    //   if data.len() < NUMBER {return;}
    // The pattern captures the NUMBER as a group.
    let re = Regex::new(r"if\s+data\.len\(\)\s*<\s*(\d+)\s*\{\s*return;\s*\}").unwrap();
    
    // Find the first match and extract the first capturing group.
    let caps = re.captures(s)?;
    let num_match = caps.get(1)?;
    let num = num_match.as_str().parse::<u32>().ok()?;
    let doubled = num * 2;
    Some(doubled)
}

fn surf_replace_fuzzer_input_len(project_dir: &PathBuf, fuzzer_slice_len: u32){
    //println!("---Populating fuzzer len file.");
    let fuzzer_input_len_path = format!("{}/len", project_dir.to_string_lossy().to_string());
    let mut file = File::create(fuzzer_input_len_path).unwrap();
    file.write_all(fuzzer_slice_len.to_string().as_bytes()).expect("Unable to write to input0 file");
    file.flush().expect("Unable to flush to input0 file");
}

fn surf_get_retry_system_prompt_llm_only() -> String {
    let mut prompt = String::new();

    // Role
    prompt.push_str(
        "You are a Rust code-generation assistant. \n\
        I define a URAPI, as a safe API of a Rust library that can reach unsafe code and potentially expose memory safety vulnerabilities.\n\
        I will provide you a Rust harness that stresses a targeting URAPI.\n\
        This harness CANNOT compile. I will provide you the compiler errors and additional supportive input.\n\
        Your GOAL is to fix the errors of the given harness, WITHOUT VIOLATING any of the DIRECTIONS given bellow (see II and III).\n\n\
        To achieve the goal, I will provide you the following:\n\
        (I) Descriptions of the INPUT files.\n\
        (II) Directions on WHAT TO DO and WHAT NOT TO DO to fix the harness.\n\
        (III) Description of the desired OUTPUT.\n\
        (IV) INPUT files (given in the user prompt).\n\n"
    );

    // I) INPUT FILES
    prompt.push_str("I) INPUT files description:\n");
    prompt.push_str(
        "1. A Rust fuzz harness targeting a specific URAPI.\n\
         2. Error message when I try `cargo check` the harness.\n\
         3. Additional documentation of the input library.\n\n"
    );
   
    // II) DIRECTIONS
    prompt.push_str("(II) You MUST follow all the 14 directions bellow. They are all important. Feel free to do more if it furthers the goal:\n");
    prompt.push_str(
        "\t1. For structs and enums, use multiple constructors from those available:\n\
        \t a) Dynamically choose which constructor to call based on fuzzer input.\n\
        \t b) If possible select a set of diverse constructors from the provided ones.\n\
        \t2. Do NOT return the existing type conversion functions such as `_to_u8`, `_to_usize`, `_to_str` etc.:\n\
        \t a) They supposed to remain unchanged.\n\
        \t b) Do NOT include their definitions in the output I'll manually add them later before I compile the harness.\n\
        \t3. Custom function and method implementations:\n\
        \t a) For arguments that expect closures, generate custom functions with the appropriate signature and sustitute with pointers to these functions.\n\
        \t b) Custom implementations of functions and methods (including trait functions) should be driven by the fuzzer's input. That is, their return values should be derived from the fuzzer's input, appropriately crafted to match the return type of the function or method.\n\
        \t c) Based on fuzzer's input allow custom functions and methods (including trait functions) to panic (used to expose panic safety bugs).\n\
        \t4. Create more complex API call sequences:\n\
        \t a) Focus on invoking the target URAPI.\n\
        \t b) **Before** and **after** the URAPI call, invoke as many additional syntactically and semantically relevant URAPIs/APIs as possible.\n\
        \t c) Use static analysis and documentation to discover available URAPIs/APIs.\n\
        \t d) If feasible, call APIs/URAPIs multiple times based on fuzzer input.\n\
        \t e) A possible structure: an outer loop for the number of operations, an inner match to dispatch each operation (including the targeting URAPI).\n\
        \t5. The fuzzer's input is via `data`.\n\
        \t a) GLOBAL_DATA extends `data`'s lifetime and is used to access fuzzer's data from all the functions of the harness.\n\
        \t b) Do not change GLOBAL_DATA logic.\n\
        \t6. Incorporate the fuzzer's random input to steer harness logic:\n\
        \t a) Use it to build arguments and decide iteration counts.\n\
        \t7. Bounds checks and modulo usage:\n\
        \t a) Keep the initial check on `data.len()`. Set data.len() bounds to be sufficiently large to accommodate all harness-generated data types while avoiding unnecessary excess. Also keep in mind that `global_data.first_half.len() + global_data.second_half.len() = data.len()`.\n\
        \t b) Do NOT exceed `data.len()` (and similarly GLOBAL_DATA) when converting bytes.\n\
        \t c) Vector sizes, should be bounded by modulo 65.\n\
        \t d) Every other URAPI and API that gets numeric arguments, should get these arguments directly from the fuzzer's input, AVOIDING any bounds checking or modulo logic, even if that leads exceeding its length.
        \t8. Force reference dereference.\n\
        \t a) You must dereference any references, when possible.\n\
        \t b) References can be returned from APIs, URAPIs or can occur when indexing collections.\n\
        \t c) You can use `println!` to access the point-to data of a reference. In this case, you may have to add `#[derive(Debug)]` as needed.\n\
        \t d) Use the static analysis and documentation input I provide you to see what APIs/URAPIs return references.\n\
        \t9. Handling of **unsafe trait** impls:\n\
        \t a) Custom implementations of **unsafe traits** ARE NOT ALLOWED in the resulting harness.\n\
        \t b) However you are allowed to use types that implement unsafe traits and these usafe trait implementations are part of the library.\n\
        \t10. Handling generic arguments and associated trait types:\n\
        \t a) Generate custom types to substitute generic arguments and associated types e.g. `struct CustomType0(String)`.\n\
        \t11. Handling of **safe trait** impls:\n\
        \t a) If a generic argument requires implementation of safe traits, then implement these traits for the custom type that will substitute the generic argument.\n\
        \t b) The custom implementations of trait functions should follow the DIRECTION 3.\n\
        \t12. Defining new array types:\n\
        \t a) If new arrays are introduced, use lengths larger than 10.\n\
        \t13. The use of unsafe code IS PROHIBITED.\n\
        \t a) Do not call unsafe functions even if they are part of the library's API.\n\
        \t b) Do not insert unsafe blocks.\n\
        \t c) The resulting harness should maintain the attribute #![forbid(unsafe_code)] at the beginning.\n\
        \t14. Imports/Externs & unused variables:\n\
        \t a) Do NOT alter existing imports/externs from the sample harness.\n\
        \t b) Only import/extern additional traits if absolutely necessary.\n\
        \t c) Do NOT include comments, dead code, or unused variables.\n\n"
    );

    // III) OUTPUT DESCRIPTION
    prompt.push_str("(III) Directions for the OUTPUT:\n");
    prompt.push_str(
        "\t1. Return ONLY the fixed harness as compilable Rust source code:\n\
        \t a) Do NOT violate any of the 14 directions (see II).
        \t b) Do NOT include type converters.\n\
        \t c) Do NOT return explanations of the changes you made.\n\
        \t d) Do NOT return markers of where the fixed harness starts and ends.\n\
        \t e) Do NOT insert `#![no_main]` attribute.\n\
        \t f) Do NOT include comments.\n"
    );
    prompt
}

fn surf_get_retry_system_prompt() -> String {
    let mut prompt = String::new();

    // Role
    prompt.push_str(
        "You are a Rust code-generation assistant. \n\
        I define a URAPI, as a safe API of a Rust library that can reach unsafe code and potentially expose memory safety vulnerabilities.\n\
        I will provide you a Rust harness that stresses a targeting URAPI.\n\
        This harness CANNOT compile. I will provide you the compiler errors and additional supportive input.\n\
        Your GOAL is to fix the errors of the given harness, WITHOUT VIOLATING any of the DIRECTIONS given bellow (see II and III).\n\n\
        To achieve the goal, I will provide you the following:\n\
        (I) Descriptions of the INPUT files.\n\
        (II) Directions on WHAT TO DO and WHAT NOT TO DO to fix the harness.\n\
        (III) Description of the desired OUTPUT.\n\
        (IV) INPUT files (given in the user prompt).\n\n"
    );

    // I) INPUT FILES
    prompt.push_str("I) INPUT files description:\n");
    prompt.push_str(
        "1. A Rust fuzz harness targeting a specific URAPI.\n\
         2. Error message when I try `cargo check` the harness.\n\
         3. JSON files from a static analysis of the library that contains this URAPI:\n\
         \t a) A mapping from complex types (structs/enums) to their constructor APIs.\n\
         \t b) The constructor APIs (after argument analysis).\n\
         \t c) The URAPIs of the input library (after argument analysis).\n\
         4. Additional documentation of the input library.\n\n"
    );
   
    // II) DIRECTIONS
    prompt.push_str("(II) You MUST follow all the 14 directions bellow. They are all important. Feel free to do more if it furthers the goal:\n");
    prompt.push_str(
        "\t1. For structs and enums, use multiple constructors from those available:\n\
        \t a) Dynamically choose which constructor to call based on fuzzer input.\n\
        \t b) If possible select a set of diverse constructors from the provided ones.\n\
        \t2. Do NOT return the existing type conversion functions such as `_to_u8`, `_to_usize`, `_to_str` etc.:\n\
        \t a) They supposed to remain unchanged.\n\
        \t b) Do NOT include their definitions in the output I'll manually add them later before I compile the harness.\n\
        \t3. Preserve intentional panic logic:\n\
        \t a) Do NOT alter or remove code that deliberately causes panics (used to expose panic safety bugs).\n\
        \t b) Closures, custom functions and trait functions should maintain their panic logic.\n\
        \t4. Create more complex API call sequences:\n\
        \t a) Focus on invoking the target URAPI.\n\
        \t b) **Before** and **after** the URAPI call, invoke as many additional syntactically and semantically relevant URAPIs/APIs as possible.\n\
        \t c) Use static analysis and documentation to discover available URAPIs/APIs.\n\
        \t d) If feasible, call APIs/URAPIs multiple times based on fuzzer input.\n\
        \t e) A possible structure: an outer loop for the number of operations, an inner match to dispatch each operation (including the targeting URAPI).\n\
        \t5. The fuzzer's input is via `data`.\n\
        \t a) GLOBAL_DATA extends `data`'s lifetime and is used to access fuzzer's data from all the functions of the harness.\n\
        \t b) Do not change GLOBAL_DATA logic.\n\
        \t6. Incorporate the fuzzer's random input to steer harness logic:\n\
        \t a) Use it to build arguments and decide iteration counts.\n\
        \t7. Bounds checks and modulo usage:\n\
        \t a) Keep the initial check on `data.len()`. Set data.len() bounds to be sufficiently large to accommodate all harness-generated data types while avoiding unnecessary excess. Also keep in mind that `global_data.first_half.len() + global_data.second_half.len() = data.len()`.\n\
        \t b) Do NOT exceed `data.len()` (and similarly GLOBAL_DATA) when converting bytes.\n\
        \t c) Vector sizes, should be bounded by modulo 65.\n\
        \t d) Every other URAPI and API that gets numeric arguments, should get these arguments directly from the fuzzer's input, AVOIDING any bounds checking or modulo logic, even if that leads exceeding its length.
        \t8. Force reference dereference.\n\
        \t a) You must dereference any references, when possible.\n\
        \t b) References can be returned from APIs, URAPIs or can occur when indexing collections.\n\
        \t c) You can use `println!` to access the point-to data of a reference. In this case, you may have to add `#[derive(Debug)]` as needed.\n\
        \t d) Use the static analysis and documentation input I provide you to see what APIs/URAPIs return references.\n\
        \t9. Handling of **unsafe trait** impls:\n\
        \t a) Custom implementations of **unsafe traits** ARE NOT ALLOWED in the improved harness and you MUST REMOVE THEM.\n\
        \t b) Replace all custom types that come with custom unsafe trait implementations with library data types that already implement these unsafe traits.\n\
        \t c) Search in the documentation and static analysis output I provide, to find library data type candidates.\n\
        \t10. Handling of **safe trait** impls:\n\
        \t a) Preserve any existing custom implementations for **safe traits**.\n\
        \t b) The improved harness must still use custom trait implementations where provided (for safe traits).\n\
        \t11. Handling custom types:\n\
        \t a) If not implementing unsafe traits, prefer to use existing custom types; extend or wrap them if needed.\n\
        \t12. Defining new array types:\n\
        \t a) If new arrays are introduced, use lengths larger than 10.\n\
        \t13. The use of unsafe code IS PROHIBITED.\n\
        \t a) Do not call unsafe functions even if they are part of the library's API.\n\
        \t b) Do not insert unsafe blocks.\n\
        \t c) The improved harness should maintain the attribute #![forbid(unsafe_code)] at the beginning.\n\
        \t14. Imports/Externs & unused variables:\n\
        \t a) Do NOT alter existing imports/externs from the sample harness.\n\
        \t b) Only import/extern additional traits if absolutely necessary.\n\
        \t c) Do NOT include comments, dead code, or unused variables.\n\n"
    );

    // III) OUTPUT DESCRIPTION
    prompt.push_str("(III) Directions for the OUTPUT:\n");
    prompt.push_str(
        "\t1. Return ONLY the fixed harness as compilable Rust source code:\n\
        \t a) Do NOT violate any of the 14 directions (see II).
        \t b) Do NOT include type converters.\n\
        \t c) Do NOT return explanations of the changes you made.\n\
        \t d) Do NOT return markers of where the fixed harness starts and ends.\n\
        \t e) Do NOT insert `#![no_main]` attribute.\n\
        \t f) Do NOT include comments.\n"
    );
    prompt
}

fn surf_build_retry_prompt_llm_only(
    harness_content: &str,
    error_message: &str,
    docs: &str,
) -> String {
    // IV) INPUT files
    format!(
        "(V) INPUT files:\n\
        === HARNESS START ===\n{harness_content}\n=== HARNESS END ===\n\n\
        === ERROR MESSAGE START ===\n{error_message}\n=== ERROR MESSAGE END ===\n\n\
        === DOCUMENTATION START ===\n{docs}\n=== DOCUMENTATION END ===\n\n\
        Now fixed the errors of the provided harness.\n",
    )
}

fn surf_build_retry_prompt(
    harness_content: &str,
    error_message: &str,
    analysis_jsons: &str,
    docs: &str,
) -> String {
    // IV) INPUT files
    format!(
        "(V) INPUT files:\n\
        === HARNESS START ===\n{harness_content}\n=== HARNESS END ===\n\n\
        === ERROR MESSAGE START ===\n{error_message}\n=== ERROR MESSAGE END ===\n\n\
        === STATIC ANALYSIS START ===\n{analysis_jsons}\n=== STATIC ANALYSIS END ===\n\n\
        === DOCUMENTATION START ===\n{docs}\n=== DOCUMENTATION END ===\n\n\
        Now fixed the errors of the provided harness.\n",
    )
}

/* -------------------------------------------------------------------------
                        DEPENDENCY TREE FUNCTIONS
--------------------------------------------------------------------------*/

fn surf_build_dep_tree(urapi_def_id: &String, surf_urapi: &SurfURAPI, urapi_dep_tree: &mut SurfTree){
    // Construct AO Tree
    surf_get_dep_tree(urapi_def_id, surf_urapi, urapi_dep_tree);
    //println!("{:?}", Dot::with_config(&urapi_dep_tree.tree, &[Config::EdgeNoLabel]));
    // println!("Building OA-Tree --> Ok!");

    // Migrate the hidden complex enum variants
    surf_migrate_complex_variants(urapi_dep_tree);
    // println!("Migrating Complex Variants --> Ok!");

    // Prune Empty Simple Variants Nodes
    surf_prune_empty_simple_variants(urapi_dep_tree);
    // println!("Pruning Empty Simple Variants --> Ok!");

    // Mark Empty Dyn Traits (as non-supported for now)
    surf_mark_unsupported_dyntraits(urapi_dep_tree);
    // println!("Marking Non-Supported Dyn Traits --> Ok!");

    // Mark Complex Enums that do not have constructors
    surf_mark_unsupported_complex_variants(urapi_dep_tree);
    // println!("Marking Non-Supported Complex Variants --> Ok!");

    // Mark Constructor Paths that Exceed the Max Lenght
    surf_mark_too_deep_paths(urapi_dep_tree);
    // println!("Marking Too Deep Paths --> Ok!");

    // Prune Implicit Generics that Are Unsupported
    surf_empty_unresolved_implicit_generics(urapi_dep_tree);
    //println!("{:?}", Dot::with_config(&urapi_dep_tree.tree, &[Config::EdgeNoLabel]));
}

fn surf_build_optz_fuzz_trees<'a>(urapi_def_id: &'a str, urapi_dep_tree: SurfTree, seed: u64) -> Vec<FuzzTree>{
    // Produce the Fuzz Trees (AO Decomposition)
    let mut urapi_fuzz_trees = surf_build_filtered_fuzz_trees_select_with_ids(urapi_def_id, urapi_dep_tree, MAX_TARGETS_TO_CHECK, seed);
    //println!("Producting Fuzz Trees --> Ok!");
   
    // Add the Api Output here
    surf_add_urapis_output_node(&mut urapi_fuzz_trees);

    // Add Unwrapping Nodes Before Constructors
    surf_add_unwrapping_nodes(&mut urapi_fuzz_trees);
    // println!("Unwrapping Output Nodes --> Ok");

    urapi_fuzz_trees
}

fn surf_build_fuzz_trees<'a>(urapi_def_id: &'a str, urapi_dep_tree: SurfTree) -> Vec<FuzzTree>{
    // Produce the Fuzz Trees (AO Decomposition)
    let mut urapi_fuzz_trees = surf_get_fuzz_trees(urapi_dep_tree.root.unwrap(), &urapi_dep_tree.tree);
    
    //println!("Producting Fuzz Trees --> Ok!");
   
    // Assign Ids to Fuzz Trees
    surf_assign_fuzz_trees_ids(urapi_def_id, &mut urapi_fuzz_trees);

    // Filter Out Remaining Non-Supported Fuzz Trees
    surf_filter_fuzz_trees(&mut urapi_fuzz_trees);
    // println!("Filtering Out Remaining Non-Supported Fuzz Trees --> Ok!");

    // Add the Api Output here
    surf_add_urapis_output_node(&mut urapi_fuzz_trees);

    // Add Unwrapping Nodes Before Constructors
    surf_add_unwrapping_nodes(&mut urapi_fuzz_trees);
    // println!("Unwrapping Output Nodes --> Ok");

    urapi_fuzz_trees
}

fn generate_id<K: Hash, V: Hash>(key: &K, value: &V) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);   // Hash the key
    value.hash(&mut hasher); // Hash the value
    hasher.finish() // Return the hash as a unique ID
}

fn surf_assign_fuzz_trees_ids<'a>(urapi_def_id: &'a str, urapi_fuzz_trees: &mut Vec<FuzzTree>){
    let mut seq_num: u64 = 0;
    //println!("{:#?}", urapi_fuzz_trees.len());
    for urapi_fuzz_tree in urapi_fuzz_trees{
        //println!("{:?}", Dot::with_config(&urapi_fuzz_tree.tree, &[Config::EdgeNoLabel]));
        urapi_fuzz_tree.id = generate_id(&urapi_def_id.to_string(), &seq_num);
        seq_num += 1;
    }
}

fn surf_get_urapi_output_node(
    urapi_node: NodeIndex,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
) -> Option<NodeIndex>
{
    for urapi_neighbor in fuzz_tree.neighbors(urapi_node){
        if let SurfTreeNode::ApiOutput = &fuzz_tree[urapi_neighbor]{
            return Some(urapi_neighbor)
        }
    }
    None
}

fn surf_add_urapis_output_node(urapi_fuzz_trees: &mut Vec::<FuzzTree>){
    for surf_fuzz_tree in urapi_fuzz_trees.iter_mut(){
        let fuzz_tree_root = surf_fuzz_tree.root.unwrap();
        //let fuzz_tree = &mut surf_fuzz_tree.tree;
        let urapi_node_data = surf_fuzz_tree.tree[fuzz_tree_root].clone();
        if let SurfTreeNode::Api { output, .. } = urapi_node_data{
            if let Some(api_output_node) = surf_get_urapi_output_node(fuzz_tree_root, &surf_fuzz_tree.tree){
                if let SurfTreeNode::ApiOutput = &surf_fuzz_tree.tree[api_output_node]{
                    if let Some(output_arg) = &output{
                        let mut seen_cmplx_types = HashSet::<String>::new();
                        let output_arg_node = surf_create_and_add_node(&output_arg, &mut surf_fuzz_tree.tree, &mut seen_cmplx_types);
                        surf_fuzz_tree.tree.add_edge(api_output_node, output_arg_node, ());
                    }
                }
            }
        }
        //println!("{:?}", Dot::with_config(&surf_fuzz_tree.tree, &[Config::EdgeNoLabel]));
    }
}

fn surf_get_dep_tree(urapi_def_id: &String, surf_urapi: &SurfURAPI, urapi_dep_tree: &mut SurfTree){
    let dep_tree_root_node = urapi_dep_tree
                                        .tree
                                        .add_node(SurfTreeNode::Api{
                                                    def_id: urapi_def_id.clone(),
                                                    name: surf_urapi.name.clone(),
                                                    full_name: surf_urapi.full_name.clone(),
                                                    crate_name: surf_urapi.crate_name.clone(),
                                                    output: surf_urapi.output.clone(),
                                                    has_self: *surf_urapi.flags.get("has_self").unwrap(),
                                                    is_drop: *surf_urapi.flags.get("is_drop").unwrap(),
                                                    is_display: *surf_urapi.flags.get("is_display").unwrap(),
                                                    is_debug: *surf_urapi.flags.get("is_debug").unwrap(),
    });
    // Store the root node's index
    urapi_dep_tree.root = Some(dep_tree_root_node);
    // Build the URAPI AO-TREE

    // First go to implicit generics (DFS)
    let implicit_generics_node = urapi_dep_tree.tree.add_node(SurfTreeNode::ApiImplicitGenerics);
    urapi_dep_tree.tree.add_edge(dep_tree_root_node, implicit_generics_node, ());
   
    for implicit_generic in surf_urapi.implicit_generics.iter(){
        let mut seen_cmplx_types = HashSet::<String>::new();
        let implicit_generic_node = surf_create_and_add_node(implicit_generic, &mut urapi_dep_tree.tree, &mut seen_cmplx_types);
        urapi_dep_tree.tree.add_edge(implicit_generics_node, implicit_generic_node, ());
    }

    // After add inputs
    let inputs_node = urapi_dep_tree.tree.add_node(SurfTreeNode::ApiInputs);
    urapi_dep_tree.tree.add_edge(dep_tree_root_node, inputs_node, ());
    //println!("HERE");
    for arg in surf_urapi.inputs.iter(){
        let mut seen_cmplx_types = HashSet::<String>::new();
        let arg_node = surf_create_and_add_node(arg, &mut urapi_dep_tree.tree, &mut seen_cmplx_types);
        urapi_dep_tree.tree.add_edge(inputs_node, arg_node, ());
    }

    let output_node = urapi_dep_tree.tree.add_node(SurfTreeNode::ApiOutput);
    urapi_dep_tree.tree.add_edge(dep_tree_root_node, output_node, ());
    //println!("{:?}", Dot::with_config(&urapi_dep_tree.tree, &[Config::EdgeNoLabel]));
}

fn surf_migrate_complex_variants(urapi_dep_tree: &mut SurfTree) {
    let surf_tree_root = urapi_dep_tree.root.unwrap();
    let surf_tree_graph = &mut urapi_dep_tree.tree;
    let mut dfs = Dfs::new(&*surf_tree_graph, surf_tree_root);

    while let Some(next_node) = dfs.next(&*surf_tree_graph) {
        let current_node_data = &surf_tree_graph[next_node];

        // ── skip whole sub-tree under ApiOutput or FnInputs ────────────
        if matches!(current_node_data, SurfTreeNode::ApiOutput | SurfTreeNode::FnInputs) {
            // 1. collect the neighbours that were *just* pushed
            let neigh_to_drop: Vec<_> = surf_tree_graph.neighbors(next_node).collect();
            // 2. keep every stack entry that is **not** one of them
            dfs.stack.retain(|n| !neigh_to_drop.contains(n));
            continue;                     // don’t touch this node further
        }

        if let SurfTreeNode::Enum{name, ..} = current_node_data{
            let simple_variants_node = surf_tree_graph.neighbors(next_node)
                                                    .filter(|variant_type| matches!(surf_tree_graph[*variant_type], SurfTreeNode::SimpleVariants))
                                                    .next()
                                                    .unwrap();
            let mut nodes_to_migrate = Vec::<NodeIndex>::new();
            for simple_variant in surf_tree_graph.neighbors(simple_variants_node){
                if surf_is_complex_variant(simple_variant, &surf_tree_graph){
                    nodes_to_migrate.push(simple_variant);
                }
            }
            let complex_variants_node = surf_tree_graph.neighbors(next_node)
                                                    .filter(|variant_type| matches!(surf_tree_graph[*variant_type], SurfTreeNode::ComplexVariants))
                                                    .next()
                                                    .unwrap();
            for node_to_migrate in nodes_to_migrate{
                let edge_to_remove = surf_tree_graph.find_edge(simple_variants_node, node_to_migrate).unwrap();
                surf_tree_graph.remove_edge(edge_to_remove);
                surf_tree_graph.add_edge(complex_variants_node, node_to_migrate, ());
            }
        }
    }
}

fn surf_mark_unsupported_complex_variants(urapi_dep_tree: &mut SurfTree){
    let surf_tree_root = urapi_dep_tree.root.unwrap();
    let surf_tree_graph = &mut urapi_dep_tree.tree;
    let mut dfs = Dfs::new(&*surf_tree_graph, surf_tree_root);
    while let Some(next_node) = dfs.next(&*surf_tree_graph) {
        let current_node_data = &surf_tree_graph[next_node];
        if let SurfTreeNode::ComplexVariants = current_node_data{
            if surf_tree_graph.neighbors(next_node).next().is_none(){
                let todo_node = surf_tree_graph.add_node(SurfTreeNode::Todo(String::from("No Constructors")));
                surf_tree_graph.add_edge(next_node, todo_node, ());
            }
        }
    }
}

fn surf_mark_too_deep_paths(urapi_dep_tree: &mut SurfTree){
    let mut edges_to_prune = HashSet::<EdgeIndex>::default();
    surf_get_too_deep_paths(urapi_dep_tree.root.unwrap(), &urapi_dep_tree.tree, 1, &mut edges_to_prune);
    for edge in edges_to_prune{
        let (source, _) = urapi_dep_tree.tree.edge_endpoints(edge).expect("Edge does not exist");
        urapi_dep_tree.tree.remove_edge(edge);
        let new_next = urapi_dep_tree.tree.add_node(SurfTreeNode::Todo(String::from("MAX PATH LIMIT REACHED")));
        urapi_dep_tree.tree.add_edge(source, new_next, ());
    }
}

fn surf_get_too_deep_paths(
    current_node: NodeIndex,
    ao_tree: &Graph<SurfTreeNode, ()>,
    running_count: usize,
    edges_to_prune: &mut HashSet<EdgeIndex>,
){
    let current_node_data = ao_tree[current_node].clone();
    let mut local_running_count = running_count;
    if ao_tree.neighbors(current_node).next().is_none(){
        return;
    }
    else if surf_is_or_node(&current_node_data){
        if let SurfTreeNode::Struct { .. } 
        | SurfTreeNode::DynTrait
        | SurfTreeNode::ComplexVariants = current_node_data{
            if local_running_count == MAX_PATH{
                let next_node_opt = ao_tree.neighbors(current_node).next();
                if let Some(next_node) = next_node_opt{
                    match &ao_tree[next_node]{
                        SurfTreeNode::Todo(_) => {return},
                        _ => {
                                edges_to_prune.insert(ao_tree.find_edge(current_node, next_node).unwrap());
                                return
                            },
                    }
                }
            }
            local_running_count += 1;
        }
        for neighbor in ao_tree.neighbors(current_node){
            surf_get_too_deep_paths(neighbor, ao_tree, local_running_count, edges_to_prune);
        }
    }
    else {
        if let SurfTreeNode::TraitFn { .. }
        | SurfTreeNode::Closure(_, _) = current_node_data{
            local_running_count = 1;
        }
        for neighbor in ao_tree.neighbors(current_node){
            surf_get_too_deep_paths(neighbor, ao_tree, local_running_count, edges_to_prune);
        }
    }
}

fn surf_mark_unsupported_dyntraits(urapi_dep_tree: &mut SurfTree){
    let surf_tree_root = urapi_dep_tree.root.unwrap();
    let surf_tree_graph = &mut urapi_dep_tree.tree;
    let mut dfs = Dfs::new(&*surf_tree_graph, surf_tree_root);
    while let Some(next_node) = dfs.next(&*surf_tree_graph) {
        let current_node_data = &surf_tree_graph[next_node];
        if let SurfTreeNode::DynTrait = current_node_data{
            if surf_tree_graph.neighbors(next_node).next().is_none(){
                let todo_node = surf_tree_graph.add_node(SurfTreeNode::Todo(String::from("No Candidates")));
                surf_tree_graph.add_edge(next_node, todo_node, ());
            }
        }
    }
}

fn surf_prune_empty_simple_variants(urapi_dep_tree: &mut SurfTree){
    let surf_tree_root = urapi_dep_tree.root.unwrap();
    let surf_tree_graph = &mut urapi_dep_tree.tree;
    let mut dfs = Dfs::new(&*surf_tree_graph, surf_tree_root);
    let mut nodes_to_remove = HashSet::<NodeIndex>::new();
    let mut edges_to_remove = HashSet::<EdgeIndex>::new();
    while let Some(next_node) = dfs.next(&*surf_tree_graph) {
        let current_node_data = &surf_tree_graph[next_node];
        if let SurfTreeNode::SimpleVariants = current_node_data{
            if surf_tree_graph.neighbors(next_node).next().is_none(){
                let parent_node = surf_tree_graph.edges_directed(next_node, Direction::Incoming)
                                                            .next()
                                                            .map(|edge| edge.source()).unwrap();
                edges_to_remove.insert(surf_tree_graph.find_edge(parent_node, next_node).unwrap());
                nodes_to_remove.insert(next_node);
            }
        }
    }
    for edge in edges_to_remove{
        surf_tree_graph.remove_edge(edge);
    }
    for node in nodes_to_remove{
        surf_tree_graph.remove_node(node);
    }
}

fn surf_empty_unresolved_implicit_generics(urapi_dep_tree: &mut SurfTree){
    let surf_tree_root = urapi_dep_tree.root.unwrap();
    let surf_tree_graph = &mut urapi_dep_tree.tree;

    let mut dfs = Dfs::new(&*surf_tree_graph, surf_tree_root);
    let mut implicit_nodes_to_inspect = HashSet::<NodeIndex>::new();
    while let Some(next_node) = dfs.next(&*surf_tree_graph) {
        let current_node_data = &surf_tree_graph[next_node];
        if let SurfTreeNode::ApiImplicitGenerics | SurfTreeNode::ConstructorImplicitGenerics = current_node_data{
            implicit_nodes_to_inspect.insert(next_node);
        }
    }

    let mut nodes_to_empty = HashSet::<NodeIndex>::new();
    for node_to_inspect in implicit_nodes_to_inspect{
        dfs = Dfs::new(&*surf_tree_graph, node_to_inspect);
        while let Some(next_node) = dfs.next(&*surf_tree_graph) {
            let current_node_data = &surf_tree_graph[next_node];
            if surf_is_unsupported_node(current_node_data){
                nodes_to_empty.insert(node_to_inspect);
                break;
            }
        }
    }

    let mut edges_to_remove = HashSet::<EdgeIndex>::new();
    for node_to_empty in nodes_to_empty{
        for gen_implicit_neighbor in surf_tree_graph.neighbors(node_to_empty){
            edges_to_remove.insert(surf_tree_graph.find_edge(node_to_empty, gen_implicit_neighbor).unwrap());
        }
    }

    for edge_to_remove in edges_to_remove{
        surf_tree_graph.remove_edge(edge_to_remove);
    }
    
    
    
}


fn surf_filter_fuzz_trees(urapi_fuzz_trees: &mut Vec::<FuzzTree>){
    urapi_fuzz_trees.retain(|fuzz_tree| surf_is_supported(fuzz_tree));
}

fn surf_add_unwrapping_nodes(urapi_fuzz_trees: &mut Vec::<FuzzTree>){
    for surf_fuzz_tree in urapi_fuzz_trees.iter_mut(){
        let fuzz_tree_root = surf_fuzz_tree.root.unwrap();
        let fuzz_tree = &mut surf_fuzz_tree.tree;
        let mut dfs = Dfs::new(&*fuzz_tree, fuzz_tree_root);
        while let Some(next_node) = dfs.next(&*fuzz_tree) {
            let current_node_data = &fuzz_tree[next_node];
            match current_node_data{
                SurfTreeNode::Struct{..} 
                // | SurfTreeNode::Enum {..} is gonna be through complex variant
                | SurfTreeNode::ComplexVariants => { // we may need to remove cmplx variants
                    if let Some(constructor_node) = fuzz_tree.neighbors(next_node).next(){
                        let constructor_data = &fuzz_tree[constructor_node].clone();
                        if let SurfTreeNode::Constructor { output , cmplx_type_def_id, ..} = constructor_data{
                            if let Some(output) = output.clone() {
                                //let output = output.clone().unwrap();
                                let new_neighbor = surf_unwrap_output(
                                                                                    constructor_node,
                                                                                    &output,
                                                                                    &cmplx_type_def_id,
                                                                                    fuzz_tree);
                                fuzz_tree.add_edge(next_node, new_neighbor, ());
                                fuzz_tree.remove_edge(fuzz_tree.find_edge(next_node, constructor_node).unwrap());
                            }
                        }
                    }
                },
                _ => {},
            }
        }
    }
}

// Returns the last node in the unwrapping that is going to be the closest to the Complex Type node
fn surf_unwrap_output(
    output_node: NodeIndex,
    output: &SurfFnArg,
    cmplx_type_def_id: &str,
    fuzz_tree: &mut Graph<SurfTreeNode, ()>
) -> NodeIndex
{    
    match output{
        SurfFnArg::Option(inner_type) => {
            let from_option_node = fuzz_tree.add_node(SurfTreeNode::FromOption);
            fuzz_tree.add_edge(from_option_node, output_node, ());
            //output = &*inner_type;
            return surf_unwrap_output(from_option_node, &inner_type, cmplx_type_def_id, fuzz_tree);
            //prev_node = from_option_node;
        },
        SurfFnArg::Result{ok, err} => {
            let from_result_node = fuzz_tree.add_node(SurfTreeNode::FromResult(err.clone()));
            fuzz_tree.add_edge(from_result_node, output_node, ());
            return surf_unwrap_output(from_result_node, &ok, cmplx_type_def_id, fuzz_tree);
            //output_content = *ok;
            //prev_node = from_result_node;
        },
        SurfFnArg::Tuple(tuple_args) => {
            let mut curr_arg_count = 0;
            for tuple_arg in tuple_args{
                if surf_tuple_field_has_complex_type(tuple_arg, cmplx_type_def_id){
                    let from_tuple_node = fuzz_tree.add_node(SurfTreeNode::FromTuple{
                                                                                                        field_num: curr_arg_count
                                                                                                    });
                    fuzz_tree.add_edge(from_tuple_node, output_node, ());
                    return surf_unwrap_output(from_tuple_node, tuple_arg, cmplx_type_def_id, fuzz_tree);
                }
                curr_arg_count += 1;
            }
            return output_node;
        },
        _ => return output_node,
    }
}

fn surf_complex_types_cmp(canditate_def_id: &str, target_def_id: &str) -> bool{
    canditate_def_id == target_def_id
}

fn surf_tuple_field_has_complex_type(tuple_field_fn_arg: &SurfFnArg, cmplx_type_def_id: &str) -> bool{
    match tuple_field_fn_arg{
        SurfFnArg::Struct { def_id, .. }
        | SurfFnArg::Enum { def_id, .. } => {
            return surf_complex_types_cmp(def_id, cmplx_type_def_id);
        },
        SurfFnArg::Option(inner_type) => {
            return surf_tuple_field_has_complex_type(inner_type, cmplx_type_def_id);
        },
        SurfFnArg::Result{ok, ..} => {
            return surf_tuple_field_has_complex_type(ok, cmplx_type_def_id);
        },
        SurfFnArg::Tuple(tuple_args) => {
            for tuple_arg in tuple_args.iter(){
                if surf_tuple_field_has_complex_type(tuple_arg, cmplx_type_def_id){
                    return true;
                }
            }
        },
        _ => {},
    }
    false
}

fn surf_get_node_inputs_output(fn_node: NodeIndex, fuzz_tree: &Graph<SurfTreeNode, ()>) -> (Option<NodeIndex>, Option<NodeIndex>){
    let (mut inputs, mut output) = (None, None);
    for neighbor in fuzz_tree.neighbors(fn_node){
        let neighbor_data = &fuzz_tree[neighbor];
        if let SurfTreeNode::FnInputs = neighbor_data {
            inputs = Some(neighbor)
        }
        else if let SurfTreeNode::FnOutput = neighbor_data {
            output = Some(neighbor)
        }
    }
    (inputs, output)
}


fn surf_build_closure_sig(
    closure_name: String,
    closure_node: NodeIndex,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    generic_subst_map: &HashMap::<String, SubstType>,
    custom_structs: &HashMap<String, (String, String)>,
) -> (String, Option<String>)
{
    let (inputs_node_opt, output_node_opt) = surf_get_node_inputs_output(closure_node, fuzz_tree);
    let mut inputs_str = String::from("");
    let mut output_str = String::from("");
    let mut has_custom_arg = None;
    if let Some(output_node) = output_node_opt{
        output_str = surf_get_type_name_mono(fuzz_tree.neighbors(output_node).next(), fuzz_tree, generic_subst_map);
        output_str = format!(" -> {output_str}");
    }
    if let Some(inputs_node) = inputs_node_opt {
        let mut custom_inputs: usize = 0;
        let mut inputs = Vec::<String>::new();
        for input in fuzz_tree.neighbors(inputs_node){
            let input_type = surf_get_type_name_mono(Some(input), fuzz_tree, generic_subst_map);
            if let Some(inner_type) = surf_arg_is_custom_type(Some(input), fuzz_tree, generic_subst_map, custom_structs){
                if inner_type == "String"{
                    if has_custom_arg.is_none(){
                        has_custom_arg = Some(format!("str{custom_inputs}"))
                    }
                    inputs.push(format!("str{custom_inputs}: {input_type}"));
                }
                else{
                    if has_custom_arg.is_none(){
                        has_custom_arg = Some(format!("usz{custom_inputs}"))
                    }
                    inputs.push(format!("usz{custom_inputs}: {input_type}"));
                }
                custom_inputs += 1;
            }
            else{
                inputs.push(format!("_: {input_type}"));
            }
        }
        inputs_str = inputs.join(", ");
    }
    (format!("{closure_name}({inputs_str}){output_str}"), has_custom_arg)
}

fn surf_rebuild_fn_sig(
    sig_str: String,
    fn_node: NodeIndex,
    fuzz_tree: &Graph<SurfTreeNode, ()>
) -> String
{
    let first_open_par = sig_str.find('(').expect("No opening parenthesis");
    let name = sig_str[..first_open_par].trim();
    let (inputs_node_opt, output_node_opt) = surf_get_node_inputs_output(fn_node, &fuzz_tree);
    let mut inputs_str = String::from("");
    let mut output_str = String::from("");
    let has_self = surf_fn_has_self(fn_node, fuzz_tree);
    if sig_str.find("->").is_some(){
        if let Some(output_node) = output_node_opt{
            output_str = surf_get_type_name(fuzz_tree.neighbors(output_node).next(), &fuzz_tree, false);
            output_str = format!(" -> {output_str}");
        }
    }
    if let Some(inputs_node) = inputs_node_opt {
        let mut inputs = Vec::<String>::new();
        for input in fuzz_tree.neighbors(inputs_node){
            if inputs.len() == 0 && has_self{
                let input_type = surf_get_type_name(Some(input), fuzz_tree, true);
                inputs.push(input_type);
            }
            else{
                let input_type = surf_get_type_name(Some(input), fuzz_tree, false);
                inputs.push(format!("_: {input_type}"));
            }
        }
        inputs_str = inputs.join(", ");
    }
    format!("{name}({inputs_str}){output_str}")
}

fn surf_fn_has_self(fn_node:NodeIndex, fuzz_tree: &Graph<SurfTreeNode, ()>) -> bool{
    let fn_node_data = &fuzz_tree[fn_node];
    if let SurfTreeNode::Api { has_self, .. } 
            | SurfTreeNode::TraitFn { has_self, .. } 
            | SurfTreeNode::Constructor { has_self, .. } = fn_node_data {
        return *has_self
    }
    false
}

fn surf_api_returns_consumable(
    current_node_opt: Option<NodeIndex>,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
) -> bool
{
    if let Some(current_node) = current_node_opt{
        match &fuzz_tree[current_node]{
            SurfTreeNode::Struct{is_consumable, ..}
            | SurfTreeNode::Enum{is_consumable, ..} => *is_consumable,
            SurfTreeNode::FromOption
            | SurfTreeNode::FromResult(_) => surf_api_returns_consumable(fuzz_tree.neighbors(current_node).next(), fuzz_tree),
            _ => {
                false
            },
        }
    }
    else{
        false
    }
}

fn surf_arg_is_custom_type(
    current_node_opt: Option<NodeIndex>,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    generic_subst_map: &HashMap<String, SubstType>,
    custom_structs: &HashMap<String, (String, String)>,
) -> Option<String>{
    if let Some(current_node) = current_node_opt{
        match &fuzz_tree[current_node]{
            SurfTreeNode::Generic{name, ..} => {
                match generic_subst_map.get(name){
                    Some(subst_type) => {
                        match &subst_type.kind{
                            SubstTypeKind::Custom{..} => Some(custom_structs.get(name).unwrap().1.clone()),
                            _ => None,
                        }
                    },
                    _ => None,
                }
            },
            SurfTreeNode::Reference(_) => {                
                surf_arg_is_custom_type(fuzz_tree.neighbors(current_node).next(), fuzz_tree, generic_subst_map, custom_structs)
            },
            _ => None,
        }
    }
    else{
        return None
    }
    
}

fn surf_get_type_name_mono(
    current_node_opt: Option<NodeIndex>,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    generic_subst_map: &HashMap<String, SubstType>,
) -> String
{
    if let Some(current_node) = current_node_opt{
        match &fuzz_tree[current_node]{
            SurfTreeNode::Primitive(primitive_type) => primitive_type.clone(),
            SurfTreeNode::Str => "str".to_string(),
            SurfTreeNode::ReferencedStr => "&str".to_string(),
            SurfTreeNode::String => "String".to_string(),
            SurfTreeNode::Struct{name, ..} => name.clone(),
            SurfTreeNode::Enum{name, ..} => name.clone(),
            SurfTreeNode::Generic{name, ..} => {
                if let Some(subst_type) = generic_subst_map.get(name){
                    match &subst_type.kind{
                        SubstTypeKind::Custom{name, ..} => name.clone(),
                        SubstTypeKind::Candidate{..} => {
                            let candidates_node = fuzz_tree.neighbors(current_node).next().unwrap();
                            let candidate_node = fuzz_tree.neighbors(candidates_node).next();
                            surf_get_type_name_mono(candidate_node, fuzz_tree, generic_subst_map)
                        },
                    } 
                }
                else{
                    format!("TypeNotExist")
                }
            },
            SurfTreeNode::TraitType { .. } =>{
                unreachable!("INSPECT");
            }
            SurfTreeNode::AssocType{ assoc_type_id, .. } => {
                //println!("MONO DEBUG: {:?}", assoc_type_id);
                if let Some(subst_type) = generic_subst_map.get(assoc_type_id){
                    match &subst_type.kind{
                        SubstTypeKind::Custom{name, ..} => name.clone(),
                        SubstTypeKind::Candidate{..} => {
                            let candidates_node = fuzz_tree.neighbors(current_node).next().unwrap();
                            let candidate_node = fuzz_tree.neighbors(candidates_node).next();
                            surf_get_type_name_mono(candidate_node, fuzz_tree, generic_subst_map)
                        },
                    } 
                }
                else{
                    format!("TypeNotExist")
                }
            },
            SurfTreeNode::Reference(is_mutable) => {
                let mutability = {
                    match is_mutable{
                        true => "mut ",
                        false => "",
                    }
                };
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name_mono(inner_node, fuzz_tree, generic_subst_map);
                format!("&{mutability}{inner_type_str}")
                
            },
            SurfTreeNode::RawPointer(is_mutable) => {
                let mutability = {
                    match is_mutable{
                        true => "mut ",
                        false => "const ",
                    }
                };
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name_mono(inner_node, fuzz_tree, generic_subst_map);
                format!("*{mutability}{inner_type_str}")
            },
            SurfTreeNode::Box => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type = surf_get_type_name_mono(inner_node, fuzz_tree, generic_subst_map);
                format!("Box<{inner_type}>")
            },
            SurfTreeNode::Vector => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type = surf_get_type_name_mono(inner_node, fuzz_tree, generic_subst_map);
                format!("Vec<{inner_type}>")
            },
            SurfTreeNode::ReferencedSlice(is_mutable) => {
                let mutability = {
                    match is_mutable{
                        true => "mut ",
                        false => "",
                    }
                };
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name_mono(inner_node, fuzz_tree, generic_subst_map);
                format!("&{mutability}[{inner_type_str}]")
            },
            // Possible when inside Box
            SurfTreeNode::Slice => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type = surf_get_type_name_mono(inner_node, fuzz_tree, generic_subst_map);
                format!("[{inner_type}]")
            },
            SurfTreeNode::Array(len) => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name_mono(inner_node, fuzz_tree, generic_subst_map);
                format!("[{inner_type_str}; {len}]")
            },
            SurfTreeNode::Tuple => {
                let mut field_types = Vec::<String>::new();
                for neighbor in fuzz_tree.neighbors(current_node){
                    let field_type = surf_get_type_name_mono(Some(neighbor), fuzz_tree, generic_subst_map);
                    field_types.push(field_type);
                }
                let field_types_str = field_types.join(", ");
                format!("({field_types_str})")
            },
            SurfTreeNode::ToOption => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name_mono(inner_node, fuzz_tree, generic_subst_map);
                format!("Option<{inner_type_str}>")
            },
            SurfTreeNode::ToResult(error_type) => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name_mono(inner_node, fuzz_tree, generic_subst_map);
                format!("Result<{inner_type_str}, {error_type}>")
            },
            SurfTreeNode::DynTrait => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name_mono(inner_node, fuzz_tree, generic_subst_map);
                format!("dyn {inner_type_str}")
            },
            _ => {
                String::from("Todo")
            },
        }
    }
    else{
        String::from("")
    }
}

fn surf_get_type_name(
    current_node_opt: Option<NodeIndex>,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    is_self: bool,
)-> String
{
    if let Some(current_node) = current_node_opt{
        match &fuzz_tree[current_node]{
            SurfTreeNode::Primitive(primitive_type) => primitive_type.clone(),
            SurfTreeNode::Str => "str".to_string(),
            SurfTreeNode::ReferencedStr => "&str".to_string(),
            SurfTreeNode::String => "String".to_string(),
            SurfTreeNode::Struct{full_name, ..} => full_name.clone(),
            SurfTreeNode::Enum{full_name, ..} => full_name.clone(),
            SurfTreeNode::Generic{name, ..} => {
                match is_self{
                    true => String::from("self"),
                    false => name.clone(),
                }
            },
            SurfTreeNode::AssocType{placeholder_name, ..} => {
                // may not be correct, risk it for now
                format!("Self::{placeholder_name}")
                //todo!();
            },
            SurfTreeNode::Reference(is_mutable) => {
                let mutability = {
                    match is_mutable{
                        true => "mut ",
                        false => "",
                    }
                };
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name(inner_node, fuzz_tree, is_self);
                format!("&{mutability}{inner_type_str}")
            },
            SurfTreeNode::RawPointer(is_mutable) => {
                let mutability = {
                    match is_mutable{
                        true => "mut ",
                        false => "const ",
                    }
                };
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name(inner_node, fuzz_tree, is_self);
                format!("*{mutability}{inner_type_str}")
            },
            SurfTreeNode::Box => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type = surf_get_type_name(inner_node, fuzz_tree, is_self);
                format!("Box<{inner_type}>")
            },
            SurfTreeNode::Vector => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type = surf_get_type_name(inner_node, fuzz_tree, is_self);
                format!("Vec<{inner_type}>")
            },
            SurfTreeNode::ReferencedSlice(is_mutable) => {
                let mutability = {
                    match is_mutable{
                        true => "mut ",
                        false => "",
                    }
                };
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name(inner_node, fuzz_tree, is_self);
                format!("&{mutability}[{inner_type_str}]")
            },
            SurfTreeNode::Slice => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type = surf_get_type_name(inner_node, fuzz_tree, is_self);
                format!("[{inner_type}]")
            },
            SurfTreeNode::Array(len) => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name(inner_node, fuzz_tree, is_self);
                format!("[{inner_type_str}; {len}]")
            },
            SurfTreeNode::Tuple => {
                let mut field_types = Vec::<String>::new();
                for neighbor in fuzz_tree.neighbors(current_node){
                    let field_type = surf_get_type_name(Some(neighbor), fuzz_tree, is_self);
                    field_types.push(field_type);
                }
                let field_types_str = field_types.join(", ");
                format!("({field_types_str})")
            },
            SurfTreeNode::ToOption => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name(inner_node, fuzz_tree, is_self);
                format!("Option<{inner_type_str}>")
            },
            SurfTreeNode::ToResult(error_type) => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name(inner_node, fuzz_tree, is_self);
                format!("Result<{inner_type_str}, {error_type}>")
            },
            SurfTreeNode::DynTrait => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                let inner_type_str = surf_get_type_name(inner_node, fuzz_tree, is_self);
                format!("dyn {inner_type_str}")
            },
            _ => String::from("Todo"),
        }
    }
    else{
        String::from("")
    }
}

fn surf_trim_trailing_semicolon(input: &str) -> String {
    input.strip_suffix(';').unwrap_or(input).to_string()
}

fn surf_is_supported(fuzz_tree: &FuzzTree) -> bool {
    let graph = &fuzz_tree.tree;

    // debug‐dump the graph
    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    // start our own DFS stack at the root
    let root: NodeIndex = fuzz_tree.root.unwrap();
    let mut stack = vec![root];
    let mut seen = HashSet::new();

    while let Some(node) = stack.pop() {
        // skip revisiting
        if !seen.insert(node) {
            continue;
        }

        // if this node should cause us to skip its whole subtree…
        if surf_is_skip_node(&graph[node]) {
            //println!("SKIP subtree at {:?}", node);
            continue;
        }

        // if this node is unsupported, bail out
        if surf_is_unsupported_node(&graph[node]) {
            //println!("UNSUPPORTED at {:?}", node);
            return false;
        }

        // otherwise push its neighbors for further traversal
        for nbr in graph.neighbors(node) {
            stack.push(nbr);
        }
    }

    // no unsupported node found
    true
}


fn surf_create_and_add_input_node(
    arg: &SurfFnArg,
    urapi_ao_tree: &mut Graph<SurfTreeNode, ()>,
    seen_cmplx_types: &mut HashSet<String>
) -> NodeIndex
{
    match arg{
        SurfFnArg::Primitive(primitive_type) => {
            if primitive_type == "str"{
                urapi_ao_tree.add_node(SurfTreeNode::Str)
            }
            else{
                urapi_ao_tree.add_node(SurfTreeNode::Primitive(primitive_type.clone()))
            }
        },
        SurfFnArg::String => urapi_ao_tree.add_node(SurfTreeNode::String),
        SurfFnArg::Todo(unsupported_type) => urapi_ao_tree.add_node(SurfTreeNode::Todo(unsupported_type.clone())),
        SurfFnArg::Slice(inner_type) => {
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Slice);
            let inner_node = surf_create_and_add_input_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Array(inner_type, len) => {
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Array(len.clone()));
            let inner_node = surf_create_and_add_input_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Vector(inner_type) =>{
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Vector);
            let inner_node = surf_create_and_add_input_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Box(inner_type) =>{
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Box);
            let inner_node = surf_create_and_add_input_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Uinit(inner_type) =>{
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Uinit);
            let inner_node = surf_create_and_add_input_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Result{ok, err} =>{
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::ToResult(err.clone()));
            let inner_node = surf_create_and_add_input_node(ok, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Option(inner_type) =>{
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::ToOption);
            let inner_node = surf_create_and_add_input_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Reference(value, is_mutable) =>{
            let inner_node = surf_create_and_add_input_node(value, urapi_ao_tree, seen_cmplx_types);
            match urapi_ao_tree[inner_node].borrow_mut(){
                SurfTreeNode::Slice => {
                    urapi_ao_tree[inner_node] = SurfTreeNode::ReferencedSlice(*is_mutable);
                    inner_node
                },
                SurfTreeNode::Str => {
                    urapi_ao_tree[inner_node] = SurfTreeNode::ReferencedStr;
                    inner_node
                },
                _ => {
                    let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Reference(*is_mutable));
                    urapi_ao_tree.add_edge(outer_node, inner_node, ());
                    outer_node
                },
            }
        },
        SurfFnArg::RawPointer(value, is_mutable) => {
            let inner_node = surf_create_and_add_input_node(value, urapi_ao_tree, seen_cmplx_types);
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::RawPointer(*is_mutable));
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Struct{name, full_name, is_consumable, ..} =>{
            let cmplx_type_node = urapi_ao_tree.add_node(SurfTreeNode::Struct{
                                                                                                    name: name.clone(),
                                                                                                    full_name: full_name.clone(),
                                                                                                    is_consumable: *is_consumable,
            });
            cmplx_type_node
        },
        SurfFnArg::Tuple(tuple_args) =>{
            let tuple_type_node = urapi_ao_tree.add_node(SurfTreeNode::Tuple);
            
            for tuple_arg in tuple_args{
                let tuple_arg_node = surf_create_and_add_input_node(tuple_arg, urapi_ao_tree, seen_cmplx_types);
                urapi_ao_tree.add_edge(tuple_type_node, tuple_arg_node, ());
            }
            tuple_type_node
        },
        SurfFnArg::Enum{name, full_name, is_consumable, ..} => {
            let enum_type_node = urapi_ao_tree.add_node(SurfTreeNode::Enum{
                                                                                                name: name.clone(),
                                                                                                full_name: full_name.clone(),
                                                                                                is_consumable: *is_consumable,
            });
            enum_type_node
        },
        SurfFnArg::Generic{name, traits} => {
            let generic_node = urapi_ao_tree.add_node(SurfTreeNode::Generic{
                name: name.clone(),
            });
            let custom_type_node = urapi_ao_tree.add_node(SurfTreeNode::GenericCustomTy);
            urapi_ao_tree.add_edge(generic_node, custom_type_node, ());
            surf_add_trait_nodes(traits, custom_type_node, urapi_ao_tree, seen_cmplx_types);
            generic_node
        },
        SurfFnArg::AssocType { def_id, trait_def_id, placeholder_name, assoc_type_id, traits } =>{
            let assoc_type_node = urapi_ao_tree.add_node(SurfTreeNode::AssocType {
                                                                                                        def_id: def_id.clone(),
                                                                                                        trait_def_id: trait_def_id.clone(),
                                                                                                        placeholder_name: placeholder_name.clone(),
                                                                                                        assoc_type_id: assoc_type_id.clone(),
                                                                                                    });
            surf_add_trait_nodes(traits, assoc_type_node, urapi_ao_tree, seen_cmplx_types);
            assoc_type_node
        },
        SurfFnArg::TraitType { def_id, placeholder_name, assoc_type_id, concrete_type_id, traits } => {
            let trait_type_node = urapi_ao_tree.add_node(SurfTreeNode::TraitType {
                                                                                                        def_id: def_id.clone(),
                                                                                                        placeholder_name: placeholder_name.clone(),
                                                                                                        assoc_type_id: assoc_type_id.clone(),
                                                                                                        concrete_type_id: concrete_type_id.clone(),
                                                                                                    });
            surf_add_trait_nodes(traits, trait_type_node, urapi_ao_tree, seen_cmplx_types);
            trait_type_node
        },
        SurfFnArg::Closure {param_name, inputs, output, is_mutable} => {
            let closure_node = urapi_ao_tree.add_node(SurfTreeNode::Closure(param_name.clone(), *is_mutable));
            surf_add_closure_inputs_output_nodes(inputs, output, closure_node, urapi_ao_tree, seen_cmplx_types);
            closure_node
        },
        SurfFnArg::DynTrait(_cmplx_tys) => { // Need more work
            let dyn_node = urapi_ao_tree.add_node(SurfTreeNode::DynTrait);
            dyn_node
        },
        SurfFnArg::Trait { def_id, types, funcs, .. } =>{
            surf_create_trait_node(def_id, types, funcs, urapi_ao_tree, seen_cmplx_types)
        }
    }
}

fn surf_create_and_add_node(
    arg: &SurfFnArg,
    urapi_ao_tree: &mut Graph<SurfTreeNode, ()>,
    seen_cmplx_types: &mut HashSet<String>
) -> NodeIndex
{
    match arg{
        SurfFnArg::Primitive(primitive_type) => {
            if primitive_type == "str"{
                urapi_ao_tree.add_node(SurfTreeNode::Str)
            }
            else{
                urapi_ao_tree.add_node(SurfTreeNode::Primitive(primitive_type.clone()))
            }
        },
        SurfFnArg::String => urapi_ao_tree.add_node(SurfTreeNode::String),
        SurfFnArg::Todo(unsupported_type) => urapi_ao_tree.add_node(SurfTreeNode::Todo(unsupported_type.clone())),
        SurfFnArg::Slice(inner_type) => {
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Slice);
            let inner_node = surf_create_and_add_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Array(inner_type, len) => {
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Array(len.clone()));
            let inner_node = surf_create_and_add_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Vector(inner_type) =>{
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Vector);
            let inner_node = surf_create_and_add_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Uinit(inner_type) =>{
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Uinit);
            let inner_node = surf_create_and_add_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Box(inner_type) =>{
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Box);
            let inner_node = surf_create_and_add_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Result{ok, err} =>{
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::ToResult(err.clone()));
            let inner_node = surf_create_and_add_node(ok, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Option(inner_type) =>{
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::ToOption);
            let inner_node = surf_create_and_add_node(inner_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Reference(value, is_mutable) =>{
            let inner_node = surf_create_and_add_node(value, urapi_ao_tree, seen_cmplx_types);
            match urapi_ao_tree[inner_node].borrow_mut(){
                SurfTreeNode::Slice => {
                    urapi_ao_tree[inner_node] = SurfTreeNode::ReferencedSlice(*is_mutable);
                    inner_node
                },
                SurfTreeNode::Str => {
                    urapi_ao_tree[inner_node] = SurfTreeNode::ReferencedStr;
                    inner_node
                },
                _ => {
                    let outer_node = urapi_ao_tree.add_node(SurfTreeNode::Reference(*is_mutable));
                    urapi_ao_tree.add_edge(outer_node, inner_node, ());
                    outer_node
                },
            }
        },
        SurfFnArg::RawPointer(value, is_mutable) => {
            let inner_node = surf_create_and_add_node(value, urapi_ao_tree, seen_cmplx_types);
            let outer_node = urapi_ao_tree.add_node(SurfTreeNode::RawPointer(*is_mutable));
            urapi_ao_tree.add_edge(outer_node, inner_node, ());
            outer_node
        },
        SurfFnArg::Struct{def_id, name, full_name, is_consumable} =>{
            if seen_cmplx_types.contains(def_id){
                urapi_ao_tree.add_node(SurfTreeNode::Todo(String::from("Infinite Loop")))
            }
            else{
                seen_cmplx_types.insert(def_id.clone());
                let cmplx_type_node = urapi_ao_tree.add_node(SurfTreeNode::Struct{
                                                                                                        name: name.clone(),
                                                                                                        full_name: full_name.clone(),
                                                                                                        is_consumable: *is_consumable,
                                                                                                    });
                surf_add_constructor_nodes(def_id, cmplx_type_node, urapi_ao_tree, seen_cmplx_types, false);
                if urapi_ao_tree.neighbors(cmplx_type_node).next().is_none(){
                    urapi_ao_tree[cmplx_type_node] = SurfTreeNode::Todo(String::from("No Constructors"));
                }
                cmplx_type_node
            }
        },
        SurfFnArg::Tuple(tuple_args) =>{
            let tuple_type_node = urapi_ao_tree.add_node(SurfTreeNode::Tuple);
            
            for tuple_arg in tuple_args{
                let tuple_arg_node = surf_create_and_add_node(tuple_arg, urapi_ao_tree, seen_cmplx_types);
                urapi_ao_tree.add_edge(tuple_type_node, tuple_arg_node, ());
            }
            tuple_type_node
        },
        SurfFnArg::Enum{def_id, name, full_name, is_consumable} => {
            if seen_cmplx_types.contains(def_id){
                urapi_ao_tree.add_node(SurfTreeNode::Todo(String::from("Infinite Loop")))
            }
            else{
                seen_cmplx_types.insert(def_id.clone());
                let enum_type_node = urapi_ao_tree.add_node(SurfTreeNode::Enum{
                                                                                                    name: name.clone(),
                                                                                                    full_name: full_name.clone(),
                                                                                                    is_consumable: *is_consumable,
                                                                                                });
                let enum_complex_variants_node = urapi_ao_tree.add_node(SurfTreeNode::ComplexVariants);
                urapi_ao_tree.add_edge(enum_type_node, enum_complex_variants_node, ());
                
                let variants = SURF_ENUMS.get(def_id).unwrap().clone();
                if variants.len() > 0{
                    surf_add_constructor_nodes(def_id, enum_complex_variants_node, urapi_ao_tree, seen_cmplx_types, true);
                }

                let enum_simple_variants_node = urapi_ao_tree.add_node(SurfTreeNode::SimpleVariants);
                urapi_ao_tree.add_edge(enum_type_node, enum_simple_variants_node, ());
                for (variant_name, variant_args) in variants{
                    let variant_node = urapi_ao_tree.add_node(SurfTreeNode::Variant(variant_name.clone()));
                    for variant_arg in variant_args{
                        let variant_arg_node = surf_create_and_add_node(&variant_arg, urapi_ao_tree, seen_cmplx_types);
                        urapi_ao_tree.add_edge(variant_node, variant_arg_node, ());
                    }
                    urapi_ao_tree.add_edge(enum_simple_variants_node, variant_node, ());
                }
                enum_type_node
            }
        },
        SurfFnArg::Generic{name, traits} => {
            //println!("Generic: {:?}", gen_traits);
            let generic_node = urapi_ao_tree.add_node(SurfTreeNode::Generic{
                                                                                                name: name.clone(),
            });

            // Generate Custom Type Node
            let custom_type_node = urapi_ao_tree.add_node(SurfTreeNode::GenericCustomTy);
            urapi_ao_tree.add_edge(generic_node, custom_type_node, ());
            surf_add_trait_nodes(traits, custom_type_node, urapi_ao_tree, seen_cmplx_types);
            generic_node
        },
        SurfFnArg::AssocType { def_id, trait_def_id, placeholder_name, assoc_type_id, traits } =>{
            let assoc_type_node = urapi_ao_tree.add_node(SurfTreeNode::AssocType {
                                                                                                        def_id: def_id.clone(),
                                                                                                        trait_def_id: trait_def_id.clone(),
                                                                                                        placeholder_name: placeholder_name.clone(),
                                                                                                        assoc_type_id: assoc_type_id.clone(),
                                                                                                    });
            surf_add_trait_nodes(traits, assoc_type_node, urapi_ao_tree, seen_cmplx_types);
            assoc_type_node
        },
        SurfFnArg::TraitType { def_id, placeholder_name, assoc_type_id, concrete_type_id, traits } => {
            let trait_type_node = urapi_ao_tree.add_node(SurfTreeNode::TraitType {
                                                                                                        def_id: def_id.clone(),
                                                                                                        placeholder_name: placeholder_name.clone(),
                                                                                                        assoc_type_id: assoc_type_id.clone(),
                                                                                                        concrete_type_id: concrete_type_id.clone(),
                                                                                                    });
            surf_add_trait_nodes(traits, trait_type_node, urapi_ao_tree, seen_cmplx_types);
            trait_type_node
        },
        SurfFnArg::Closure{param_name, inputs, output, is_mutable} => {
            let closure_node = urapi_ao_tree.add_node(SurfTreeNode::Closure(param_name.clone(), *is_mutable));
            surf_add_closure_inputs_output_nodes(inputs, output, closure_node, urapi_ao_tree, seen_cmplx_types);
            closure_node
        },
        SurfFnArg::DynTrait(cmplx_tys) => {
            let dyn_node = urapi_ao_tree.add_node(SurfTreeNode::DynTrait);
            surf_add_dyn_trait_nodes(cmplx_tys, dyn_node, urapi_ao_tree, seen_cmplx_types);
            dyn_node
        },
        SurfFnArg::Trait { def_id, types, funcs, .. } =>{
            surf_create_trait_node(def_id, types, funcs, urapi_ao_tree, seen_cmplx_types)
        }
    }
}

fn surf_add_constructor_nodes(
    cmplx_type_def_id: &String,
    cmplx_type_node: NodeIndex,
    urapi_ao_tree: &mut Graph<SurfTreeNode, ()>,
    seen_cmplx_types: &mut HashSet<String>,
    limit_enum_constructors: bool,
){
    let constructors: HashSet<String> = SURF_COMPLEX_TYPES_TO_CONSTRUCTORS.get(cmplx_type_def_id).unwrap().clone();
    let sorted_constructors: Vec<String> = constructors.into_iter().sorted().collect();
    let mut rng = StdRng::seed_from_u64(SEED);
    let constructor_collection: Vec<&String> = match limit_enum_constructors {
        true => sorted_constructors.choose_multiple(&mut rng, MAX_ENUM_CONSTRUCTORS).collect(),
        false => sorted_constructors.choose_multiple(&mut rng, MAX_STRUCT_CONSTRUCTORS).collect(),
    };
    //println!("{:?}", sorted_constructors);
    for constructor_def_id in constructor_collection{ // Limit
        //println!("CONST: {:?}", constructor_def_id);
        let constructor = SURF_CONSTRUCTORS.get(constructor_def_id).unwrap().clone();
        let constructor_node = urapi_ao_tree.add_node(SurfTreeNode::Constructor{
                                                                                                def_id:constructor_def_id.clone(),
                                                                                                name: constructor.name.clone(),
                                                                                                full_name: constructor.full_name.clone(),
                                                                                                has_self: *constructor.flags.get("has_self").unwrap(),
                                                                                                output: constructor.output.clone(),
                                                                                                cmplx_type_def_id: cmplx_type_def_id.clone(),
                                                                                            });
        
        // First go to implicit generics (DFS)
        let implicit_generics_node = urapi_ao_tree.add_node(SurfTreeNode::ConstructorImplicitGenerics);
        urapi_ao_tree.add_edge(constructor_node, implicit_generics_node, ());
        for implicit_generic in constructor.implicit_generics.iter(){
            let mut new_seen_cmplx_types = seen_cmplx_types.clone();
            let implicit_generic_node = surf_create_and_add_node(implicit_generic, urapi_ao_tree, &mut new_seen_cmplx_types);
            urapi_ao_tree.add_edge(implicit_generics_node, implicit_generic_node, ());
        }

        // Infinite Loop may occur if A needs B and B needs A, where A and B complex types
        let inputs_node = urapi_ao_tree.add_node(SurfTreeNode::ConstructorInputs);
        urapi_ao_tree.add_edge(constructor_node, inputs_node, ());
        for arg in constructor.inputs.iter(){
            let mut new_seen_cmplx_types = seen_cmplx_types.clone();
            let arg_node = surf_create_and_add_node(arg, urapi_ao_tree, &mut new_seen_cmplx_types);
            urapi_ao_tree.add_edge(inputs_node, arg_node, ());
        }

        urapi_ao_tree.add_edge(cmplx_type_node, constructor_node, ());
    }
}

fn surf_add_trait_nodes(
    gen_traits: &HashMap<String, Box<SurfFnArg>>,
    generic_node: NodeIndex,
    urapi_ao_tree: &mut Graph<SurfTreeNode, ()>,
    seen_cmplx_types: &mut HashSet<String>
){
    for (_, trait_bound) in gen_traits{
        let trait_node = surf_create_and_add_node(trait_bound, urapi_ao_tree, seen_cmplx_types);
        urapi_ao_tree.add_edge(generic_node, trait_node, ());
    }
}

fn surf_create_trait_node(
    trait_def_id: &String,
    trait_types: &Vec<Box<SurfFnArg>>,
    trait_funcs: &HashSet<String>,
    urapi_ao_tree: &mut Graph<SurfTreeNode, ()>,
    seen_cmplx_types: &mut HashSet<String>
) -> NodeIndex
{    
    let trait_name = SURF_TRAITS.get(trait_def_id).unwrap().name.clone();
    let is_unsafe_trait = SURF_TRAITS.get(trait_def_id).unwrap().is_unsafe.clone();
    let trait_use = SURF_TRAITS.get(trait_def_id).unwrap().external.clone();
    
    // Create Trait Node
    let trait_node = urapi_ao_tree.add_node(SurfTreeNode::Trait{
                                                                                    def_id: trait_def_id.clone(),
                                                                                    name: trait_name,
                                                                                    is_unsafe: is_unsafe_trait,
                                                                                    external_data: trait_use
                                                                                });
    
    // Create TraitFns Node
    if !trait_types.is_empty(){
        let trait_types_node = urapi_ao_tree.add_node(SurfTreeNode::TraitTypes);
        urapi_ao_tree.add_edge(trait_node, trait_types_node, ());

        // Create and extend TraitType Nodes
        for assoc_type in trait_types.iter(){
            let assoc_type_node = surf_create_and_add_node(assoc_type, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(trait_types_node, assoc_type_node, ());
        }
    }
    
    // Create TraitFns Node
    if !trait_funcs.is_empty(){
        let trait_fns_node = urapi_ao_tree.add_node(SurfTreeNode::TraitFns);
        urapi_ao_tree.add_edge(trait_node, trait_fns_node, ());

        // Create and Extend TraitFn Nodes
        for trait_fn_def_id in trait_funcs.iter(){
            //println!("THERE!");
            let surf_trait_fn = SURF_TRAIT_FNS.get(trait_fn_def_id).unwrap().clone();
            let trait_fn_node = urapi_ao_tree.add_node(SurfTreeNode::TraitFn {
                                                                                                    def_id: trait_fn_def_id.clone(),
                                                                                                    name: surf_trait_fn.name.clone(),
                                                                                                    full_name: surf_trait_fn.full_name.clone(),
                                                                                                    is_unsafe: surf_trait_fn.is_unsafe.clone(),
                                                                                                    sig_str: surf_trim_trailing_semicolon(&surf_trait_fn.span_str),
                                                                                                    has_self: *surf_trait_fn.flags.get("has_self").unwrap(),
                                                                                                    //inputs: surf_trait_fn.inputs.clone(),
            });
            urapi_ao_tree.add_edge(trait_fns_node, trait_fn_node, ());
            let trait_fn_output = surf_trait_fn.output.clone();
            if let Some(output) = trait_fn_output{
                let output_node = urapi_ao_tree.add_node(SurfTreeNode::FnOutput);
                urapi_ao_tree.add_edge(trait_fn_node, output_node, ());
                let mut new_seen_cmplx_types = seen_cmplx_types.clone();
                let output_arg_node = surf_create_and_add_node(&output, urapi_ao_tree, &mut new_seen_cmplx_types);
                urapi_ao_tree.add_edge(output_node, output_arg_node, ());
            }
            if !surf_trait_fn.inputs.is_empty(){
                let inputs_node = urapi_ao_tree.add_node(SurfTreeNode::FnInputs);
                for trait_fn_input in surf_trait_fn.inputs.iter(){
                    let mut new_seen_cmplx_types = seen_cmplx_types.clone();
                    let input_arg_node = surf_create_and_add_input_node(trait_fn_input, urapi_ao_tree, &mut new_seen_cmplx_types);
                    urapi_ao_tree.add_edge(inputs_node, input_arg_node, ());
                }
                urapi_ao_tree.add_edge(trait_fn_node, inputs_node, ());
            }
        }
    }
    trait_node
}

fn surf_add_closure_inputs_output_nodes(
    inputs: &Vec<Box<SurfFnArg>>,
    output_opt: &Option<Box<SurfFnArg>>,
    closure_node: NodeIndex,
    urapi_ao_tree: &mut Graph<SurfTreeNode, ()>,
    seen_cmplx_types: &mut HashSet<String>
){
    if !inputs.is_empty(){
        let inputs_node = urapi_ao_tree.add_node(SurfTreeNode::FnInputs);
        for input in inputs.iter(){
            let input_arg_node = surf_create_and_add_input_node(input, urapi_ao_tree, seen_cmplx_types);
            urapi_ao_tree.add_edge(inputs_node, input_arg_node, ());
        }
        urapi_ao_tree.add_edge(closure_node, inputs_node, ());
    }
    if let Some(output) = output_opt{
        let output_node = urapi_ao_tree.add_node(SurfTreeNode::FnOutput);
        urapi_ao_tree.add_edge(closure_node, output_node, ());
        let mut new_seen_cmplx_types = seen_cmplx_types.clone();
        let output_arg_node = surf_create_and_add_node(&output, urapi_ao_tree, &mut new_seen_cmplx_types);
        urapi_ao_tree.add_edge(output_node, output_arg_node, ());
    }
}

fn surf_add_dyn_trait_nodes(
    cmplx_tys: &Vec<Box<SurfFnArg>>,
    dyn_node: NodeIndex,
    urapi_ao_tree: &mut Graph<SurfTreeNode, ()>,
    seen_cmplx_types: &mut HashSet<String>
){    
    for cmplx_ty in cmplx_tys{
        let cmplx_ty_node = surf_create_and_add_node(cmplx_ty, urapi_ao_tree, seen_cmplx_types);
        urapi_ao_tree.add_edge(dyn_node, cmplx_ty_node, ());
    }
}

fn surf_add_subtree(
    surf_tree: &FuzzTree,
    surf_subtree: &FuzzTree,
) -> FuzzTree
{
    let main_tree = &surf_tree.tree;
    let subtree = &surf_subtree.tree;

    // The root of the main tree is going to be the new root
    let main_tree_root = surf_tree.root.unwrap();
    
    // The merged_tree is going to be the resulting tree
    let mut merged_tree: Graph<SurfTreeNode, ()> = main_tree.clone();
    // We need a mapping between the nodes of the subtree with the corresponfing nodes in the merged tree
    let mut node_map = HashMap::new();
    
    // Add all nodes from the given subtree to the merged tree
    for node in subtree.node_indices() {
        let cloned_node = merged_tree.add_node(subtree[node].clone());
        node_map.insert(node, cloned_node);
    }

    // Add all edges from the given subtree to the merged tree
    for edge in subtree.edge_indices() {
        let (source, target) = subtree.edge_endpoints(edge).unwrap();
        let new_source = node_map[&source];
        let new_target = node_map[&target];
        merged_tree.add_edge(new_source, new_target, ());
    }

    // Get the root of the subtree inside the merged tree and connect it with the new root
    let subtree_root = node_map[&surf_subtree.root.unwrap()];
    merged_tree.add_edge(main_tree_root, subtree_root, ());

    // Return the resulting surf_tree
    FuzzTree::new(main_tree_root, merged_tree)
}

// Return how many fuzz trees are produced by the subtree rooted at node without any filtering.
fn total_count(
    node: NodeIndex,
    g: &Graph<SurfTreeNode, ()>,
    memo: &mut HashMap<NodeIndex, usize>,
) -> usize {
    if let Some(&c) = memo.get(&node) { return c; }
    let children: Vec<NodeIndex> = g.neighbors(node).collect();
    let c = if children.is_empty() {
        1
    } else if surf_is_or_node(&g[node]) {
        children.into_iter().map(|ch| total_count(ch, g, memo))
            .fold(0usize, |a,b| a.saturating_add(b))
    } else {
        children.into_iter().map(|ch| total_count(ch, g, memo))
            .fold(1usize, |a,b| a.saturating_mul(b))
    };
    memo.insert(node, c);
    c
}

// Return how many fuzz trees are produced by the subtree rooted at node after filtering.
fn supported_count(
    node: NodeIndex,
    g: &Graph<SurfTreeNode, ()>,
    tot: &mut HashMap<NodeIndex, usize>,
    memo: &mut HashMap<NodeIndex, usize>,
) -> usize {
    if let Some(&c) = memo.get(&node) { return c; }

    let nd = &g[node];
    // Accept the entire subtree
    if surf_is_skip_node(nd) {
        let c = total_count(node, g, tot);
        memo.insert(node, c);
        return c;
    }
    // Accept none
    if surf_is_unsupported_node(nd) {
        memo.insert(node, 0);
        return 0;
    }

    // Otherwise recurse exactly like total_count, but using supported_count on children and an early break for AND if any child is 0.
    let children: Vec<NodeIndex> = g.neighbors(node).collect();
    let c = if children.is_empty() {
        1
    } else if surf_is_or_node(nd) {
        children.into_iter()
            .map(|ch| supported_count(ch, g, tot, memo))
            .fold(0usize, |a,b| a.saturating_add(b))
    } else {
        // AND: all children must be supported; multiply their counts
        let mut prod = 1usize;
        for ch in children {
            let cc = supported_count(ch, g, tot, memo);
            if cc == 0 { prod = 0; break; }
            prod = prod.saturating_mul(cc);
        }
        prod
    };
    memo.insert(node, c);
    c
}

// Build the r-th fuzz tree (0-based) without filtering in the generator’s lexicographic order.
fn build_by_rank_unfiltered(
    node: NodeIndex,
    g: &Graph<SurfTreeNode, ()>,
    tot: &mut HashMap<NodeIndex, usize>,
    mut r: usize,
) -> FuzzTree {
    let node_data = g[node].clone();
    let children: Vec<NodeIndex> = g.neighbors(node).collect();

    if children.is_empty() {
        let mut leaf_graph: Graph<SurfTreeNode, ()> = Graph::new();
        let root = leaf_graph.add_node(node_data);
        return FuzzTree::new(root, leaf_graph);
    }

    if surf_is_or_node(&g[node]) {
        for &ch in &children {
            let c = total_count(ch, g, tot);
            if r < c {
                let mut t = build_by_rank_unfiltered(ch, g, tot, r);
                surf_add_new_root(&mut t, &g[node]);
                return t;
            }
            r -= c;
        }
        unreachable!("rank out of bounds (unfiltered OR)");
    }

    // AND (unfiltered)
    let cnts: Vec<usize> = children.iter().map(|&ch| total_count(ch, g, tot)).collect();
    let m = children.len();
    let mut strides = vec![1usize; m];
    for j in (0..m-1).rev() { strides[j] = strides[j+1].saturating_mul(cnts[j+1]); }
    let mut acc = build_by_rank_unfiltered(children[0], g, tot, (r / strides[0]) % cnts[0]);
    surf_add_new_root(&mut acc, &g[node]);
    for j in 1..m {
        let rj = (r / strides[j]) % cnts[j];
        let tj = build_by_rank_unfiltered(children[j], g, tot, rj);
        acc = surf_add_subtree(&acc, &tj);
    }
    acc
}

/// Build the r_sup-th *supported* fuzz tree under `node`.
fn build_supported_by_rank(
    node: NodeIndex,
    g: &Graph<SurfTreeNode, ()>,
    tot: &mut HashMap<NodeIndex, usize>,
    sup: &mut HashMap<NodeIndex, usize>,
    r_sup: usize,
) -> FuzzTree {
    let nd = &g[node];

    // NEW: skip means "ignore filtering below" → enumerate UNFILTERED here
    if surf_is_skip_node(nd) {
        let ttot = total_count(node, g, tot);
        debug_assert!(r_sup < ttot, "r_sup={} out of bounds for skip node {:?} (tot={})", r_sup, node, ttot);
        return build_by_rank_unfiltered(node, g, tot, r_sup);
    }

    // Should never try to build an unsupported node
    debug_assert!(!surf_is_unsupported_node(nd), "attempted to build unsupported node {:?}", node);

    let children: Vec<NodeIndex> = g.neighbors(node).collect();

    // Leaf
    if children.is_empty() {
        let mut leaf_graph: Graph<SurfTreeNode, ()> = Graph::new();
        let root = leaf_graph.add_node(nd.clone());
        return FuzzTree::new(root, leaf_graph);
    }

    // OR: choose by *supported* block sizes
    if surf_is_or_node(nd) {
        let mut r = r_sup;
        for &ch in &children {
            let s_i = supported_count(ch, g, tot, sup);
            if r < s_i {
                let mut t = build_supported_by_rank(ch, g, tot, sup, r);
                surf_add_new_root(&mut t, nd);
                return t;
            }
            r -= s_i;
        }
        unreachable!("supported rank out of bounds in OR");
    }

    // AND: split supported rank via supported strides; combine children
    let s: Vec<usize> = children.iter().map(|&c| supported_count(c, g, tot, sup)).collect();
    let m = children.len();

    // Guard: any zero → this AND has 0 supported trees; we should never be here
    if let Some(j) = s.iter().position(|&x| x == 0) {
        panic!(
            "internal error: AND node {:?} has child {} with supported_count=0 (r_sup={})",
            node, j, r_sup
        );
    }

    // (optional) bound check with overflow-safe product
    let total_supported = s.iter().try_fold(1usize, |acc, &x| acc.checked_mul(x))
        .expect("supported product overflow at AND node");
    assert!(r_sup < total_supported,
        "internal error: r_sup={} out of bounds for AND node {:?} (total supported={})",
        r_sup, node, total_supported);

    // supported strides (last child varies fastest)
    let mut stride_sup = vec![1usize; m];
    for j in (0..m - 1).rev() {
        stride_sup[j] = stride_sup[j + 1].saturating_mul(s[j + 1]);
    }

    // build first child and add the AND root once
    let r0 = (r_sup / stride_sup[0]) % s[0];
    let mut acc = build_supported_by_rank(children[0], g, tot, sup, r0);
    surf_add_new_root(&mut acc, nd);

    // graft remaining children
    for j in 1..m {
        let rj = (r_sup / stride_sup[j]) % s[j];
        let tj = build_supported_by_rank(children[j], g, tot, sup, rj);
        acc = surf_add_subtree(&acc, &tj);
    }
    acc
}
// ---------- Public entry: build only K filtered trees in random order ----------

fn surf_build_filtered_fuzz_trees_select_with_ids(
    urapi_def_id: &str,
    urapi_dep_tree: SurfTree,
    max_targets: Option<usize>,   // None => ALL filtered
    seed: u64,
) -> Vec<FuzzTree> {
    use rand::{rngs::StdRng, SeedableRng};
    use rand::seq::SliceRandom;
    use std::collections::HashMap;

    let root = urapi_dep_tree.root.expect("root required");
    let g = &urapi_dep_tree.tree;

    let mut tot = HashMap::new(); // for supported_count (skip)
    let mut sup = HashMap::new();

    let s = supported_count(root, g, &mut tot, &mut sup);
    if s == 0 { return Vec::new(); }

    let k = max_targets.map_or(s, |n| n.min(s));

    // IDs assigned BEFORE shuffling (ID = generate_id(def_id, supported_rank))
    let mut items: Vec<(usize /* r_sup */, u64 /* pre_id */)> = (0..s)
        .map(|r_sup| {
            let seq_num = r_sup as u64;
            let id = generate_id(&urapi_def_id.to_string(), &seq_num);
            (r_sup, id)
        })
        .collect();

    // Shuffle all ranks+ids to exactly mirror "filter -> shuffle all -> take K"
    let mut rng = StdRng::seed_from_u64(seed);
    items.shuffle(&mut rng);

    // Build only the first K in that shuffled order, stamping the precomputed IDs
    let mut out = Vec::with_capacity(k);
    for (r_sup, pre_id) in items.into_iter().take(k) {
        let mut tree = build_supported_by_rank(root, g, &mut tot, &mut sup, r_sup);
        tree.id = pre_id;
        out.push(tree);
    }

    out
}

fn surf_get_fuzz_trees(
    current_node: NodeIndex,
    ao_tree: &Graph<SurfTreeNode, ()>,
) -> Vec<FuzzTree>
{
    let current_node_data = ao_tree[current_node].clone();
    let mut trees = Vec::<FuzzTree>::new();
    if ao_tree.neighbors(current_node).next().is_none(){
        let mut leaf_tree: Graph<SurfTreeNode, ()> = Graph::new();
        let leaf_root_node = leaf_tree.add_node(current_node_data);
        let surf_tree = FuzzTree::new(leaf_root_node, leaf_tree);
        trees.push(surf_tree);
    }
    else if surf_is_or_node(&current_node_data){
        for neighbor in ao_tree.neighbors(current_node){
            let mut neighbor_trees = surf_get_fuzz_trees(neighbor, ao_tree);
            for neighbor_tree in neighbor_trees.iter_mut(){
                surf_add_new_root(neighbor_tree, &current_node_data)
            }
            trees.extend(neighbor_trees);
        }
    }
    else{ // surf_is_and_node
        for neighbor in ao_tree.neighbors(current_node){
            let mut neighbor_trees = surf_get_fuzz_trees(neighbor, ao_tree);
            if trees.is_empty(){
                for neighbor_tree in neighbor_trees.iter_mut(){
                    surf_add_new_root(neighbor_tree, &current_node_data)
                }
                trees = neighbor_trees;
            }
            else{
                let mut new_trees = Vec::<FuzzTree>::new();
                for tree in trees.iter(){
                    for neighbor_tree in neighbor_trees.iter(){
                        new_trees.push(surf_add_subtree(tree, neighbor_tree))
                    } 
                }
                trees = new_trees;
            }
        }
    }
    trees
}

fn surf_add_new_root(
    surf_ao_tree: &mut FuzzTree,
    new_root_data: &SurfTreeNode,
){
    let ao_tree_graph = &mut surf_ao_tree.tree;
    let ao_tree_root = surf_ao_tree.root.unwrap();
    let new_root_node = ao_tree_graph.add_node(new_root_data.clone());
    ao_tree_graph.add_edge(new_root_node, ao_tree_root, ());
    surf_ao_tree.root = Some(new_root_node);

}

fn surf_is_complex_variant(variant_node: NodeIndex, graph: &Graph<SurfTreeNode, ()>) -> bool{
    let mut dfs = Dfs::new(&graph, variant_node);
    while let Some(next_node) = dfs.next(&graph) {
        if surf_is_complex_node(&graph[next_node]){
            return true;
        }
    }
    false
}

fn surf_is_or_node(surf_tree_node: &SurfTreeNode) -> bool{
    match surf_tree_node{
        SurfTreeNode::Struct{..}
        | SurfTreeNode::Enum{..}
        | SurfTreeNode::ComplexVariants
        //| SurfTreeNode::Generic {..}
        //| SurfTreeNode::GenericCandidates
        | SurfTreeNode::DynTrait => true,
        _ => false,
    }
}

fn surf_is_complex_node(surf_tree_node: &SurfTreeNode) -> bool{
    match surf_tree_node{
        SurfTreeNode::Struct{..}
        | SurfTreeNode::Enum{..} 
        | SurfTreeNode::Generic{..}
        | SurfTreeNode::Reference(_)
        | SurfTreeNode::ReferencedSlice(_)
        | SurfTreeNode::DynTrait => true,
        _ => false,
    }
}

fn surf_is_unsupported_node(surf_tree_node: &SurfTreeNode) -> bool{
    match surf_tree_node{
        SurfTreeNode::Todo(_) => true,
        SurfTreeNode::Trait{name, is_unsafe, ..} => {
            if name == "*Unresolved*"{
                return true;
            }
            if *is_unsafe{
                match env::var("SURF_DISABLE_UNSAFE_TRAITS"){
                    Ok(_) => return true, // force prune
                    _ => return false
                }
            }
            false
        },

         
        | SurfTreeNode::TraitFn{name, is_unsafe,..} => {
            if name == "*Unresolved*"{
                return true;
            }
            if *is_unsafe{
                match env::var("SURF_DISABLE_UNSAFE_TRAITS"){
                    Ok(_) => return true, // force prune
                    _ => return false
                }
            }
            false
        }

        SurfTreeNode::Api{name, ..} 
        | SurfTreeNode::Constructor{name, ..} => {
            name == "*Unresolved*"
        },
        _ => false,
    }
}

fn surf_is_skip_node(surf_tree_node: &SurfTreeNode) -> bool{
    match surf_tree_node{
        SurfTreeNode::Constructor { name, .. } => name.starts_with("std::fmt::Formatter"),
        SurfTreeNode::ConstructorImplicitGenerics => {true},
        SurfTreeNode::ApiImplicitGenerics => {true},
        SurfTreeNode::ApiOutput => {true},
        _ => false,
    }
}

/* -------------------------------------------------------------------------
                        HARNESS OUTPUT FUNCTIONS
--------------------------------------------------------------------------*/

fn surf_compile_harnesses(target_pair: &TargetPair) -> bool{
    let fuzz_target= &target_pair.fuzz_target;
    let fuzz_target_name = &fuzz_target.fuzz_target_id.target_name;
    surf_create_initial_input(&fuzz_target_name);
    surf_create_fuzzer_input_len(&fuzz_target_name, fuzz_target.fuzz_target_harness.fuzzer_slice_len.unwrap());
    
    let working_path = &SURF_WORKING_PATH.clone();
    let fuzz_target_dir = PathBuf::from(working_path).join(format!("deepSURF/fuzz/no_llm/compilable/{fuzz_target_name}"));
    let asan_build_status = surf_cargo_build_fuzz_target_asan(&fuzz_target_dir);
    let non_asan_build_status = surf_cargo_build_fuzz_target_non_asan(&fuzz_target_dir);
    surf_remove_target_dir(&fuzz_target_dir);
    asan_build_status || non_asan_build_status
}

fn surf_check_harness(target_pair: &TargetPair) -> bool{
    let fuzz_target= &target_pair.fuzz_target;
    let replay_target = &target_pair.replay_target;
    let fuzz_target_name = &fuzz_target.fuzz_target_id.target_name;
    let replay_target_name = &replay_target.replay_target_id.target_name;
    let req_deps = fuzz_target.fuzz_target_harness.req_deps.clone().unwrap();
    surf_create_target_dirs(fuzz_target_name, replay_target_name);
    let (fuzz_toml_contents, replay_toml_contents) = surf_get_toml_contents(fuzz_target_name, replay_target_name, Some(&req_deps));
    surf_create_toml(fuzz_target_name, replay_target_name, &fuzz_toml_contents, &replay_toml_contents);
    surf_create_target_src_dirs(fuzz_target_name, replay_target_name);
    surf_create_fuzz_target(&fuzz_target_name, &fuzz_target.fuzz_target_harness.harness);
    surf_cargo_check_fuzz_target(&fuzz_target_name)
}

fn surf_create_fuzz_and_replay_dirs(){
    surf_create_compilable_dirs();
}

fn surf_move_to_uncompilable(target_pair: &TargetPair){
    let working_path = &SURF_WORKING_PATH.clone();
    let fuzz_target_name = &target_pair.fuzz_target.fuzz_target_id.target_name;
    let replay_target_name = &target_pair.replay_target.replay_target_id.target_name;
    let non_compilable_fuzz_dir_path = format!("{working_path}/deepSURF/fuzz/no_llm/non_compilable");
    let non_compilable_replay_dir_path = format!("{working_path}/deepSURF/replay/no_llm/non_compilable");
    let fuzz_target_dir_path = format!("{working_path}/deepSURF/fuzz/no_llm/compilable/{fuzz_target_name}");
    let replay_target_dir_path = format!("{working_path}/deepSURF/replay/no_llm/compilable/{replay_target_name}");
    let fuzz_target_non_compilable_path = format!("{non_compilable_fuzz_dir_path}/{fuzz_target_name}");
    let replay_target_non_compilable_path = format!("{non_compilable_replay_dir_path}/{replay_target_name}");

    let src = Path::new(&fuzz_target_dir_path);
    let dst = Path::new(&fuzz_target_non_compilable_path);
    if dst.exists() {
        fs::remove_dir_all(dst).expect("Cannot remove the directory!");
    }
    fs::rename(src, dst).expect("Cannot execute fs::rename!");
    
    let src = Path::new(&replay_target_dir_path);
    let dst = Path::new(&replay_target_non_compilable_path);
    if dst.exists() {
        fs::remove_dir_all(dst).expect("Cannot remove the directory!");
    }
    fs::rename(src, dst).expect("Cannot execute fs::rename!");
}

fn surf_remove_uncompilable(target_pair: &TargetPair){
    let working_path = &SURF_WORKING_PATH.clone();
    let fuzz_target_name = &target_pair.fuzz_target.fuzz_target_id.target_name;
    let replay_target_name = &target_pair.replay_target.replay_target_id.target_name;
    let fuzz_target_dir_path = format!("{working_path}/deepSURF/fuzz/no_llm/compilable/{fuzz_target_name}");
    let replay_target_dir_path = format!("{working_path}/deepSURF/replay/no_llm/compilable/{replay_target_name}");

    let src = Path::new(&fuzz_target_dir_path);
    if src.exists() {
        fs::remove_dir_all(src).expect("Cannot remove the directory!");
    }    
    let src = Path::new(&replay_target_dir_path);
    if src.exists() {
        fs::remove_dir_all(src).expect("Cannot remove the directory!");
    }
}

fn surf_get_bin_name(project_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    // Construct the path to Cargo.toml in the project directory.
    let cargo_toml_path = project_path.join("Cargo.toml");
    
    // Read the contents of Cargo.toml.
    let content = fs::read_to_string(cargo_toml_path)?;
    
    // Parse the TOML content.
    let value: Value = toml::from_str(&content)?;
    
    // Check if there's an explicit [[bin]] section.
    if let Some(bins) = value.get("bin").and_then(|v| v.as_array()) {
        for bin in bins {
            if let Some(name) = bin.get("name").and_then(|v| v.as_str()) {
                return Ok(name.to_string());
            }
        }
    }
    
    // If no [[bin]] section exists, use the package name as the default binary name.
    if let Some(package) = value.get("package") {
        if let Some(name) = package.get("name").and_then(|v| v.as_str()) {
            return Ok(name.to_string());
        }
    }
    
    Err("No binary name found in Cargo.toml".into())
}

fn surf_cargo_check_fuzz_target(fuzz_target_name: &str) -> bool{
    let working_path = &SURF_WORKING_PATH.clone();
    let fuzz_target_dir_path = format!("{working_path}/deepSURF/fuzz/no_llm/compilable/{fuzz_target_name}");
    //println!("\n[{fuzz_target_name}]: cargo afl check...");
    let cargo_check_output = Command::new("cargo")
        .arg("afl")
        .arg("check")
        .arg("-j")
        .arg(CARGO_JOBS.to_string())
        .current_dir(fuzz_target_dir_path)
        .env("RUSTFLAGS", "-Zub-checks=no -C debuginfo=0")
        .output()
        .expect("Failed to execute cargo afl check");

    // Check if the command was successful
    if cargo_check_output.status.success() {
        //println!("[{fuzz_target_name}]: cargo check -> Ok!");
    } else {
        //eprintln!("[{fuzz_target_name}]: cargo check -> Err.");
        //let stderr = String::from_utf8_lossy(&cargo_check_output.stderr);
        //eprintln!("Error: {}", stderr);
    }
    cargo_check_output.status.success()
}

fn surf_cargo_build_fuzz_target_asan(fuzz_target_dir: &PathBuf) -> bool{
    //println!("[{fuzz_target_name}]: cargo afl build w/ ASAN...");
    let bin_path = format!("{}/bins", fuzz_target_dir.to_string_lossy().to_string());
    let path = Path::new(&bin_path);
    if !path.exists() {
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {bin_path}"));
    }
    let mut cmd = Command::new("cargo");
    cmd.arg("afl")
        .arg("build")
        .arg("-j")
        .arg(CARGO_JOBS.to_string())
        .current_dir(fuzz_target_dir)
        .env("RUSTDOCFLAGS", "-Zsanitizer=address");

    if env::var("SURF_DISABLE_TARGET_FLAG").is_err() {
        cmd.arg("--target").arg("x86_64-unknown-linux-gnu");
    }
    if let Ok(_) = env::var("SURF_ENABLE_LINE_COVERAGE"){
        cmd.env("RUSTFLAGS", "-Zsanitizer=address -Zub-checks=no -C instrument-coverage");
    }
    else{
        cmd.env("RUSTFLAGS", "-Zsanitizer=address -Zub-checks=no");
    }
    let cargo_build_output = cmd.output().expect("Failed to execute cargo afl build");

    if cargo_build_output.status.success() {
        //println!("[{fuzz_target_name}_asan]: cargo afl build -> Ok!");
        let fuzz_target_name = surf_get_bin_name(&fuzz_target_dir).unwrap();
        let target_subdir = if env::var("SURF_DISABLE_TARGET_FLAG").is_ok() {
            "target/debug"
        } else {
            "target/x86_64-unknown-linux-gnu/debug"
        };
        let fuzz_target_bin_path = format!(
            "{}/{}/{}",
            fuzz_target_dir.to_string_lossy(),
            target_subdir,
            fuzz_target_name
        );
        let fuzz_target_new_bin_name = format!("{fuzz_target_name}_asan");
        surf_move_to_bins(&fuzz_target_bin_path, &bin_path, &fuzz_target_new_bin_name);
    } else {
        //eprintln!("[{fuzz_target_name}_asan]: cargo afl build -> Err!");
        //let stderr = String::from_utf8_lossy(&cargo_build_output.stderr);
        //eprintln!("Error: {}", stderr);
    }
    cargo_build_output.status.success()
}

fn surf_cargo_build_fuzz_target_non_asan(fuzz_target_dir: &PathBuf) -> bool{
    //println!("[{fuzz_target_name}]: cargo afl build w/o ASAN...");    
    let bin_path = format!("{}/bins", fuzz_target_dir.to_string_lossy().to_string());
    let path = Path::new(&bin_path);
    if !path.exists() {
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {bin_path}"));
    }

    let dict2file_path = format!("{}/dict.txt", fuzz_target_dir.to_string_lossy().to_string());
    let path = Path::new(&dict2file_path);
    if path.exists() {
        fs::remove_file(path).expect("Cannot remove the file");
    }
    let mut cmd = Command::new("cargo");
    cmd.arg("afl")
        .arg("build")
        .arg("-j")
        .arg(CARGO_JOBS.to_string())
        .current_dir(fuzz_target_dir)
        //.env("AFL_LLVM_DICT2FILE", dict2file_path)
        .env("CARGO_INCREMENTAL", "0");
    
    if env::var("SURF_DISABLE_TARGET_FLAG").is_err() {
        cmd.arg("--target").arg("x86_64-unknown-linux-gnu");
    }
    if let Ok(_) = env::var("SURF_ENABLE_LINE_COVERAGE"){
        cmd.env("RUSTFLAGS", "-Zub-checks=no -C instrument-coverage");
    }
    else{
        cmd.env("RUSTFLAGS", "-Zub-checks=no");
    }
    let cargo_build_output = cmd.output().expect("Failed to execute cargo afl build");

    if cargo_build_output.status.success() {
        //println!("[{fuzz_target_name}_non_asan]: cargo afl build -> Ok!");
        let fuzz_target_name = surf_get_bin_name(&fuzz_target_dir).unwrap();
        let target_subdir = if env::var("SURF_DISABLE_TARGET_FLAG").is_ok() {
            "target/debug"
        } else {
            "target/x86_64-unknown-linux-gnu/debug"
        };
        let fuzz_target_bin_path = format!(
            "{}/{}/{}",
            fuzz_target_dir.to_string_lossy(),
            target_subdir,
            fuzz_target_name
        );
        let fuzz_target_new_bin_name = format!("{fuzz_target_name}_non_asan");
        surf_move_to_bins(&fuzz_target_bin_path, &bin_path, &fuzz_target_new_bin_name);
    } else {
        //eprintln!("[{fuzz_target_name}_non_asan]: cargo afl build -> Err!");
        //let stderr = String::from_utf8_lossy(&cargo_build_output.stderr);
        //eprintln!("Error: {}", stderr);
    }
    cargo_build_output.status.success()
}

fn surf_cargo_build_replay_target_asan(replay_target_name: &str){
    let working_path = &SURF_WORKING_PATH.clone();
    let replay_target_dir_path = format!("{working_path}/deepSURF/replay/no_llm/compilable/{replay_target_name}");
    println!("[{replay_target_name}]: cargo build w/ ASAN...");
    let bin_path = format!("{replay_target_dir_path}/bins");
    let path = Path::new(&bin_path);
    if !path.exists() {
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {bin_path}"));
    }
    
    let cargo_build_output = Command::new("cargo")
        .arg("build")
        .arg("-j")
        .arg(CARGO_JOBS.to_string())
        .arg("--target")
        .arg("x86_64-unknown-linux-gnu")
        .current_dir(replay_target_dir_path.clone())
        .env("RUSTFLAGS", "-Zsanitizer=address -Zub-checks=no -Copt-level=3 -Cdebuginfo=0")
        .env("RUSTDOCFLAGS", "-Zsanitizer=address")
        .output()
        .expect("Failed to execute cargo cargo");
    if cargo_build_output.status.success() {
        println!("[{replay_target_name}_asan]: cargo build -> Ok!");
        let replay_target_bin_path = format!("{replay_target_dir_path}/target/x86_64-unknown-linux-gnu/debug/{replay_target_name}");
        let replay_target_new_bin_name = format!("{replay_target_name}_asan");
        surf_move_to_bins(&replay_target_bin_path, &bin_path, &replay_target_new_bin_name);
    } else {
        eprintln!("[{replay_target_name}_asan]: cargo build -> Err!");
        //let stderr = String::from_utf8_lossy(&cargo_build_output.stderr);
        //eprintln!("Error: {}", stderr);
    }
}

fn surf_cargo_build_replay_target_non_asan(replay_target_name: &str){
    let working_path = &SURF_WORKING_PATH.clone();
    let replay_target_dir_path = format!("{working_path}/deepSURF/replay/no_llm/compilable/{replay_target_name}");
    println!("[{replay_target_name}]: cargo build w/o ASAN...");
    let bin_path = format!("{replay_target_dir_path}/bins");
    let path = Path::new(&bin_path);
    if !path.exists() {
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {bin_path}"));
    }
    
    let cargo_build_output = Command::new("cargo")
        .arg("build")
        .arg("-j")
        .arg(CARGO_JOBS.to_string())
        .current_dir(replay_target_dir_path.clone())
        .env("RUSTFLAGS", "-Zub-checks=no -Copt-level=3 -Cdebuginfo=0")
        .output()
        .expect("Failed to execute cargo cargo");
    if cargo_build_output.status.success() {
        println!("[{replay_target_name}_non_asan]: cargo build -> Ok!");
        let replay_target_bin_path = format!("{replay_target_dir_path}/target/debug/{replay_target_name}");
        let replay_target_new_bin_name = format!("{replay_target_name}_non_asan");
        surf_move_to_bins(&replay_target_bin_path, &bin_path, &replay_target_new_bin_name);
    } else {
        eprintln!("[{replay_target_name}_non_asan]: cargo build -> Err!");
        //let stderr = String::from_utf8_lossy(&cargo_build_output.stderr);
        //eprintln!("Error: {}", stderr);
    }
}

fn surf_move_to_bins(fuzz_target_bin_path: &str, bins_dir_path: &str, fuzz_target_new_bin_name: &str){
    let fuzz_target_new_bin_path = format!("{bins_dir_path}/{fuzz_target_new_bin_name}");
    let src = Path::new(fuzz_target_bin_path);
    let dst = Path::new(&fuzz_target_new_bin_path);
    if dst.exists() {
        fs::remove_file(dst).expect("Cannot remove the file");
    }
    fs::rename(src, dst).expect("Cannot execute fs::rename!");
}

fn surf_remove_target_dir(fuzz_target_dir: &PathBuf){
    fs::remove_dir_all(fuzz_target_dir.join("target")).expect("Unable to delete target directory");
}

fn surf_get_current_fuzz_target_name(crate_name: &str, harness_id: u64) -> String{
    format!("{crate_name}_fuzz_{harness_id}")
}

fn surf_get_current_replay_target_name(crate_name: &str, harness_id: u64) -> String{
    format!("{crate_name}_replay_{harness_id}")
}

fn surf_create_compilable_dirs(){
    let working_path = &SURF_WORKING_PATH.clone();
    let replay_dir_path = format!("{working_path}/deepSURF/replay/no_llm/compilable");
    let path = Path::new(&replay_dir_path);
    if path.exists() && path.is_dir() {
        for entry in fs::read_dir(path).expect("Failed to read directory contents") {
            let entry = entry.expect("Failed to read directory entry");
            let entry_path = entry.path();
            if entry_path.is_dir() {
                fs::remove_dir_all(entry_path).expect("Failed to remove directory");
            } else {
                fs::remove_file(entry_path).expect("Failed to remove file");
            }
        }
    }
    fs::create_dir_all(path).expect("Failed to create directory");
    
    let fuzz_dir_path = format!("{working_path}/deepSURF/fuzz/no_llm/compilable");
    let path = Path::new(&fuzz_dir_path);
    if path.exists() && path.is_dir() {
        for entry in fs::read_dir(path).expect("Failed to read directory contents") {
            let entry = entry.expect("Failed to read directory entry");
            let entry_path = entry.path();
            if entry_path.is_dir() {
                fs::remove_dir_all(entry_path).expect("Failed to remove directory");
            } else {
                fs::remove_file(entry_path).expect("Failed to remove file");
            }
        }
    }
    fs::create_dir_all(path).expect("Failed to create directory");
}

fn surf_create_non_compilable_dirs(){
    let working_path = &SURF_WORKING_PATH.clone();
    let replay_dir_path = format!("{working_path}/deepSURF/replay/no_llm/non_compilable");
    let path = Path::new(&replay_dir_path);
    if path.exists() && path.is_dir() {
        for entry in fs::read_dir(path).expect("Failed to read directory contents") {
            let entry = entry.expect("Failed to read directory entry");
            let entry_path = entry.path();
            if entry_path.is_dir() {
                fs::remove_dir_all(entry_path).expect("Failed to remove directory");
            } else {
                fs::remove_file(entry_path).expect("Failed to remove file");
            }
        }
    }
    fs::create_dir_all(path).expect("Failed to create directory");

    let fuzz_dir_path = format!("{working_path}/deepSURF/fuzz/no_llm/non_compilable");
    let path = Path::new(&fuzz_dir_path);
    if path.exists() && path.is_dir() {
        for entry in fs::read_dir(path).expect("Failed to read directory contents") {
            let entry = entry.expect("Failed to read directory entry");
            let entry_path = entry.path();
            if entry_path.is_dir() {
                fs::remove_dir_all(entry_path).expect("Failed to remove directory");
            } else {
                fs::remove_file(entry_path).expect("Failed to remove file");
            }
        }
    }
    fs::create_dir_all(path).expect("Failed to create directory");
}

fn surf_create_target_dirs(fuzz_target_name: &str, replay_target_name: &str){
    let working_path = &SURF_WORKING_PATH.clone();
    let fuzz_target_dir_path = format!("{working_path}/deepSURF/fuzz/no_llm/compilable/{}", &fuzz_target_name);
    let path = Path::new(&fuzz_target_dir_path);
    if !path.exists() {
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {fuzz_target_dir_path}"));
    }
}

fn surf_create_target_src_dirs(fuzz_target_name: &str, replay_target_name: &str){
    let working_path = &SURF_WORKING_PATH.clone();
    let fuzz_target_src_dir_path = format!("{working_path}/deepSURF/fuzz/no_llm/compilable/{}/src", &fuzz_target_name);
    let path = Path::new(&fuzz_target_src_dir_path);
    if !path.exists() {
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {fuzz_target_src_dir_path}"));
    }
}

fn surf_get_toml_contents(fuzz_target_name: &str, replay_target_name: &str, req_deps_opt: Option<&HashMap<String, SurfDepType>>) -> (String, String){
    let working_path = &SURF_WORKING_PATH.clone();
    let global_data_path = &GLOBAL_DATA_PATH.clone();
    let crate_name = &TARGET_CRATE_NAME.clone();
    let mut fuzz_toml_segs = Vec::<String>::new();
    fuzz_toml_segs.push(format!("[workspace]\n"));
    fuzz_toml_segs.push(format!("[package]"));
    fuzz_toml_segs.push(format!("name = \"{fuzz_target_name}\""));
    fuzz_toml_segs.push(format!("version = \"0.1.0\""));
    fuzz_toml_segs.push(format!("edition = \"2021\""));
    fuzz_toml_segs.push(format!("\n[[bin]]"));
    fuzz_toml_segs.push(format!("name = \"{fuzz_target_name}\""));
    fuzz_toml_segs.push(format!("path = \"src/{fuzz_target_name}.rs\""));
    fuzz_toml_segs.push(format!("\n[dependencies]"));
    fuzz_toml_segs.push(format!("afl = \"=0.15.7\""));
    fuzz_toml_segs.push(format!("home = \"=0.5.9\""));
    fuzz_toml_segs.push(format!("global_data = {{ path = \"{global_data_path}\" }}"));
    let features_str = {
        match env::var("SURF_ENABLE_FEATURES"){
            Ok(features) => {
                let features_vec: Vec<String> = features
                    .split(',')
                    .map(|s| format!("\"{}\"", s.trim()))
                    .collect();
                format!(", features = [{}]", features_vec.join(", "))
            },
            _ => format!(""),
        }
    };
    fuzz_toml_segs.push(format!("{crate_name} = {{ path = \"{working_path}\" {features_str}}}"));

    if let Ok(extra) = env::var("SURF_EXTRA_DEPS") {
        for raw in extra.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            fuzz_toml_segs.push(raw.to_string());
        }
    }

    if let Some(req_deps) = req_deps_opt{
        for (package_name, package_details) in req_deps{
            match package_details{
                SurfDepType::Version(version) => fuzz_toml_segs.push(format!("{package_name} = {version}")),
                SurfDepType::Path(path) => fuzz_toml_segs.push(format!("{package_name} = {{ path = {path} }}")),
                SurfDepType::Empty => {},
            }
        }
    }

    let mut replay_toml_segs = Vec::<String>::new();
    replay_toml_segs.push(format!("[workspace]\n"));
    replay_toml_segs.push(format!("[package]"));
    replay_toml_segs.push(format!("name = \"{replay_target_name}\""));
    replay_toml_segs.push(format!("version = \"0.1.0\""));
    replay_toml_segs.push(format!("edition = \"2021\""));
    replay_toml_segs.push(format!("\n[[bin]]"));
    replay_toml_segs.push(format!("name = \"{replay_target_name}\""));
    replay_toml_segs.push(format!("path = \"src/{replay_target_name}.rs\""));
    replay_toml_segs.push(format!("\n[dependencies]"));
    replay_toml_segs.push(format!("{crate_name} = {{ path = \"{working_path}\" }}"));
    replay_toml_segs.push(format!("global_data = {{ path = \"{global_data_path}\" }}"));
    if let Some(req_deps) = req_deps_opt{
        for (package_name, package_details) in req_deps{
            match package_details{
                SurfDepType::Version(version) => replay_toml_segs.push(format!("{package_name} = {version}")),
                SurfDepType::Path(path) => replay_toml_segs.push(format!("{package_name} = {{ path = {path} }}")),
                SurfDepType::Empty => {},
            }
        }
    }
    (fuzz_toml_segs.join("\n"), replay_toml_segs.join("\n"))
}

fn surf_create_toml(fuzz_target_name: &str, replay_target_name: &str, fuzz_toml_contents: &str, replay_toml_contents: &str){
    let working_path = &SURF_WORKING_PATH.clone();
    let fuzz_toml_path = format!("{working_path}/deepSURF/fuzz/no_llm/compilable/{}/Cargo.toml", &fuzz_target_name);
    let mut file = File::create(fuzz_toml_path).unwrap();
    file.write_all(fuzz_toml_contents.as_bytes()).expect("Unable to write to Cargo.toml file");
    file.flush().expect("Unable to flush to Cargo.toml file");
}

fn surf_create_fuzz_target(fuzz_target_name: &str, fuzz_target_harness: &str){
    let working_path = &SURF_WORKING_PATH.clone();
    let fuzz_target_src_path = format!("{working_path}/deepSURF/fuzz/no_llm/compilable/{fuzz_target_name}/src/{fuzz_target_name}.rs");
    let mut file = File::create(fuzz_target_src_path).unwrap();
    file.write_all(fuzz_target_harness.as_bytes()).expect("Unable to write to fuzz_target file");
    file.flush().expect("Unable to flush to fuzz_target file");
}

fn surf_create_replay_target(replay_target_name: &str, replay_target_harness: &str){
    let working_path = &SURF_WORKING_PATH.clone();
    let replay_target_src_path = format!("{working_path}/deepSURF/replay/no_llm/compilable/{replay_target_name}/src/{replay_target_name}.rs");
    let mut file = File::create(replay_target_src_path).unwrap();
    file.write_all(replay_target_harness.as_bytes()).expect("Unable to write to replay_target file");
    file.flush().expect("Unable to flush to replay_target file");
}

fn surf_create_initial_input(fuzz_target_name: &str){
    //println!("Populating input file.");
    let working_path = &SURF_WORKING_PATH.clone();
    let initial_input_dir_path = format!("{working_path}/deepSURF/fuzz/no_llm/compilable/{fuzz_target_name}/input");
    let path = Path::new(&initial_input_dir_path);
    if !path.exists() {
        fs::create_dir_all(path).expect(&format!("Failed to create folder: {initial_input_dir_path}"));
    }
    let initial_input_path = format!("{initial_input_dir_path}/input0");
    //let inital_input = "0".repeat(fuzzer_slice_len.checked_sub(1).unwrap_or(fuzzer_slice_len));
    let mut file = File::create(initial_input_path).unwrap();
    file.write_all("0".as_bytes()).expect("Unable to write to input0 file");
    file.flush().expect("Unable to flush to input0 file");
}

fn surf_create_fuzzer_input_len(fuzz_target_name: &str, fuzzer_slice_len: usize){
    //println!("Populating fuzzer len file.");
    let working_path = &SURF_WORKING_PATH.clone();
    let fuzzer_input_len_dir_path = format!("{working_path}/deepSURF/fuzz/no_llm/compilable/{fuzz_target_name}");
    let fuzzer_input_len_path = format!("{fuzzer_input_len_dir_path}/len");
    let mut file = File::create(fuzzer_input_len_path).unwrap();
    let doubled_fuzzer_slice_len = fuzzer_slice_len * 2;
    file.write_all(doubled_fuzzer_slice_len.to_string().as_bytes()).expect("Unable to write to input0 file");
    file.flush().expect("Unable to flush to input0 file");
}

/* -------------------------------------------------------------------------
                        HARNESS GENERATION FUNCTIONS
--------------------------------------------------------------------------*/

fn surf_write_output_body(
    output_data: &str,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap
){
    let indent_str = "\t".repeat(indent_level);
    match output_body_locator {
        OutputBodyLocator::Main => output_body_map.main.push(format!("{indent_str}{output_data}")),
        OutputBodyLocator::Traits(trait_name, custom_type) => {
            output_body_map
                        .traits
                        .get_mut(trait_name).unwrap()
                        .get_mut(custom_type).unwrap()
                        .push(format!("{indent_str}{output_data}"))
        },
        OutputBodyLocator::Closures(closure_name) => {
            output_body_map
                        .closures
                        .get_mut(closure_name).unwrap()
                        .push(format!("{indent_str}{output_data}"));
        },
    }
}

fn surf_get_fuzz_harness(urapi_def_id: &str, surf_fuzz_tree: &FuzzTree, mode: &HarnessGenerationMode) -> (Harness, Harness){
    // Dependencies need to import in Fuzz Target's Cargo.toml
    let mut req_deps = HashMap::<String, SurfDepType>::new();

    // Add URAPI_DefId as comment
    let mut harness_urapi_def_id = Vec::<String>::new();
    harness_urapi_def_id.push(format!("//{urapi_def_id}"));
    
    // Fuzz Target Header
    let mut harness_header = Vec::<String>::new();
    if let Ok(_) = env::var("SURF_DISABLE_UNSAFE_TRAITS"){
        harness_header.push("#![forbid(unsafe_code)]".to_string());
    }
    
    harness_header.push("#[macro_use]".to_string());
    harness_header.push("extern crate afl;\n".to_string());

    // Global Use Statements
    let mut use_stmts = Vec::<String>::new();
    use_stmts.push(format!("{}::*", TARGET_CRATE_NAME.to_string().replace("-", "_")));
    
    use_stmts.push(format!("global_data::*"));
    use_stmts.push(format!("std::str::FromStr"));
    use_stmts.push(format!("std::ops::{{Deref, DerefMut, Index, IndexMut}}"));

    // Global Custom Structs
    //let custom_type_id -> (custom_name, inner_type);
    let mut custom_structs = HashMap::<String, (String, String)>::new();

    // Custom Closures
    let mut custom_closures = Vec::<String>::new();

    // With the panic flags should be the outputs too.
    // Of the custom traits implementations.

    // Main Function (where the fuzzing happens)
    // HashMap::<func_def_id, HashMap<Generic, CustomType>> unique per fuzz tree
    let mut generic_subst_map = HashMap::<String, SubstType>::new();
    let mut custom_closure_map = HashMap::<String, String>::new();
    let mut output_body_map = OutputBodyMap::new();
    let output_body_locator = OutputBodyLocator::Main;

    let fuzzer_slice_len = surf_get_main_fn_harness(
                                                        surf_fuzz_tree,
                                                        &mode,
                                                        &mut use_stmts,
                                                        &mut req_deps,
                                                        &mut custom_structs,
                                                        &mut custom_closures,
                                                        &mut generic_subst_map,
                                                        &mut custom_closure_map,
                                                        &output_body_locator,
                                                        &mut output_body_map,
    );
    
    
    for use_stmt in use_stmts.iter_mut(){
        *use_stmt = format!("use {};", &use_stmt);
    }
    if !use_stmts.is_empty() {use_stmts.push("".to_string());}


    let mut common_code = use_stmts.clone();
    //common_code.extend(mutable_statics);
    for (custom_struct_name, custom_type_inner_type) in custom_structs.values(){
        common_code.push(format!("struct {custom_struct_name}({custom_type_inner_type});"));
    }

    common_code.push(String::from(""));
    for trait_def in output_body_map.traits.values(){
        for custom_type_impl in trait_def.values(){
            common_code.extend(custom_type_impl.clone());
        }
        common_code.push(String::from(""));
    }

    for closure_body in output_body_map.closures.values(){
        common_code.extend(closure_body.clone());
        common_code.push(String::from(""));
    }

    let mut replay_harness = common_code.clone();
    let replay_main_body = surf_replay_main_body(&output_body_map.main);
    replay_harness.extend(replay_main_body);
    surf_add_converters(&mut replay_harness);
    surf_add_data_reader(&mut replay_harness);
   
    // Write to fuzz target
    let mut fuzz_harness = harness_urapi_def_id.clone();
    fuzz_harness.extend(harness_header);
    fuzz_harness.extend(common_code);
    fuzz_harness.extend(output_body_map.main);
    surf_add_converters(&mut fuzz_harness);
    //println!("{:#?}", generic_subst_map);
    (
        Harness::new(fuzz_harness.join("\n"), Some(fuzzer_slice_len), Some(req_deps)),
        Harness::new(replay_harness.join("\n"), None, None)
    )
}

fn surf_get_crate_name(item_full_name: &str) -> &str{
    item_full_name.split("::").next().unwrap_or("")
}

fn surf_get_main_fn_harness(
    surf_fuzz_tree: &FuzzTree,
    mode: &HarnessGenerationMode,
    use_stmts: &mut Vec<String>,
    req_deps: &mut HashMap<String, SurfDepType>,
    custom_structs: &mut HashMap<String, (String, String)>,
    custom_closures: &mut Vec<String>,
    generic_subst_map: &mut HashMap<String, SubstType>,
    custom_closure_map: &mut HashMap<String, String>,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> usize
{
    surf_write_output_body("fn main (){", 0, output_body_locator, output_body_map);
    surf_write_output_body("fuzz_nohook!(|data: &[u8]| {", 1, output_body_locator, output_body_map);
    let fuzzer_slice_len = surf_get_main_fn_body_harness(
                                    surf_fuzz_tree,
                                    &mode,
                                    use_stmts,
                                    req_deps,
                                    custom_structs,
                                    custom_closures,
                                    generic_subst_map,
                                    custom_closure_map,
                                    1,
                                    output_body_locator,
                                    output_body_map,
    );
    surf_write_output_body("});", 1, output_body_locator, output_body_map);
    surf_write_output_body("}", 0, output_body_locator, output_body_map);
    fuzzer_slice_len
}

fn surf_replay_main_body(main_body: &Vec<String>) -> Vec<String>{
    let mut replay_body = main_body.clone();
    replay_body.drain(1..2);
    replay_body.drain(replay_body.len() - 2 .. replay_body.len() - 1);

    for line in replay_body.iter_mut(){
        if let Some(stripped) = line.strip_prefix('\t') {
            *line = stripped.to_string()
        }
    }
    replay_body.insert(1, format!("\tlet _content = _read_data();"));
    replay_body.insert(2, format!("\tlet data = &_content;"));
    replay_body.insert(3, format!("\tprintln!(\"data = {{:?}}\", data);"));
    replay_body.insert(4, format!("\tprintln!(\"data len = {{:?}}\", data.len());"));
    replay_body
}

fn surf_get_main_fn_body_harness(
    surf_fuzz_tree: &FuzzTree,
    mode: &HarnessGenerationMode,
    use_stmts: &mut Vec<String>,
    req_deps: &mut HashMap<String, SurfDepType>,
    custom_structs: &mut HashMap<String, (String, String)>,
    custom_closures: &mut Vec<String>,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    custom_closure_map: &mut HashMap::<String, String>,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> usize
{
    // Tree/Root handles
    let fuzz_tree = &surf_fuzz_tree.tree;
    let fuzz_tree_root = surf_fuzz_tree.root.unwrap();

    // The size of the fuzzer's byte slice
    let mut fuzzer_byte_slice_len: usize = 0;
    let mut local_var_count: usize = 0;
    
    surf_get_node_harness(
                fuzz_tree_root,
                            false,
                            fuzz_tree,
                            &mode,
                            use_stmts,
                            req_deps,
                            custom_structs,
                            custom_closures,
                            &mut fuzzer_byte_slice_len,
                            &mut local_var_count,
                            generic_subst_map,
                            custom_closure_map,
                            indent_level+1,
                            output_body_locator,
                            output_body_map,
    );

    // Insert the data len bound
    output_body_map.main.insert(2, format!("\t\tif data.len() < {} {{return;}}", fuzzer_byte_slice_len*2));
    output_body_map.main.insert(3, format!("\t\tset_global_data(data);"));
    output_body_map.main.insert(4, format!("\t\tlet global_data = get_global_data();"));
    output_body_map.main.insert(5, format!("\t\tlet GLOBAL_DATA = global_data.first_half;"));
    fuzzer_byte_slice_len
}

fn surf_declare_local(
    local_type: &str,
    is_mutable: bool,
    is_bounded: Option<usize>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let type_converter_call = surf_get_primitive_type_converter_call(local_type, *fuzzer_byte_slice_len, is_bounded);
    let mutability = {
        match is_mutable
        | is_bounded.is_some(){
            true => "mut ",
            false => "",
        }
    };
    let output_data = format!("let {mutability}t_{} = {type_converter_call}", *local_var_count);
    surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *fuzzer_byte_slice_len += surf_get_size_of_type(local_type);
    *local_var_count += 1;
    local_name
}

fn surf_declare_ref(
    local_var_name: &str,
    is_mutable_var: bool,
    is_mutable_ref: bool,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let var_mutability = {
        match is_mutable_var{
            true => "mut ",
            false => "",
        }
    };
    let ref_mutability = {
        match is_mutable_ref{
            true => "mut ",
            false => "",
        }
    };
    let ref_decl = format!("let {ref_mutability}t_{} = &{ref_mutability}{local_var_name};", *local_var_count);
    surf_write_output_body(&ref_decl, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_raw_ptr(
    local_var_name: &str,
    //value_type_name: &str,
    is_mutable_var: bool,
    is_mutable_raw_ptr: bool,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let var_mutability = {
        match is_mutable_var{
            true => "mut ",
            false => "",
        }
    };
    let raw_ptr_mutability = {
        match is_mutable_raw_ptr{
            true => "mut ",
            false => "const ",
        }
    };
    let raw_ptr_conv = format!("let {var_mutability}t_{} = {local_var_name} as *{raw_ptr_mutability}usize as *{raw_ptr_mutability}_;", *local_var_count);
    surf_write_output_body(&raw_ptr_conv, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_local_str(
    is_mutable: bool,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let size_var = surf_declare_local("u8",
                                                false,
                                                Some(STR_SIZE),
                                                fuzzer_byte_slice_len,
                                                local_var_count,
                                                indent_level,
                                                output_body_locator,
                                                output_body_map);
    let str_converter_call = surf_get_str_type_converter_call(*fuzzer_byte_slice_len, &size_var);
    let output_data = format!("let {mutability}t_{} = {str_converter_call}", *local_var_count);
    surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *fuzzer_byte_slice_len += surf_get_size_of_type("str");
    *local_var_count += 1;
    local_name
}

fn surf_declare_local_string(
    local_var_name: &str,
    is_mutable: bool,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let string_decl = format!("let {mutability}t_{} = String::from({local_var_name});", *local_var_count);
    surf_write_output_body(&string_decl, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_local_vector(
    current_node: NodeIndex,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    mode: &HarnessGenerationMode,
    use_stmts: &mut Vec<String>,
    req_deps: &mut HashMap<String, SurfDepType>,
    custom_structs: &mut HashMap<String, (String, String)>,
    custom_closures: &mut Vec<String>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    custom_closure_map: &mut HashMap::<String, String>,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let size_var = surf_declare_local("u8",
                                                false,
                                                Some(VECTOR_SIZE),
                                                fuzzer_byte_slice_len,
                                                local_var_count,
                                                indent_level,
                                                output_body_locator,
                                                output_body_map);

    let local_vector = format!("t_{}", *local_var_count);
    let mut output_data = format!("// Start vector declaration.");
    surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    output_data = format!("let mut t_{} = std::vec::Vec::with_capacity({VECTOR_SIZE});", *local_var_count);
    surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    *local_var_count += 1;

    let vector_inner_node = fuzz_tree.neighbors(current_node).next().unwrap();
    for _elem in 0..VECTOR_SIZE{
        let inner_local = surf_get_node_harness(vector_inner_node,
                                                            false,
                                                                        fuzz_tree,
                                                                        &mode,
                                                                        use_stmts,
                                                                        req_deps,
                                                                        custom_structs,
                                                                        custom_closures,
                                                                        fuzzer_byte_slice_len,
                                                                        local_var_count,
                                                                        generic_subst_map,
                                                                        custom_closure_map,
                                                                        indent_level,
                                                                        output_body_locator,
                                                                        output_body_map
        );
        output_data = format!("{local_vector}.push({inner_local});");
        surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    }
    output_data = format!("{local_vector}.truncate({size_var} as usize);");
    surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    output_data = format!("// End vector declaration.");
    surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    local_vector
}

fn surf_declare_local_box(
    current_node: NodeIndex,
    is_mutable: bool,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    mode: &HarnessGenerationMode,
    use_stmts: &mut Vec<String>,
    req_deps: &mut HashMap<String, SurfDepType>,
    custom_structs: &mut HashMap<String, (String, String)>,
    custom_closures: &mut Vec<String>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    custom_closure_map: &mut HashMap::<String, String>,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let box_inner_node = fuzz_tree.neighbors(current_node).next().unwrap();
    let inner_local = surf_get_node_harness(
                                                        box_inner_node,
                                                        false,
                                                        fuzz_tree,
                                                        &mode,
                                                        use_stmts,
                                                        req_deps,
                                                        custom_structs,
                                                        custom_closures,
                                                        fuzzer_byte_slice_len,
                                                        local_var_count,
                                                        generic_subst_map,
                                                        custom_closure_map,
                                                        indent_level,
                                                        output_body_locator,
                                                        output_body_map
                                                    );
    let inner_type = surf_get_type_name_mono(Some(box_inner_node), fuzz_tree, generic_subst_map);
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let box_decl = format!("let {mutability}t_{}: Box<{inner_type}> = Box::new({inner_local});", *local_var_count);
    surf_write_output_body(&box_decl, indent_level, output_body_locator, output_body_map);
    let local_vector = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_vector
}

fn surf_declare_local_uinit(
    current_node: NodeIndex,
    is_mutable: bool,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let uinit_inner_node = fuzz_tree.neighbors(current_node).next();
    let uinit_type = surf_get_type_name_mono(uinit_inner_node, fuzz_tree, generic_subst_map);
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let uinit_decl = format!("let {mutability}t_{} = std::mem::MaybeUninit::<{uinit_type}>::uninit();", *local_var_count);
    surf_write_output_body(&uinit_decl, indent_level, output_body_locator, output_body_map);
    let local_vector = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_vector
}

fn surf_declare_local_sized_vector(
    current_node: NodeIndex,
    len: &String,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    mode: &HarnessGenerationMode,
    use_stmts: &mut Vec<String>,
    req_deps: &mut HashMap<String, SurfDepType>,
    custom_structs: &mut HashMap<String, (String, String)>,
    custom_closures: &mut Vec<String>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    custom_closure_map: &mut HashMap::<String, String>,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let local_vector = format!("t_{}", *local_var_count);
    let mut output_data = format!("// Start vector declaration.");
    surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    output_data = format!("let mut t_{} = std::vec::Vec::with_capacity({VECTOR_SIZE});", *local_var_count);
    surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    *local_var_count += 1;

    let vector_inner_node = fuzz_tree.neighbors(current_node).next().unwrap();
    for _elem in 0..VECTOR_SIZE{
        let inner_local = surf_get_node_harness(
                                                vector_inner_node,
                                                false,
                                                            fuzz_tree,
                                                            &mode,
                                                            use_stmts,
                                                            req_deps,
                                                            custom_structs,
                                                            custom_closures,
                                                            fuzzer_byte_slice_len,
                                                            local_var_count,
                                                            generic_subst_map,
                                                            custom_closure_map,
                                                            indent_level,
                                                            output_body_locator,
                                                            output_body_map
        );
        output_data = format!("{local_vector}.push({inner_local});");
        surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    }
    output_data = format!("{local_vector}.truncate({len});");
    surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    output_data = format!("// End vector declaration.");
    surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    local_vector
}

fn surf_declare_local_slice(
    local_var_name: &str,
    is_mutable_var: bool,
    is_mutable_slice: bool,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let var_mutability = {
        match is_mutable_var{
            true => "mut ",
            false => "",
        }
    };
    let slice_mutability = {
        match is_mutable_slice{
            true => "mut ",
            false => "",
        }
    };
    let slice_decl = format!("let {var_mutability}t_{} = &{slice_mutability}{local_var_name}[..];", *local_var_count);
    surf_write_output_body(&slice_decl, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_local_unreferenced_slice(
    inner_name: &str,
    is_mutable_var: bool,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let var_mutability = {
        match is_mutable_var{
            true => "mut ",
            false => "",
        }
    };
    let slice_decl = format!("let {var_mutability}t_{} = [{inner_name}];", *local_var_count);
    surf_write_output_body(&slice_decl, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_local_array(
    local_var_name: &str,
    is_mutable: bool,
    len: &String,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let array_decl = format!("let {mutability}t_{}: [_; {len}] = {local_var_name}.try_into().unwrap();", *local_var_count);
    surf_write_output_body(&array_decl, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_option_wrapper(
    local_var_name: &str,
    is_mutable: bool,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let option_wrapper_decl = format!("let {mutability}t_{} = Some({local_var_name});", *local_var_count);
    surf_write_output_body(&option_wrapper_decl, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_result_wrapper(
    local_var_name: &str,
    is_mutable: bool,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let result_wrapper_decl = format!("let {mutability}t_{} = Ok({local_var_name});", *local_var_count);
    surf_write_output_body(&result_wrapper_decl, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_option_unwrapper(
    local_var_name: &str,
    is_mutable: bool,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let option_unwrapper_decl = format!("let {mutability}t_{} = _unwrap_option({local_var_name});", *local_var_count);
    surf_write_output_body(&option_unwrapper_decl, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_result_unwrapper(
    local_var_name: &str,
    is_mutable: bool,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let result_unwrapper_decl = format!("let {mutability}t_{} = _unwrap_result({local_var_name});", *local_var_count);
    surf_write_output_body(&result_unwrapper_decl, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_tuple_unwrapper(
    local_var_name: &str,
    field_num: usize,
    is_mutable: bool,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let tuple_unwrapper_decl = format!("let {mutability}t_{} = {local_var_name}.{field_num};", *local_var_count);
    surf_write_output_body(&tuple_unwrapper_decl, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_complex_variant_with_inner(
    variant_name: &str,
    variant_inner_value: &str,
    is_mutable: bool,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let complex_variant_decl = format!("let {mutability}t_{} = {variant_name}({variant_inner_value});", *local_var_count);
    surf_write_output_body(&complex_variant_decl, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}

fn surf_declare_simple_variants_switch(
    selection_flag: &str,
    simple_variants_node: NodeIndex,
    is_mutable: bool,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    mode: &HarnessGenerationMode,
    use_stmts: &mut Vec<String>,
    req_deps: &mut HashMap<String, SurfDepType>,
    custom_structs: &mut HashMap<String, (String, String)>,
    custom_closures: &mut Vec<String>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    custom_closure_map: &mut HashMap::<String, String>,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    let simple_variants_count = fuzz_tree.neighbors(simple_variants_node).count();
    let match_header = format!("let {mutability}t_{} = match ({selection_flag} % {simple_variants_count}usize) {{", *local_var_count);
    surf_write_output_body(&match_header, indent_level, output_body_locator, output_body_map);
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;

    let mut variant_current_count = 0;
    for simple_variant in fuzz_tree.neighbors(simple_variants_node){
        let simple_variant_data = &fuzz_tree[simple_variant];
        if let SurfTreeNode::Variant(variant_name) = simple_variant_data{
            let shoulder_header = format!("{variant_current_count} => {{");
            surf_write_output_body(&shoulder_header, indent_level+1, output_body_locator, output_body_map);
            if let Some(inner_type_node) = fuzz_tree.neighbors(simple_variant).next(){
                let inner_type_local = surf_get_node_harness(
                                                            inner_type_node,
                                                            false,
                                                            fuzz_tree,
                                                            &mode,
                                                            use_stmts,
                                                            req_deps,
                                                            custom_structs,
                                                            custom_closures,
                                                            fuzzer_byte_slice_len,
                                                            local_var_count,
                                                            generic_subst_map,
                                                            custom_closure_map,
                                                            indent_level+2,
                                                            output_body_locator,
                                                            output_body_map
                                                        );
                let variant_return = format!("{variant_name}({inner_type_local})");
                surf_write_output_body(&variant_return, indent_level+2, output_body_locator, output_body_map);
            }
            else{
                let variant_return = format!("{variant_name}");
                surf_write_output_body(&variant_return, indent_level+2, output_body_locator, output_body_map);
            }
            let shoulder_epilogue = format!("}},");
            surf_write_output_body(&shoulder_epilogue, indent_level+1, output_body_locator, output_body_map);
            variant_current_count += 1;
        }
    }
    let unreachable_shoulder = format!("_ => unreachable!(),");
    surf_write_output_body(&unreachable_shoulder, indent_level+1, output_body_locator, output_body_map);
    let match_epilogue = format!("}};");
    surf_write_output_body(&match_epilogue, indent_level, output_body_locator, output_body_map);
    local_name
}

fn surf_get_api_ouput_harness(
    current_node: NodeIndex,
    api_output_local_name: String,
    is_mutable: bool,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    local_var_count: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let cur_node_data = &fuzz_tree[current_node];
    match cur_node_data{
        SurfTreeNode::Struct{..}
        | SurfTreeNode::Enum{..} => api_output_local_name,
        SurfTreeNode::FromOption => surf_declare_option_unwrapper(
                                                                    &api_output_local_name,
                                                                    is_mutable,
                                                                    local_var_count,
                                                                    indent_level,
                                                                    output_body_locator,
                                                                    output_body_map
        ),
        SurfTreeNode::FromResult(_) => surf_declare_result_unwrapper(
                                                                        &api_output_local_name,
                                                                        is_mutable,
                                                                        local_var_count,
                                                                        indent_level,
                                                                        output_body_locator,
                                                                        output_body_map
        ),
        _ => String::from(""),
    }
}


fn surf_get_node_harness(
    current_node: NodeIndex,
    is_mutable: bool,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    mode: &HarnessGenerationMode,
    use_stmts: &mut Vec<String>,
    req_deps: &mut HashMap<String, SurfDepType>,
    custom_structs: &mut HashMap<String, (String, String)>,
    custom_closures: &mut Vec<String>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    custom_closure_map: &mut HashMap::<String, String>,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let cur_node_data = &fuzz_tree[current_node];
    match cur_node_data{
        SurfTreeNode::Primitive(primitive_type) => {
            surf_declare_local(primitive_type,
                                            is_mutable,
                                            None,
                                            fuzzer_byte_slice_len,
                                            local_var_count,
                                            indent_level,
                                            output_body_locator,
                                            output_body_map,
            )
        },
        SurfTreeNode::ReferencedStr => {
            surf_declare_local_str(is_mutable, fuzzer_byte_slice_len, local_var_count, indent_level, output_body_locator, output_body_map)
        },
        SurfTreeNode::String => {
            let local_str = surf_declare_local_str(false,
                                                                fuzzer_byte_slice_len,
                                                                local_var_count,
                                                                indent_level,
                                                                output_body_locator,
                                                                output_body_map);
            surf_declare_local_string(&local_str, is_mutable, local_var_count, indent_level, output_body_locator, output_body_map)
        },
        SurfTreeNode::Reference(is_mutable_ref) => {
            let value_node = fuzz_tree.neighbors(current_node).next().unwrap();
            let value_name = surf_get_node_harness(
                                                            value_node,
                                                            *is_mutable_ref,
                                                            fuzz_tree,
                                                            &mode,
                                                            use_stmts,
                                                            req_deps,
                                                            custom_structs,
                                                            custom_closures,
                                                            fuzzer_byte_slice_len,
                                                            local_var_count,
                                                            generic_subst_map,
                                                            custom_closure_map,
                                                            indent_level,
                                                            output_body_locator,
                                                            output_body_map
            );
            surf_declare_ref(&value_name, is_mutable, *is_mutable_ref, local_var_count, indent_level, output_body_locator, output_body_map)
        },
        SurfTreeNode::RawPointer(is_mutable_raw_ptr) => {
            let local_usize_name = surf_declare_local("usize",
                                                        is_mutable,
                                                        None,
                                                        fuzzer_byte_slice_len,
                                                        local_var_count,
                                                        indent_level,
                                                        output_body_locator,
                                                        output_body_map);
            surf_declare_raw_ptr(&local_usize_name,
                                    //&value_type_name,
                                    is_mutable,
                                    *is_mutable_raw_ptr,
                                    local_var_count,
                                    indent_level,
                                    output_body_locator,
                                    output_body_map)
        },
        SurfTreeNode::Vector => {
            // No need for is_mutable here. The vec variable is mutable regardless.
            surf_declare_local_vector(current_node,
                                        fuzz_tree,
                                        &mode,
                                        use_stmts,
                                        req_deps,
                                        custom_structs,
                                        custom_closures,
                                        fuzzer_byte_slice_len,
                                        local_var_count,
                                        generic_subst_map,
                                        custom_closure_map,
                                        indent_level,
                                        output_body_locator,
                                        output_body_map,
            )
        },
        SurfTreeNode::Box => {
            surf_declare_local_box(
                                        current_node,
                                        is_mutable,
                                        fuzz_tree,
                                        &mode,
                                        use_stmts,
                                        req_deps,
                                        custom_structs,
                                        custom_closures,
                                        fuzzer_byte_slice_len,
                                        local_var_count,
                                        generic_subst_map,
                                        custom_closure_map,
                                        indent_level,
                                        output_body_locator,
                                        output_body_map,
                                    )
        },
        SurfTreeNode::Uinit => {
            surf_declare_local_uinit(
                                        current_node,
                                        is_mutable,
                                        fuzz_tree,
                                        local_var_count,
                                        generic_subst_map,
                                        indent_level,
                                        output_body_locator,
                                        output_body_map,
                                    )
        },
        SurfTreeNode::ReferencedSlice(is_mutable_slice) =>{
            let local_vector = surf_declare_local_vector(current_node,
                                                                    fuzz_tree,
                                                                    &mode,
                                                                    use_stmts,
                                                                    req_deps,
                                                                    custom_structs,
                                                                    custom_closures,
                                                                    fuzzer_byte_slice_len,
                                                                    local_var_count,
                                                                    generic_subst_map,
                                                                    custom_closure_map,
                                                                    indent_level,
                                                                    output_body_locator,
                                                                    output_body_map,
            );
            surf_declare_local_slice(
                        &local_vector,
                        is_mutable,
                        *is_mutable_slice,
                        local_var_count,
                        indent_level,
                        output_body_locator,
                        output_body_map)
        },
        SurfTreeNode::Slice => {
            let inner_node = fuzz_tree.neighbors(current_node).next().unwrap();
            let inner_name = surf_get_node_harness(
                                                            inner_node,
                                                            is_mutable,
                                                            fuzz_tree,
                                                            &mode,
                                                            use_stmts,
                                                            req_deps,
                                                            custom_structs,
                                                            custom_closures,
                                                            fuzzer_byte_slice_len,
                                                            local_var_count,
                                                            generic_subst_map,
                                                            custom_closure_map,
                                                            indent_level,
                                                            output_body_locator,
                                                            output_body_map
            );
            surf_declare_local_unreferenced_slice(
                                                    &inner_name,
                                                    is_mutable,
                                                    local_var_count,
                                                    indent_level,
                                                    output_body_locator,
                                                    output_body_map
                                                )
        },
        SurfTreeNode::Array(len) => {
            let local_vector = surf_declare_local_sized_vector(
                                                                        current_node,
                                                                        len,
                                                                        fuzz_tree,
                                                                        &mode,
                                                                        use_stmts,
                                                                        req_deps,
                                                                        custom_structs,
                                                                        custom_closures,
                                                                        fuzzer_byte_slice_len,
                                                                        local_var_count,
                                                                        generic_subst_map,
                                                                        custom_closure_map,
                                                                        indent_level,
                                                                        output_body_locator,
                                                                        output_body_map,
        );
        surf_declare_local_array(
                                    &local_vector,
                                    is_mutable,
                                    len,
                                    local_var_count,
                                    indent_level,
                                    output_body_locator,
                                    output_body_map)
        },
        SurfTreeNode::ToOption => {
            let option_node = fuzz_tree.neighbors(current_node).next().unwrap();
            let local_var_name = surf_get_node_harness(
                                                                option_node,
                                                                is_mutable,
                                                                fuzz_tree,
                                                                &mode,
                                                                use_stmts,
                                                                req_deps,
                                                                custom_structs,
                                                                custom_closures,
                                                                fuzzer_byte_slice_len,
                                                                local_var_count,
                                                                generic_subst_map,
                                                                custom_closure_map,
                                                                indent_level,
                                                                output_body_locator,
                                                                output_body_map
                                                            );
            surf_declare_option_wrapper(
                                            &local_var_name,
                                            is_mutable,
                                            local_var_count,
                                            indent_level,
                                            output_body_locator,
                                            output_body_map
                                        )
        },
        SurfTreeNode::ToResult(_) => {
            let result_node = fuzz_tree.neighbors(current_node).next().unwrap();
            let local_var_name = surf_get_node_harness(
                                                                result_node,
                                                                is_mutable,
                                                                fuzz_tree,
                                                                &mode,
                                                                use_stmts,
                                                                req_deps,
                                                                custom_structs,
                                                                custom_closures,
                                                                fuzzer_byte_slice_len,
                                                                local_var_count,
                                                                generic_subst_map,
                                                                custom_closure_map,
                                                                indent_level,
                                                                output_body_locator,
                                                                output_body_map
                                                            );
            surf_declare_result_wrapper(
                                            &local_var_name,
                                            is_mutable,
                                            local_var_count,
                                            indent_level,
                                            output_body_locator,
                                            output_body_map
                                        )
        },
        SurfTreeNode::Struct{..} => {
            if let Ok(_) =  env::var("SURF_DISABLE_CMPLX"){
               String::from("NO_CMPLX_SUPPORT")
            }
            else{
                let constructor_node = fuzz_tree.neighbors(current_node).next().unwrap();
                surf_get_node_harness(
                                        constructor_node,
                                        is_mutable,
                                        fuzz_tree,
                                        &mode,
                                        use_stmts,
                                        req_deps,
                                        custom_structs,
                                        custom_closures,
                                        fuzzer_byte_slice_len,
                                        local_var_count,
                                        generic_subst_map,
                                        custom_closure_map,
                                        indent_level,
                                        output_body_locator,
                                        output_body_map
                                    )
            }
        },
        SurfTreeNode::Enum{..} => {
            if let Ok(_) =  env::var("SURF_DISABLE_CMPLX"){
                String::from("NO_CMPLX_SUPPORT")
            }
            else{
                let variants_node = fuzz_tree.neighbors(current_node).next().unwrap();
                if let SurfTreeNode::ComplexVariants = &fuzz_tree[variants_node]{
                    let complex_variant_node = fuzz_tree.neighbors(variants_node).next().unwrap();
                    surf_get_node_harness(
                                            complex_variant_node,
                                            is_mutable,
                                            fuzz_tree,
                                            &mode,
                                            use_stmts,
                                            req_deps,
                                            custom_structs,
                                            custom_closures,
                                            fuzzer_byte_slice_len,
                                            local_var_count,
                                            generic_subst_map,
                                            custom_closure_map,
                                            indent_level,
                                            output_body_locator,
                                            output_body_map
                                        )
                }
                else{ // Pending Simple variants logic
                    let selection_flag = surf_declare_local(
                                                                        "usize",
                                                                        false,
                                                                        None,
                                                                        fuzzer_byte_slice_len,
                                                                        local_var_count,
                                                                        indent_level,
                                                                        output_body_locator,
                                                                        output_body_map
                                                                    );
                    surf_declare_simple_variants_switch(    
                                                            &selection_flag,
                                                            variants_node,
                                                            is_mutable,
                                                            fuzz_tree,
                                                            &mode,
                                                            use_stmts,
                                                            req_deps,
                                                            custom_structs,
                                                            custom_closures,
                                                            fuzzer_byte_slice_len,
                                                            local_var_count,
                                                            generic_subst_map,
                                                            custom_closure_map,
                                                            indent_level,
                                                            output_body_locator,
                                                            output_body_map,
                                                        )
                }
            }
        },
        // Should go here only for complex variant nodes with inner complex types
        SurfTreeNode::Variant(variant_name) => {
            let inner_node = fuzz_tree.neighbors(current_node).next().unwrap();
            let variant_inner_value = surf_get_node_harness(
                                                                        inner_node,
                                                                        is_mutable,
                                                                        fuzz_tree,
                                                                        &mode,
                                                                        use_stmts,
                                                                        req_deps,
                                                                        custom_structs,
                                                                        custom_closures,
                                                                        fuzzer_byte_slice_len,
                                                                        local_var_count,
                                                                        generic_subst_map,
                                                                        custom_closure_map,
                                                                        indent_level,
                                                                        output_body_locator,
                                                                        output_body_map
                                                                    );
            surf_declare_complex_variant_with_inner(
                                                        &variant_name, // maybe add the crate name to the variant?
                                                        &variant_inner_value,
                                                        is_mutable,
                                                        local_var_count,
                                                        indent_level,
                                                        output_body_locator,
                                                        output_body_map
                                                    )
        },
        SurfTreeNode::Tuple => {
            let mut tuple_fields = Vec::<String>::new();
            for neighbor in fuzz_tree.neighbors(current_node){
                let local_name = surf_get_node_harness(
                                                                neighbor,
                                                                is_mutable,
                                                                fuzz_tree,
                                                                &mode,
                                                                use_stmts,
                                                                req_deps,
                                                                custom_structs,
                                                                custom_closures,
                                                                fuzzer_byte_slice_len,
                                                                local_var_count,
                                                                generic_subst_map,
                                                                custom_closure_map,
                                                                indent_level,
                                                                output_body_locator,
                                                                output_body_map
                                                            );
                tuple_fields.push(local_name);
            }
            let mutability = {
                match is_mutable{
                    true => "mut ",
                    false => "",
                }
            };
            let tuple_fields_str = tuple_fields.join(", ");
            let tuple_decl = format!("let {mutability}t_{} = ({tuple_fields_str});", *local_var_count);
            surf_write_output_body(&tuple_decl, indent_level, output_body_locator, output_body_map);
            let local_name = format!("t_{}", *local_var_count);
            *local_var_count += 1;
            local_name
        },
        SurfTreeNode::FromOption => {
            let from_option_node = fuzz_tree.neighbors(current_node).next().unwrap();
            let local_var_name = surf_get_node_harness(
                                                                from_option_node,
                                                                false,
                                                                fuzz_tree,
                                                                &mode,
                                                                use_stmts,
                                                                req_deps,
                                                                custom_structs,
                                                                custom_closures,
                                                                fuzzer_byte_slice_len,
                                                                local_var_count,
                                                                generic_subst_map,
                                                                custom_closure_map,
                                                                indent_level,
                                                                output_body_locator,
                                                                output_body_map
                                                            );
            surf_declare_option_unwrapper(
                                            &local_var_name,
                                            is_mutable,
                                            local_var_count,
                                            indent_level,
                                            output_body_locator,
                                            output_body_map
                                        )
        },
        SurfTreeNode::FromResult(_) => {
            let from_result_node = fuzz_tree.neighbors(current_node).next().unwrap();
            let local_var_name = surf_get_node_harness(
                                                                from_result_node,
                                                                false,
                                                                fuzz_tree,
                                                                &mode,
                                                                use_stmts,
                                                                req_deps,
                                                                custom_structs,
                                                                custom_closures,
                                                                fuzzer_byte_slice_len,
                                                                local_var_count,
                                                                generic_subst_map,
                                                                custom_closure_map,
                                                                indent_level,
                                                                output_body_locator,
                                                                output_body_map
                                                            );
            surf_declare_result_unwrapper(
                                            &local_var_name,
                                            is_mutable,
                                            local_var_count,
                                            indent_level,
                                            output_body_locator,
                                            output_body_map
                                        )
        },
        SurfTreeNode::FromTuple {field_num } => {
            let from_tuple_node = fuzz_tree.neighbors(current_node).next().unwrap();
            let local_var_name = surf_get_node_harness(
                                                                from_tuple_node,
                                                                false,
                                                                fuzz_tree,
                                                                &mode,
                                                                use_stmts,
                                                                req_deps,
                                                                custom_structs,
                                                                custom_closures,
                                                                fuzzer_byte_slice_len,
                                                                local_var_count,
                                                                generic_subst_map,
                                                                custom_closure_map,
                                                                indent_level,
                                                                output_body_locator,
                                                                output_body_map
                                                            );
            surf_declare_tuple_unwrapper(
                                            &local_var_name,
                                            *field_num,
                                            is_mutable,
                                            local_var_count,
                                            indent_level,
                                            output_body_locator,
                                            output_body_map
                                        )
        },
        SurfTreeNode::Api {name,  has_self, is_drop, is_display, is_debug,.. } => {
            
            let mut implicit_generics = Vec::<String>::new();
            let implicit_generics_node = fuzz_tree.neighbors(current_node).next().unwrap();
            if *mode == HarnessGenerationMode::SubstituteImplicitGens{
                for neighbor in fuzz_tree.neighbors(implicit_generics_node){
                    surf_generate_generic_subst_types_if_needed(
                                                                    Some(neighbor),
                                                                    fuzz_tree,
                                                                    &mode,
                                                                    use_stmts,
                                                                    req_deps,
                                                                    custom_structs,
                                                                    custom_closures,
                                                                    fuzzer_byte_slice_len,
                                                                    local_var_count,
                                                                    generic_subst_map,
                                                                    custom_closure_map,
                                                                    indent_level,
                                                                    output_body_map
                    );
                    let implicit_generic_type_name = surf_get_type_name_mono(Some(neighbor), fuzz_tree, generic_subst_map);
                    implicit_generics.push(implicit_generic_type_name);
                }
            }
            let mut api_args = Vec::<String>::new();
            if !has_self{
                if let Some(inputs_node) = fuzz_tree.neighbors(current_node).skip(1).next(){
                    for neighbor in fuzz_tree.neighbors(inputs_node){
                        let local_name = surf_get_node_harness(
                                                            neighbor,
                                                                        is_mutable,
                                                                        fuzz_tree,
                                                                        &mode,
                                                                        use_stmts,
                                                                        req_deps,
                                                                        custom_structs,
                                                                        custom_closures,
                                                                        fuzzer_byte_slice_len,
                                                                        local_var_count,
                                                                        generic_subst_map,
                                                                        custom_closure_map,
                                                                        indent_level,
                                                                        output_body_locator,
                                                                        output_body_map
                        );
                        api_args.push(local_name);
                    }
                }
                let api_args_str = api_args.join(", ");
                let api_final_name = match *mode == HarnessGenerationMode::NoSubstitution{
                    true => &format!("{name}"),
                    false => {
                        if implicit_generics.len() > 0{
                            let implicit_types_str = implicit_generics.join(", ");
                            &insert_type_in_full_name(&name, &implicit_types_str)
                        }
                        else{
                            &format!("{name}")
                        }
                    },
                };
                //println!("{:?}", api_final_name);
                if let Some(api_output_node) = fuzz_tree.neighbors(current_node).skip(2).next(){
                    if surf_api_returns_consumable(fuzz_tree.neighbors(api_output_node).next(), fuzz_tree){
                        let mutability = {
                            match is_mutable{
                                true => "mut ",
                                false => "",
                            }
                        };
                        let api_call = format!("let {mutability}t_{} = {api_final_name}({api_args_str});", *local_var_count);
                        surf_write_output_body(&api_call, indent_level, output_body_locator, output_body_map);
                        let local_name = format!("t_{}", *local_var_count);
                        *local_var_count += 1;

                        let consumable_local_name = surf_get_api_ouput_harness(
                                                                                        fuzz_tree.neighbors(api_output_node).next().unwrap(),
                                                                                        local_name,
                                                                                        is_mutable,
                                                                                        fuzz_tree,
                                                                                        local_var_count,
                                                                                        indent_level,
                                                                                        output_body_locator,
                                                                                        output_body_map
                                                                                    );
                        let consumable_call = format!("{consumable_local_name}.count();");
                        surf_write_output_body(
                                                &consumable_call,
                                                indent_level,
                                                output_body_locator,
                                                output_body_map
                        );
                    }
                    else{
                        let api_call = format!("{api_final_name}({api_args_str});");
                        surf_write_output_body(
                                                &api_call,
                                                indent_level,
                                                output_body_locator,
                                                output_body_map
                                            );
                    }
                }
                else{
                    let api_call = format!("{api_final_name}({api_args_str});");
                    surf_write_output_body(
                                            &api_call,
                                            indent_level,
                                            output_body_locator,
                                            output_body_map
                                        );
                }
            }
            else{
                if let Some(inputs_node) = fuzz_tree.neighbors(current_node).skip(1).next(){
                    //println!("{:?}", &fuzz_tree[inputs_node]);
                    let self_node = fuzz_tree.neighbors(inputs_node).next().unwrap();
                    let self_arg = surf_get_node_harness(
                                                                    self_node,
                                                                    is_mutable,
                                                                    fuzz_tree,
                                                                    &mode,
                                                                    use_stmts,
                                                                    req_deps,
                                                                    custom_structs,
                                                                    custom_closures,
                                                                    fuzzer_byte_slice_len,
                                                                    local_var_count,
                                                                    generic_subst_map,
                                                                    custom_closure_map,
                                                                    indent_level,
                                                                    output_body_locator,
                                                                    output_body_map
                                                                );
                    if !*is_display && !*is_debug{ // Don't generate argument if you don't need it
                        for neighbor in fuzz_tree.neighbors(inputs_node).skip(1){
                            let local_name = surf_get_node_harness(
                                                                            neighbor,
                                                                            is_mutable,
                                                                            fuzz_tree,
                                                                            &mode,
                                                                            use_stmts,
                                                                            req_deps,
                                                                            custom_structs,
                                                                            custom_closures,
                                                                            fuzzer_byte_slice_len,
                                                                            local_var_count,
                                                                            generic_subst_map,
                                                                            custom_closure_map,
                                                                            indent_level,
                                                                            output_body_locator,
                                                                            output_body_map
                                                                        );
                            api_args.push(local_name);
                        }
                    }
                    let api_args_str = api_args.join(", ");
                    let api_call = {
                        if *is_drop{
                            format!("let _ = {self_arg};")
                        }
                        else if *is_display{
                            format!("println!(\"{{}}\", {self_arg});")
                        }
                        else if *is_debug{
                            format!("println!(\"{{:?}}\", {self_arg});")
                        }
                        else{
                            if let Some(api_output_node) = fuzz_tree.neighbors(current_node).skip(2).next(){
                                if surf_api_returns_consumable(fuzz_tree.neighbors(api_output_node).next(), fuzz_tree){
                                    let mutability = {
                                        match is_mutable{
                                            true => "mut ",
                                            false => "",
                                        }
                                    };
                                    let api_call = format!("let {mutability}t_{} = {self_arg}.{name}({api_args_str});", *local_var_count);
                                    surf_write_output_body(&api_call, indent_level, output_body_locator, output_body_map);
                                    let local_name = format!("t_{}", *local_var_count);
                                    *local_var_count += 1;
                                    let consumable_local_name = surf_get_api_ouput_harness(
                                                                fuzz_tree.neighbors(api_output_node).next().unwrap(),
                                                                local_name,
                                                                is_mutable,
                                                                fuzz_tree,
                                                                local_var_count,
                                                                indent_level,
                                                                output_body_locator,
                                                                output_body_map
                                                            );
                                    format!("{consumable_local_name}.count();")
                                }
                                else{
                                    format!("{self_arg}.{name}({api_args_str});")
                                }
                            }
                            else{
                                format!("{self_arg}.{name}({api_args_str});")
                            }
                            
                        }
                    };

                    surf_write_output_body(
                                            &api_call,
                                            indent_level,
                                            output_body_locator,
                                            output_body_map
                                        );
                }
                else{
                    surf_write_output_body(
                        &format!("Invalid Harness"),
                        indent_level,
                        output_body_locator,
                        output_body_map
                    );
                }
            }
            //println!("{:?}", Dot::with_config(&fuzz_tree, &[Config::EdgeNoLabel]));
            //println!("{:?}", &fuzz_tree[current_node]);
            String::from("")
        },
        SurfTreeNode::Constructor {full_name, .. } => {

            let mut implicit_generics = Vec::<String>::new();
            // Generate trait impls only if is needed, prevent errors to happend if trait impl is not supported
            if *mode == HarnessGenerationMode::SubstituteImplicitGens{
                let implicit_generics_node = fuzz_tree.neighbors(current_node).next().unwrap();
                for neighbor in fuzz_tree.neighbors(implicit_generics_node){
                    surf_generate_generic_subst_types_if_needed(
                                                                    Some(neighbor),
                                                                    fuzz_tree,
                                                                    &mode,
                                                                    use_stmts,
                                                                    req_deps,
                                                                    custom_structs,
                                                                    custom_closures,
                                                                    fuzzer_byte_slice_len,
                                                                    local_var_count,
                                                                    generic_subst_map,
                                                                    custom_closure_map,
                                                                    indent_level,
                                                                    output_body_map
                    );
                    let implicit_generic_type_name = surf_get_type_name_mono(Some(neighbor), fuzz_tree, generic_subst_map);
                    implicit_generics.push(implicit_generic_type_name);
                }
            }
            
            //println!("-> {:?}", full_name);
            //println!("{:?}", Dot::with_config(&fuzz_tree, &[Config::EdgeNoLabel]));
            // Case where there is todo leaf that gets pruned from the fuzz tree filter
            
            
            let mut constructor_args = Vec::<String>::new();
            if let Some(inputs_node) = fuzz_tree.neighbors(current_node).skip(1).next(){
                for neighbor in fuzz_tree.neighbors(inputs_node){
                    let local_name = surf_get_node_harness(
                                                                    neighbor,
                                                                    false,
                                                                    fuzz_tree,
                                                                    &mode,
                                                                    use_stmts,
                                                                    req_deps,
                                                                    custom_structs,
                                                                    custom_closures,
                                                                    fuzzer_byte_slice_len,
                                                                    local_var_count,
                                                                    generic_subst_map,
                                                                    custom_closure_map,
                                                                    indent_level,
                                                                    output_body_locator,
                                                                    output_body_map
                                                                );
                    constructor_args.push(local_name);
                }
            }
            
            let mutability = {
                match is_mutable{
                    true => "mut ",
                    false => "",
                }
            };
            let constructor_args_str = constructor_args.join(", ");
            // Update dependencies
            let item_crate_name = surf_get_crate_name(full_name);
            
            
            let constructor_name = match *mode == HarnessGenerationMode::NoSubstitution{
                true => full_name,
                false => {
                    if implicit_generics.len() > 0{
                        let implicit_types_str = implicit_generics.join(", ");
                        &insert_type_in_full_name(&full_name, &implicit_types_str)
                    }
                    else{
                        full_name
                    }
                },
            };
            if let Some(dependency) = SURF_USED_DEPS.get(item_crate_name){
                if !use_stmts.contains(&format!("{item_crate_name}::*")){
                    use_stmts.push(format!("{item_crate_name}::*"));
                    req_deps.entry(item_crate_name.to_string()).or_insert(dependency.clone());
                }
            }

            let constructor_call = format!("let {mutability}t_{} = {constructor_name}({constructor_args_str});", *local_var_count);
            surf_write_output_body(
                                    &constructor_call,
                                    indent_level,
                                    output_body_locator,
                                    output_body_map
                                );
            let local_name = format!("t_{}", *local_var_count);
            *local_var_count += 1;
            local_name
        },
        SurfTreeNode::Closure(param_name, is_mutable) => {
            if let Ok(_) =  env::var("SURF_DISABLE_CLOSURES"){
                String::from("NO_CLOSURES_SUPPORT")
            }
            else{
                if !custom_closure_map.contains_key(param_name){
                    let custom_closure_name = format!("_custom_fn{}", custom_closures.len());
                    custom_closures.push(custom_closure_name.clone());
                    custom_closure_map.entry(param_name.clone())
                                        .or_insert(custom_closure_name.clone());
                    // Generate Function That Substitutes Closure
                    let mut custom_closure_inputs_node_opt = None;
                    let mut custom_closure_output_node_opt = None;
                    for closure_neighbor in fuzz_tree.neighbors(current_node){
                        match &fuzz_tree[closure_neighbor]{
                            SurfTreeNode::FnInputs => custom_closure_inputs_node_opt = Some(closure_neighbor),
                            SurfTreeNode::FnOutput => custom_closure_output_node_opt = Some(closure_neighbor),
                            _ => {},
                        }
                    }
                    // Produce Generic Types that Haven't Generated Yet
                    if let Some(custom_closure_inputs_node) = custom_closure_inputs_node_opt{
                        for input_node in fuzz_tree.neighbors(custom_closure_inputs_node){
                            surf_generate_generic_subst_types_if_needed(
                                                                        Some(input_node),
                                                                        fuzz_tree,
                                                                        &mode,
                                                                        use_stmts,
                                                                        req_deps,
                                                                        custom_structs,
                                                                        custom_closures,
                                                                        fuzzer_byte_slice_len,
                                                                        local_var_count,
                                                                        generic_subst_map,
                                                                        custom_closure_map,
                                                                        indent_level,
                                                                        output_body_map
                                                                    );
                        }
                    }
                    if let Some(custom_closure_output_node) = custom_closure_output_node_opt{
                        let output_node = fuzz_tree.neighbors(custom_closure_output_node).next();
                        surf_generate_generic_subst_types_if_needed(
                                                                    output_node,
                                                                    fuzz_tree,
                                                                    &mode,
                                                                    use_stmts,
                                                                    req_deps,
                                                                    custom_structs,
                                                                    custom_closures,
                                                                    fuzzer_byte_slice_len,
                                                                    local_var_count,
                                                                    generic_subst_map,
                                                                    custom_closure_map,
                                                                    indent_level,
                                                                    output_body_map
                                                                );
                    }
                    let (closure_sig_str, has_custom_arg) = surf_build_closure_sig(custom_closure_name.clone(), current_node, fuzz_tree, generic_subst_map, custom_structs);
                    output_body_map
                                .closures
                                .entry(custom_closure_name.clone())
                                .or_insert(Vec::<String>::new());
                    let new_output_body_locator = OutputBodyLocator::Closures(custom_closure_name.clone());
                    let mut output_data = format!("fn {} {{", closure_sig_str.clone());
                    surf_write_output_body(
                                            &output_data, 
                                            0,
                                            &new_output_body_locator,
                                            output_body_map
                                        );
                    if let Some(custom_arg) = has_custom_arg{
                        surf_declare_fuzzer_slice_first_half_selector(indent_level-1, &new_output_body_locator, output_body_map);
                        surf_declare_custom_impl_num(fuzzer_byte_slice_len, indent_level-1, &new_output_body_locator, output_body_map);
                        match custom_arg.starts_with("str"){
                            true => surf_declare_string_custom_impl_inst_num(&custom_arg, indent_level-1, &new_output_body_locator, output_body_map),
                            false => surf_declare_usize_custom_impl_inst_num(&custom_arg, indent_level-1, &new_output_body_locator, output_body_map),
                        }
                        surf_declare_selector(indent_level-1, &new_output_body_locator, output_body_map);
                        surf_declare_selector_panic_flag(indent_level-1, &new_output_body_locator, output_body_map);
                        surf_declare_fuzzer_slice_selector(indent_level-1, &new_output_body_locator, output_body_map);
                    }
                    else{
                        surf_declare_panic_flag(
                                                fuzzer_byte_slice_len,
                                                local_var_count,
                                                indent_level-1,
                                                &new_output_body_locator,
                                                output_body_map
                        );
                    }
                    if let Some(custom_closure_output_node) = custom_closure_output_node_opt{
                        let return_local = surf_get_node_harness(
                                                                        custom_closure_output_node,
                                                                        false, // Ok for now
                                                                        fuzz_tree,
                                                                        &mode,
                                                                        use_stmts,
                                                                        req_deps,
                                                                        custom_structs,
                                                                        custom_closures,
                                                                        fuzzer_byte_slice_len,
                                                                        local_var_count,
                                                                        generic_subst_map,
                                                                        custom_closure_map,
                                                                        indent_level-1,
                                                                        &new_output_body_locator,
                                                                        output_body_map
                        );
                        output_data = format!("return {return_local};");
                        surf_write_output_body(
                                                &output_data,
                                                indent_level-1,
                                                &new_output_body_locator,
                                                output_body_map
                                            );
                    }
                    output_data = format!("}}");
                    surf_write_output_body(
                                            &output_data, 
                                            0,
                                            &new_output_body_locator,
                                            output_body_map
                                        );
                }
                let mutability = {
                    match is_mutable{
                        true => "mut ",
                        false => "",
                    }
                };
                let custom_closure_name = custom_closure_map.get(param_name).unwrap();
                let custom_closure_local_decl = format!("let {mutability}t_{} = {custom_closure_name};", *local_var_count);
                surf_write_output_body(
                                        &custom_closure_local_decl,
                                        indent_level,
                                        output_body_locator,
                                        output_body_map
                                    );
                let local_name = format!("t_{}", *local_var_count);
                *local_var_count += 1;
                local_name
            }
        },
        SurfTreeNode::Generic{name, ..} => {
            if let Ok(_) =  env::var("SURF_DISABLE_GENERICS"){
                String::from("NO_GENERICS_SUPPORT")
            }
            else{
                let generics_mode_node = fuzz_tree.neighbors(current_node).next().unwrap();
            
                match &fuzz_tree[generics_mode_node]{
                    SurfTreeNode::GenericCustomTy => {
                        // If it doesn't exist it will generate a new custom type and add it in the substitutions map
                        surf_generate_custom_type_if_needed(
                                                            generics_mode_node,
                                                            name,
                                                            fuzz_tree,
                                                            custom_structs,
                                                            generic_subst_map,
                                                        );

                        // If it is needed it will generate custom implementations of the traits that the custom type should implement
                        surf_generate_custom_trait_impls_if_needed(
                                                                    generics_mode_node,
                                                                    name,
                                                                    fuzz_tree,
                                                                    &mode,
                                                                    use_stmts,
                                                                    req_deps,
                                                                    custom_structs,
                                                                    custom_closures,
                                                                    fuzzer_byte_slice_len,
                                                                    local_var_count,
                                                                    generic_subst_map,
                                                                    custom_closure_map,
                                                                    indent_level,
                                                                    output_body_map
                        );
                        //println!("{:?}", Dot::with_config(&fuzz_tree, &[Config::EdgeNoLabel]));
                        //println!("Node: {:?}", current_node);
                        // Declare the custom type instance in the code
                        surf_declare_generic_subst_type(
                                                        current_node,
                                                        name,
                                                        is_mutable,
                                                        fuzz_tree,
                                                        fuzzer_byte_slice_len,
                                                        local_var_count,
                                                        generic_subst_map,
                                                        indent_level,
                                                        output_body_locator,
                                                        output_body_map,
                        )
                    },
                    SurfTreeNode::GenericCandidates => {
                        if let Some(candidate_node) = fuzz_tree.neighbors(generics_mode_node).next(){

                            surf_substitute_generic_with_candidate_type_if_needed(
                                                                                    name,
                                                                                    candidate_node,
                                                                                    generic_subst_map,
                                                                                );
                        
                            let candidate_decl = surf_get_node_harness(
                                                                                candidate_node,
                                                                                false,
                                                                                fuzz_tree,
                                                                                &mode,
                                                                                use_stmts,
                                                                                req_deps,
                                                                                custom_structs,
                                                                                custom_closures,
                                                                                fuzzer_byte_slice_len,
                                                                                local_var_count,
                                                                                generic_subst_map,
                                                                                custom_closure_map,
                                                                                indent_level,
                                                                                output_body_locator,
                                                                                output_body_map
                            );
                            candidate_decl
                        }
                        else{
                            String::from("Todo")
                        }
                    },
                    _ => unreachable!()
                }
            }
        },
        SurfTreeNode::AssocType { placeholder_name, assoc_type_id, trait_def_id,.. } => {
            if let Some(subst_type) = generic_subst_map.get_mut(assoc_type_id){
                if let SubstTypeKind::Custom { inner_type, .. } = &mut subst_type.kind{
                    let new_inner_type = surf_get_custom_type_inner_type(fuzz_tree, current_node);
                    if new_inner_type != *inner_type{
                        if let Some((_, custom_type_inner_type)) = custom_structs.get_mut(assoc_type_id){
                            *inner_type = new_inner_type.clone();
                            *custom_type_inner_type = new_inner_type;
    
                        }
                    }
                }
            }
            

            // // If it is needed it will generate custom implementations of the traits that the custom type should implement
            surf_generate_custom_trait_impls_if_needed(
                                                        current_node,
                                                        &assoc_type_id,
                                                        fuzz_tree,
                                                        &mode,
                                                        use_stmts,
                                                        req_deps,
                                                        custom_structs,
                                                        custom_closures,
                                                        fuzzer_byte_slice_len,
                                                        local_var_count,
                                                        generic_subst_map,
                                                        custom_closure_map,
                                                        indent_level,
                                                        output_body_map
            );
            
            surf_declare_assoc_type_subst_type(
                                            current_node,
                                            &assoc_type_id,
                                            placeholder_name,
                                            &trait_def_id,
                                            is_mutable,
                                            fuzz_tree,
                                            fuzzer_byte_slice_len,
                                            local_var_count,
                                            generic_subst_map,
                                            indent_level,
                                            output_body_locator,
                                            output_body_map,
                            )
        },
        SurfTreeNode::TraitType { assoc_type_id, concrete_type_id, def_id, .. } => {
            //If it doesn't exist it will generate a new custom type and add it in the substitutions map            
            let new_assoc_type_id = {
                match concrete_type_id{
                    Some(concrete_value) => concrete_value.clone(),
                    _ => assoc_type_id.clone(),
                }
            };

            // // If it is needed it will generate custom implementations of the traits that the custom type should implement
            surf_generate_custom_trait_impls_if_needed(
                                                        current_node,
                                                        &new_assoc_type_id,
                                                        fuzz_tree,
                                                        &mode,
                                                        use_stmts,
                                                        req_deps,
                                                        custom_structs,
                                                        custom_closures,
                                                        fuzzer_byte_slice_len,
                                                        local_var_count,
                                                        generic_subst_map,
                                                        custom_closure_map,
                                                        indent_level,
                                                        output_body_map
            );
            String::from("")
        },
        SurfTreeNode::FnOutput => {
            let has_output = fuzz_tree.neighbors(current_node).next();
            match has_output{
                Some(output_neighbor) => surf_get_node_harness(
                                                                            output_neighbor,
                                                                            is_mutable,
                                                                            fuzz_tree,
                                                                            &mode,
                                                                            use_stmts,
                                                                            req_deps,
                                                                            custom_structs,
                                                                            custom_closures,
                                                                            fuzzer_byte_slice_len,
                                                                            local_var_count,
                                                                            generic_subst_map,
                                                                            custom_closure_map,
                                                                            indent_level,
                                                                            output_body_locator,
                                                                            output_body_map
                                                                        ),
                None => String::from("")
            }
        },
        SurfTreeNode::ApiOutput => {
            let has_output = fuzz_tree.neighbors(current_node).next();
            match has_output{
                Some(output_neighbor) => surf_get_node_harness(
                                                                            output_neighbor,
                                                                            is_mutable,
                                                                            fuzz_tree,
                                                                            &mode,
                                                                            use_stmts,
                                                                            req_deps,
                                                                            custom_structs,
                                                                            custom_closures,
                                                                            fuzzer_byte_slice_len,
                                                                            local_var_count,
                                                                            generic_subst_map,
                                                                            custom_closure_map,
                                                                            indent_level,
                                                                            output_body_locator,
                                                                            output_body_map
                                                                        ),
                None => String::from("")
            }
        },
        SurfTreeNode::DynTrait => {
            let cmplx_ty_node = fuzz_tree.neighbors(current_node).next().unwrap();
            surf_get_node_harness(
                                    cmplx_ty_node,
                                    is_mutable,
                                    fuzz_tree,
                                    &mode,
                                    use_stmts,
                                    req_deps,
                                    custom_structs,
                                    custom_closures,
                                    fuzzer_byte_slice_len,
                                    local_var_count,
                                    generic_subst_map,
                                    custom_closure_map,
                                    indent_level,
                                    output_body_locator,
                                    output_body_map
                                )
        },
        _ => String::from(""),
    }
}

fn insert_type_in_full_name(full_name: &str, type_to_insert: &str) -> String {
    // Collect the parts into a Vec<String> instead of Vec<&str>
    let mut parts: Vec<String> = full_name.split("::").map(|s| s.to_string()).collect();

    // Insert the generic type before the last token
    if let Some(last_part) = parts.pop() {
        if let Some(second_last) = parts.last_mut() {
            *second_last = format!("{}::<{}>", second_last, type_to_insert);  // No need for reference here
        }
        parts.push(last_part);
    }

    // Rebuild the full name with the type inserted
    parts.join("::")
}


// This function will generate custom types and their corresponding custom impl blocks if it is needed
// It will go inside the nested levels and search for generic types that haven't been generated
// It supposed to be called followed by a surf_get_type_name_mono call which gets the names of these generated custom types
fn surf_generate_generic_subst_types_if_needed(
    current_node_opt: Option<NodeIndex>,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    mode: &HarnessGenerationMode,
    use_stmts: &mut Vec<String>,
    req_deps: &mut HashMap<String, SurfDepType>,
    custom_structs: &mut HashMap<String, (String, String)>,
    custom_closures: &mut Vec<String>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    custom_closure_map: &mut HashMap::<String, String>,
    indent_level: usize,
    output_body_map: &mut OutputBodyMap,
){

    if let Some(current_node) = current_node_opt{
        match &fuzz_tree[current_node]{
            SurfTreeNode::Generic{name, ..} => {
                let generics_mode_node = fuzz_tree.neighbors(current_node).next().unwrap();
                if let SurfTreeNode::GenericCustomTy = &fuzz_tree[generics_mode_node]{
                    surf_generate_custom_type_if_needed(
                                                        generics_mode_node,
                                                        name,
                                                        fuzz_tree,
                                                        custom_structs,
                                                        generic_subst_map,
                                                    );
                    surf_generate_custom_trait_impls_if_needed(
                                                                generics_mode_node,
                                                                name,
                                                                fuzz_tree,
                                                                &mode,
                                                                use_stmts,
                                                                req_deps,
                                                                custom_structs,
                                                                custom_closures,
                                                                fuzzer_byte_slice_len,
                                                                local_var_count,
                                                                generic_subst_map,
                                                                custom_closure_map,
                                                                indent_level,
                                                                output_body_map
                    );
                }
                else{
                    if let Some(candidate_node) = fuzz_tree.neighbors(generics_mode_node).next(){
                        surf_substitute_generic_with_candidate_type_if_needed(
                                                                                name,
                                                                                candidate_node,
                                                                                generic_subst_map,
                                                                            );
                    }
                }
            },
            SurfTreeNode::Reference(_)
            | SurfTreeNode::RawPointer(_)
            | SurfTreeNode::Box
            | SurfTreeNode::Vector
            | SurfTreeNode::ReferencedSlice(_) 
            | SurfTreeNode::Array(_)
            | SurfTreeNode::ToOption 
            | SurfTreeNode::ToResult(_) 
            | SurfTreeNode::DynTrait => {
                let inner_node = fuzz_tree.neighbors(current_node).next();
                surf_generate_generic_subst_types_if_needed(
                                                                inner_node,
                                                                fuzz_tree,
                                                                &mode,
                                                                use_stmts,
                                                                req_deps,
                                                                custom_structs,
                                                                custom_closures,
                                                                fuzzer_byte_slice_len,
                                                                local_var_count,
                                                                generic_subst_map,
                                                                custom_closure_map,
                                                                indent_level,
                                                                output_body_map
                                                        )
            },
            SurfTreeNode::Tuple => {
                for neighbor in fuzz_tree.neighbors(current_node){
                    surf_generate_generic_subst_types_if_needed(
                                                                    Some(neighbor),
                                                                    fuzz_tree,
                                                                    &mode,
                                                                    use_stmts,
                                                                    req_deps,
                                                                    custom_structs,
                                                                    custom_closures,
                                                                    fuzzer_byte_slice_len,
                                                                    local_var_count,
                                                                    generic_subst_map,
                                                                    custom_closure_map,
                                                                    indent_level,
                                                                    output_body_map
                                                                );
                }
            },
            _ => {},
        }
    }
}

fn surf_generate_custom_type_if_needed(
    generic_custom_ty_node: NodeIndex,
    custom_type_id: &String,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    custom_structs: &mut HashMap<String, (String, String)>,
    generic_subst_map: &mut HashMap::<String, SubstType>,
){    
    
    if custom_type_id != "Self" && !generic_subst_map.contains_key(custom_type_id){
        let custom_type_name = format!("CustomType{}", custom_structs.len());
        // Go check the traits and return the inner type of the custom type
        let custom_type_inner_type = surf_get_custom_type_inner_type(fuzz_tree, generic_custom_ty_node);
        custom_structs.insert(custom_type_id.clone(), (custom_type_name.clone(), custom_type_inner_type.clone()));
        generic_subst_map.entry(custom_type_id.clone())
                            .or_insert(SubstType::new(
                                                                SubstTypeKind::Custom{
                                                                                                name: custom_type_name.clone(),
                                                                                                inner_type: custom_type_inner_type
                                                                                            }));
    }
}

fn surf_substitute_generic_with_candidate_type_if_needed(
    custom_type_id: &String,
    candidate_type_node: NodeIndex,
    generic_subst_map: &mut HashMap::<String, SubstType>,
){
    // Omit Self check here, maybe is not needed?
    if custom_type_id != "Self" && !generic_subst_map.contains_key(custom_type_id){
        generic_subst_map.entry(custom_type_id.clone())
                            .or_insert(SubstType::new(
                                                                SubstTypeKind::Candidate { 
                                                                                                    node_index: candidate_type_node}
                                                                                                ));
    }
}

fn surf_generate_custom_trait_impls_if_needed(
    generic_custom_ty_node: NodeIndex,
    custom_type_id: &String,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    mode: &HarnessGenerationMode,
    use_stmts: &mut Vec<String>,
    req_deps: &mut HashMap<String, SurfDepType>,
    custom_structs: &mut HashMap<String, (String, String)>,
    custom_closures: &mut Vec<String>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    custom_closure_map: &mut HashMap::<String, String>,
    indent_level: usize,
    output_body_map: &mut OutputBodyMap,
){
    // Get from the substitutions map the information that has been already generated for the current custom type
    if custom_type_id!="Self" && generic_subst_map.contains_key(custom_type_id){
        let (custom_type_name, _) = {
            if let SubstTypeKind::Custom { name, inner_type } =  &generic_subst_map.get(custom_type_id).unwrap().kind{
                (name.clone(), inner_type.clone())
            }
            else{
                return;
                //unreachable!()
            }
        };
        let custom_type_traits = generic_subst_map.get(custom_type_id).unwrap().traits.clone();

        // Add into the traits field of the substitutions map entry the new traits that need to be generated for the fetched custom type
        // Generate them too
        for trait_node in fuzz_tree.neighbors(generic_custom_ty_node){
            let trait_node_data = &fuzz_tree[trait_node];
            if let SurfTreeNode::Trait{name, is_unsafe, ..} = trait_node_data{
                if !custom_type_traits.contains(name){
                    if surf_is_sized_trait(name) {continue;}
                    generic_subst_map.get_mut(custom_type_id).unwrap().traits.insert(name.clone());
                    output_body_map
                                .traits
                                .entry(name.clone())
                                .or_insert(HashMap::<String, Vec<String>>:: new())
                                .insert(custom_type_name.clone(), Vec::<String>::new());
                    let new_output_body_locator = OutputBodyLocator::Traits(name.clone(), custom_type_name.clone());
                    let trait_safety = match is_unsafe{
                        true => {
                            match env::var("SURF_DISABLE_UNSAFE_TRAITS"){
                                Ok(_) => "_unsafe ", // make it fail
                                _ => "unsafe "
                            }
                        },
                        false => "",
                    };
                    
                    let mut output_data = format!("{trait_safety}impl {name} for {} {{", custom_type_name.clone());
                    surf_write_output_body(
                                            &output_data, 
                                            0,
                                            &new_output_body_locator,
                                            output_body_map
                                        );
                    
                    
                    // Generate Trait Assoc Types
                    // Check if there are associated types the first child node
                    if let Some(trait_neighbor) = fuzz_tree.neighbors(trait_node).next(){
                        if let SurfTreeNode::TraitTypes = &fuzz_tree[trait_neighbor]{
                            surf_generate_assoc_types(
                                                        trait_neighbor,
                                                        fuzz_tree,
                                                        mode,
                                                        use_stmts,
                                                        req_deps,
                                                        custom_structs,
                                                        custom_closures,
                                                        fuzzer_byte_slice_len,
                                                        local_var_count,
                                                        generic_subst_map,
                                                        custom_closure_map,
                                                        indent_level,
                                                        &new_output_body_locator,
                                                        output_body_map,
                                                    );
                            if let Some(trait_neighbor) = fuzz_tree.neighbors(trait_node).skip(1).next(){
                                
                                surf_generate_trait_fns_harness(
                                                                    trait_neighbor,
                                                                    &custom_type_id,
                                                                    fuzz_tree,
                                                                    mode,
                                                                    use_stmts,
                                                                    req_deps,
                                                                    custom_structs,
                                                                    custom_closures,
                                                                    fuzzer_byte_slice_len,
                                                                    local_var_count,
                                                                    generic_subst_map,
                                                                    custom_closure_map,
                                                                    indent_level,
                                                                    &new_output_body_locator,
                                                                    output_body_map,
                                                                );
                            }
                        }
                        else{
                            
                            surf_generate_trait_fns_harness(
                                                                trait_neighbor,
                                                                &custom_type_id,
                                                                fuzz_tree,
                                                                mode,
                                                                use_stmts,
                                                                req_deps,
                                                                custom_structs,
                                                                custom_closures,
                                                                fuzzer_byte_slice_len,
                                                                local_var_count,
                                                                generic_subst_map,
                                                                custom_closure_map,
                                                                indent_level,
                                                                &new_output_body_locator,
                                                                output_body_map,
                                                            );
                        }
                    }
                    output_data = format!("}}");
                    surf_write_output_body(
                                            &output_data,
                                            0,
                                            &new_output_body_locator,
                                            output_body_map
                    );
                }
            }   
        }
    }
}


                    
fn surf_generate_trait_fns_harness(
    trait_fns_node: NodeIndex,
    custom_type_id: &str,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    mode: &HarnessGenerationMode,
    use_stmts: &mut Vec<String>,
    req_deps: &mut HashMap<String, SurfDepType>,
    custom_structs: &mut HashMap<String, (String, String)>,
    custom_closures: &mut Vec<String>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    custom_closure_map: &mut HashMap::<String, String>,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
){
    //println!("{:?}", Dot::with_config(&fuzz_tree, &[Config::EdgeNoLabel]));
    for trait_fn_node in fuzz_tree.neighbors(trait_fns_node){
        //println!("HERE -> {:?}", fuzz_tree[trait_fn_node]);
        let trait_fn_node_data = &fuzz_tree[trait_fn_node];
        if let SurfTreeNode::TraitFn { sig_str, has_self, .. } = trait_fn_node_data{
            
            let rebuilt_sig_str = surf_rebuild_fn_sig(sig_str.clone(), trait_fn_node, fuzz_tree);
            let mut output_data = format!("");
            surf_write_output_body(
                &output_data,
                1,
                &output_body_locator,
                output_body_map
            );
            output_data = format!("{rebuilt_sig_str} {{");
            surf_write_output_body(
                                    &output_data,
                                    1,
                                    &output_body_locator,
                                    output_body_map
                                );
            if *has_self{
                if let Some((_, inner_type)) = custom_structs.get(custom_type_id){
                    surf_declare_fuzzer_slice_first_half_selector(indent_level, output_body_locator, output_body_map);
                    surf_declare_custom_impl_num(fuzzer_byte_slice_len, indent_level, output_body_locator, output_body_map);
                    match inner_type == "String"{
                        true => surf_declare_string_custom_impl_inst_num("self", indent_level, output_body_locator, output_body_map),
                        false => surf_declare_usize_custom_impl_inst_num("self", indent_level, output_body_locator, output_body_map),
                    }
                    surf_declare_selector(indent_level, output_body_locator, output_body_map);
                    surf_declare_selector_panic_flag(indent_level, output_body_locator, output_body_map);
                    surf_declare_fuzzer_slice_selector(indent_level, output_body_locator, output_body_map);
                }
                else{
                    surf_declare_panic_flag(
                                            fuzzer_byte_slice_len,
                                            local_var_count,
                                            indent_level,
                                            &output_body_locator,
                                            output_body_map
                    );
                }
            }
            else{
                surf_declare_panic_flag(
                                fuzzer_byte_slice_len,
                                local_var_count,
                                indent_level,
                                &output_body_locator,
                                output_body_map
                );
            }
            if let Some(output_node) = fuzz_tree.neighbors(trait_fn_node).next(){
                let return_local = surf_get_node_harness(
                                                    output_node,
                                                                false, // Ok for now
                                                                fuzz_tree,
                                                                &mode,
                                                                use_stmts,
                                                                req_deps,
                                                                custom_structs,
                                                                custom_closures,
                                                                fuzzer_byte_slice_len,
                                                                local_var_count,
                                                                generic_subst_map,
                                                                custom_closure_map,
                                                                indent_level,
                                                                &output_body_locator,
                                                                output_body_map
                );
                output_data = format!("return {return_local};");
                surf_write_output_body(
                                        &output_data,
                                        indent_level,
                                        &output_body_locator,
                                        output_body_map
                                    );
            }
            output_data = format!("}}");
            surf_write_output_body(
                                    &output_data,
                                    1,
                                    &output_body_locator,
                                    output_body_map
                                );
        }
    }

}


fn surf_generate_assoc_types(
    trait_types_node: NodeIndex,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    mode: &HarnessGenerationMode,
    use_stmts: &mut Vec<String>,
    req_deps: &mut HashMap<String, SurfDepType>,
    custom_structs: &mut HashMap<String, (String, String)>,
    custom_closures: &mut Vec<String>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    custom_closure_map: &mut HashMap::<String, String>,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
){
    // Print the types inside the trait first
    for trait_type_node in fuzz_tree.neighbors(trait_types_node){
        let trait_type_node_data = &fuzz_tree[trait_type_node];
        if let SurfTreeNode::TraitType { placeholder_name, assoc_type_id, concrete_type_id, .. } = trait_type_node_data{
            let new_assoc_type_id = {
                match concrete_type_id{
                    Some(concrete_value) => concrete_value.clone(),
                    _ => assoc_type_id.clone(),
                }
            };
            surf_generate_custom_type_if_needed(
                                                trait_type_node,
                                                &new_assoc_type_id,
                                                fuzz_tree,
                                                custom_structs,
                                                generic_subst_map,
                                            );
            let subst_type_name = {
                match &generic_subst_map.get(&new_assoc_type_id).unwrap().kind{
                    SubstTypeKind::Custom{name, ..} => name.clone(),
                    _ => String::from("Unresolved"),
                }
            };
            // Implement logic to find the rhs.
            let output_data = format!("type {placeholder_name} = {subst_type_name};");
            surf_write_output_body(
                                    &output_data, 
                                    1,
                                    &output_body_locator,
                                    output_body_map
            );
        }
    }

    // Produce the traits now
    for trait_type_node in fuzz_tree.neighbors(trait_types_node){
        let trait_type_node_data = &fuzz_tree[trait_type_node];
        if let SurfTreeNode::TraitType { placeholder_name, .. } = trait_type_node_data{
            // Send it to generate the traits if any for this type
            surf_get_node_harness(
                                    trait_type_node,
                                    false,
                                    fuzz_tree,
                                    mode,
                                    use_stmts,
                                    req_deps,
                                    custom_structs,
                                    custom_closures,
                                    fuzzer_byte_slice_len,
                                    local_var_count,
                                    generic_subst_map,
                                    custom_closure_map,
                                    indent_level,
                                    output_body_locator,
                                    output_body_map
                                );
        }
    }
}

fn surf_declare_fuzzer_slice_selector(
    indent_level: usize,
    new_output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
){
    let mut output_data = format!("let GLOBAL_DATA = match selector{{");
    surf_write_output_body(
                            &output_data,
                            indent_level,
                            &new_output_body_locator,
                            output_body_map
    );
    output_data = format!("1 => global_data.first_half,");
    surf_write_output_body(
                            &output_data,
                            indent_level+2,
                            &new_output_body_locator,
                            output_body_map
    );
    output_data = format!("_ => global_data.second_half,");
    surf_write_output_body(
                            &output_data,
                            indent_level+2,
                            &new_output_body_locator,
                            output_body_map
    );
    output_data = format!("}};");
    surf_write_output_body(
                            &output_data,
                            indent_level,
                            &new_output_body_locator,
                            output_body_map
    );
}

fn surf_declare_selector_panic_flag(
    indent_level: usize,
    new_output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
){
    
    let mut output_data = format!("if selector == 0{{");
    surf_write_output_body(
                            &output_data,
                            indent_level,
                            &new_output_body_locator,
                            output_body_map
    );

    output_data = format!("panic!(\"INTENTIONAL PANIC!\");");
    surf_write_output_body(
                            &output_data,
                            indent_level+1,
                            &new_output_body_locator,
                            output_body_map
    );

    output_data = format!("}}");
    surf_write_output_body(
                            &output_data,
                            indent_level,
                            &new_output_body_locator,
                            output_body_map
    );
}

fn surf_declare_custom_impl_num(
    fuzzer_byte_slice_len: &mut usize,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
){
    let type_converter_call = surf_get_primitive_type_converter_call("usize", *fuzzer_byte_slice_len, None);
    let output_data = format!("let custom_impl_num = {type_converter_call}");
    surf_write_output_body(&output_data, indent_level, output_body_locator, output_body_map);
    *fuzzer_byte_slice_len += surf_get_size_of_type("usize");
}

fn surf_declare_string_custom_impl_inst_num(
    string_arg_name: &str,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
){
    let output_data = format!("let custom_impl_inst_num = {string_arg_name}.0.len();");
    surf_write_output_body(
                            &output_data,
                            indent_level,
                            &output_body_locator,
                            output_body_map
    );
}

fn surf_declare_usize_custom_impl_inst_num(
    usize_arg_name: &str,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
){
    let output_data = format!("let custom_impl_inst_num = {usize_arg_name}.0;");
    surf_write_output_body(
                            &output_data,
                            indent_level,
                            &output_body_locator,
                            output_body_map
    );
}

fn surf_declare_selector(
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
){
    let output_data = format!("let selector = (custom_impl_num + custom_impl_inst_num) % 3;");
    surf_write_output_body(
                            &output_data,
                            indent_level,
                            &output_body_locator,
                            output_body_map
    );
}

fn surf_declare_fuzzer_slice_first_half_selector(
    indent_level: usize,
    new_output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
){
    let mut output_data = format!("let global_data = get_global_data();");
    surf_write_output_body(
                            &output_data,
                            indent_level,
                            &new_output_body_locator,
                            output_body_map
    );
    output_data = format!("let GLOBAL_DATA = global_data.first_half;");
    surf_write_output_body(
                            &output_data,
                            indent_level,
                            &new_output_body_locator,
                            output_body_map
    );
}

fn surf_declare_panic_flag(
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    indent_level: usize,
    new_output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
){
    surf_declare_fuzzer_slice_first_half_selector(indent_level, new_output_body_locator, output_body_map);

    let panic_flag_local = surf_declare_local(
                                                        "u8",
                                                        false,
                                                        None,
                                                        fuzzer_byte_slice_len,
                                                        local_var_count,
                                                        indent_level,
                                                        &new_output_body_locator,
                                                        output_body_map,
                                                    );
    
    let mut output_data = format!("if {panic_flag_local} % 2 == 0{{");
    surf_write_output_body(
                            &output_data,
                            indent_level,
                            &new_output_body_locator,
                            output_body_map
    );

    output_data = format!("panic!(\"INTENTIONAL PANIC!\");");
    surf_write_output_body(
                            &output_data,
                            indent_level+1,
                            &new_output_body_locator,
                            output_body_map
    );

    output_data = format!("}}");
    surf_write_output_body(
                            &output_data,
                            indent_level,
                            &new_output_body_locator,
                            output_body_map
    );
}

fn surf_declare_assoc_type_subst_type(
    assoc_type_node: NodeIndex,
    custom_type_id: &str,
    placeholder_name: &str,
    trait_def_id: &str,
    is_mutable: bool,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };
    
    let subst_type = match custom_type_id.starts_with("Self::"){
        true => {
            //println!("-> {:#?}", generic_subst_map);
            //println!("{:?}", Dot::with_config(&fuzz_tree, &[Config::EdgeNoLabel]));
            //println!("{:?}", assoc_type_node);
            let generic_name_opt = surf_map_assoc_type_self_to_custom_name(fuzz_tree, assoc_type_node, placeholder_name, trait_def_id);
            match generic_name_opt{
                Some(generic_name) => {
                    match generic_subst_map.get(&generic_name){
                        Some(subst_type) => subst_type,
                        _ => return format!("Invalid Type Substitution!"),
                    }
                },
                _ => return format!("Invalid Type Substitution!"),
            }
        },
        false => {
            //println!("{:?}", Dot::with_config(&fuzz_tree, &[Config::EdgeNoLabel]));
            //println!("-> {:?}", custom_type_id);
            //println!("Map: {:#?}", generic_subst_map);
            match generic_subst_map.get(custom_type_id){
                Some(subst_type) => subst_type,
                _ => return format!("Invalid Type Substitution!"),
            }
        },
    };

    let (custom_type_name, custom_type_inner_type_decl) = {
        if let SubstTypeKind::Custom { name, inner_type } = &subst_type.kind{
            match inner_type.as_str() {
                "usize" => (
                            name.clone(),
                            surf_declare_local(
                                                "usize",
                                                false,
                                                None,
                                                fuzzer_byte_slice_len,
                                                local_var_count,
                                                indent_level,
                                                output_body_locator,
                                                output_body_map
                                            )
                            ),
                _ => {
                    let str_local_var = surf_declare_local_str(
                                                                        false,
                                                                        fuzzer_byte_slice_len,
                                                                        local_var_count,
                                                                        indent_level,
                                                                        output_body_locator,
                                                                        output_body_map
                                                                    );
                        (
                        name.clone(),
                        surf_declare_local_string(
                                                &str_local_var,
                                                false, local_var_count,
                                                indent_level,
                                                output_body_locator,
                                                output_body_map
                                            )
                        )
                },
            }
        }
        else{
            //return String::from("Todo")
            unreachable!()
        }

    };
    let custom_type_decl = format!("let {mutability}t_{} = {custom_type_name}({custom_type_inner_type_decl});", *local_var_count);
    surf_write_output_body(
                            &custom_type_decl,
                            indent_level,
                            output_body_locator,
                            output_body_map
                        );
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}


fn surf_declare_generic_subst_type(
    generic_node: NodeIndex,
    custom_type_id: &str,
    is_mutable: bool,
    fuzz_tree: &Graph<SurfTreeNode, ()>,
    fuzzer_byte_slice_len: &mut usize,
    local_var_count: &mut usize,
    generic_subst_map: &mut HashMap::<String, SubstType>,
    indent_level: usize,
    output_body_locator: &OutputBodyLocator,
    output_body_map: &mut OutputBodyMap,
) -> String
{
    let mutability = {
        match is_mutable{
            true => "mut ",
            false => "",
        }
    };

    let subst_type = match custom_type_id == "Self"{
        true => {
            let generic_name = surf_map_generic_self_to_custom_name(fuzz_tree, generic_node);
            generic_subst_map.get(&generic_name).unwrap()
        },
        false => generic_subst_map.get(custom_type_id).unwrap(),
    };

    let (custom_type_name, custom_type_inner_type_decl) = {
        if let SubstTypeKind::Custom { name, inner_type } = &subst_type.kind{
            match inner_type.as_str() {
                "usize" => (
                            name.clone(),
                            surf_declare_local(
                                                "usize",
                                                false,
                                                None,
                                                fuzzer_byte_slice_len,
                                                local_var_count,
                                                indent_level,
                                                output_body_locator,
                                                output_body_map
                                            )
                            ),
                _ => {
                    let str_local_var = surf_declare_local_str(
                                                                        false,
                                                                        fuzzer_byte_slice_len,
                                                                        local_var_count,
                                                                        indent_level,
                                                                        output_body_locator,
                                                                        output_body_map
                                                                    );
                        (
                        name.clone(),
                        surf_declare_local_string(
                                                &str_local_var,
                                                false, local_var_count,
                                                indent_level,
                                                output_body_locator,
                                                output_body_map
                                            )
                        )
                },
            }
        }
        else{
            //return String::from("Todo")
            unreachable!()
        }

    };
    let custom_type_decl = format!("let {mutability}t_{} = {custom_type_name}({custom_type_inner_type_decl});", *local_var_count);
    surf_write_output_body(
                            &custom_type_decl,
                            indent_level,
                            output_body_locator,
                            output_body_map
                        );
    let local_name = format!("t_{}", *local_var_count);
    *local_var_count += 1;
    local_name
}


// Works only for generic Self  as argument of a trait function
fn surf_map_generic_self_to_custom_name(
    fuzz_tree: &Graph<SurfTreeNode, ()>, 
    generic_custom_ty_node: NodeIndex,
) -> String {
    //println!("{:?}", Dot::with_config(&fuzz_tree, &[Config::EdgeNoLabel]));
    let mut current_node = generic_custom_ty_node;
    // Loop until you find the desired node or there are no more parents
    while let Some(parent_node) = fuzz_tree.edges_directed(current_node, Direction::Incoming)
                                       .next()
                                       .map(|edge| edge.source()) {
        let parent_data = &fuzz_tree[parent_node];
        if let SurfTreeNode::Generic{name, ..} = parent_data{
            return name.clone();
        }
        else if let SurfTreeNode::AssocType{assoc_type_id, ..} = parent_data{
            return assoc_type_id.clone();
        }

        current_node = parent_node;
    }

    panic!("Should not be reached! -> {:?}", generic_custom_ty_node)
}

fn surf_search_trait(
    fuzz_tree: &Graph<SurfTreeNode, ()>, 
    traits_node: NodeIndex,
    assoc_type_placeholder_name: &str,
    trait_def_id: &str,
) -> Option<String>
{
    for trait_node in fuzz_tree.neighbors(traits_node){
        if let SurfTreeNode::Trait { def_id, .. } = &fuzz_tree[trait_node]{
            if def_id == trait_def_id{
                if let Some(trait_types_node) = fuzz_tree.neighbors(trait_node).next(){
                    if let SurfTreeNode::TraitTypes = &fuzz_tree[trait_types_node]{
                        return surf_search_trait_type(fuzz_tree, trait_types_node, assoc_type_placeholder_name);
                    }
                }
            }
        }
    }
    None
}

fn surf_search_trait_type(
    fuzz_tree: &Graph<SurfTreeNode, ()>, 
    trait_types_node: NodeIndex,
    assoc_type_placeholder_name: &str,
) -> Option<String>
{
    for trait_type in fuzz_tree.neighbors(trait_types_node){
        if let SurfTreeNode::TraitType { placeholder_name, assoc_type_id, concrete_type_id, .. } = &fuzz_tree[trait_type]{
            if placeholder_name == assoc_type_placeholder_name{
                if let Some(concrete_value) = concrete_type_id{
                    return Some(concrete_value.clone());
                }
                else{
                    return Some(assoc_type_id.clone());
                }
            }
        }
    }
    None
}




// Works only for generic Self and Self:: as argument of a trait function
fn surf_map_assoc_type_self_to_custom_name(
    fuzz_tree: &Graph<SurfTreeNode, ()>, 
    assoc_type_node: NodeIndex,
    assoc_type_placeholder_name: &str,
    trait_def_id: &str,
) -> Option<String> {
    let mut current_node = assoc_type_node;
    // Loop until you find the desired node or there are no more parents
    while let Some(parent_node) = fuzz_tree.edges_directed(current_node, Direction::Incoming)
                                       .next()
                                       .map(|edge| edge.source()) {
        let parent_data = &fuzz_tree[parent_node];
        match parent_data{
            SurfTreeNode::Generic { .. } => {
                if let Some(custom_ty_node) = fuzz_tree.neighbors(parent_node).next(){
                    if let SurfTreeNode::GenericCustomTy = &fuzz_tree[custom_ty_node]{
                        return surf_search_trait(fuzz_tree, custom_ty_node, assoc_type_placeholder_name, trait_def_id);
                    }
                }
            },
            SurfTreeNode::TraitType { .. } => {
                return surf_search_trait(fuzz_tree, parent_node, assoc_type_placeholder_name, trait_def_id);
            },
            SurfTreeNode::Trait { def_id, .. } => {
                if def_id == trait_def_id{
                    if let Some(trait_types_node) = fuzz_tree.neighbors(parent_node).next(){
                        if let SurfTreeNode::TraitTypes = &fuzz_tree[trait_types_node]{
                            return surf_search_trait_type(fuzz_tree, trait_types_node, assoc_type_placeholder_name);
                        }
                    }
                }
            },
            _ => {},
        }
        current_node = parent_node;
    }
    None
}

fn surf_get_custom_type_inner_type(
    fuzz_tree: &Graph<SurfTreeNode, ()>, 
    generic_custom_ty_node: NodeIndex,
) -> String {
    for neighbor in fuzz_tree.neighbors(generic_custom_ty_node){
        if let SurfTreeNode::Trait{name, ..} = &fuzz_tree[neighbor]{
            let copy_trait_paths = ["Copy", "std::marker::Copy", "core::marker::Copy"]; 
            if copy_trait_paths.contains(&name.as_str()){
                return String::from("usize")
            }
        }
    }
    String::from("String")
}

fn surf_is_sized_trait(trait_name: &str) -> bool {
    let sized_trait_paths = ["Sized", "std::marker::Sized", "core::marker::Sized"]; 
    return sized_trait_paths.contains(&trait_name);
}

fn surf_get_primitive_type_converter_call(primitive_type: &str, fuzzer_byte_slice_len: usize, is_bounded: Option<usize>) -> String{
    let bound = {
        match is_bounded{
            Some(bound_size) => format!(" % {}", bound_size+1),
            None => String::from(""),
        }
    };
    match primitive_type {
        "usize" => format!("_to_usize(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};", ),
        "isize" => format!("_to_isize(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "u8" => format!("_to_u8(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "i8" => format!("_to_i8(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "u16" => format!("_to_u16(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "i16" => format!("_to_i16(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "u32" => format!("_to_u32(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "i32" => format!("_to_i32(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "u64" => format!("_to_u64(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "i64" => format!("_to_i64(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "u128" => format!("_to_u128(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "i128" => format!("_to_i128(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "f32" => format!("_to_f32(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "f64" => format!("_to_f64(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "char" => format!("_to_char(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        "bool" => format!("_to_bool(GLOBAL_DATA, {fuzzer_byte_slice_len}){bound};"),
        _ => panic!(),
    }
}

fn surf_get_str_type_converter_call(fuzzer_byte_slice_len: usize, size_var: &str) -> String{
    format!("_to_str(GLOBAL_DATA, {}, {} + {} as usize);", fuzzer_byte_slice_len, fuzzer_byte_slice_len, size_var)
}

fn surf_get_size_of_type(type_name: &str) -> usize {
    match type_name {
        "usize" => mem::size_of::<usize>(),
        "isize" => mem::size_of::<isize>(),
        "u8" => mem::size_of::<u8>(),
        "i8" => mem::size_of::<i8>(),
        "u16" => mem::size_of::<u16>(),
        "i16" => mem::size_of::<i16>(),
        "u32" => mem::size_of::<u32>(),
        "i32" => mem::size_of::<i32>(),
        "u64" => mem::size_of::<u64>(),
        "i64" => mem::size_of::<i64>(),
        "u128" => mem::size_of::<u128>(),
        "i128" => mem::size_of::<i128>(),
        "f32" => mem::size_of::<f32>(),
        "f64" => mem::size_of::<f64>(),
        "char" => mem::size_of::<char>(),
        "bool" => mem::size_of::<bool>(),
        "str" => STR_SIZE,
        _ => panic!(), // Return None if the type is not recognized
    }
}

fn surf_add_converters(body: &mut Vec<String>){
    body.push(_data_to_u8().to_string());
    body.push(_data_to_u16().to_string());
    body.push(_data_to_u32().to_string());
    body.push(_data_to_u64().to_string());
    body.push(_data_to_u128().to_string());
    body.push(_data_to_usize().to_string());
    body.push(_data_to_i8().to_string());
    body.push(_data_to_i16().to_string());
    body.push(_data_to_i32().to_string());
    body.push(_data_to_i64().to_string());
    body.push(_data_to_i128().to_string());
    body.push(_data_to_isize().to_string());
    body.push(_data_to_f32().to_string());
    body.push(_data_to_f64().to_string());
    body.push(_data_to_char().to_string());
    body.push(_data_to_bool().to_string());
    body.push(_data_to_str().to_string());
    body.push(_unwrap_option_function().to_string());
    body.push(_unwrap_result_function().to_string());
}

fn surf_add_data_reader(body: &mut Vec<String>){
    body.push(_read_crash_file_data().to_string());
}

/* -------------------------------------------------------------------------
                        FUNCTIONS FOR STATISTICS
--------------------------------------------------------------------------*/
fn surf_calculate_and_write_stats(
    targets_map: &HashMap::<String, UrapiTargets>,
    fuzzing_targets_map: &HashMap::<String, FuzzableURAPI>,
    total_gen_time: Duration
){
    
    let mut macro_expanded_urapis = HashSet::<String>::new();
    let mut urapis_count = 0;
    for surf_urapi in SURF_URAPIS.values(){
        if SURF_MACRO_URAPIS.contains_key(&surf_urapi.def_path_str){
            if !macro_expanded_urapis.contains(&surf_urapi.def_path_str){
                urapis_count += 1;
                macro_expanded_urapis.insert(surf_urapi.def_path_str.clone());
            }
        }
        else{
            urapis_count += 1;
        }
    }
    
    let covered_urapis: usize = fuzzing_targets_map.len();
    let mut crate_checked_targets_count = 0;
    let mut crate_compilable_targets_count: usize = 0;
    let mut compilable_targets = HashMap::<String, (usize, usize)>::new(); // HashMap: URAPI: (compilable targets, total targets)
    let mut crate_target_buckets = CrateBuckets::new();
    
    let urapi_coverage_rate = ((covered_urapis as f64) / (urapis_count as f64))*100.0;

    // Calculate Target Compilable Ratio
    for (urapi_def_id, surf_urapi) in SURF_URAPIS.iter(){
        if SURF_MACRO_URAPIS.contains_key(&surf_urapi.def_path_str){
            continue;
        }
        compilable_targets.entry(urapi_def_id.clone()).or_insert((0, 0));
        if let Some(urapi_targets) = targets_map.get(urapi_def_id){
            compilable_targets.get_mut(urapi_def_id).unwrap().1 += urapi_targets.checked_targets; // update target total count
            crate_checked_targets_count += urapi_targets.checked_targets; // update crate target total count
            for urapi_target in urapi_targets.target_pairs.iter(){
                if urapi_target.compilable{
                    let urapi_target_name = &urapi_target.fuzz_target.fuzz_target_id.target_name;
                    let urapi_target_gen_time = urapi_target.fuzz_target.time_to_gen.unwrap();
                    let urapi_compilable_targets = &mut compilable_targets.get_mut(urapi_def_id).unwrap().0;
                    *urapi_compilable_targets += 1; // update compilable count
                    match *urapi_compilable_targets{
                        1 => {
                            crate_target_buckets.bucket_1ft.bucket.entry(urapi_target_name.clone()).or_insert(urapi_target_gen_time);
                            crate_target_buckets.bucket_1ft.total_gen_time += urapi_target_gen_time;
                            },
                        2 => {
                            crate_target_buckets.bucket_2ft.bucket.entry(urapi_target_name.clone()).or_insert(urapi_target_gen_time);
                            crate_target_buckets.bucket_2ft.total_gen_time += urapi_target_gen_time;
                        },
                        3 => {
                            crate_target_buckets.bucket_3ft.bucket.entry(urapi_target_name.clone()).or_insert(urapi_target_gen_time);
                            crate_target_buckets.bucket_3ft.total_gen_time += urapi_target_gen_time;
                        },
                        4 => {
                            crate_target_buckets.bucket_4ft.bucket.entry(urapi_target_name.clone()).or_insert(urapi_target_gen_time);
                            crate_target_buckets.bucket_4ft.total_gen_time += urapi_target_gen_time;
                        },
                        _ => {}, // no more than 4ft
                    }
                    crate_compilable_targets_count += 1;
                }
            }
        }
    }
    
    // For macro-expanded URAPIs, select the variant that offers the most compilable targets and calculate time and coverage based on it
    for (macro_expanded_urapi_stable_def_id, variants) in SURF_MACRO_URAPIS.iter(){
        let mut max_complilable_fuzz_targets = 0;
        let mut variant_with_max_compilable_targets_opt = None; 
        for variant_def_id in variants{
            let mut variant_compilable_fuzz_targets = 0;
            if let Some(urapi_targets) = targets_map.get(variant_def_id){
                crate_checked_targets_count += urapi_targets.checked_targets;
                for urapi_target in urapi_targets.target_pairs.iter(){
                    if urapi_target.compilable{
                        variant_compilable_fuzz_targets += 1;
                        crate_compilable_targets_count += 1;
                    }
                }
            }
            if variant_compilable_fuzz_targets > max_complilable_fuzz_targets{
                max_complilable_fuzz_targets = variant_compilable_fuzz_targets;
                variant_with_max_compilable_targets_opt = Some(variant_def_id);
            }
        }
        if let Some(variant_with_max_compilable_targets) = variant_with_max_compilable_targets_opt{
            compilable_targets.entry(macro_expanded_urapi_stable_def_id.clone()).or_insert((0, 0));
            if let Some(urapi_targets) = targets_map.get(variant_with_max_compilable_targets){
                compilable_targets.get_mut(macro_expanded_urapi_stable_def_id).unwrap().1 += urapi_targets.checked_targets; // update target total count
                for urapi_target in urapi_targets.target_pairs.iter(){
                    if urapi_target.compilable{
                        let urapi_target_name = &urapi_target.fuzz_target.fuzz_target_id.target_name;
                        let urapi_target_gen_time = urapi_target.fuzz_target.time_to_gen.unwrap();
                        let urapi_compilable_targets = &mut compilable_targets.get_mut(macro_expanded_urapi_stable_def_id).unwrap().0;
                        *urapi_compilable_targets += 1; // update compilable count
                        match *urapi_compilable_targets{
                            1 => {
                                crate_target_buckets.bucket_1ft.bucket.entry(urapi_target_name.clone()).or_insert(urapi_target_gen_time);
                                crate_target_buckets.bucket_1ft.total_gen_time += urapi_target_gen_time;
                                },
                            2 => {
                                crate_target_buckets.bucket_2ft.bucket.entry(urapi_target_name.clone()).or_insert(urapi_target_gen_time);
                                crate_target_buckets.bucket_2ft.total_gen_time += urapi_target_gen_time;
                            },
                            3 => {
                                crate_target_buckets.bucket_3ft.bucket.entry(urapi_target_name.clone()).or_insert(urapi_target_gen_time);
                                crate_target_buckets.bucket_3ft.total_gen_time += urapi_target_gen_time;
                            },
                            4 => {
                                crate_target_buckets.bucket_4ft.bucket.entry(urapi_target_name.clone()).or_insert(urapi_target_gen_time);
                                crate_target_buckets.bucket_4ft.total_gen_time += urapi_target_gen_time;
                            },
                            _ => {}, // no more than 4ft
                        }
                    }
                }
            }
        }
    }
    
    let mut total_compilable_target_rate = 0f64;
    if crate_checked_targets_count > 0{
        total_compilable_target_rate = ((crate_compilable_targets_count as f64) / (crate_checked_targets_count as f64))*100.0;
    }

    let mut target_to_urapi_rate_0ft = 0f64;
    if urapis_count > 0{
        target_to_urapi_rate_0ft = (((urapis_count - crate_target_buckets.bucket_1ft.bucket.len()) as f64) / (urapis_count as f64))*100.0;
    }

    let mut target_to_urapi_rate_1ft = 0f64;
    let mut target_to_urapi_rate_2ft = 0f64;
    let mut target_to_urapi_rate_3ft = 0f64;
    let mut target_to_urapi_rate_4ft = 0f64;
    if urapis_count > 0 {
        target_to_urapi_rate_1ft = ((crate_target_buckets.bucket_1ft.bucket.len() as f64) / (urapis_count as f64))*100.0;
        target_to_urapi_rate_2ft = ((crate_target_buckets.bucket_2ft.bucket.len() as f64) / (urapis_count as f64))*100.0;
        target_to_urapi_rate_3ft = ((crate_target_buckets.bucket_3ft.bucket.len() as f64) / (urapis_count as f64))*100.0;
        target_to_urapi_rate_4ft = ((crate_target_buckets.bucket_4ft.bucket.len() as f64) / (urapis_count as f64))*100.0;
    }

    let mut gen_time_per_target_1ft = Duration::ZERO;
    let mut gen_time_per_target_2ft = Duration::ZERO;
    let mut gen_time_per_target_3ft = Duration::ZERO;
    let mut gen_time_per_target_4ft = Duration::ZERO;

    if crate_target_buckets.bucket_1ft.bucket.len() > 0 {
        gen_time_per_target_1ft = (crate_target_buckets.bucket_1ft.total_gen_time) / (crate_target_buckets.bucket_1ft.bucket.len() as u32);
    }
    if crate_target_buckets.bucket_2ft.bucket.len() > 0 {
        gen_time_per_target_2ft = (crate_target_buckets.bucket_2ft.total_gen_time) / (crate_target_buckets.bucket_2ft.bucket.len() as u32);
    }
    if crate_target_buckets.bucket_3ft.bucket.len() > 0 {
        gen_time_per_target_3ft = (crate_target_buckets.bucket_3ft.total_gen_time) / (crate_target_buckets.bucket_3ft.bucket.len() as u32);
    }
    if crate_target_buckets.bucket_4ft.bucket.len() > 0 {
        gen_time_per_target_4ft = (crate_target_buckets.bucket_4ft.total_gen_time) / (crate_target_buckets.bucket_4ft.bucket.len() as u32);
    }

    let stats_path = format!("{}/deepSURF/surf_stats.txt", SURF_WORKING_PATH.to_string());
    let mut file = File::create(stats_path).unwrap();
    let mut stats_output = Vec::<String>::new();

    stats_output.push("\nTotal Stats:".to_string());
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    stats_output.push(format!("* URAPI Coverage: {}/{} ({}%).",
            covered_urapis,
            urapis_count,
            match urapis_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", urapi_coverage_rate)
            }
    ));
    stats_output.push(format!("* Targets Compiled: {}/{} ({}%) in [{:?}].",
            crate_compilable_targets_count,
            crate_checked_targets_count,
            match crate_checked_targets_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", total_compilable_target_rate)
            },
            total_gen_time
    ));
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    
    stats_output.push("\nTarget Stats:".to_string());
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    stats_output.push(format!("* URAPIs w/ ≥ 1ft: {}/{} ({}%).",
            crate_target_buckets.bucket_1ft.bucket.len(),
            urapis_count,
            match urapis_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", target_to_urapi_rate_1ft)
            },
    ));
    stats_output.push(format!("* Average Generation Time per Target: {:?}/{} ({}).\n",
            crate_target_buckets.bucket_1ft.total_gen_time,
            crate_target_buckets.bucket_1ft.bucket.len(),
            match crate_target_buckets.bucket_1ft.bucket.len(){
                0 => String::from("Na"),
                _ => format!("{:?}", gen_time_per_target_1ft)
            },
    ));
    stats_output.push(format!("* URAPIs w/ ≥ 2ft: {}/{} ({}%).",
            crate_target_buckets.bucket_2ft.bucket.len(),
            urapis_count,
            match urapis_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", target_to_urapi_rate_2ft)
            },
    ));
    stats_output.push(format!("* Average Generation Time per Target: {:?}/{} ({}).\n",
            crate_target_buckets.bucket_2ft.total_gen_time,
            crate_target_buckets.bucket_2ft.bucket.len(),
            match crate_target_buckets.bucket_2ft.bucket.len(){
                0 => String::from("Na"),
                _ => format!("{:?}", gen_time_per_target_2ft)
            },
    ));
    stats_output.push(format!("* URAPIs w/ ≥ 3ft: {}/{} ({}%).",
            crate_target_buckets.bucket_3ft.bucket.len(),
            urapis_count,
            match urapis_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", target_to_urapi_rate_3ft)
            },
    ));
    stats_output.push(format!("* Average Generation Time per Target: {:?}/{} ({}).\n",
            crate_target_buckets.bucket_3ft.total_gen_time,
            crate_target_buckets.bucket_3ft.bucket.len(),
            match crate_target_buckets.bucket_3ft.bucket.len(){
                0 => String::from("Na"),
                _ => format!("{:?}", gen_time_per_target_3ft)
            },
    ));
    stats_output.push(format!("* URAPIs w/ ≥ 4ft: {}/{} ({}%).",
            crate_target_buckets.bucket_4ft.bucket.len(),
            urapis_count,
            match urapis_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", target_to_urapi_rate_4ft)
            },
    ));
    stats_output.push(format!("* Average Generation Time per Target: {:?}/{} ({}).\n",
            crate_target_buckets.bucket_4ft.total_gen_time,
            crate_target_buckets.bucket_4ft.bucket.len(),
            match crate_target_buckets.bucket_4ft.bucket.len(){
                0 => String::from("Na"),
                _ => format!("{:?}", gen_time_per_target_4ft)
            },
    ));
    stats_output.push(format!("* URAPIs w/o any compilable targets: {}/{} ({}%).",
            (urapis_count - crate_target_buckets.bucket_1ft.bucket.len()),
            urapis_count,
            match urapis_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", target_to_urapi_rate_0ft)
            },
    ));
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    
    stats_output.push("\nDetailed Results: ".to_string());
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    for urapi_def_id in SURF_URAPIS.keys(){
        stats_output.push(format!("  URAPI: {urapi_def_id},"));
        match targets_map.get(urapi_def_id){
            Some(urapi_targets) => {
                let mut compilable_targets = Vec::<FuzzTarget>::new();
                let mut non_compilable_targets = Vec::<FuzzTarget>::new();
                for urapi_target in urapi_targets.target_pairs.iter(){
                    match urapi_target.compilable{
                        true => {
                            compilable_targets.push(urapi_target.fuzz_target.clone());
                        },
                        false => {
                            non_compilable_targets.push(urapi_target.fuzz_target.clone());
                        },
                    }
                }
                crate_checked_targets_count += urapi_targets.checked_targets;
                stats_output.push(format!("    - Compilable Targets ({}/{} ~ {:.2}%):",
                                                        compilable_targets.len(),
                                                        urapi_targets.checked_targets,
                                                        ((compilable_targets.len() as f64) / (urapi_targets.checked_targets as f64))*100.0,
                ));

                for compilable_target in compilable_targets{
                    stats_output.push(format!("        * {} [{:?}]", compilable_target.fuzz_target_id.target_name, compilable_target.time_to_gen.unwrap()));
                }


                if let Ok(_) = env::var("SURF_ENABLE_NON_COMPILABLE_REPORT")  {
                    stats_output.push(format!("    - Non-Compilable Targets ({}/{} ~ {:.2}%):",
                                                        non_compilable_targets.len(),
                                                        urapi_targets.checked_targets,
                                                        ((non_compilable_targets.len() as f64) / (urapi_targets.checked_targets as f64))*100.0,
                    ));
                    for non_compilable_target in non_compilable_targets{
                        stats_output.push(format!("        * {} [{:?}]", non_compilable_target.fuzz_target_id.target_name, non_compilable_target.time_to_gen.unwrap()));
                    }
                }
            },
            None => {
                stats_output.push(format!("    - 0 targets generated."));
            },
        }
    }
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    file.write_all(stats_output.join("\n").as_bytes()).expect("Cannot write the stats file!");
    println!("Stats w/o LLM-integration: {covered_urapis} out of {urapis_count} URAPIs: ({}%)", 
                                                                                                match urapis_count{
                                                                                                    0 => String::from("Na"),
                                                                                                    _ => format!("{:.2}", ((covered_urapis as f64) / (urapis_count as f64))*100.0)
    });
    println!("-----------------------------------------------------------------------------------------------------------------------------------");
}

fn surf_write_fuzzing_targets(fuzzing_targets_map: &HashMap::<String, FuzzableURAPI>){
    
    let mut stats_output = Vec::<String>::new();
    let stats_path = format!("{}/deepSURF/surf_fuzzing_targets.txt", SURF_WORKING_PATH.to_string());
    let mut file = File::create(stats_path).unwrap();
    stats_output.push("SURF: Generated Targets to be Fuzzed".to_string());
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    for (stable_urapi_def_id, fuzzable_urapi) in fuzzing_targets_map.iter(){
        stats_output.push(format!("  StableURAPI: {stable_urapi_def_id},"));
        stats_output.push(format!("    - URAPI: {},", fuzzable_urapi.urapi_def_id));
        stats_output.push(format!("    - Macro-expanded: {},",fuzzable_urapi.is_macro_expanded));
        stats_output.push(format!("    - LLM-improved: {},",fuzzable_urapi.is_llm_improved));
        stats_output.push(format!("    - Fuzzing Targets:"));
        for fuzz_target_name in fuzzable_urapi.fuzz_targets_names.iter(){
            stats_output.push(format!("      * {fuzz_target_name}"));
        }
    }
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    file.write_all(stats_output.join("\n").as_bytes()).expect("Cannot write the stats file!");
}

fn surf_write_fuzzing_targets_llm(fuzzing_targets_map: &HashMap::<String, FuzzableURAPI>){
    
    let mut stats_output = Vec::<String>::new();
    let stats_path = format!("{}/deepSURF/deepsurf_fuzzing_targets.txt", SURF_WORKING_PATH.to_string());
    let mut file = File::create(stats_path).unwrap();
    stats_output.push("deepSURF: Generated Targets to be Fuzzed".to_string());
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    for (stable_urapi_def_id, fuzzable_urapi) in fuzzing_targets_map.iter(){
        stats_output.push(format!("  StableURAPI: {stable_urapi_def_id},"));
        stats_output.push(format!("    - URAPI: {},", fuzzable_urapi.urapi_def_id));
        stats_output.push(format!("    - Macro-expanded: {},",fuzzable_urapi.is_macro_expanded));
        stats_output.push(format!("    - LLM-improved: {},",fuzzable_urapi.is_llm_improved));
        stats_output.push(format!("    - Fuzzing Targets:"));
        for fuzz_target_name in fuzzable_urapi.fuzz_targets_names.iter(){
            stats_output.push(format!("      * {fuzz_target_name}"));
        }
    }
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    file.write_all(stats_output.join("\n").as_bytes()).expect("Cannot write the stats file!");
}

fn surf_write_fuzzing_targets_llm_only(fuzzing_targets_map: &HashMap::<String, FuzzableURAPI>){
    
    let mut stats_output = Vec::<String>::new();
    let stats_path = format!("{}/deepSURF/deepsurf_llm-only_fuzzing_targets.txt", SURF_WORKING_PATH.to_string());
    let mut file = File::create(stats_path).unwrap();
    stats_output.push("deepSURF: Generated Targets to be Fuzzed".to_string());
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    for (stable_urapi_def_id, fuzzable_urapi) in fuzzing_targets_map.iter(){
        stats_output.push(format!("  StableURAPI: {stable_urapi_def_id},"));
        stats_output.push(format!("    - URAPI: {},", fuzzable_urapi.urapi_def_id));
        stats_output.push(format!("    - Macro-expanded: {},",fuzzable_urapi.is_macro_expanded));
        stats_output.push(format!("    - LLM-improved: {},",fuzzable_urapi.is_llm_improved));
        stats_output.push(format!("    - Fuzzing Targets:"));
        for fuzz_target_name in fuzzable_urapi.fuzz_targets_names.iter(){
            stats_output.push(format!("      * {fuzz_target_name}"));
        }
    }
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    file.write_all(stats_output.join("\n").as_bytes()).expect("Cannot write the stats file!");
}

fn surf_calculate_and_write_llm_stats(
    llm_covered_urapis: &HashSet<String>,
    fuzzing_targets_map: &HashMap::<String, FuzzableURAPI>,
    total_gen_time: Duration,
){
    let custom_urapis_lock = CUSTOM_URAPIS.lock().unwrap();
    let mut macro_expanded_urapis = HashSet::<String>::new();
    let mut urapis_count = 0;
    for surf_urapi in SURF_URAPIS.values(){
        if SURF_MACRO_URAPIS.contains_key(&surf_urapi.def_path_str){
            if !macro_expanded_urapis.contains(&surf_urapi.def_path_str){
                urapis_count += 1;
                macro_expanded_urapis.insert(surf_urapi.def_path_str.clone());
            }
        }
        else{
            urapis_count += 1;
        }
    }

    let total_covered_urapis_count: usize = fuzzing_targets_map.len();
    let llm_covered_urapis_count: usize = llm_covered_urapis.len();
    let custom_urapis_count = custom_urapis_lock.len();
    
    let total_urapi_coverage_rate = ((total_covered_urapis_count as f64) / (urapis_count as f64))*100.0;
    let llm_urapi_coverage_rate = ((llm_covered_urapis_count as f64) / (urapis_count as f64))*100.0;
    let custom_urapis_rate = ((custom_urapis_count as f64) / (urapis_count as f64))*100.0;
    
    let mut total_fuzzing_targets = HashSet::<String>::new();
    for fuzzing_urapi in fuzzing_targets_map.values(){
        for fuzz_target_name in fuzzing_urapi.fuzz_targets_names.iter(){
            total_fuzzing_targets.insert(fuzz_target_name.clone());
        }
    }
    let total_fuzzing_targets_count = total_fuzzing_targets.len();

    let stats_path = format!("{}/deepSURF/deepsurf_stats.txt", SURF_WORKING_PATH.to_string());
    let mut file = File::create(stats_path).unwrap();
    let mut stats_output = Vec::<String>::new();

    stats_output.push("\nTotal Stats:".to_string());
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    stats_output.push(format!("* Total URAPI Coverage: {}/{} ({}%).",
            total_covered_urapis_count,
            urapis_count,
            match urapis_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", total_urapi_coverage_rate)
            }
    ));
    stats_output.push(format!("* LLM URAPI Coverage: {}/{} ({}%).",
            llm_covered_urapis_count,
            urapis_count,
            match urapis_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", llm_urapi_coverage_rate)
            }
    ));
    stats_output.push(format!("* Total Targets (including targets directly imported from SURF): {} in [{:?}].",
            total_fuzzing_targets_count,
            total_gen_time
    ));
    stats_output.push(format!("* URAPIs w/ Custom Functionality: {}/{} ({}%).",
            custom_urapis_count,
            urapis_count,
            match urapis_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", custom_urapis_rate)
            }
    ));
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    
    // Redundant targets mean targets covering already covered URAPIs
    let redundant_target_lock = REDUNDANT_TARGETS.lock().unwrap();
    stats_output.push("\nRedundant-Targets:".to_string());
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    for redundant_fuzz_target in redundant_target_lock.iter(){
        stats_output.push(format!("  * {}", redundant_fuzz_target));
    }
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    
    // NON-URAPI-Targeted targets are targets that do not cover the originally targeted URAPI afte LLM-improvement
    let targeted_urapi_miss_targets = TARGETED_URAPI_MISS_TARGETS.lock().unwrap();
    stats_output.push("\nTargeted-URAPI-Miss-Targets:".to_string());
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    for non_urapi_targeted_target in targeted_urapi_miss_targets.iter(){
        stats_output.push(format!("  * {}", non_urapi_targeted_target));
    }
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());

    // The Final Set of Targets to be Fuzzed Depends on the Skip Option
    stats_output.push("\nTargets For Fuzzing:".to_string());
    let mut final_fuzzing_targets = total_fuzzing_targets.clone();
    if let SkipOption::NoSkip = SKIP_OPTION.unwrap(){
        stats_output.push(format!("  - [NoSkip]: Set of Fuzzing Targets = Total-Targets - Targeted-URAPI-Miss-Targets"));
        stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
        final_fuzzing_targets = final_fuzzing_targets.difference(&targeted_urapi_miss_targets).cloned().collect();
        stats_output.push(format!("Count: {}", final_fuzzing_targets.clone().into_iter().count()));
        for final_fuzz_target in &final_fuzzing_targets{
            stats_output.push(format!("  * {}", final_fuzz_target));
        }
    }
    else{
        stats_output.push(format!("  - [{:?}]: Set of Fuzzing Targets = Total-Targets - Redundant-Targets", SKIP_OPTION.unwrap()));
        stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
        final_fuzzing_targets = final_fuzzing_targets.difference(&redundant_target_lock).cloned().collect();
        stats_output.push(format!("Count: {}", final_fuzzing_targets.clone().into_iter().count()));
        for final_fuzz_target in &final_fuzzing_targets{
            stats_output.push(format!("  * {}", final_fuzz_target));
        }
    }
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());

    // DEBUG: print the list of Custom URAPIs
    // for custom_urapi_def_id in custom_urapis_lock.iter(){
    //     stats_output.push(format!("  - {}", custom_urapi_def_id));
    // }

    file.write_all(stats_output.join("\n").as_bytes()).expect("Cannot write the stats file!");
    println!("Stats w/ LLM-integration: {} out of {} URAPIs: ({}%)", 
                        total_covered_urapis_count,
                        urapis_count,
                        match urapis_count{
                            0 => String::from("Na"),
                            _ => format!("{:.2}", total_urapi_coverage_rate)
    });
    println!("-----------------------------------------------------------------------------------------------------------------------------------");
    move_dirs_to_fuzzing_corpus(&final_fuzzing_targets).expect("Failed to move targets to the fuzzing corpus!");
}

fn move_dirs_to_fuzzing_corpus(final_fuzzing_targets: &HashSet<String>) -> std::io::Result<()> {
    let fuzz_path = PathBuf::from(format!("{}/deepSURF/fuzz/", SURF_WORKING_PATH.to_string()));
    let fuzzing_corpus_path = fuzz_path.join("fuzzing_corpus");
    fs::create_dir_all(&fuzzing_corpus_path)?;

    for fuzzing_target in final_fuzzing_targets {
        let (fuzzing_target_name, fuzzing_target_type) =
            if fuzzing_target.contains(&format!(
                "_{}_turn",
                LLM_BACKEND.rsplit('/').next().unwrap_or("openrouter")
            )) {
                (fuzzing_target, "llm/compilable")
            } else {
                (fuzzing_target, "no_llm/compilable")
            };

        let source_path = fuzz_path.join(fuzzing_target_type).join(fuzzing_target_name);
        let target_path = fuzzing_corpus_path.join(fuzzing_target_name);

        if source_path.exists() {
            fs::rename(&source_path, &target_path)?;
            println!("Moved {} to fuzzing_corpus", fuzzing_target_name);
        } else {
            eprintln!("Warning: {} does not exist in {}", fuzzing_target_name, fuzzing_target_type);
        }
    }

    Ok(())
}

fn surf_calculate_and_write_llm_only_stats(
    llm_covered_urapis: &HashSet<String>,
    fuzzing_targets_map: &HashMap::<String, FuzzableURAPI>,
    total_gen_time: Duration,
){
    
    let mut macro_expanded_urapis = HashSet::<String>::new();
    let mut urapis_count = 0;
    for surf_urapi in SURF_URAPIS.values(){
        if SURF_MACRO_URAPIS.contains_key(&surf_urapi.def_path_str){
            if !macro_expanded_urapis.contains(&surf_urapi.def_path_str){
                urapis_count += 1;
                macro_expanded_urapis.insert(surf_urapi.def_path_str.clone());
            }
        }
        else{
            urapis_count += 1;
        }
    }

    let total_covered_urapis_count: usize = fuzzing_targets_map.len();
    let llm_covered_urapis_count: usize = llm_covered_urapis.len();
    
    let total_urapi_coverage_rate = ((total_covered_urapis_count as f64) / (urapis_count as f64))*100.0;
    let llm_urapi_coverage_rate = ((llm_covered_urapis_count as f64) / (urapis_count as f64))*100.0;
    
    let mut total_fuzzing_targets = HashSet::<String>::new();
    for fuzzing_urapi in fuzzing_targets_map.values(){
        for fuzz_target_name in fuzzing_urapi.fuzz_targets_names.iter(){
            total_fuzzing_targets.insert(fuzz_target_name.clone());
        }
    }
    let total_fuzzing_targets_count = total_fuzzing_targets.len();

    let stats_path = format!("{}/deepSURF/deepsurf_llm-only_stats.txt", SURF_WORKING_PATH.to_string());
    let mut file = File::create(stats_path).unwrap();
    let mut stats_output = Vec::<String>::new();

    stats_output.push("\nTotal Stats:".to_string());
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    stats_output.push(format!("* Total URAPI Coverage: {}/{} ({}%).",
            total_covered_urapis_count,
            urapis_count,
            match urapis_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", total_urapi_coverage_rate)
            }
    ));
    stats_output.push(format!("* LLM URAPI Coverage: {}/{} ({}%).",
            llm_covered_urapis_count,
            urapis_count,
            match urapis_count{
                0 => String::from("Na"),
                _ => format!("{:.2}", llm_urapi_coverage_rate)
            }
    ));
    stats_output.push(format!("* Total Targets (including targets directly imported from SURF): {} in [{:?}].",
            total_fuzzing_targets_count,
            total_gen_time
    ));
    stats_output.push("------------------------------------------------------------------------------------------------------------------------------------".to_string());
    
    file.write_all(stats_output.join("\n").as_bytes()).expect("Cannot write the stats file!");
    println!("Stats LLM-only: {} out of {} URAPIs: ({}%)", 
                        total_covered_urapis_count,
                        urapis_count,
                        match urapis_count{
                            0 => String::from("Na"),
                            _ => format!("{:.2}", total_urapi_coverage_rate)
    });
    println!("-----------------------------------------------------------------------------------------------------------------------------------");
}

/* -------------------------------------------------------------------------
                        STATIC ANALYSIS DATA LOADERS
--------------------------------------------------------------------------*/

fn surf_load_urapis_file(root_crate_name: &str) -> HashMap<String, SurfURAPI>{
    let urapis_file_path = format!("{}.urapi.json", &root_crate_name);
    let file = File::open(&urapis_file_path).expect("Cannot open the file.");
    let reader = BufReader::new(file);
    let mut urapis_map: HashMap<String, SurfURAPI> = serde_json::from_reader(reader).expect("JSON was not well-formatted");
    if let Ok(_) =  env::var("SURF_DISABLE_URAPI_CONSTR"){
        let keys_to_remove: Vec<String> = urapis_map
            .keys()
            .filter(|urapi| SURF_CONSTRUCTORS.contains_key(*urapi))
            .cloned()
            .collect();
        for urapi in keys_to_remove {
            urapis_map.remove(&urapi);
        }
    }
    urapis_map
}

fn surf_load_cmplx_tys_file(root_crate_name: &str) -> HashMap<String, HashSet<String>>{
    let cmplx_tys_file_path = format!("{}.cmplx_tys.json", &root_crate_name);
    let file = File::open(&cmplx_tys_file_path).expect("Cannot open the file.");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON was not well-formatted")
}

fn surf_load_macro_urapis_file(root_crate_name: &str) -> HashMap<String, HashSet<String>>{
    let macro_urapis_file_path = format!("{}.macro-urapi.json", &root_crate_name);
    let file = File::open(&macro_urapis_file_path).expect("Cannot open the file.");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON was not well-formatted")
}


fn surf_load_constructors_file(root_crate_name: &str) -> HashMap<String, SurfConstructorData>{
    let constructors_file_path = format!("{}.constrs.json", &root_crate_name);
    let file = File::open(&constructors_file_path).expect("Cannot open the file.");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON was not well-formatted")
}

fn surf_load_enums_file(root_crate_name: &str) -> HashMap<String, HashMap::<String, Vec<SurfFnArg>>>{
    let enums_file_path = format!("{}.enums.json", &root_crate_name);
    let file = File::open(&enums_file_path).expect("Cannot open the file.");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON was not well-formatted")
}

fn surf_load_traits_file(root_crate_name: &str) -> HashMap<String, SurfTraitData>{
    let traits_file_path = format!("{}.traits.json", &root_crate_name);
    let file = File::open(&traits_file_path).expect("Cannot open the file.");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON was not well-formatted")
}

fn surf_load_deps_file(root_crate_name: &str) -> HashMap<String, SurfDepType>{
    let deps_file_path = format!("{}.deps.json", &root_crate_name);
    let file = File::open(&deps_file_path).expect("Cannot open the file.");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON was not well-formatted")
}

fn surf_load_trait_fns_file(root_crate_name: &str) -> HashMap<String, SurfTraitFnData>{
    let trait_fns_file_path = format!("{}.trait_fns.json", &root_crate_name);
    let file = File::open(&trait_fns_file_path).expect("Cannot open the file.");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("JSON was not well-formatted")
}

fn surf_load_working_path() -> String{
    if let Ok(working_path) =  env::var("SURF_WORKING_PATH"){
        working_path
    }
    else{
        panic!("Please set the env variable 'SURF_WORKING_PATH'");
    }
}

fn surf_load_global_data_path() -> String{
    if let Ok(global_data_path) =  env::var("GLOBAL_DATA_PATH"){
        global_data_path
    }
    else{
        panic!("Please set the env variable 'GLOBAL_DATA_PATH'");
    }
}

fn try_get_skip_option() -> Option<SkipOption>{
    if let Ok(_) = env::var("SURF_ENABLE_LLMS"){
        let skip_option_env_var = env::var("SURF_SKIP_OPTION")
                                            .expect("SURF_SKIP_OPTION environment variable not set");
        match SkipOption::from_str(&skip_option_env_var) {
            Ok(skip_option) => {
                return Some(skip_option);
            },
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(-1);
            },
        }
    }
    None
}

fn get_llm_backend() -> String{
    std::env::var("LLM_BACKEND")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_LLM_BACKEND.to_string())
}

fn get_stable_urapi_def_ids_set() -> HashSet<String>{
    let mut stable_urapi_def_ids_set = HashSet::<String>::new();
    for surf_urapi in SURF_URAPIS.values(){
        stable_urapi_def_ids_set.insert(surf_urapi.def_path_str.clone());
    }
    stable_urapi_def_ids_set
}

fn surf_load_llm_only_paths(llm_urapi: String, template_name: &str, library_dir: &str) -> LlmOnlyPaths {
    
    let template_dir = PathBuf::from(format!("{library_dir}deepSURF/fuzz/llm-only/templates/{template_name}"));
    let template_harness = PathBuf::from(format!("{library_dir}deepSURF/fuzz/llm-only/templates/{template_name}/src/{template_name}.rs"));
    
    let documentation = match surf_generate_documentation(&library_dir){
        Some(md_file) => md_file,
        _ => PathBuf::from(""),
    };
    LlmOnlyPaths {
        llm_urapi,
        template_dir,
        template_harness,
        documentation,
    }
}

fn surf_load_paths(target_urapi: String, fuzz_target_id: &str, library_dir: &str) -> Paths {
    
    let target_dir = PathBuf::from(format!("{library_dir}deepSURF/fuzz/no_llm/compilable/{fuzz_target_id}"));
    let sample_harness = PathBuf::from(format!("{library_dir}deepSURF/fuzz/no_llm/compilable/{fuzz_target_id}/src/{fuzz_target_id}.rs"));
    let static_analysis_dir = format!("{library_dir}deepSURF/report/");
    let static_analysis = surf_find_analysis_files(PathBuf::from(static_analysis_dir)).unwrap();
    
    let documentation = match surf_generate_documentation(&library_dir){
        Some(md_file) => md_file,
        _ => PathBuf::from(""),
    };
    Paths {
        target_urapi,
        target_dir,
        sample_harness,
        static_analysis,
        documentation,
    }
}

fn surf_generate_documentation(project_dir: &str) -> Option<PathBuf> {
    
    // Generate the documentation json files
    let cargo_doc_output = Command::new("cargo")
        .arg("+rustc-rustdoc-md")
        .arg("doc")
        .arg("--no-deps")
        .arg("--document-private-items")
        .env("RUSTDOCFLAGS", "-Z unstable-options --output-format json")
        .env("RUSTFLAGS", "-Z ub-checks=no")
        .current_dir(project_dir)
        .output()
        .expect("Failed to run `cargo doc`");

    if !cargo_doc_output.status.success() {
        println!("cargo doc failed!");
        return None;
    }

    let crate_name = &TARGET_CRATE_NAME.clone();
    let normalized_crate_name = crate_name.to_string().replace("-", "_");
    
    // Construct paths properly
    let project_path = Path::new(project_dir);

    // Build paths using PathBuf
    let primary_path = {
        let mut path = PathBuf::from(project_path);
        path.push("target");
        path.push("doc");
        path.push(format!("{}.json", normalized_crate_name));
        path
    };

    let secondary_path = {
        let mut path = PathBuf::from(project_path);
        path.push("..");  // Go up one directory
        path.push("target");
        path.push("doc");
        path.push(format!("{}.json", normalized_crate_name));
        path
    };

    // Check which path exists
    let json_path = if primary_path.exists() {
        primary_path
    } else if secondary_path.exists() {
        secondary_path
    } else {
        println!(
            "rustdoc-md failed! Could not find {} in:\n- {}\n- {}",
            normalized_crate_name,
            primary_path.display(),
            secondary_path.display()
        );
        return None;
    };

    let rustdoc_md_output = Command::new("rustdoc-md")
        .arg("--path")
        .arg(json_path)
        .arg("--output")
        .arg(format!("{}.md", crate_name))
        .current_dir(project_dir)  // Works with &str
        .output()
        .expect("Failed to run `rustdoc-md`");

    if !rustdoc_md_output.status.success() {
        println!("rustdoc-md failed!");
        return None;
    }

    Some(PathBuf::from(format!("{project_dir}{crate_name}.md")))
}

fn surf_find_analysis_files(dir: PathBuf) -> Result<StaticAnalysisMetadata, Box<dyn Error>> {
    let mut static_analysis = StaticAnalysisMetadata::new();
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            let path_str = path.to_string_lossy();
            if path_str.ends_with(".cmplx_tys.json") {
                static_analysis.cmplx_types_to_cons = path.to_path_buf();
            } else if path_str.ends_with(".constrs-llm.json") {
                static_analysis.analyzed_cons = path.to_path_buf();
            } else if path_str.ends_with(".urapi-llm.json") {
                static_analysis.analyzed_urapis = path.to_path_buf();
            }
        }
    }
    Ok(static_analysis)
}

#[derive(Debug, Clone, Copy)]
pub enum SkipOption {
    Skip,
    CondSkip,
    NoSkip,
}

impl SkipOption {
    // Convert user input to enum variant
    fn from_str(input: &str) -> Result<Self, String> {
        match input.trim().to_lowercase().as_str() {
            "skip" => Ok(SkipOption::Skip),
            "condskip" => Ok(SkipOption::CondSkip),
            "noskip" => Ok(SkipOption::NoSkip),
            _ => Err(format!("'{}' is not a valid option", input.trim())),
        }
    }
}

#[derive(Debug)]
struct Paths {
    target_urapi: String,
    target_dir: PathBuf,
    sample_harness: PathBuf,
    static_analysis: StaticAnalysisMetadata,
    documentation: PathBuf,
}

#[derive(Debug)]
struct LlmOnlyPaths {
    llm_urapi: String,
    template_dir: PathBuf,
    template_harness: PathBuf,
    documentation: PathBuf,
}

#[derive(Debug)]
pub struct StaticAnalysisMetadata{
    cmplx_types_to_cons: PathBuf,
    analyzed_cons: PathBuf,
    analyzed_urapis: PathBuf,
}

impl StaticAnalysisMetadata{
    pub fn new() -> Self{
        Self{
            cmplx_types_to_cons: PathBuf::default(),
            analyzed_cons: PathBuf::default(),
            analyzed_urapis: PathBuf::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum SurfDepType{
    Version(String),
    Path(String),
    Empty
}

#[derive(Clone, Debug, PartialEq)]
pub enum HarnessGenerationMode{
    SubstituteImplicitGens,
    NoSubstitution,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct TargetId{
    urapi_def_id: String,
    target_name: String,
}

impl TargetId{
    fn new(urapi_def_id: String, target_name: String) -> Self{
        Self {
            urapi_def_id,
            target_name,
            }
    }
}

#[derive(Debug, Clone)]
struct Harness{
    harness: String,
    fuzzer_slice_len: Option<usize>,
    req_deps: Option<HashMap<String, SurfDepType>>
}

impl Harness{
    fn new(harness: String, fuzzer_slice_len: Option<usize>, req_deps: Option<HashMap<String, SurfDepType>>) -> Self{
        Self {
            harness,
            fuzzer_slice_len,
            req_deps
        }
    }
}

#[derive(Debug, Clone)]
struct FuzzTarget{
    fuzz_target_id: TargetId,
    fuzz_target_harness: Harness,
    time_to_gen: Option<Duration>,
}

impl FuzzTarget{
    pub fn new(fuzz_target_id: TargetId, fuzz_target_harness: Harness) -> Self{
        Self {
            fuzz_target_id,
            fuzz_target_harness,
            time_to_gen: None,
        }
    }
}

#[derive(Debug, Clone)]
struct ReplayTarget{
    replay_target_id: TargetId,
    replay_target_harness: Harness,
}

impl ReplayTarget{
    pub fn new(replay_target_id: TargetId, replay_target_harness: Harness) -> Self{
        Self {
            replay_target_id,
            replay_target_harness
        }
    }
}

#[derive(Debug, Clone)]
struct TargetPair{
    compilable: bool,
    fuzz_target: FuzzTarget,
    replay_target: ReplayTarget,
}

impl TargetPair{
    pub fn new(fuzz_target: FuzzTarget, replay_target: ReplayTarget) -> Self{
        Self {
            compilable: false,
            fuzz_target,
            replay_target
        }
    }
}

struct FuzzTargetBucket{
    bucket: HashMap<String, Duration>,
    total_gen_time: Duration,
}

impl FuzzTargetBucket{
    pub fn new() -> Self{
        Self {
            bucket: HashMap:: <String, Duration>::new(),
            total_gen_time: Duration::ZERO,
        }
    }
}

struct CrateBuckets{
    bucket_1ft: FuzzTargetBucket,
    bucket_2ft: FuzzTargetBucket,
    bucket_3ft: FuzzTargetBucket,
    bucket_4ft: FuzzTargetBucket,
}

impl CrateBuckets{
    pub fn new() -> Self{
        Self {
            bucket_1ft: FuzzTargetBucket::new(),
            bucket_2ft: FuzzTargetBucket::new(),
            bucket_3ft: FuzzTargetBucket::new(),
            bucket_4ft: FuzzTargetBucket::new(),
        }
    }
}


#[derive(Debug)]
pub struct FuzzableURAPI{
    urapi_def_id: String,
    is_macro_expanded: bool,
    is_llm_improved: bool,
    fuzz_targets_names: HashSet<String>,
}

#[derive(Debug)]
pub struct UrapiTargets{
    checked_targets: usize,
    target_pairs: Vec<TargetPair>,
}

impl UrapiTargets{
    fn new() -> Self{
        Self {
            checked_targets: 0,
            target_pairs: Vec::<TargetPair>::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct OutputBodyMap{
    main: Vec<String>,
    traits: HashMap<String, HashMap<String, Vec<String>>>,
    closures: HashMap<String, Vec<String>>,
}

impl OutputBodyMap {
    fn new() -> OutputBodyMap{
        Self {
            main: Vec::<String>::new(),
            traits: HashMap::<String, HashMap<String, Vec<String>>>::new(),
            closures: HashMap::<String, Vec<String>>::new(),
        }
    }
}

enum OutputBodyLocator{
    Main,
    Traits(String, String),
    Closures(String),
}

#[derive(Debug, Clone)]
struct SubstType{
    kind: SubstTypeKind,
    traits: HashSet<String>
}

impl SubstType {
    fn new(kind: SubstTypeKind) -> SubstType{
        Self {
            kind,
            traits: HashSet::<String>::default(),
        }
    }
}

#[derive(Debug, Clone)]
enum SubstTypeKind{
    Custom{
        name: String,
        inner_type: String,
    },
    Candidate{
        node_index: NodeIndex,
    }
}


struct FuzzTree{
    id: u64,
    root: Option<NodeIndex>,
    tree: Graph<SurfTreeNode, ()>,
}

impl FuzzTree{
    fn new(root: NodeIndex, tree: Graph<SurfTreeNode, ()>) -> Self{
        Self {
            id: 0,
            root: Some(root),
            tree,
        }
    }
}

#[derive(Debug, Clone)]
struct SurfTree{
    root: Option<NodeIndex>,
    tree: Graph<SurfTreeNode, ()>,
}

impl SurfTree{
    fn new() -> Self{
        Self {
            root: None,
            tree: Graph::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct SurfURAPI{
    pub name: String,
    pub full_name: String,
    pub def_path_str: String,
    pub inputs: Vec<SurfFnArg>,
    pub output: Option<SurfFnArg>,
    pub implicit_generics: Vec<SurfFnArg>,
    pub flags: HashMap<String, bool>,
    pub crate_name: String,
    //#[serde(skip_deserializing)]
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct SurfConstructorData{
    pub name: String,
    pub full_name: String,
    pub def_path_str: String,
    pub inputs: Vec<SurfFnArg>,
    pub output: Option<SurfFnArg>,
    pub implicit_generics: Vec<SurfFnArg>,
    pub flags: HashMap<String, bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SurfTraitData{
    pub name: String,
    pub is_unsafe: bool,
    pub external: Option<SurfExternalData>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct SurfExternalData{
    pub crate_name: String,
    pub trait_path: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct SurfTraitFnData{
    pub name: String,
    pub full_name: String,
    pub is_unsafe: bool,
    pub span_str: String,
    pub inputs: Vec<SurfFnArg>,
    pub output: Option<SurfFnArg>,
    pub flags: HashMap<String, bool>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
enum SurfTreeNode{
    Api{
        def_id: String,
        name: String,
        full_name: String,
        crate_name: String,
        output: Option<SurfFnArg>,
        has_self: bool,
        is_drop: bool,
        is_display: bool,
        is_debug: bool
    },
    ApiInputs,
    ApiImplicitGenerics,
    ApiOutput,
    Constructor{
        def_id: String,
        name: String,
        full_name: String,
        has_self: bool,
        output: Option<SurfFnArg>,
        cmplx_type_def_id: String
    },
    ConstructorInputs,
    ConstructorImplicitGenerics,
    Closure(String, bool),
    Trait{
            def_id: String,
            name: String,
            is_unsafe: bool,
            external_data: Option<SurfExternalData>,
    },
    TraitFns,
    TraitTypes,
    TraitFn{
                def_id: String,
                name: String,
                full_name: String,
                is_unsafe: bool,
                sig_str:String,
                has_self: bool
            },
    TraitType{
        def_id: String,
        placeholder_name: String,
        assoc_type_id: String,
        concrete_type_id: Option<String>,
    },
    Struct{name: String, full_name: String, is_consumable: bool},
    Enum{name: String, full_name: String, is_consumable: bool},
    SimpleVariants,
    ComplexVariants,
    Variant(String),
    Generic{
        name: String,
    },
    AssocType{
        def_id: String,
        trait_def_id: String,
        placeholder_name: String,
        assoc_type_id: String,
    },
    GenericCustomTy,
    GenericCandidates,
    Primitive(String),
    String,
    Reference(bool),
    RawPointer(bool),
    DynTrait,
    Slice,
    ReferencedSlice(bool),
    Str,
    ReferencedStr,
    Array(String),
    Tuple,
    Vector,
    Uinit,
    Box,
    ToOption,
    FromOption,
    FromTuple{field_num: usize},
    ToResult(String),
    FromResult(String),
    FnInputs,
    FnOutput,
    Todo(String),
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum SurfFnArg{
    Primitive(String),
    String,
    Generic{
        name: String,
        traits: HashMap<String, Box<SurfFnArg>>, // trait_def_id -> SurfFnArg::Trait
    },
    AssocType{
        def_id: String,
        trait_def_id: String,
        placeholder_name: String,
        assoc_type_id: String,
        traits: HashMap<String, Box<SurfFnArg>>, 
    },
    Trait{
        def_id: String,
        types: Vec<Box<SurfFnArg>>, // vec![AssoType(...)]
        funcs: HashSet<String>
    },
    TraitType{
        def_id: String,
        placeholder_name: String,
        assoc_type_id: String,
        concrete_type_id: Option<String>,
        traits: HashMap<String, Box<SurfFnArg>>, 
    },
    Closure{param_name: String, inputs: Vec<Box<SurfFnArg>>, output: Option<Box<SurfFnArg>>, is_mutable: bool},
    Struct{def_id: String, name: String, full_name: String, is_consumable: bool},
    Reference(Box<SurfFnArg>, bool),
    RawPointer(Box<SurfFnArg>, bool),
    DynTrait(Vec<Box<SurfFnArg>>),
    Slice(Box<SurfFnArg>),
    Array(Box<SurfFnArg>, String),
    Tuple(Vec<Box<SurfFnArg>>),
    Enum{def_id: String, name: String, full_name: String, is_consumable: bool},
    Vector(Box<SurfFnArg>),
    Uinit(Box<SurfFnArg>),
    Box(Box<SurfFnArg>),
    Option(Box<SurfFnArg>),
    Result{ok: Box<SurfFnArg>, err: String},
    Todo(String),
}