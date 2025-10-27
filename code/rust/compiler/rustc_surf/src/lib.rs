#![allow(rustc::potential_query_instability)]
use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use lazy_static::lazy_static;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{DefId, LOCAL_CRATE};
use rustc_hir::PrimTy;
use rustc_hir::{ItemKind, Node, Safety, LangItem};
use rustc_middle::query::Key;
use rustc_middle::ty::DynKind::Dyn;
use std::sync::Mutex;
use serde::Serialize;
use rustc_middle::ty::{self, AliasTy, AssocItem, AliasTyKind, AssocKind, ClauseKind, ExistentialPredicate, Instance, InstanceKind, ParamTy, Ty, TyCtxt, Visibility, GenericParamDefKind, GenericArgKind};
use rustc_middle::mir::TerminatorKind;
use petgraph::graph::*;
use petgraph::dot::{Config, Dot};
use std::fs::{self, File};
use std::env;
use std::path::Path;
use std::io::{BufWriter, Write};
use petgraph::visit::Bfs;
use petgraph::algo::*;
use rustc_middle::middle::privacy::Level;
use toml::Value;
use rustc_span::{symbol::sym, ExpnKind};
use std::path::PathBuf;
//use rustc_span::def_id::DefPathHash;
use regex::Regex;
const STD_LIB_CRATES: &[&str] = &[
                                "core",
                                "alloc",
                                "std",
                                "test",
                                "term",
                                "unwind",
                                "proc_macro",
                                "panic_abort",
                                "panic_unwind",
                                "profiler_builtins",
                                "rtstartup",
                                "rustc-std-workspace-core",
                                "rustc-std-workspace-alloc",
                                "rustc-std-workspace-std",
                                "backtrace",
                            ];

lazy_static! {
    #[derive(Debug)]
    pub static ref SURF_FNS: Mutex<FxHashMap<DefId, SurfFunction>> = Mutex::new(FxHashMap::default());
    
    #[derive(Debug)]
    pub static ref SURF_TRAITS_TO_CMPLX: Mutex<FxHashMap<DefId, FxHashSet<DefId>>> = Mutex::new(FxHashMap::default());
    pub static ref SURF_CONSUMABLE_CMPLX: Mutex<FxHashSet<DefId>> = Mutex::new(FxHashSet::default());
    pub static ref SURF_TRAITS_TO_PRIM: Mutex<FxHashMap<DefId, FxHashSet<PrimTy>>> = Mutex::new(FxHashMap::default());
    pub static ref SURF_UNSAFE_BLOCK_FNS: Mutex<FxHashSet<DefId>> = Mutex::new(FxHashSet::default());
    pub static ref SURF_TRAIT_DEF_TO_IMPL: Mutex<FxHashMap<DefId, FxHashSet<DefId>>> = Mutex::new(FxHashMap::<DefId, FxHashSet<DefId>>::default());
    pub static ref SURF_CFG: Mutex<Graph<DefId, ()>> = Mutex::new(Graph::<DefId, ()>::new()); // If you don't need it globally remove it
    pub static ref SURF_FNS_STATS: Mutex<FxHashMap<&'static str, FxHashMap<&'static str, u64>>> = Mutex::new(FxHashMap::<&'static str, FxHashMap::<&'static str, u64>>::default());
    pub static ref SURF_APIS_REACH_UNSAFE: Mutex<FxHashMap<DefId, SurfURAPI>> = Mutex::new(FxHashMap::default());
    
    // Analyzed Complex type Constructors
    pub static ref SURF_COMPLEX_TYPES_TO_CONSTRUCTORS: Mutex<FxHashMap<DefId, FxHashSet<DefId>>> = Mutex::new(FxHashMap::<DefId, FxHashSet<DefId>>::default());
    pub static ref SURF_COMPLEX_PENDING_ANALYSIS: Mutex<FxHashSet<DefId>> = Mutex::new(FxHashSet::<DefId>::default());

    pub static ref SURF_CONSTRUCTORS: Mutex<FxHashMap<DefId, SurfConstructorData>> = Mutex::new(FxHashMap::<DefId, SurfConstructorData>::default());
    pub static ref SURF_TRAIT_FNS: Mutex<FxHashMap<DefId, SurfTraitFnData>> = Mutex::new(FxHashMap::<DefId, SurfTraitFnData>::default());
    pub static ref SURF_TRAITS: Mutex<FxHashMap<DefId, SurfTraitData>> = Mutex::new(FxHashMap::<DefId, SurfTraitData>::default());
    pub static ref SURF_ENUMS: Mutex<FxHashMap<DefId, FxHashMap::<String, Vec<SurfFnArg>>>> = Mutex::new(FxHashMap::<DefId, FxHashMap::<String, Vec<SurfFnArg>>>::default());
    
    // CFG helper data structures
    pub static ref DEF_ID_TO_NODE: Mutex<FxHashMap<DefId, NodeIndex>> = Mutex::new(FxHashMap::<DefId, NodeIndex>::default());
    pub static ref NODE_TO_DEF_ID: Mutex<FxHashMap<NodeIndex, DefId>> = Mutex::new(FxHashMap::<NodeIndex, DefId>::default());

    pub static ref TARGET_CRATE_NAME: Option<String> = load_target_crate_name();
    pub static ref TARGET_CRATE_USED_DEPS: Mutex<FxHashMap<String, CrateDepType>> = Mutex::new(FxHashMap::default());
    pub static ref TARGET_CRATE_DEPS: Mutex<FxHashSet<String>> = Mutex::new(FxHashSet::default());

    // Macros Recording
    pub static ref SURF_MACRO_URAPIS: Mutex<FxHashMap<String, FxHashSet<DefId>>> = Mutex::new(FxHashMap::<String, FxHashSet<DefId>>::default());

}

/*
------------------------------------------------------------------------------------------------------------------
    Functions for loading data from external files
------------------------------------------------------------------------------------------------------------------
*/

/*
    If env var `SURF_WORKING_PATH` is set, try to set TARGET_CRATE_NAME by reading the related Cargo.toml.
*/
fn load_target_crate_name() -> Option<String> {
    match env::var("SURF_WORKING_PATH"){
        Ok(targeting_crate_path) => {
            let targeting_toml_path = format!("{targeting_crate_path}/Cargo.toml");
            if let Ok(toml_contents) = fs::read_to_string(&targeting_toml_path){
                if let Ok(toml_entries) = toml_contents.parse::<Value>(){
                    match toml_entries.get("package").and_then(|pkg| pkg.get("name")){
                        Some(crate_name) => {
                            match crate_name.as_str(){
                                Some(crate_name_str) => return Some(crate_name_str.replace("-", "_")),
                                None => eprintln!("Unknown crate name."),
                            }
                        },
                        None => eprintln!("Crate name not found."),
                    }
                }
            }
        },
        _ => eprintln!("The env variable 'SURF_WORKING_PATH' is not set!"),
    }
    None
}

/*
    If env var `SURF_WORKING_PATH` is set, try to find the versions of the dependencies
    used by the targeting crate (TARGET_CRATE_USED_DEPS).
    We open and read the related Cargo.toml to get the versions of the dependencies.
*/
pub fn load_target_toml_data(){
    if let Ok(targeting_crate_path) = env::var("SURF_WORKING_PATH"){
        let targeting_toml_path = format!("{targeting_crate_path}/Cargo.toml");
        if let Ok(toml_contents) = fs::read_to_string(&targeting_toml_path){
            if let Ok(toml_entries) = toml_contents.parse::<Value>(){
                if let Some(dependencies) = toml_entries.get("dependencies"){
                    for (key, val) in dependencies.as_table().unwrap() {
                        if let Some(dep_type) = TARGET_CRATE_USED_DEPS.lock().unwrap().get_mut(key){
                            if let Some(version) = val.as_str() {
                                *dep_type = CrateDepType::Version(version.to_string());
                            } else if let Some(table) = val.as_table() {
                                if let Some(path) = table.get("path") {
                                    *dep_type = CrateDepType::Path(path.to_string());
                                }
                                else if let Some(version) = table.get("version") {
                                    *dep_type = CrateDepType::Version(version.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}



/*
------------------------------------------------------------------------------------------------------------------
    Functions for checking if an argument is of a specific type or has a specific attribute
------------------------------------------------------------------------------------------------------------------
*/

/*
    Check if the input DefId is the root crate (i.e., has no parent).
*/
fn item_is_root_crate<'tcx>(tcx: TyCtxt<'tcx>, item_def_id: DefId) -> bool {
    tcx.opt_parent(item_def_id).is_some()
}

/*
    Check if the the crate where the input DefId is defined either in the dependencies
    of the current crate or the standard libraries.
    If is defined inside one of the dependencies, then update the `TARGET_CRATE_USED_DEPS` map.
*/
fn item_is_imported<'tcx>(tcx: TyCtxt<'tcx>, item_def_id: DefId) -> bool{
    let item_crate_name = tcx.crate_name(item_def_id.krate).to_ident_string();
    if TARGET_CRATE_DEPS.lock().unwrap().contains(&item_crate_name){
        if !STD_LIB_CRATES.contains(&item_crate_name.as_str()){
            TARGET_CRATE_USED_DEPS.lock().unwrap().insert(item_crate_name.to_string(), CrateDepType::Empty);
        }
        return true;
    }
    return false;
}

/*
    Check if the input item is publicly accessible from the users of the crate.
*/
fn item_is_exported<'tcx>(tcx: TyCtxt<'tcx>, item_def_id: DefId) -> bool{
    match item_def_id.as_local(){
        Some(local_def_id) => tcx.effective_visibilities(()).is_exported(local_def_id),
        _ => false,
    }
}

/*
    Check if the associate input item is Fn kind.
*/
fn assoc_item_is_fn(item: &AssocItem) -> bool {
    item.kind == AssocKind::Fn
}

/*
    Check if the associate input item is Type kind.
*/
fn assoc_item_is_type(item: &AssocItem) -> bool {
    item.kind == AssocKind::Type
}

/*
    Check if the associate input item is Fn kind and safe.
*/
fn assoc_item_is_safe_fn<'tcx>(tcx: TyCtxt<'tcx>, item: &AssocItem) -> bool {
    assoc_item_is_fn(item) && tcx.fn_sig(item.def_id).skip_binder().skip_binder().safety == Safety::Safe
}

/*
    Check if the input surf function is unsafe.
*/
fn surf_fn_is_unsafe(surf_fn: &SurfFunction) -> bool {
    surf_fn.safety == SurfFnSafety::UnsafeFn
}

/*
    Check if the input function item has self argument.
*/
fn fn_item_has_self<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> bool{
    let parent_def_id = tcx.parent(def_id);
    match tcx.def_kind(parent_def_id){
        DefKind::Trait
        | DefKind::Enum
        | DefKind::Struct
        | DefKind::Impl { .. } => {
            let associated_item = tcx.associated_item(def_id);
            match assoc_item_is_fn(&associated_item){
                true => associated_item.fn_has_self_parameter,
                false => false,
            }
        },
        _ => false,
    }
}

/*
    Check if the input function item has default implementation.
*/
fn fn_item_has_default_impl<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> bool{
    let defaultness = tcx.defaultness(def_id);
    match defaultness.is_default(){
        true => defaultness.has_value(),
        _ => false,
    }
}

pub fn surf_has_output<'tcx>(output: Ty<'tcx>) -> bool{
    match output.kind() {
        ty::Tuple(list) => !list.is_empty(),
        _ => true,
    }
}

pub fn surf_record_macro_expanded_urapis<'tcx>(tcx: TyCtxt<'tcx>) {
    let mut macro_exp_urapis_lock = SURF_MACRO_URAPIS.lock().unwrap();
    for urapi_def_id in SURF_APIS_REACH_UNSAFE.lock().unwrap().keys(){
        let span = tcx.def_span(urapi_def_id);
        let expn_data = span.ctxt().outer_expn_data();
        if let ExpnKind::Macro(_, _) = expn_data.kind{
            if let Some(_) = expn_data.macro_def_id {
                macro_exp_urapis_lock.entry(get_stable_def_id_location(tcx, *urapi_def_id)).or_insert(FxHashSet::<DefId>::default()).insert(*urapi_def_id);
            }
        }

    }
}

pub fn surf_fn_has_unsafe_block(def_id: DefId) -> bool{
    SURF_UNSAFE_BLOCK_FNS.lock().unwrap().contains(&def_id)
}

pub fn surf_has_unresolved_calls(surf_urapi: &SurfURAPI, trait_def_id: DefId, fn_def_id: DefId) -> bool{
    match surf_urapi.reachable_unresolved_callees.get(&trait_def_id){
        Some(unresolved_fns) => unresolved_fns.contains(&fn_def_id),
        None => false,
    }
}

pub fn surf_fn_calls_unresolved(surf_func: &SurfFunction) -> bool{
    !surf_func.unresolved_callees.is_empty()
}

pub fn surf_is_directly_public<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> bool{
    //println!("DEF: {:?}", def_id);
    let local_def_id = def_id.as_local().unwrap();
    return tcx.effective_visibilities(()).is_directly_public(local_def_id)
}

pub fn surf_add_fn(
    def_id: DefId, 
    func_name: &str,
    safety: SurfFnSafety,
    crate_name: &str,
    visibility: SurfFnVisibility,
    fn_type: SurfFnType
){
    SURF_FNS.lock().unwrap()
            .insert(def_id, SurfFunction::new(func_name, safety, crate_name, visibility, fn_type));
}

pub fn surf_add_unsafe_block_fn(def_id: DefId){
    SURF_UNSAFE_BLOCK_FNS.lock().unwrap().insert(def_id);
}

pub fn surf_add_caller(callee_def_id: DefId, caller_def_id: DefId){
    //println!("HERE!: {:?} -> {:?}", caller_def_id, callee_def_id);
    if let Some(callee_surf_func) = SURF_FNS.lock().unwrap().get_mut(&callee_def_id){
        //println!("INSIDE!");
        callee_surf_func.callers.insert(caller_def_id);
    }
}

pub fn surf_add_unresolved_callee(caller_def_id: DefId, callee_def_id: DefId){
    if let Some(caller_surf_func) = SURF_FNS.lock().unwrap().get_mut(&caller_def_id){
        caller_surf_func.unresolved_callees.insert(callee_def_id);
    }
}

pub fn surf_get_fuzz_closure_def_id<'tcx>(tcx: TyCtxt<'tcx>, main_def_id: DefId) -> Option<DefId>{
    if tcx.is_mir_available(main_def_id){
        let main_body = tcx.optimized_mir(main_def_id);
        let main_local_def_id = main_body.source.def_id().expect_local();
        if !tcx.hir().body_owner_kind(main_local_def_id).is_fn_or_closure(){
            return None;
        }
        let param_env = tcx.param_env_reveal_all_normalized(main_local_def_id);
        let blocks = main_body.basic_blocks.iter();
        for bb_data in blocks{
            let terminator = bb_data.terminator();
            if let TerminatorKind::Call { ref func, .. } = terminator.kind {
                let func_type = func.ty(main_body, tcx);
                if let ty::FnDef(callee_def_id, args) = *func_type.kind(){
                    let args = tcx.try_normalize_erasing_regions(param_env, args).ok().unwrap();
                    if let Some(callee) = Instance::resolve(tcx, param_env, callee_def_id, args).ok().flatten(){
                        if let InstanceKind::Item(callee_impl_def_id) = callee.def{
                            if let Some(item_name) = tcx.opt_item_name(callee_impl_def_id){
                                if item_name.to_ident_string() == "fuzz" && tcx.crate_name(callee_impl_def_id.krate).to_ident_string() == "afl"{
                                    if let ty::GenericArgKind::Type(arg_ty) = args[0].unpack(){
                                        if let ty::Closure(fuzz_closure_def_id, _) = arg_ty.kind(){
                                            return Some(*fuzz_closure_def_id)
                                        }
                                    }
                                }                                            
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn surf_get_called_urapis<'tcx>(tcx: TyCtxt<'tcx>, fuzz_closure_def_id: DefId) -> FxHashSet<String>{
    let mut called_urapis = FxHashSet::<String>::default();
    if tcx.is_mir_available(fuzz_closure_def_id){
        let fuzz_closure_body = tcx.optimized_mir(fuzz_closure_def_id);
        let fuzz_closure_local_def_id = fuzz_closure_body.source.def_id().expect_local();
        if tcx.hir().body_owner_kind(fuzz_closure_local_def_id).is_fn_or_closure(){
            let param_env = tcx.param_env_reveal_all_normalized(fuzz_closure_local_def_id);
            let blocks = fuzz_closure_body.basic_blocks.iter();
            for bb_data in blocks{
                let terminator = bb_data.terminator();
                if let TerminatorKind::Call { ref func, .. } = terminator.kind {
                    let func_type = func.ty(fuzz_closure_body, tcx);
                    if let ty::FnDef(callee_def_id, args) = *func_type.kind(){
                        let args = tcx.try_normalize_erasing_regions(param_env, args).ok().unwrap();
                        if let Some(callee) = Instance::resolve(tcx, param_env, callee_def_id, args).ok().flatten(){
                            if let InstanceKind::Item(callee_impl_def_id) = callee.def {
                                called_urapis.insert(get_stable_def_id_location(tcx, callee_impl_def_id));
                            }
                        }
                    }
                }
            }
        }
    }
    called_urapis
}

pub fn surf_print_called_urapis(called_urapis: FxHashSet<String>){
    let json = serde_json::to_string(&called_urapis).unwrap();
    println!("---BEGIN-CALLED-URAPIS---");
    println!("{}", json);
    println!("---END-CALLED-URAPIS---");
}

///rust/compiler/rustc_mir_transform/src/inline.rs
pub fn surf_record_calls<'tcx>(tcx: TyCtxt<'tcx>, caller_def_id: DefId) {
    let caller_body;
    if tcx.is_mir_available(caller_def_id){
        caller_body = tcx.optimized_mir(caller_def_id);
        let caller_local_def_id = caller_body.source.def_id().expect_local();
        if !tcx.hir().body_owner_kind(caller_local_def_id).is_fn_or_closure(){
            return;
        }
        let param_env = tcx.param_env_reveal_all_normalized(caller_local_def_id);
        let blocks = caller_body.basic_blocks.iter();
        for bb_data in blocks{
            let terminator = bb_data.terminator();
            if let TerminatorKind::Call { ref func, .. } = terminator.kind {
                let func_type = func.ty(caller_body, tcx);
                //println!("FUNC: {:?} -> {:?}", caller_def_id, func_type);
                match *func_type.kind() {
                    ty::FnDef(callee_def_id, args) => {
                        let args = tcx.try_normalize_erasing_regions(param_env, args).ok().unwrap();
                        match Instance::resolve(tcx, param_env, callee_def_id, args).ok().flatten(){
                            Some(callee) => {
                                match callee.def {
                                    // The dyn case
                                    InstanceKind::Virtual(gen_fn_def_id, _) => {
                                        //println!("1: {:?}", gen_fn_def_id);
                                        if let Some(trait_impls) = SURF_TRAIT_DEF_TO_IMPL.lock().unwrap().get(&gen_fn_def_id){
                                            for callee_impl_def_id in trait_impls{
                                                surf_add_caller(*callee_impl_def_id, caller_def_id);
                                            }
                                        }
                                        surf_add_unresolved_callee(caller_def_id, gen_fn_def_id);
                                    },
                                    // The normal case
                                    InstanceKind::Item(callee_impl_def_id) => {
                                        //println!("2: {:?}", callee_impl_def_id);
                                        surf_add_caller(callee_impl_def_id, caller_def_id);
                                    },
                                    // The unexpected case
                                    _ => {
                                        eprintln!("INVESTIGATION NEED: {:?}.", caller_def_id);
                                    },
                                }
                            },
                            None => {
                                //println!("3: {:#?}", SURF_TRAIT_DEF_TO_IMPL.lock().unwrap().get(&callee_def_id));
                                if let Some(trait_impls) = SURF_TRAIT_DEF_TO_IMPL.lock().unwrap().get(&callee_def_id){
                                    for callee_impl_def_id in trait_impls{
                                        surf_add_caller(*callee_impl_def_id, caller_def_id);
                                    }
                                }
                                surf_add_unresolved_callee(caller_def_id, callee_def_id);
                            },
                        }
                    },
                    ty::FnPtr(_) => {
                        eprintln!("UNIMPLEMENTED: Call to FnPtr From: {:?}.", caller_def_id);
                    },
                    _ => {
                        eprintln!("INVESTIGATION NEED: {:?}.", caller_def_id);
                    },
                }
            }
        }
    }
}

pub fn surf_cfg_analysis<'tcx>(tcx: TyCtxt<'tcx>){
    // Construct The Nodes
    for (func_def_id, _) in SURF_FNS.lock().unwrap().iter(){
        let node = SURF_CFG.lock().unwrap().add_node(*func_def_id);
        DEF_ID_TO_NODE.lock().unwrap().entry(*func_def_id).or_insert(node);
        NODE_TO_DEF_ID.lock().unwrap().entry(node).or_insert(*func_def_id);
    }
    // Construct The Edges
    let def_id_to_node = DEF_ID_TO_NODE.lock().unwrap().clone();
    let node_to_def_id = NODE_TO_DEF_ID.lock().unwrap().clone();
    for (callee_def_id, callee_surf_func) in SURF_FNS.lock().unwrap().iter(){
        for caller_def_id in callee_surf_func.callers.iter(){
            let caller_node = def_id_to_node.get(caller_def_id).unwrap();
            let callee_node = def_id_to_node.get(callee_def_id).unwrap();
            SURF_CFG.lock().unwrap().add_edge(*callee_node, *caller_node, ());
        }
    }
    // Record The Public Predecessors That Reach Unsafe Blocks
    let mut cfg_copy = SURF_CFG.lock().unwrap().clone();
    let surf_fns_copy = SURF_FNS.lock().unwrap().clone();
    for (func_def_id, funch_node) in def_id_to_node.iter(){
        if let Some(callee_surf_func) = SURF_FNS.lock().unwrap().get_mut(&func_def_id){
            match callee_surf_func.safety{
                SurfFnSafety::UnsafeBlockFn => {
                    let mut bfs = Bfs::new(&cfg_copy, *funch_node);
                    while let Some(next_node) = bfs.next(&cfg_copy) {
                        if let Some(caller_def_id) = node_to_def_id.get(&next_node){
                            if let Some(caller_surf_func) = surf_fns_copy.get(caller_def_id){
                                if item_is_exported(tcx, *caller_def_id) && !surf_fn_is_unsafe(caller_surf_func){
                                    callee_surf_func.public_predecessors.insert(*caller_def_id);
                                    let (caller_name, caller_full_name) = surf_get_fn_names(tcx, *caller_def_id);
                                    SURF_APIS_REACH_UNSAFE.lock().unwrap()
                                                    .entry(*caller_def_id)
                                                    .or_insert(SurfURAPI::new(caller_name,
                                                                                caller_full_name,
                                                                                get_stable_def_id_location(tcx, *caller_def_id),
                                                                                tcx.crate_name(caller_def_id.krate).to_ident_string()));
                                }   
                            }
                        }
                    }
                }
                _ => ()
            }
        }
    }
    // Find the Unresolved Function Calls from the Above Public Predecessors
    cfg_copy.reverse();
    for (urapi_def_id, surf_urapi) in SURF_APIS_REACH_UNSAFE.lock().unwrap().iter_mut(){
        let urapi_node = def_id_to_node.get(urapi_def_id).unwrap();
        let mut bfs = Bfs::new(&cfg_copy, *urapi_node);
        while let Some(next_node) = bfs.next(&cfg_copy) {
            if let Some(callee_def_id) = node_to_def_id.get(&next_node){
                if let Some(callee_surf_func) = surf_fns_copy.get(&callee_def_id){
                    if surf_fn_calls_unresolved(callee_surf_func){
                        for unresolved_callee_def_id in callee_surf_func.unresolved_callees.iter(){
                            let parent_trait_def_id = tcx.parent(*unresolved_callee_def_id);
                            surf_urapi.reachable_unresolved_callees.entry(parent_trait_def_id)
                                                                        .or_insert(FxHashSet::<DefId>::default())
                                                                        .insert(*unresolved_callee_def_id);
                        }
                    }
                }
            }
        }
    }
}

pub fn surf_get_call_paths(callee: DefId, caller: DefId){
    let def_id_to_node = DEF_ID_TO_NODE.lock().unwrap().clone();
    let cfg_copy = &SURF_CFG.lock().unwrap().clone();
    if let Some(caller_node) = def_id_to_node.get(&caller){
        if let Some(callee_node) = def_id_to_node.get(&callee){
            let paths: Vec<_> = all_simple_paths::<Vec<_>, _>(cfg_copy, *callee_node, *caller_node, 0, None)
                                                            .collect();
            eprintln!("{:#?}", paths);
        }
    }
}

pub fn surf_remove_generic_part(input: &str) -> String {
    let re = Regex::new(r"::<[^>]+>").unwrap();
    re.replace_all(input, "").to_string()
}

pub fn surf_get_external_item_name<'tcx>(tcx: TyCtxt<'tcx>, item_def_id: DefId) -> String{
    surf_remove_generic_part(&tcx.def_path_str(item_def_id))
}

/*
    This is Called by Exported (Public) Functions and Structure/Enum Types (that may be external).
    If External try to get the name otherwise Unresolved.
*/ 
pub fn surf_get_accessible_full_path<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> String{
    if !item_is_exported(tcx, def_id){
        if item_is_imported(tcx, def_id){
            return surf_get_external_item_name(tcx, def_id);
        }
        if let Some(item_name) = tcx.opt_item_name(def_id){
            return item_name.to_ident_string();
        }
        return String::from("*Unresolved*");
    }

    let item_name = tcx.item_name(def_id).to_ident_string();
    if surf_is_directly_public(tcx, def_id){
        surf_get_direct_path(tcx, def_id, item_name)
    }
    else{
        surf_get_reexported_path(tcx, def_id, item_name)
    }
}


pub fn get_stable_def_id_location(tcx: TyCtxt<'_>, def_id: DefId) -> String {
    let span = tcx.def_span(def_id);
    let source_map = tcx.sess.source_map();
    
    // Handle macro expansions
    let orig_span = span.source_callee()
        .map(|expn_data| expn_data.def_site)
        .unwrap_or(span);

    // Get local path from span
    let filename = source_map
                            .filename_for_diagnostics(&source_map.span_to_filename(orig_span))
                            .to_string();
    
    // Get line number (1-based)
    let line = source_map.lookup_line(orig_span.lo())
        .map(|loc| loc.line + 1)
        .unwrap_or(0);

    let abs_path = match surf_resolve_absolute_path(&filename){
        Some(path) => path.to_string_lossy().to_string(),
        _ => String::from("*Unresolved*"),
    };

    format!("{}::{}::{}", tcx.item_name(def_id).to_ident_string(), abs_path, line)
}

fn surf_resolve_absolute_path(relative_path: &str) -> Option<PathBuf> {
    let path = Path::new(relative_path);
    
    if path.is_absolute() {
        Some(path.to_path_buf())
    } else {
        // Get current working directory and join with relative path
        if let Ok(cwd) = std::env::current_dir(){
            let full_path = cwd.join(path);
            if !full_path.exists() { // Explicit existence check
                None
            }
            else{
                Some(full_path)
            }
        }
        else{
            None
        }
    }
}


pub fn surf_get_fn_names<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> (String, String){
    let full_name = surf_get_accessible_full_path(tcx, def_id);
    let name = {
        match fn_item_has_self(tcx, def_id){
            true => tcx.item_name(def_id).to_ident_string(),
            false => full_name.clone(),
        }
    };
    (name, full_name)
}

pub fn surf_get_direct_path<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId, item_name: String) -> String{
    let mut item_def_id = def_id;
    let mut full_name = String::from(item_name);
    while let Some(parent_item_def_id) = tcx.opt_parent(item_def_id) { // It stops when reaches root crate
        let (parent_name, opt_switched_parent) = surf_get_item_name(tcx, parent_item_def_id);
        full_name = parent_name + "::" + &full_name;
        match opt_switched_parent{
            Some(switched_parent) => item_def_id = switched_parent,
            None => item_def_id = parent_item_def_id,
        }
    }
    full_name
}

pub fn surf_get_reexported_path<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId, item_name: String) -> String{
    let mut full_name = String::from(item_name);
    let mut item_def_id = tcx.parent(def_id); // Starts from the father
    loop {
        match surf_get_reexported_visibility(tcx, item_def_id){
            Some(Visibility::Public) => {
                let (item_name, opt_switched_item) = surf_get_item_name(tcx, item_def_id);
                full_name = item_name + "::" + &full_name;
                item_def_id = match opt_switched_item{
                    Some(switched_item) => tcx.parent(switched_item),
                    None => tcx.parent(item_def_id),
                };
            },
            Some(Visibility::Restricted(next_local_def_id)) => item_def_id = next_local_def_id.to_def_id(),
            None => return String::from("*Unresolved*"),
        }
        if surf_is_directly_public(tcx, item_def_id) {
            full_name = tcx.item_name(item_def_id).to_ident_string() + "::" + &full_name; // Add the first parent that is directly public
            break;
        }
    }
    surf_get_direct_path(tcx, item_def_id, full_name) 
}

pub fn surf_get_reexported_visibility<'tcx>(tcx: TyCtxt<'tcx>, item_def_id: DefId) -> Option<Visibility>{
    let item_local_def_id = item_def_id.as_local().unwrap();
    match tcx.effective_visibilities(()).effective_vis(item_local_def_id){
        Some(item_visibility) => Some(*item_visibility.at_level(Level::Reexported)),
        None => None,
    }
}

pub fn surf_get_item_name<'tcx>(tcx: TyCtxt<'tcx>, item_def_id: DefId) -> (String, Option<DefId>){
    if let Some(item_local_def_id) = item_def_id.as_local(){
        match tcx.def_kind(item_def_id){
            DefKind::Mod => {
                if item_is_root_crate(tcx, item_def_id){
                    let mod_item = tcx.hir().expect_item(item_local_def_id);
                    match mod_item.kind{
                        ItemKind::Mod(_)=> (mod_item.ident.name.to_ident_string(), None),
                        _ => {
                            (String::from("*Unresolved*"), None)
                        }
                    }
                }
                else{
                    (tcx.crate_name(LOCAL_CRATE).to_ident_string(), None)
                }
            },
            DefKind::Trait => {
                let trait_item = tcx.hir().expect_item(item_local_def_id);
                (trait_item.ident.name.to_ident_string(), None)
            },
            DefKind::Impl { of_trait } => {
                if of_trait{
                    let trait_header = tcx.impl_trait_header(item_def_id).unwrap();
                    let trait_args = trait_header.trait_ref.skip_binder().args;
                    if trait_args.types().count() > 0 {
                        if let ty::Adt(adt_def, _) = trait_args.type_at(0).kind(){
                            if adt_def.is_struct() || adt_def.is_enum(){
                                return (tcx.item_name(adt_def.did()).to_ident_string(), None)
                            }
                        }
                    }
                    let trait_def_id = trait_header.trait_ref.skip_binder().def_id;
                    (tcx.item_name(trait_def_id).to_ident_string(), None)
                }
                else{
                    let opt_inherent_impls = tcx.with_stable_hashing_context(|hcx| {
                        match tcx.crate_inherent_impls(()){
                            Ok (crate_inherent_impls) => Some(crate_inherent_impls.inherent_impls.to_sorted(&hcx, true)),
                            _ => None,
                        }
                    });
                    if let Some(inherent_impls) = opt_inherent_impls{
                        for (def_id, impls) in inherent_impls{
                            if let Node::Item(hir_item) = tcx.hir_node(tcx.local_def_id_to_hir_id(def_id)){
                                if let ItemKind::Enum(_,_) | ItemKind::Struct(_,_) = hir_item.kind{
                                    if impls.contains(&item_def_id){
                                        return (tcx.item_name(def_id.to_def_id()).to_ident_string(), Some(def_id.to_def_id()));
                                    } 
                                }
                            }
                        }
                    }
                    (String::from("*Unresolved*"), None)
                }
            },
            DefKind::Fn | DefKind::AssocFn => {
                (tcx.item_name(item_def_id).to_ident_string(), Some(item_def_id))
            },
            DefKind::Struct => {
                (tcx.item_name(item_def_id).to_ident_string(), None)
            },
            _ => {
                (String::from("*Unresolved*"), None)
            },
        }
    }
    else{
        (String::from("*Unresolved*"), None)
    }
}

pub fn surf_get_span_str<'tcx>(tcx: TyCtxt<'tcx>, func_def_id: DefId) -> String{
    let source_map = tcx.sess.source_map();
    match source_map.span_to_snippet(tcx.def_span(func_def_id)) {
        Ok(snippet) =>  snippet,
        Err(_) => String::from(""),
    }
}

pub fn surf_unsafe_reaching_apis_analysis<'tcx>(tcx: TyCtxt<'tcx>){
    let urapi_def_ids: Vec<DefId> = SURF_APIS_REACH_UNSAFE.lock().unwrap().keys().cloned().collect();
    for urapi_def_id in &urapi_def_ids{
        //println!("{:?}", urapi_def_id);
        let (urapi_inputs, urapi_output) = surf_analyze_fn_args(tcx, *urapi_def_id);
        let mut urapi_lock = SURF_APIS_REACH_UNSAFE.lock().unwrap();
        urapi_lock.get_mut(&urapi_def_id).unwrap().inputs = urapi_inputs;
        urapi_lock.get_mut(&urapi_def_id).unwrap().output = urapi_output;
        //println!("{:?}", urapi_def_id);
    }

    let mut trait_fns_to_analyse = FxHashSet::<DefId>::default();
    for (urapi_def_id, surf_urapi) in SURF_APIS_REACH_UNSAFE.lock().unwrap().iter_mut(){
        // The if-clause alone is the old version
        if let Some(surf_constructor) = SURF_CONSTRUCTORS.lock().unwrap().get(urapi_def_id){
            surf_urapi.implicit_generics = surf_constructor.implicit_generics.clone();
        }
        // This is part of the new version
        // This analysis supports only generics params.
        // TODO support trait types.
        else{

            let mut urapi_implicit_trait_bounds_map = FxHashMap::<GenericType, FxHashSet<DefId>>::default();            
            // Get the trait bounds of the parent of the URAPI
            if let Some(parent_def_id) = tcx.predicates_of(urapi_def_id).parent{
                if let DefKind::Trait | DefKind::Impl { .. } = tcx.def_kind(parent_def_id) {
                    surf_get_item_trait_bounds(tcx, parent_def_id, &mut urapi_implicit_trait_bounds_map);
                }
            }
            // Get the trait bounds of the URAPI, this includes trait bounds that are associated with generics in complex type arguments
            surf_get_item_trait_bounds(tcx, *urapi_def_id, &mut urapi_implicit_trait_bounds_map);

            // Analyze the bounds, converting them to SurfFnArgs
            let mut implicit_analyzed_generics = Vec::<SurfFnArg>::default();
            let mut additional_trait_fns_to_analyze = FxHashSet::<DefId>::default();
            let urapi_implicit_generics = urapi_implicit_trait_bounds_map.keys();
            for urapi_implicit_generic in urapi_implicit_generics{
                // let param_ty_str = match urapi_implicit_generic.clone().get_param_ty_opt(){
                //     Some(param_ty) => param_ty.name.to_ident_string(),
                //     _ => String::from(""),
                // };
                if let GenericType::Param(param_ty) = urapi_implicit_generic{
                    let param_ty_str = param_ty.name.to_ident_string();
                    let mut seen_trait_types = FxHashSet::<GenericType>::default();
                    let arg_required_bounds = surf_get_urapi_required_bounds(
                                                                                tcx,
                                                                                &Some(surf_urapi.clone()),
                                                                                *urapi_def_id,
                                                                                &urapi_implicit_generic,
                                                                                &param_ty_str,
                                                                                &urapi_implicit_trait_bounds_map,
                                                                                &FxHashMap::<GenericType, GenericType>::default(), //to be extended if needed
                                                                                &mut seen_trait_types,
                                                                                &mut additional_trait_fns_to_analyze
                                                                            );
                    // Check if is implicit
                    let mut arg_required_bounds_filtered = FxHashMap::<DefId, Box<SurfFnArg>>::default();
                    let mut generic_match = false;
                    // Search the generic inputs of the URAPI
                    for surf_fn_arg in surf_urapi.inputs.iter(){
                        if let SurfFnArg::Generic { name, traits } = surf_fn_arg{
                            // Check if the generic is implicit
                            if param_ty_str == *name{
                                // If there is match then add as implicit only the trait bounds that has not been covered from the explicit params
                                for (implicit_trait_def_id, implicit_trait_surf_fn_arg) in arg_required_bounds.iter(){
                                    if !traits.contains_key(implicit_trait_def_id){
                                        arg_required_bounds_filtered.insert(implicit_trait_def_id.clone(), implicit_trait_surf_fn_arg.clone());
                                    }
                                }
                                generic_match = true;
                                break;
                            }
                        }
                    }
                    if generic_match{
                        if !arg_required_bounds_filtered.is_empty(){
                            implicit_analyzed_generics.push(SurfFnArg::Generic{
                                                                                name: param_ty_str.clone(),
                                                                                traits: arg_required_bounds_filtered.clone(),
                                                                            }
                            );
                        }
                    }
                    else{
                        implicit_analyzed_generics.push(SurfFnArg::Generic{
                                                                                name: param_ty_str.clone(),
                                                                                traits: arg_required_bounds.clone(),
                                                                            }
                        );
                    }
                    
                    for trait_fn_def_id in additional_trait_fns_to_analyze.iter(){
                        if !SURF_TRAIT_FNS.lock().unwrap().contains_key(trait_fn_def_id){
                            trait_fns_to_analyse.insert(*trait_fn_def_id);
                        }
                    }
                }
            }
            surf_urapi.implicit_generics = implicit_analyzed_generics.clone();
        }
    }
    let mut trait_fns_to_analyse: Vec<DefId> = trait_fns_to_analyse.iter().cloned().collect();
    while !trait_fns_to_analyse.is_empty(){
        if let Some(trait_fn_def_id) = trait_fns_to_analyse.pop(){
            let (trait_fn_name, trait_fn_full_name) = surf_get_fn_names(tcx, trait_fn_def_id);
            let trait_fn_span_str = surf_get_span_str(tcx, trait_fn_def_id);
            let is_trait_fn_unsafe = tcx.fn_sig(trait_fn_def_id).skip_binder().skip_binder().safety == Safety::Unsafe;
            SURF_TRAIT_FNS.lock().unwrap().insert(trait_fn_def_id, SurfTraitFnData::new(trait_fn_name, trait_fn_full_name, trait_fn_span_str, is_trait_fn_unsafe));
            let (trait_fn_inputs, trait_fn_output) = surf_analyze_fn_args(tcx, trait_fn_def_id);
            SURF_TRAIT_FNS.lock().unwrap().get_mut(&trait_fn_def_id).unwrap().inputs = trait_fn_inputs;
            SURF_TRAIT_FNS.lock().unwrap().get_mut(&trait_fn_def_id).unwrap().output = trait_fn_output;
        }
    }
}

pub fn surf_collect_param_defaults<'tcx>(
    tcx: TyCtxt<'tcx>,
    item_def_id: DefId,
) -> FxHashMap<ParamTy, ParamTy>
{
    let mut param_defaults = FxHashMap::<ParamTy, ParamTy>::default();
    if let Some(parent_def_id) = tcx.predicates_of(item_def_id).parent{
        if let DefKind::Trait | DefKind::Impl { .. } = tcx.def_kind(parent_def_id) {
            for param in tcx.generics_of(parent_def_id).own_params.iter() {
                if let GenericParamDefKind::Type { .. } = param.kind{
                    if let Some(default_value) = param.default_value(tcx){
                        if let GenericArgKind::Type(default_type) = default_value.skip_binder().unpack(){
                            if let ty::Param(default_param_ty) = default_type.kind() {
                                let param_ty = ParamTy::new(param.index, param.name);
                                param_defaults.entry(param_ty)
                                                .or_insert(*default_param_ty);
                            }
                        }
                    }
                }
            }
        }
    }
    param_defaults
}

pub fn surf_get_param_owner<'tcx>(tcx: TyCtxt<'tcx>, item_def_id: DefId) -> Option<DefId>{
    if let Some(_item_local_def_id) = item_def_id.as_local(){
        if let  DefKind::Impl { of_trait } = tcx.def_kind(item_def_id){
            if of_trait{
                let trait_header = tcx.impl_trait_header(item_def_id).unwrap();
                let trait_args = trait_header.trait_ref.skip_binder().args;
                if trait_args.types().count() > 0 {
                    if let ty::Adt(adt_def, _) = trait_args.type_at(0).kind(){
                        if adt_def.is_struct() || adt_def.is_enum(){
                            return Some(adt_def.did());
                        }
                    }
                }
            }
            else{
                let opt_inherent_impls = tcx.with_stable_hashing_context(|hcx| {
                    match tcx.crate_inherent_impls(()){
                        Ok (crate_inherent_impls) => Some(crate_inherent_impls.inherent_impls.to_sorted(&hcx, true)),
                        _ => None,
                    }
                });
                if let Some(inherent_impls) = opt_inherent_impls{
                    for (def_id, impls) in inherent_impls{
                        if let Node::Item(hir_item) = tcx.hir_node(tcx.local_def_id_to_hir_id(def_id)){
                            if let ItemKind::Enum(_,_) | ItemKind::Struct(_,_) = hir_item.kind{
                                if impls.contains(&item_def_id){
                                    return Some(def_id.to_def_id());
                                } 
                            }
                        }
                    }
                }
            }
        }
    }
    return None;
}


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum SubstType<'tcx>{
    Generic(ParamTy),
    AssocTy(AliasTy<'tcx>),
    Other(DefId)
}


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum GenericType{
    Param(ParamTy),
    TraitType(Option<ParamTy>, DefId)
}

impl GenericType{
    pub fn get_param_ty_opt(self) -> Option<ParamTy>{
        match self{
            GenericType::Param(param_ty) => Some(param_ty.clone()),
            GenericType::TraitType(param_ty_opt, _) => match param_ty_opt{
                Some(param_ty) => Some(param_ty.clone()),
                _ => None,
            }
        }
    }
}

pub fn surf_collect_bounds_projections_closures<'tcx>(
    tcx: TyCtxt<'tcx>,
    func_def_id: DefId
) -> (
    FxHashMap<GenericType, FxHashSet<DefId>>,
    FxHashMap<GenericType, GenericType>,
    FxHashMap<ParamTy, SurfClosureSig<'tcx>>)
{
    let mut func_trait_bounds_map = FxHashMap::<GenericType, FxHashSet<DefId>>::default();
    let mut func_projections_map = FxHashMap::<GenericType, GenericType>::default();
    let mut func_closures_map= FxHashMap::<ParamTy, SurfClosureSig<'tcx>>::default();

    if let Some(parent_def_id) = tcx.predicates_of(func_def_id).parent{
        if let DefKind::Trait | DefKind::Impl { .. } = tcx.def_kind(parent_def_id) {
            surf_get_item_trait_bounds(tcx, parent_def_id, &mut func_trait_bounds_map);
            surf_get_item_closures(tcx, parent_def_id, &mut func_trait_bounds_map, &mut func_closures_map);
            surf_get_item_projections(tcx, parent_def_id, &mut func_projections_map);
        }
    }
    surf_get_item_trait_bounds(tcx, func_def_id, &mut func_trait_bounds_map);
    surf_get_item_closures(tcx, func_def_id, &mut func_trait_bounds_map, &mut func_closures_map);
    surf_get_item_projections(tcx, func_def_id, &mut func_projections_map);
    surf_get_implicit_bounds(tcx, &mut func_trait_bounds_map);
    surf_get_implicit_projections(tcx, &func_trait_bounds_map, &mut func_projections_map);
    //println!("Func: {:?} Param_to_trait: {:#?}, Type_alias: {:#?}", func_def_id, param_to_trait_pred, type_alias_map);
    //println!("FUNC: {:?}", func_def_id);
    //println!("BOUNDS: {:#?}", func_trait_bounds_map);
    //println!("PROJECTIONS: {:#?}", func_projections_map);
    (func_trait_bounds_map, func_projections_map, func_closures_map)
}

pub fn surf_trait_has_unresolved_generics<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_pred_def_id: DefId,
) -> bool
{
    for generic_param in tcx.generics_of(trait_pred_def_id).own_params.iter(){
        if generic_param.name.to_ident_string() == "Self" {
            continue;
        }
        if let GenericParamDefKind::Type{ has_default: false, .. } = generic_param.kind{
            return true;
        }
    }
    false
}

pub fn surf_fn_has_non_inferable_generics<'tcx>(
    tcx: TyCtxt<'tcx>,
    func_def_id: DefId,
) -> bool{
    let fn_generic_preds = surf_get_fn_generic_preds(tcx, func_def_id);
    let mut fn_generic_inputs = FxHashSet::<ParamTy>::default();
    let mut fn_generic_output = FxHashSet::<ParamTy>::default();
    for arg in tcx.fn_sig(func_def_id).skip_binder().skip_binder().inputs(){
        surf_get_arg_generics(tcx, *arg, &mut fn_generic_inputs);
    }
    surf_get_arg_generics(tcx, tcx.fn_sig(func_def_id).skip_binder().skip_binder().output(), &mut fn_generic_output);
    // println!("FUNC: {:?}: ", func_def_id);
    // println!("INPUTS: {:#?}: ", fn_generic_inputs);
    // println!("OUTPUT: {:#?}: ", fn_generic_output);
    // println!("PREDS: {:#?}: ", fn_generic_preds);
    // println!("GENERICS: {:#?}: ", tcx.generics_of(func_def_id)); 
    for generic in fn_generic_preds{
        if !fn_generic_inputs.contains(&generic){
            return true;
        }
    }
    for generic in fn_generic_output{
        if !fn_generic_inputs.contains(&generic){
            return true;
        }
    }
    false
}

pub fn surf_get_fn_generic_preds<'tcx>(
    tcx: TyCtxt<'tcx>,
    func_def_id: DefId,
) -> FxHashSet<ParamTy>
{
    let mut fn_generic_preds = FxHashSet::<ParamTy>::default();
    let func_preds = tcx.predicates_of(func_def_id).predicates;
    for func_pred in func_preds{
        if let ClauseKind::Trait(trait_pred) = func_pred.0.kind().skip_binder(){
            if let ty::Param(param_type) = trait_pred.self_ty().kind(){
                if param_type.name.to_ident_string() != "Self"{
                    fn_generic_preds.insert(*param_type);
                }
            }
        }
    }
    fn_generic_preds
}

pub fn surf_get_arg_generics<'tcx>(
    tcx: TyCtxt<'tcx>,
    current_ty: Ty<'tcx>,
    generic_inputs: &mut FxHashSet<ParamTy>,
){
    match current_ty.kind(){
        ty::Adt(adt_def, adt_args) => {
            let adt_def_id = adt_def.did();
            if surf_is_vec(tcx, adt_def_id) 
            || adt_def.is_box()
            || surf_is_maybeuinit(tcx, adt_def_id)
            || surf_is_option(tcx, adt_def_id)
            || surf_is_result(tcx, adt_def_id) {
                if adt_args.types().count() > 0 {
                    surf_get_arg_generics(tcx, adt_args.type_at(0), generic_inputs);
                }
            }
            else if adt_def.is_struct() || adt_def.is_enum(){
                for adt_arg in adt_args.types(){
                    surf_get_arg_generics(tcx, adt_arg, generic_inputs);
                }
            }
        },
        ty::Array(inner_type, _)
        | ty::Slice(inner_type)
        | ty::Ref(_, inner_type, _)
        | ty::RawPtr(inner_type, _) => surf_get_arg_generics(tcx, *inner_type, generic_inputs),
        ty::Tuple(tuple_args) => {
            for tuple_arg in tuple_args.iter(){
                surf_get_arg_generics(tcx, tuple_arg, generic_inputs);
            }
        },
        ty::Param(param_type) => {
            generic_inputs.insert(*param_type);
        },
        _ => {},
    }
}

pub fn surf_get_item_trait_bounds<'tcx>(
    tcx: TyCtxt<'tcx>,
    item_def_id: DefId,
    func_trait_bounds_map: &mut FxHashMap::<GenericType, FxHashSet<DefId>>,
)
{
    let item_preds = tcx.predicates_of(item_def_id).predicates;
    for item_pred in item_preds{
        if let ClauseKind::Trait(trait_bound) = item_pred.0.kind().skip_binder(){
            match trait_bound.self_ty().kind(){
                ty::Param(param_type) => {
                    func_trait_bounds_map.entry(GenericType::Param(*param_type))
                                                .or_insert(FxHashSet::<DefId>::default())
                                                .insert(trait_bound.def_id());
                },
                ty::Alias(AliasTyKind::Projection, alias_ty) => {
                    if let Some(_) = tcx.trait_of_item(alias_ty.def_id){
                        if let ty::Param(param_ty) = alias_ty.args.type_at(0).kind(){
                            func_trait_bounds_map.entry(GenericType::TraitType(Some(*param_ty), alias_ty.def_id))
                                                    .or_insert(FxHashSet::<DefId>::default())
                                                    .insert(trait_bound.def_id());
                        }
                    }
                }
                _ => eprintln!("Unsupported trait bound: {:?} of kind {:?} in {:?}!", trait_bound, trait_bound.self_ty().kind(), item_def_id),
            }
        }
    }
}

pub fn surf_get_item_projections<'tcx>(
    tcx: TyCtxt<'tcx>,
    item_def_id: DefId,
    func_projections_map: &mut FxHashMap::<GenericType, GenericType>,
)
{
    let item_preds = tcx.predicates_of(item_def_id).predicates;
    for item_pred in item_preds{
        if let ClauseKind::Projection(proj_pred) = item_pred.0.kind().skip_binder(){
            // let proj_pred_types = proj_pred.projection_term.args.types();      
            if let Some(rhs_type) = proj_pred.term.as_type(){
                // println!("->> {:#?}", rhs_type);
                let rhs_type_opt = {
                    match rhs_type.kind(){
                        ty::Param(param_ty) => Some(GenericType::Param(*param_ty)),
                        ty::Alias(AliasTyKind::Projection, alias_ty) => {
                            match tcx.trait_of_item(alias_ty.def_id){
                                Some(_) => {
                                    match alias_ty.args.type_at(0).kind(){
                                        ty::Param(param_ty) => Some(GenericType::TraitType(Some(*param_ty), alias_ty.def_id)),
                                        _ => None,
                                    }
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                };
                if let Some(rhs_type) = rhs_type_opt{
                    let lhs_alias_term = proj_pred.projection_term;
                    if let Some(_) = tcx.trait_of_item(lhs_alias_term.def_id){
                        if let ty::Param(param_ty) = lhs_alias_term.args.type_at(0).kind(){
                            func_projections_map.entry(GenericType::TraitType(Some(*param_ty), lhs_alias_term.def_id))
                                                    .or_insert(rhs_type.clone());
                        }
                    }
                }
            }
        }
    }
}

pub fn surf_get_implicit_bounds<'tcx>(
    tcx: TyCtxt<'tcx>,
    func_trait_bounds_map: &mut FxHashMap<GenericType, FxHashSet<DefId>>,
)
{
    let mut visited_traits = FxHashSet::<DefId>::default();
    let existing_func_bounds = func_trait_bounds_map.clone();
    for traits in existing_func_bounds.values(){
        for trait_def_id in traits{
            surf_get_trait_types_bounds(tcx, *trait_def_id, &mut visited_traits, func_trait_bounds_map);
        }
    }
}

pub fn surf_get_implicit_projections<'tcx>(
    tcx: TyCtxt<'tcx>,
    func_trait_bounds_map: &FxHashMap<GenericType, FxHashSet<DefId>>,
    func_projections_map: &mut FxHashMap<GenericType, GenericType>,
)
{
    let mut visited_traits = FxHashSet::<DefId>::default();
    for traits in func_trait_bounds_map.values(){
        for trait_def_id in traits{
            surf_get_trait_implicit_projections(tcx, *trait_def_id, &mut visited_traits, func_projections_map);
        }
    }
}

pub fn surf_get_trait_implicit_projections<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_def_id: DefId,
    visited_traits: &mut FxHashSet<DefId>,
    func_projections_map: &mut FxHashMap<GenericType, GenericType>,
)
{
    for trait_def_id in tcx.super_traits_of(trait_def_id){
        if !visited_traits.contains(&trait_def_id){
            visited_traits.insert(trait_def_id);
            let trait_items = tcx.associated_items(trait_def_id);
            for item in trait_items.in_definition_order(){
                let item_def_id = item.def_id;
                if assoc_item_is_type(item){
                    for item_pred in tcx.item_bounds(item_def_id).skip_binder().iter(){
                        match item_pred.kind().skip_binder(){
                            ClauseKind::Trait(trait_pred) => {
                                surf_get_trait_implicit_projections(tcx, trait_pred.def_id(), visited_traits, func_projections_map);
                            },
                            ClauseKind::Projection(proj_pred) => {
                                if let Some(rhs_type) = proj_pred.term.as_type(){
                                    let rhs_type_opt = {
                                        match rhs_type.kind(){
                                            ty::Param(param_ty) => Some(GenericType::Param(*param_ty)),
                                            ty::Alias(AliasTyKind::Projection, alias_ty) => {
                                                match tcx.trait_of_item(alias_ty.def_id){
                                                    Some(_) => {
                                                        match alias_ty.args.type_at(0).kind(){
                                                            ty::Param(_) => Some(GenericType::TraitType(None, alias_ty.def_id)),
                                                            _ => None,
                                                        }
                                                    }
                                                    _ => None,
                                                }
                                            }
                                            _ => None,
                                        }
                                    };
                                    if let Some(rhs_type) = rhs_type_opt{
                                        let lhs_alias_term = proj_pred.projection_term;
                                        if let Some(_) = tcx.trait_of_item(lhs_alias_term.def_id){
                                            match lhs_alias_term.args.type_at(0).kind(){
                                                ty::Param(_) => {
                                                    func_projections_map.entry(GenericType::TraitType(None, lhs_alias_term.def_id))
                                                                                    .or_insert(rhs_type.clone());
                                                },
                                                ty::Alias(AliasTyKind::Projection, alias_ty) => {
                                                    match tcx.trait_of_item(alias_ty.def_id){
                                                        Some(_) => {
                                                            if let ty::Param(_) = alias_ty.args.type_at(0).kind(){ // If you want to do crazier stuff check if this is Self, etc.
                                                                func_projections_map.entry(GenericType::TraitType(None, lhs_alias_term.def_id))
                                                                                    .or_insert(rhs_type.clone());
                                                            }
                                                        }
                                                        _ => {},
                                                    }
                                                }
                                                _ => {},
                                            }
                                        }
                                    }
                                }
                            },
                            _ => {},
                        }
                    }
                }
            }
        }
    }
}

pub fn surf_get_trait_types_bounds<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_def_id: DefId,
    visited_traits: &mut FxHashSet<DefId>,
    func_trait_bounds_map: &mut FxHashMap<GenericType, FxHashSet<DefId>>,
)
{
    for trait_def_id in tcx.super_traits_of(trait_def_id){
        if !visited_traits.contains(&trait_def_id){
            visited_traits.insert(trait_def_id);
            let trait_items = tcx.associated_items(trait_def_id);
            for item in trait_items.in_definition_order(){
                let item_def_id = item.def_id;
                if assoc_item_is_type(item){
                    for item_pred in tcx.item_bounds(item_def_id).skip_binder().iter(){
                        match item_pred.kind().skip_binder(){
                            ClauseKind::Trait(trait_pred) => {
                                let generic_type = GenericType::TraitType(None, item_def_id);
                                func_trait_bounds_map.entry(generic_type).or_insert(FxHashSet::<DefId>::default()).insert(trait_pred.def_id());
                                surf_get_trait_types_bounds(tcx, trait_pred.def_id(), visited_traits, func_trait_bounds_map);
                            },
                            _ => {},
                        }
                    }
                }
            }
        }
    }
}

pub fn surf_get_item_closures<'tcx>(
    tcx: TyCtxt<'tcx>,
    item_def_id: DefId,
    func_trait_bounds_map: &mut FxHashMap::<GenericType, FxHashSet<DefId>>,
    func_closures_map: &mut FxHashMap<ParamTy, SurfClosureSig<'tcx>>,
){
    let item_preds = tcx.predicates_of(item_def_id).predicates;
    for item_pred in item_preds{
        if let ClauseKind::Trait(trait_bound) = item_pred.0.kind().skip_binder(){
            if let ty::Param(param_type) = trait_bound.self_ty().kind(){
                let closure_kind_opt = tcx.fn_trait_kind_from_def_id(trait_bound.def_id());
                    let mut is_mutable = false;
                    if let Some(closure_kind) = closure_kind_opt{
                        if let ty::ClosureKind::FnMut = closure_kind{
                            is_mutable = true;
                        }
                        func_closures_map.insert(*param_type, SurfClosureSig::new(is_mutable));
                        func_trait_bounds_map.entry(GenericType::Param(*param_type))
                                            .or_insert(FxHashSet::<DefId>::default())
                                            .insert(trait_bound.def_id());
                    }
            }
        }
    }
    for item_pred in item_preds{
        if let ClauseKind::Projection(proj_pred) = item_pred.0.kind().skip_binder(){
            let proj_pred_types = proj_pred.projection_term.args.types();
            if proj_pred_types.count() > 1{
                if let ty::Param(closure_param_ty) = proj_pred.projection_term.args.type_at(0).kind(){
                    if let Some(closure_sig) = func_closures_map.get_mut(closure_param_ty){
                        if let ty::Tuple(closure_inputs) = proj_pred.projection_term.args.type_at(1).kind(){
                            closure_sig.inputs = closure_inputs.to_vec();
                        }
                        closure_sig.output = proj_pred.term.as_type();
                    }
                }
            }
        }
    }
}

pub fn surf_analyze_fn_args<'tcx>(tcx: TyCtxt<'tcx>, func_def_id: DefId) -> (Vec<SurfFnArg>, Option<SurfFnArg>){
    // Collect the Trait Bounds for all the Generic Parameters of the Function (PARAM_ID -> BOUNDS{..}) 
    let (
        func_trait_bounds_map,
        func_projections_map,
        func_closures_map) = surf_collect_bounds_projections_closures(tcx, func_def_id);
    
    let param_defaults = surf_collect_param_defaults(tcx, func_def_id);
    let fn_inputs = tcx.fn_sig(func_def_id).skip_binder().skip_binder().inputs();
    let mut surf_fn_inputs:Vec<SurfFnArg> = Vec::<SurfFnArg>::new();
    
    //println!("FUNC: {:?}, {:#?}", func_def_id, param_trait_bounds);

    for fn_input in fn_inputs{
        let analyzed_input = surf_analyze_fn_arg(
                                                                tcx,
                                                                *fn_input,
                                                                func_def_id,
                                                                &param_defaults,
                                                                &func_trait_bounds_map,
                                                                &func_projections_map,
                                                                &func_closures_map,
                                                            );
        surf_fn_inputs.push(analyzed_input);
    }

    let fn_output = tcx.fn_sig(func_def_id).skip_binder().skip_binder().output();
    let surf_fn_output = {
        match surf_has_output(fn_output){
            true => {   let analyzed_output = surf_analyze_fn_arg(
                                                                                tcx,
                                                                                fn_output,
                                                                                func_def_id,
                                                                                &param_defaults,
                                                                                &func_trait_bounds_map,
                                                                                &func_projections_map,
                                                                                &func_closures_map
                                                                            );
                        Some(analyzed_output)
                    },
            false => None,
        }
    };
    (surf_fn_inputs, surf_fn_output)
}

pub fn surf_analyze_fn_arg<'tcx>(
    tcx: TyCtxt<'tcx>,
    input_type: Ty<'tcx>,
    func_def_id: DefId,
    param_defaults: &FxHashMap<ParamTy, ParamTy>,
    func_trait_bounds_map: &FxHashMap<GenericType, FxHashSet<DefId>>,
    func_projections_map: &FxHashMap<GenericType, GenericType>,
    func_closures_map: &FxHashMap<ParamTy, SurfClosureSig<'tcx>>,
) -> SurfFnArg {
    let mut complex_types_to_analyse = FxHashSet::<DefId>::default();
    let mut trait_fns_to_analyse = FxHashSet::<DefId>::default();
    let analyzed_arg_type = {
        match input_type.kind(){
            ty::Adt(adt_def, adt_args) => {
                let adt_def_id = adt_def.did();                
                if surf_is_vec(tcx, adt_def_id){
                    if adt_args.types().count() > 0 {
                        let vec_arg_type = adt_args.types().next().unwrap();
                        SurfFnArg::Vector(Box::new(surf_analyze_fn_arg(
                                                                        tcx,
                                                                        vec_arg_type,
                                                                        func_def_id,
                                                                        param_defaults,
                                                                        func_trait_bounds_map,
                                                                        func_projections_map,
                                                                        func_closures_map
                        )))
                    }
                    else{
                        SurfFnArg::Vector(Box::new(SurfFnArg::Todo(String::from("Vector of multiple types!"))))
                    }
                }
                else if adt_def.is_box(){
                    if adt_args.types().count() > 0 {
                        
                        let box_arg_type = adt_args.types().next().unwrap();
                        SurfFnArg::Box(Box::new(surf_analyze_fn_arg(
                                                                        tcx,
                                                                        box_arg_type,
                                                                        func_def_id,
                                                                        param_defaults,
                                                                        func_trait_bounds_map,
                                                                        func_projections_map,
                                                                        func_closures_map
                        )))
                    }
                    else{
                        SurfFnArg::Box(Box::new(SurfFnArg::Todo(String::from("Box of multiple types!"))))
                    }
                }
                else if surf_is_maybeuinit(tcx, adt_def_id){
                    if adt_args.types().count() > 0 {
                        let uinit_arg_type = adt_args.types().next().unwrap();
                        SurfFnArg::Uinit(Box::new(surf_analyze_fn_arg(
                                                                        tcx,
                                                                        uinit_arg_type,
                                                                        func_def_id,
                                                                        param_defaults,
                                                                        func_trait_bounds_map,
                                                                        func_projections_map,
                                                                        func_closures_map
                        )))
                    }
                    else{
                        SurfFnArg::Uinit(Box::new(SurfFnArg::Todo(String::from("Uinit of unsupported type!"))))
                    }
                }
                else if surf_is_option(tcx, adt_def_id){
                    if adt_args.types().count()>0 {
                        let opt_arg_type = adt_args.types().next().unwrap();
                        SurfFnArg::Option(Box::new(surf_analyze_fn_arg(
                                                                        tcx,
                                                                        opt_arg_type,
                                                                        func_def_id,
                                                                        param_defaults,
                                                                        func_trait_bounds_map,
                                                                        func_projections_map,
                                                                        func_closures_map
                        )))
                    }
                    else{
                        SurfFnArg::Option(Box::new(SurfFnArg::Todo(String::from("Option of no types!"))))
                    }
                }
                else if surf_is_result(tcx, adt_def_id){
                    if adt_args.types().count()>0 {
                        //HERE
                        let res_arg_type = adt_args.types().next().unwrap();
                        let error_type = adt_args.types().skip(1).next().unwrap();
                        SurfFnArg::Result{
                                            ok: Box::new(surf_analyze_fn_arg(
                                                                        tcx,
                                                                        res_arg_type,
                                                                        func_def_id,
                                                                        param_defaults,
                                                                        func_trait_bounds_map,
                                                                        func_projections_map,
                                                                        func_closures_map
                                            )),
                                            err: match error_type.ty_def_id(){
                                                Some(error_type_def_id) => {
                                                    match error_type_def_id.is_local(){
                                                        true => surf_get_accessible_full_path(tcx, error_type_def_id),
                                                        false => surf_get_external_item_name(tcx, error_type_def_id)
                                                    }
                                                },
                                                None => {error_type.to_string()} 
                                            }
                        }
                    }
                    else{
                        SurfFnArg::Result{
                                            ok: Box::new(SurfFnArg::Todo(String::from("Result of no types!"))),
                                            err: String::from("")
                        }
                    }
                }
                else if surf_is_string(tcx, adt_def_id){
                    SurfFnArg::String
                }
                else if adt_def.is_enum(){
                    let enum_def_id = adt_def.did();
                    let mut variants = FxHashMap::<String, Vec<SurfFnArg>>::default();
                    if !SURF_ENUMS.lock().unwrap().contains_key(&enum_def_id){
                        SURF_ENUMS.lock().unwrap().insert(enum_def_id, FxHashMap::<String, Vec<SurfFnArg>>::default());
                        if item_is_imported(tcx, enum_def_id) || item_is_exported(tcx, enum_def_id){ // Variants can be accessed from userland only if the enum is public
                            for enum_variant in adt_def.variants(){
                                let variant_name = match item_is_imported(tcx, enum_variant.def_id){
                                    true => surf_get_external_item_name(tcx, enum_variant.def_id),
                                    false => format!("{}::{}", surf_get_accessible_full_path(tcx, enum_def_id), enum_variant.name.to_ident_string())
                                };
                                variants.insert(variant_name.clone(), Vec::<SurfFnArg>::new());
                                for variant_field in enum_variant.fields.iter(){                            
                                    variants.get_mut(&variant_name).unwrap().push(surf_analyze_fn_arg(
                                                                                                        tcx,
                                                                                                        variant_field.ty(tcx,adt_args),
                                                                                                        func_def_id,
                                                                                                        param_defaults,
                                                                                                        func_trait_bounds_map,
                                                                                                        func_projections_map,
                                                                                                        func_closures_map
                                    ));
                                }
                            }
                        }
                        SURF_ENUMS.lock().unwrap().insert(enum_def_id, variants);
                    }
                    if !SURF_COMPLEX_TYPES_TO_CONSTRUCTORS.lock().unwrap().contains_key(&adt_def_id){
                        if !SURF_COMPLEX_PENDING_ANALYSIS.lock().unwrap().contains(&adt_def_id){
                            SURF_COMPLEX_PENDING_ANALYSIS.lock().unwrap().insert(adt_def_id);
                            complex_types_to_analyse.insert(adt_def_id);
                        }
                    }
                    SurfFnArg::Enum{  
                                        def_id: enum_def_id,
                                        name: surf_get_accessible_full_path(tcx, enum_def_id),
                                        full_name: match enum_def_id.is_local(){
                                            true => surf_get_accessible_full_path(tcx, enum_def_id),
                                            false => surf_get_external_item_name(tcx, enum_def_id)
                                        },
                                        is_consumable: SURF_CONSUMABLE_CMPLX.lock().unwrap().contains(&enum_def_id),
                    }
                }
                else if adt_def.is_struct(){ // We don't analyze struct tuples for now
                    if !SURF_COMPLEX_TYPES_TO_CONSTRUCTORS.lock().unwrap().contains_key(&adt_def_id){
                        if !SURF_COMPLEX_PENDING_ANALYSIS.lock().unwrap().contains(&adt_def_id){
                            SURF_COMPLEX_PENDING_ANALYSIS.lock().unwrap().insert(adt_def_id);
                            complex_types_to_analyse.insert(adt_def_id);
                        }
                    }
                    SurfFnArg::Struct{
                                        def_id: adt_def_id,
                                        name: surf_get_accessible_full_path(tcx, adt_def_id),
                                        full_name: match adt_def_id.is_local(){
                                            true => surf_get_accessible_full_path(tcx, adt_def_id),
                                            false => surf_get_external_item_name(tcx, adt_def_id)
                                        },
                                        is_consumable: SURF_CONSUMABLE_CMPLX.lock().unwrap().contains(&adt_def_id),
                    }
                }
                else{
                    SurfFnArg::Todo(String::from(format!("Unsupported type: {:?}", adt_def.did())))
                }
            },
            ty::Array(inner_type, len) => {
                if let Some(count) = len.try_eval_target_usize(tcx, tcx.param_env_reveal_all_normalized(func_def_id)) {
                    SurfFnArg::Array(Box::new(surf_analyze_fn_arg(
                                                                    tcx,
                                                                    *inner_type,
                                                                    func_def_id,
                                                                    param_defaults,
                                                                    func_trait_bounds_map,
                                                                    func_projections_map,
                                                                    func_closures_map)),
                                                                    String::from(format!("{count}")))
                }
                else{
                    SurfFnArg::Todo(String::from("Unsupported Array length."))
                }
                            
            },
            ty::Slice(inner_type) => SurfFnArg::Slice(Box::new(surf_analyze_fn_arg(
                                                                            tcx,
                                                                            *inner_type,
                                                                            func_def_id,
                                                                            param_defaults,
                                                                            func_trait_bounds_map,
                                                                            func_projections_map,
                                                                            func_closures_map))
            ),
            ty::Tuple(tuple_args) => {
                let mut analyzed_tuple_args = Vec::<Box<SurfFnArg>>::new();
                for tuple_arg in tuple_args.iter(){
                    analyzed_tuple_args.push(Box::new(surf_analyze_fn_arg(
                                                                            tcx,
                                                                            tuple_arg,
                                                                            func_def_id,
                                                                            param_defaults,
                                                                            func_trait_bounds_map,
                                                                            func_projections_map,
                                                                            func_closures_map)
                    ));
                }
                SurfFnArg::Tuple(analyzed_tuple_args)
            },
            ty::Ref(_, value_type, mutability) => SurfFnArg::Reference(Box::new(surf_analyze_fn_arg(
                                                                                tcx,
                                                                                *value_type,
                                                                                func_def_id,
                                                                                param_defaults,
                                                                                func_trait_bounds_map,
                                                                                func_projections_map,
                                                                                func_closures_map)),
                                                                                mutability.is_mut()
            ),
            ty::RawPtr(value_type, mutability) => SurfFnArg::RawPointer(Box::new(surf_analyze_fn_arg(
                                                                                tcx,
                                                                                *value_type,
                                                                                func_def_id,
                                                                                param_defaults,
                                                                                func_trait_bounds_map,
                                                                                func_projections_map,
                                                                                func_closures_map)),
                                                                                mutability.is_mut()
            ),
            ty::Param(param_ty) => {
                if let Some(closure_sig) = func_closures_map.get(param_ty){
                    let mut surf_fn_closure_inputs = Vec::<Box<SurfFnArg>>::new();
                    let mut surf_fn_closure_output: Option<Box<SurfFnArg>> = None;
                    for input in closure_sig.inputs.iter(){
                        surf_fn_closure_inputs.push(Box::new(surf_analyze_fn_arg(
                                                                                    tcx,
                                                                                    *input,
                                                                                    func_def_id,
                                                                                    param_defaults,
                                                                                    func_trait_bounds_map,
                                                                                    func_projections_map,
                                                                                    func_closures_map)
                        ));
                    }
                    if let Some(output) = closure_sig.output{
                        surf_fn_closure_output = Some(Box::new(surf_analyze_fn_arg(
                                                                                    tcx,
                                                                                    output,
                                                                                    func_def_id,
                                                                                    param_defaults,
                                                                                    func_trait_bounds_map,
                                                                                    func_projections_map,
                                                                                    func_closures_map)
                        ));
                    }
                    SurfFnArg::Closure {
                                            param_name: String::from(param_ty.name.as_str()),
                                            inputs: surf_fn_closure_inputs,
                                            output: surf_fn_closure_output,
                                            is_mutable: closure_sig.is_mutable
                                        }
                }
                else{
                    //println!("FUNC: {:?}, PARAM_TYPE: {:?}, DEFAULTS: {:?}", func_def_id, param_ty, param_defaults);
                    let param_ty = match param_defaults.get(param_ty){
                        Some(default_type) => default_type,
                        _ => param_ty,
                    };

                    let surf_urapi_opt = match SURF_APIS_REACH_UNSAFE.lock().unwrap().get(&func_def_id){
                        Some(surf_urapi) => Some(surf_urapi.clone()),
                        _ => None,
                    };

                    let mut additional_trait_fns_to_analyze = FxHashSet::<DefId>::default();
                    let mut seen_trait_types = FxHashSet::<GenericType>::default();
                    let arg_required_bounds = surf_get_required_bounds(
                                                                                                                        tcx,
                                                                                                                        &surf_urapi_opt,
                                                                                                                        func_def_id,
                                                                                                                        &GenericType::Param(*param_ty),
                                                                                                                        &param_ty.name.to_ident_string(),
                                                                                                                        func_trait_bounds_map,
                                                                                                                        func_projections_map,
                                                                                                                        &mut seen_trait_types,
                                                                                                                        &mut additional_trait_fns_to_analyze
                                                                                                                    );
                    for trait_fn_def_id in additional_trait_fns_to_analyze.iter(){
                        if !SURF_TRAIT_FNS.lock().unwrap().contains_key(trait_fn_def_id){
                            trait_fns_to_analyse.insert(*trait_fn_def_id);
                        }
                    }
                    SurfFnArg::Generic{
                                        name: String::from(param_ty.name.as_str()),
                                        traits: arg_required_bounds,
                    }
                }
            },
            ty::Alias(AliasTyKind::Projection, alias_ty) => {
                if let Some(trait_def_id) = tcx.trait_of_item(alias_ty.def_id){
                    if let Some(placeholder_name) = surf_get_placeholder_if_assoc_type(tcx, trait_def_id, *alias_ty){
                        //println!("place holder: {:?} kind: {:?} -> def_id: {:?}", placeholder_name, alias_ty.args.type_at(0).kind(), alias_ty.def_id);
                        if let ty::Param(param_ty) = alias_ty.args.type_at(0).kind(){
                            let surf_urapi_opt = match SURF_APIS_REACH_UNSAFE.lock().unwrap().get(&func_def_id){
                                Some(surf_urapi) => Some(surf_urapi.clone()),
                                _ => None,
                            };
                            
                            let mut additional_trait_fns_to_analyze = FxHashSet::<DefId>::default();
                            let mut seen_trait_types = FxHashSet::<GenericType>::default();
                            let arg_required_bounds = surf_get_required_bounds(
                                                                                                                    tcx,
                                                                                                                    &surf_urapi_opt,
                                                                                                                    func_def_id,
                                                                                                                    &GenericType::TraitType(Some(*param_ty), alias_ty.def_id),
                                                                                                                    &param_ty.name.to_ident_string(),
                                                                                                                    func_trait_bounds_map,
                                                                                                                    func_projections_map,
                                                                                                                    &mut seen_trait_types,
                                                                                                                    &mut additional_trait_fns_to_analyze
                                                                                                                );
                            // Analysis later
                            for trait_fn_def_id in additional_trait_fns_to_analyze.iter(){
                                if !SURF_TRAIT_FNS.lock().unwrap().contains_key(trait_fn_def_id){
                                    trait_fns_to_analyse.insert(*trait_fn_def_id);
                                }
                            }

                            let current_type_id = format!("{}::{:?}", param_ty.name.to_ident_string(), alias_ty.def_id);
                            SurfFnArg::AssocType {
                                                    def_id: alias_ty.def_id,
                                                    trait_def_id: trait_def_id,
                                                    placeholder_name: placeholder_name.clone(),
                                                    assoc_type_id: current_type_id.clone(),
                                                    traits: arg_required_bounds.clone()
                                                }
                        }
                        else{
                            SurfFnArg::Todo(String::from("Unsupported Associated Type!"))
                        }
                    }
                    else{
                        SurfFnArg::Todo(String::from("Unsupported Trait Item!"))
                    }
                }
                else if alias_ty.args.types().count() > 0{
                    surf_analyze_fn_arg(
                        tcx,
                        alias_ty.args.type_at(0),
                        func_def_id,
                        param_defaults,
                        func_trait_bounds_map,
                        func_projections_map,
                        func_closures_map,
                    )
                }
                else{
                    SurfFnArg::Todo(String::from("Unsupported Alias!"))
                }
            },
            ty::Dynamic(predicates, _, dyn_kind) => {
                if let Dyn = dyn_kind{
                    let mut dyn_cmplx_ty_candidates = FxHashSet::<DefId>::default();
                    for predicate in predicates.iter(){
                        if let ExistentialPredicate::Trait(trait_ref) = predicate.skip_binder(){
                            if let Some(cmplx_tys) = SURF_TRAITS_TO_CMPLX.lock().unwrap().get(&trait_ref.def_id){
                                dyn_cmplx_ty_candidates = {
                                    match dyn_cmplx_ty_candidates.is_empty(){
                                        true => cmplx_tys.clone(),
                                        false => dyn_cmplx_ty_candidates.intersection(cmplx_tys).cloned().collect(),
                                    }
                                }; 
                            }
                        }
                    }
                    let mut dyn_cmplx_tys = Vec::<Box<SurfFnArg>>::default();
                    for cmplx_ty in dyn_cmplx_ty_candidates.iter(){
                        if !SURF_COMPLEX_TYPES_TO_CONSTRUCTORS.lock().unwrap().contains_key(&cmplx_ty){
                            if !SURF_COMPLEX_PENDING_ANALYSIS.lock().unwrap().contains(&cmplx_ty){
                                SURF_COMPLEX_PENDING_ANALYSIS.lock().unwrap().insert(*cmplx_ty);
                                complex_types_to_analyse.insert(*cmplx_ty);
                            }
                        }
                        dyn_cmplx_tys.push(Box::new(surf_analyze_fn_arg(
                                                                tcx,
                                                                tcx.type_of(*cmplx_ty).skip_binder(),
                                                                func_def_id,
                                                                param_defaults,
                                                                func_trait_bounds_map,
                                                                func_projections_map,
                                                                func_closures_map,
                                                            )

                        ));
                    }
                    SurfFnArg::DynTrait(dyn_cmplx_tys)
                }
                else{
                    SurfFnArg::Todo(String::from("Unsupported Dynamic Trait!"))
                }
            },
            ty::Bool => SurfFnArg::Primitive(String::from("bool")),
            ty::Char => SurfFnArg::Primitive(String::from("char")),
            ty::Int(int_type) => SurfFnArg::Primitive(String::from(int_type.name_str())),
            ty::Uint(uint_type) => SurfFnArg::Primitive(String::from(uint_type.name_str())),
            ty::Float(float_type) => SurfFnArg::Primitive(String::from(float_type.name_str())),
            ty::Str => SurfFnArg::Primitive(String::from("str")),
            _ => SurfFnArg::Todo(String::from(format!("Ref/Value to/of {:?}", input_type))),
        }
    };

    //Do the recursive arguments analysis here!!!
    let mut complex_types_to_analyse: Vec<DefId> = complex_types_to_analyse.iter().cloned().collect();
    let mut trait_fns_to_analyse: Vec<DefId> = trait_fns_to_analyse.iter().cloned().collect();
    while !complex_types_to_analyse.is_empty() || !trait_fns_to_analyse.is_empty(){
        if let Some(complex_type_def_id) = complex_types_to_analyse.pop(){
            SURF_COMPLEX_TYPES_TO_CONSTRUCTORS.lock().unwrap().insert(complex_type_def_id, FxHashSet::<DefId>::default());
            SURF_COMPLEX_PENDING_ANALYSIS.lock().unwrap().remove(&complex_type_def_id);
            let complex_type_constructors = surf_get_constructors(tcx, complex_type_def_id);
            for constructor in complex_type_constructors{

                // Do not touch the following lines (the order matters).
                let (constructor_name, constructor_full_name) = surf_get_fn_names(tcx, constructor);                

                SURF_CONSTRUCTORS.lock().unwrap().insert(constructor, SurfConstructorData::new(
                                                                                                        constructor_name.clone(),
                                                                                                        constructor_full_name,
                                                                                                        get_stable_def_id_location(tcx, constructor),
                ));
                
                let (constructor_required_implicit_generics,
                    additional_complex_types_to_analyze,
                    additional_trait_fns_to_analyze) = surf_get_constructor_implicit_generics(
                                                                                                                                tcx,
                                                                                                                                constructor,
                                                                                                                                complex_type_def_id,
                                                                                                                                param_defaults,
                                                                                                                                func_trait_bounds_map,
                                                                                                                                func_projections_map,
                                                                                                                                func_closures_map,
                                                                                                                            );
                for cmplx_type_def_id in additional_complex_types_to_analyze{
                    if !complex_types_to_analyse.contains(&cmplx_type_def_id){
                        complex_types_to_analyse.push(cmplx_type_def_id);
                    }
                }
                for trait_fn_def_id in additional_trait_fns_to_analyze{
                    if !trait_fns_to_analyse.contains(&trait_fn_def_id){
                        trait_fns_to_analyse.push(trait_fn_def_id);
                    }
                }
                
                let (constructor_inputs, constructor_output) = surf_analyze_fn_args(tcx, constructor);
                SURF_CONSTRUCTORS.lock().unwrap().get_mut(&constructor).unwrap().inputs = constructor_inputs;
                SURF_CONSTRUCTORS.lock().unwrap().get_mut(&constructor).unwrap().output = constructor_output;
                SURF_CONSTRUCTORS.lock().unwrap().get_mut(&constructor).unwrap().implicit_generics = constructor_required_implicit_generics;
                SURF_COMPLEX_TYPES_TO_CONSTRUCTORS.lock().unwrap().get_mut(&complex_type_def_id).unwrap().insert(constructor);                
            }
        }
        if let Some(trait_fn_def_id) = trait_fns_to_analyse.pop(){
            let (trait_fn_name, trait_fn_full_name) = surf_get_fn_names(tcx, trait_fn_def_id);
            let trait_fn_span_str = surf_get_span_str(tcx, trait_fn_def_id);
            let is_trait_fn_unsafe = tcx.fn_sig(trait_fn_def_id).skip_binder().skip_binder().safety == Safety::Unsafe;
            SURF_TRAIT_FNS.lock().unwrap().insert(trait_fn_def_id, SurfTraitFnData::new(trait_fn_name, trait_fn_full_name, trait_fn_span_str, is_trait_fn_unsafe));
            let (trait_fn_inputs, trait_fn_output) = surf_analyze_fn_args(tcx, trait_fn_def_id);
            SURF_TRAIT_FNS.lock().unwrap().get_mut(&trait_fn_def_id).unwrap().inputs = trait_fn_inputs;
            SURF_TRAIT_FNS.lock().unwrap().get_mut(&trait_fn_def_id).unwrap().output = trait_fn_output;
        }
    }
    analyzed_arg_type
}

pub fn surf_get_placeholder_if_assoc_type<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_def_id: DefId,
    alias_ty: AliasTy<'tcx>,
) -> Option<String>
{
    for item in tcx.associated_items(trait_def_id).in_definition_order(){
        if assoc_item_is_type(item){
            if item.def_id == alias_ty.def_id{
                return Some(item.name.to_ident_string());
            }
        }
    }
    None
}

// Returns the complex candidates for the generic type as well as complex types to analyze
pub fn surf_get_cmplx_candidates<'tcx>(
    tcx: TyCtxt<'tcx>,
    func_def_id: DefId,
    arg_required_bounds: &FxHashMap<DefId, Box<SurfFnArg>>, 
    param_defaults: &FxHashMap<ParamTy, ParamTy>,
    func_trait_bounds_map: &FxHashMap<GenericType, FxHashSet<DefId>>,
    func_projections_map: &FxHashMap<GenericType, GenericType>,
    func_closures_map: &FxHashMap<ParamTy, SurfClosureSig<'tcx>>,
) -> (Vec<Box<SurfFnArg>>, FxHashSet<DefId>)
{
    // Collect Complex Candidates
    let mut cmplx_candidates = FxHashSet::<DefId>::default();
    for trait_bound in arg_required_bounds.keys(){
        if let Some(cmplx_tys) = SURF_TRAITS_TO_CMPLX.lock().unwrap().get(trait_bound){
            cmplx_candidates = {
                match cmplx_candidates.is_empty(){
                    true => cmplx_tys.clone(),
                    false => cmplx_candidates.intersection(cmplx_tys).cloned().collect(),
                }
            };
        }
    }

    // Collect types whose constructors need to be analyzed (since they haven't yet)
    let mut complex_types_to_analyse = FxHashSet::<DefId>::default();
    for cmplx_def_id in cmplx_candidates.iter(){
        if !SURF_COMPLEX_TYPES_TO_CONSTRUCTORS.lock().unwrap().contains_key(&cmplx_def_id){
            if !SURF_COMPLEX_PENDING_ANALYSIS.lock().unwrap().contains(&cmplx_def_id){
                SURF_COMPLEX_PENDING_ANALYSIS.lock().unwrap().insert(*cmplx_def_id);
                complex_types_to_analyse.insert(*cmplx_def_id);
            }
        }                          
    }

    // Analyze the complex types (converting DefIds to SurfFnArgs)
    let mut cmplx_candidates_analyzed = Vec::<Box<SurfFnArg>>::default();
    for cmplx_def_id in cmplx_candidates.iter(){
        cmplx_candidates_analyzed.push(Box::new(surf_analyze_fn_arg(
                                                                        tcx,
                                                                        tcx.type_of(*cmplx_def_id).skip_binder(),
                                                                        func_def_id,
                                                                        param_defaults,
                                                                        func_trait_bounds_map,
                                                                        func_projections_map,
                                                                        func_closures_map
                                                                    )));           
    }

    (cmplx_candidates_analyzed, complex_types_to_analyse)
}

// Returns analyzed primitive candidates for the generic type
pub fn surf_get_prim_candidates<'tcx>(
    tcx: TyCtxt<'tcx>,
    func_def_id: DefId,
    arg_required_bounds: &FxHashMap<DefId, Box<SurfFnArg>>,
    param_defaults: &FxHashMap<ParamTy, ParamTy>,
    func_trait_bounds_map: &FxHashMap<GenericType, FxHashSet<DefId>>,
    func_projections_map: &FxHashMap<GenericType, GenericType>,
    func_closures_map: &FxHashMap<ParamTy, SurfClosureSig<'tcx>>,
) -> Vec<Box<SurfFnArg>>
{
    // Collect Primitive Candidates
    let mut prim_candidates = FxHashSet::<PrimTy>::default();
    for trait_bound in arg_required_bounds.keys(){
        if let Some(prim_tys) = SURF_TRAITS_TO_PRIM.lock().unwrap().get(trait_bound){
            prim_candidates = {
                match prim_candidates.is_empty(){
                    true => prim_tys.clone(),
                    false => prim_candidates.intersection(prim_tys).cloned().collect(),
                }
            };
        }
    }

    // Analyze Primitive Candidates
    let mut prim_candidates_analyzed = Vec::<Box<SurfFnArg>>::default();
    for prim_hir_ty in prim_candidates.iter(){
        let prim_ty = {
            match *prim_hir_ty {
                PrimTy::Bool => tcx.types.bool,
                PrimTy::Char => tcx.types.char,
                PrimTy::Int(it) => Ty::new_int(tcx, ty::int_ty(it)),
                PrimTy::Uint(uit) => Ty::new_uint(tcx, ty::uint_ty(uit)),
                PrimTy::Float(ft) => Ty::new_float(tcx, ty::float_ty(ft)),
                PrimTy::Str => tcx.types.str_,
            }
        };
        prim_candidates_analyzed.push(Box::new(surf_analyze_fn_arg(
                                                                    tcx,
                                                                    prim_ty,
                                                                    func_def_id,
                                                                    param_defaults,
                                                                    func_trait_bounds_map,
                                                                    func_projections_map,
                                                                    func_closures_map
                                                                )));
    }
    prim_candidates_analyzed
}

pub fn surf_get_item_id_from_type(arg_type: &GenericType) -> String{
    match arg_type{
        GenericType::Param(param_ty) => format!("{}", param_ty.name.to_ident_string()),
        GenericType::TraitType(trait_type_param_opt, trait_type_def_id) => format!("{}::{:?}", trait_type_param_opt.unwrap().name.to_ident_string(), trait_type_def_id),
    }
}

pub fn surf_get_arg_type_bounds(
    arg_type: &GenericType,
    func_trait_bounds_map: &FxHashMap<GenericType, FxHashSet<DefId>>,
    func_projections_map: &FxHashMap<GenericType, GenericType>,
) -> Option<FxHashSet<DefId>>
{
    let mut available_projections = func_projections_map.clone();
    let mut current_arg_type = arg_type.clone();
    loop{
        if func_trait_bounds_map.contains_key(&current_arg_type){
            //println!("Name1: {:?}", current_arg_type);
            return Some(func_trait_bounds_map.get(&current_arg_type).unwrap().clone())
        }
        else{
            if available_projections.contains_key(&current_arg_type){
                current_arg_type = available_projections.get(&current_arg_type).unwrap().clone();
                available_projections.remove(&current_arg_type);
            }
            else{
                if let GenericType::TraitType(current_arg_type_param_ty_opt, current_arg_type_def_id) = current_arg_type{
                    let bound_wildcard_arg_type = GenericType::TraitType(None, current_arg_type_def_id);
                    if func_trait_bounds_map.contains_key(&bound_wildcard_arg_type){
                        return Some(func_trait_bounds_map.get(&bound_wildcard_arg_type).unwrap().clone());
                    }
                    else{
                        let lhs_wildcard_arg_type = GenericType::TraitType(None, current_arg_type_def_id);
                        if available_projections.contains_key(&lhs_wildcard_arg_type){
                            let rhs_wildcard_arg_type = available_projections.get(&lhs_wildcard_arg_type).unwrap();
                            match rhs_wildcard_arg_type{
                                GenericType::Param(_) => current_arg_type = rhs_wildcard_arg_type.clone(),
                                GenericType::TraitType(_, rhs_def_id) => {
                                    current_arg_type = GenericType::TraitType(current_arg_type_param_ty_opt, *rhs_def_id);
                                },
                            }
                            available_projections.remove(&lhs_wildcard_arg_type);
                        }
                        else{
                            //println!("Name2: {:?}", current_arg_type);
                            return None;
                        }
                    }
                }
                else{
                    //println!("Name3: {:?}", current_arg_type);
                    return None;
                }
            }
        }
    }
}

pub fn surf_get_trait_type_concrete_type(
    arg_type: &GenericType,
    func_trait_bounds_map: &FxHashMap<GenericType, FxHashSet<DefId>>,
    func_projections_map: &FxHashMap<GenericType, GenericType>,
) -> Option<String>
{
    let mut return_val = None;
    //println!("Get the type of: {:?}", arg_type);
    let mut available_projections = func_projections_map.clone();
    let mut current_arg_type = arg_type.clone();
    loop{
        if func_trait_bounds_map.contains_key(&current_arg_type){
            return Some(surf_get_item_id_from_type(&current_arg_type));
        }
        else{
            if available_projections.contains_key(&current_arg_type){
                current_arg_type = available_projections.get(&current_arg_type).unwrap().clone();
                return_val = Some(surf_get_item_id_from_type(&current_arg_type));
                available_projections.remove(&current_arg_type);
            }
            else{
                if let GenericType::TraitType(current_arg_type_param_ty_opt, current_arg_type_def_id) = current_arg_type{
                    let lhs_wildcard_arg_type = GenericType::TraitType(None, current_arg_type_def_id);
                    if available_projections.contains_key(&lhs_wildcard_arg_type){
                        let rhs_wildcard_arg_type = available_projections.get(&lhs_wildcard_arg_type).unwrap();
                        match rhs_wildcard_arg_type{
                            GenericType::Param(_) => current_arg_type = rhs_wildcard_arg_type.clone(),
                            GenericType::TraitType(_, rhs_def_id) => {
                                current_arg_type = GenericType::TraitType(current_arg_type_param_ty_opt, *rhs_def_id);
                            },
                        }
                        available_projections.remove(&lhs_wildcard_arg_type);
                        //return_val = None;
                    }
                    else{
                        return return_val;
                    }
                }
                else{
                    return return_val;
                }
            }
        }
    }
}

pub fn surf_get_urapi_required_bounds<'tcx>(
    tcx: TyCtxt<'tcx>,
    surf_urapi_opt: &Option<SurfURAPI>,
    urapi_def_id: DefId,
    arg_type: &GenericType,
    current_type_id: &String,
    func_trait_bounds_map: &FxHashMap<GenericType, FxHashSet<DefId>>,
    func_projections_map: &FxHashMap<GenericType, GenericType>,
    seen_trait_types: &mut FxHashSet<GenericType>,
    trait_fns_to_analyze: &mut FxHashSet<DefId>,
) -> FxHashMap<DefId, Box<SurfFnArg>>
{
    let mut required_analyzed_traits = FxHashMap::<DefId, Box<SurfFnArg>>::default();
    //println!("TYPE TO FIND BOUNDS: {:?}", arg_type);
    // The type may be generic or trait type
    // First we need to find it predicates, either in the func_trait)bounds_map or by searching through the projections.
    let arg_trait_bounds_opt = surf_get_arg_type_bounds(arg_type, func_trait_bounds_map, func_projections_map);
    if let Some(arg_trait_bounds) = arg_trait_bounds_opt{
        //println!("TYPE BOUNDS: {:#?}", &arg_trait_bounds);
        for trait_bound_def_id in arg_trait_bounds.iter(){
            for required_trait_def_id in tcx.super_traits_of(*trait_bound_def_id){
                //let current_type_id = format!("{current_type_id}::{:?}", required_trait_def_id);
                if let Some(surf_urapi) = &surf_urapi_opt{
                    //println!("CURRENT TRAIT TO ANALYZE: {:?}", required_trait_def_id);
                    surf_add_trait_bound(required_trait_def_id, &mut required_analyzed_traits);  // create the trait item
                    surf_collect_trait_data(tcx, required_trait_def_id);
                    let trait_bound_items = tcx.associated_items(required_trait_def_id);
                    
                    // Iterate through the items of the required trait
                    for item in trait_bound_items.in_definition_order(){
                        //println!("ANALYZE TRAIT ITEMS: {:?}", required_trait_def_id);
                        let item_def_id = item.def_id;
                        
                        if assoc_item_is_fn(item){
                            if !fn_item_has_default_impl(tcx, item_def_id) || 
                                surf_has_unresolved_calls(surf_urapi, required_trait_def_id, item_def_id){
                                    //println!("TRAIT_FN_ANALYSIS: {:?}", item_def_id);
                                    surf_add_trait_bound_fn_item(
                                                                    required_trait_def_id,
                                                                    item_def_id,
                                                                    &mut required_analyzed_traits,
                                                                    trait_fns_to_analyze
                                                                );
                                    //println!("TRAIT_FN_ADDED: {:?}", item_def_id);
                            }
                        }
                        else if assoc_item_is_type(item){
                            let trait_item_type = {
                                match arg_type{
                                    GenericType::Param(param_ty) => GenericType::TraitType(Some(*param_ty), item_def_id),
                                    GenericType::TraitType(param_ty_opt, _) => GenericType::TraitType(*param_ty_opt, item_def_id),
                                }
                            };
                            if !seen_trait_types.contains(&trait_item_type){
                                seen_trait_types.insert(trait_item_type.clone());
                                surf_add_trait_bound_type_item(
                                                                tcx,
                                                                surf_urapi_opt,
                                                                urapi_def_id,
                                                                required_trait_def_id,
                                                                item_def_id,
                                                                item.name.to_ident_string(),
                                                                &current_type_id,
                                                                &trait_item_type,
                                                                &mut required_analyzed_traits,
                                                                func_trait_bounds_map,
                                                                func_projections_map,
                                                                seen_trait_types,
                                                                trait_fns_to_analyze,
                                );
                            }
                            //println!("TRAIT_TYPE_ADDED: {:?}", item_def_id);
                        }
                    }
                }
            }
        }
    }
    required_analyzed_traits
}

pub fn surf_get_required_bounds<'tcx>(
    tcx: TyCtxt<'tcx>,
    surf_urapi_opt: &Option<SurfURAPI>,
    urapi_def_id: DefId,
    arg_type: &GenericType,
    current_type_id: &String,
    func_trait_bounds_map: &FxHashMap<GenericType, FxHashSet<DefId>>,
    func_projections_map: &FxHashMap<GenericType, GenericType>,
    seen_trait_types: &mut FxHashSet<GenericType>,
    trait_fns_to_analyze: &mut FxHashSet<DefId>,
) -> FxHashMap<DefId, Box<SurfFnArg>>
{
    let mut required_analyzed_traits = FxHashMap::<DefId, Box<SurfFnArg>>::default();
    //println!("TYPE TO FIND BOUNDS: {:?}", arg_type);
    // The type may be generic or trait type
    // First we need to find it predicates, either in the func_trait)bounds_map or by searching through the projections.
    let arg_trait_bounds_opt = surf_get_arg_type_bounds(arg_type, func_trait_bounds_map, func_projections_map);
    if let Some(arg_trait_bounds) = arg_trait_bounds_opt{
        //println!("TYPE BOUNDS: {:#?}", &arg_trait_bounds);
        for trait_bound_def_id in arg_trait_bounds.iter(){
            for required_trait_def_id in tcx.super_traits_of(*trait_bound_def_id){
                //let current_type_id = format!("{current_type_id}::{:?}", required_trait_def_id);
                if let Some(surf_urapi) = &surf_urapi_opt{
                    //println!("CURRENT TRAIT TO ANALYZE: {:?}", required_trait_def_id);
                    surf_add_trait_bound(required_trait_def_id, &mut required_analyzed_traits);  // create the trait item
                    surf_collect_trait_data(tcx, required_trait_def_id);
                    let trait_bound_items = tcx.associated_items(required_trait_def_id);
                    
                    // Iterate through the items of the required trait
                    for item in trait_bound_items.in_definition_order(){
                        //println!("ANALYZE TRAIT ITEMS: {:?}", required_trait_def_id);
                        let item_def_id = item.def_id;
                        
                        if assoc_item_is_fn(item){
                            if !fn_item_has_default_impl(tcx, item_def_id) || 
                                surf_has_unresolved_calls(surf_urapi, required_trait_def_id, item_def_id){
                                    //println!("TRAIT_FN_ANALYSIS: {:?}", item_def_id);
                                    surf_add_trait_bound_fn_item(
                                                                    required_trait_def_id,
                                                                    item_def_id,
                                                                    &mut required_analyzed_traits,
                                                                    trait_fns_to_analyze
                                                                );
                                    //println!("TRAIT_FN_ADDED: {:?}", item_def_id);
                            }
                        }
                        else if assoc_item_is_type(item){
                            let trait_item_type = {
                                match arg_type{
                                    GenericType::Param(param_ty) => GenericType::TraitType(Some(*param_ty), item_def_id),
                                    GenericType::TraitType(param_ty_opt, _) => GenericType::TraitType(*param_ty_opt, item_def_id),
                                }
                            };
                            if !seen_trait_types.contains(&trait_item_type){
                                seen_trait_types.insert(trait_item_type.clone());
                                surf_add_trait_bound_type_item(
                                                                tcx,
                                                                surf_urapi_opt,
                                                                urapi_def_id,
                                                                required_trait_def_id,
                                                                item_def_id,
                                                                item.name.to_ident_string(),
                                                                &current_type_id,
                                                                &trait_item_type,
                                                                &mut required_analyzed_traits,
                                                                func_trait_bounds_map,
                                                                func_projections_map,
                                                                seen_trait_types,
                                                                trait_fns_to_analyze,
                                );
                            }
                            //println!("TRAIT_TYPE_ADDED: {:?}", item_def_id);
                        }
                    }
                }
                else if SURF_CONSTRUCTORS.lock().unwrap().contains_key(&urapi_def_id){
                    //println!("HERE: {:?}", urapi_def_id);
                    surf_add_trait_bound(required_trait_def_id, &mut required_analyzed_traits);
                    surf_collect_trait_data(tcx, required_trait_def_id);

                    let trait_bound_items = tcx.associated_items(required_trait_def_id);
                    for item in trait_bound_items.in_definition_order(){
                        let item_def_id = item.def_id;
                        if assoc_item_is_fn(item){
                            
                            if !fn_item_has_default_impl(tcx, item_def_id){
                                surf_add_trait_bound_fn_item( 
                                                                required_trait_def_id,
                                                                item_def_id,
                                                                &mut required_analyzed_traits,
                                                                trait_fns_to_analyze
                                                            );
                            }
                        }
                        else if assoc_item_is_type(item){
                            let trait_item_type = {
                                match arg_type{
                                    GenericType::Param(param_ty) => GenericType::TraitType(Some(*param_ty), item_def_id),
                                    GenericType::TraitType(param_ty_opt, _) => GenericType::TraitType(*param_ty_opt, item_def_id),
                                }
                            };
                            if !seen_trait_types.contains(arg_type){
                                seen_trait_types.insert(trait_item_type.clone());
                                surf_add_trait_bound_type_item(
                                                                tcx,
                                                                surf_urapi_opt,
                                                                urapi_def_id,
                                                                required_trait_def_id,
                                                                item_def_id,
                                                                item.name.to_ident_string(),
                                                                &current_type_id,
                                                                &trait_item_type,
                                                                &mut required_analyzed_traits,
                                                                func_trait_bounds_map,
                                                                func_projections_map,
                                                                seen_trait_types,
                                                                trait_fns_to_analyze,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    required_analyzed_traits
}

// Returns analyzed generic arguments as well as complex types and trait_fns to analyze
pub fn surf_get_constructor_implicit_generics<'tcx>(
    tcx: TyCtxt<'tcx>,
    constructor_def_id: DefId,
    complex_type_def_id: DefId,
    _param_defaults: &FxHashMap<ParamTy, ParamTy>,
    _func_trait_bounds_map: &FxHashMap<GenericType, FxHashSet<DefId>>,
    func_projections_map: &FxHashMap<GenericType, GenericType>,
    _func_closures_map: &FxHashMap<ParamTy, SurfClosureSig<'tcx>>,
) -> (Vec<SurfFnArg>, FxHashSet<DefId>, FxHashSet<DefId>)
{
    // Get the generic inputs of the current constructor
    let mut constructor_generic_inputs = FxHashSet::<ParamTy>::default();
    for arg in tcx.fn_sig(constructor_def_id).skip_binder().skip_binder().inputs(){
        surf_get_arg_generics(tcx, *arg, &mut constructor_generic_inputs);
    }

    // Get the generic types of complex type that cannot be inferred by the inputs of the current constructor
    let adt_generics = surf_get_adt_generic_params(tcx, complex_type_def_id);
    let mut adt_implicit_generics = Vec::<ParamTy>::new();
    for adt_generic in adt_generics{
        if !constructor_generic_inputs.contains(&adt_generic){
            adt_implicit_generics.push(adt_generic);
        }
    }
    
    // Get the predicates (trait bounds) of the complex type
    let mut adt_trait_bounds_map = FxHashMap::<GenericType, FxHashSet<DefId>>::default();
    surf_get_item_trait_bounds(tcx, complex_type_def_id, &mut adt_trait_bounds_map);
    
    // Analyze the required predicates (trait bounds) of the complex type that correspond to the implicit generics of the type
    let mut implicit_analyzed_generics = Vec::<SurfFnArg>::default();
    let additional_cmplx_types_to_analyze = FxHashSet::<DefId>::default();
    let mut additional_trait_fns_to_analyze = FxHashSet::<DefId>::default();
    for adt_implicit_generic in adt_implicit_generics.iter(){
        
        //let mut additional_trait_fns_to_analyze = FxHashSet::<DefId>::default();
        let mut seen_trait_types = FxHashSet::<GenericType>::default();
        let arg_required_bounds = surf_get_required_bounds(
                                                                                                tcx,
                                                                                                &None,
                                                                                                constructor_def_id,
                                                                                                &GenericType::Param(*adt_implicit_generic),
                                                                                                &adt_implicit_generic.name.to_ident_string(),
                                                                                                &adt_trait_bounds_map,
                                                                                                func_projections_map,
                                                                                                &mut seen_trait_types,
                                                                                                &mut additional_trait_fns_to_analyze
                                                                                            );
        implicit_analyzed_generics.push(SurfFnArg::Generic{
                                                            name: String::from(adt_implicit_generic.name.as_str()),
                                                            traits: arg_required_bounds.clone(),
                                                        }
        );
    }
    (implicit_analyzed_generics, additional_cmplx_types_to_analyze, additional_trait_fns_to_analyze)
}

pub fn surf_collect_trait_data<'tcx>(tcx: TyCtxt<'tcx>, trait_def_id: DefId){
    let trait_data = SurfTraitData::new(
        surf_get_accessible_full_path(tcx, trait_def_id),
        tcx.trait_def(trait_def_id).safety == Safety::Unsafe,
        match trait_def_id.is_local(){
            false => Some(SurfExternalData::new(
                                                    tcx.crate_name(trait_def_id.krate).to_ident_string(),
                                                    surf_get_external_item_name(tcx, trait_def_id),
                                                )),
            true => None,
        }
    );
    SURF_TRAITS.lock().unwrap().entry(trait_def_id).or_insert(trait_data);
}

pub fn surf_add_trait_bound(trait_def_id: DefId, bounds_map: &mut FxHashMap<DefId, Box<SurfFnArg>>){
    bounds_map.entry(trait_def_id)
                .or_insert(Box::new(SurfFnArg::Trait { 
                                                                def_id: trait_def_id.clone(),
                                                                types: Vec::<Box<SurfFnArg>>::new(),
                                                                funcs: FxHashSet::<DefId>::default()
                                                            }));
}

pub fn surf_add_trait_bound_fn_item(
    trait_def_id: DefId,
    trait_fn_def_id: DefId,
    required_analyzed_traits: &mut FxHashMap<DefId, Box<SurfFnArg>>,
    trait_fns_to_analyze: &mut FxHashSet<DefId>
){
    
    if let SurfFnArg::Trait { funcs, .. } = &mut **required_analyzed_traits.get_mut(&trait_def_id).unwrap(){
        funcs.insert(trait_fn_def_id);
        if !trait_fns_to_analyze.contains(&trait_fn_def_id){
            trait_fns_to_analyze.insert(trait_fn_def_id);
        }
    }
}

pub fn surf_add_trait_bound_type_item<'tcx>(
    tcx: TyCtxt<'tcx>,
    surf_urapi_opt: &Option<SurfURAPI>,
    urapi_def_id: DefId,
    required_trait_def_id: DefId,
    trait_type_def_id: DefId,
    trait_type_placeholder_name: String,
    current_type_id: &String,
    trait_item_type: &GenericType,
    required_analyzed_traits: &mut FxHashMap<DefId, Box<SurfFnArg>>,
    func_trait_bounds_map: &FxHashMap<GenericType, FxHashSet<DefId>>,
    func_projections_map: &FxHashMap<GenericType, GenericType>,
    seen_trait_types: &mut FxHashSet<GenericType>,
    trait_fns_to_analyze: &mut FxHashSet<DefId>,
){

    if let SurfFnArg::Trait { types, .. } = &mut **required_analyzed_traits.get_mut(&required_trait_def_id).unwrap(){
        
        let current_type_id = format!("{current_type_id}::{:?}", trait_type_def_id.clone());
        let trait_type_traits = surf_get_required_bounds(
                                                                                                tcx,
                                                                                                surf_urapi_opt,
                                                                                                urapi_def_id,
                                                                                                &trait_item_type,
                                                                                                &current_type_id,
                                                                                                func_trait_bounds_map,
                                                                                                func_projections_map,
                                                                                                seen_trait_types,
                                                                                                trait_fns_to_analyze
                                                                                            );
        
        let mut trait_type_concrete_type_opt = surf_get_trait_type_concrete_type(trait_item_type, func_trait_bounds_map, func_projections_map);
        if let Some(trait_type_concrete_type) = &trait_type_concrete_type_opt {
            if *trait_type_concrete_type == current_type_id{
                trait_type_concrete_type_opt = None;
            }
        }
        
        let analyzed_type = Box::new(SurfFnArg::TraitType {
            def_id: trait_type_def_id,
            placeholder_name: trait_type_placeholder_name.clone(),
            assoc_type_id: current_type_id.clone(),
            concrete_type_id: trait_type_concrete_type_opt,
            traits: trait_type_traits
        });

        types.push(analyzed_type);
    }
}

pub fn surf_get_adt_generic_params<'tcx>(tcx: TyCtxt<'tcx>, adt_def_id: DefId) -> Vec<ParamTy>{
    let mut adt_generic_params = Vec::<ParamTy>::new();
    for param in tcx.generics_of(adt_def_id).own_params.iter() {
        if let GenericParamDefKind::Type { .. } = param.kind{
            let param_ty = ParamTy::new(param.index, param.name);
            adt_generic_params.push(param_ty);
        }
    }
    adt_generic_params
}

pub fn surf_get_constructors<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> FxHashSet<DefId>{
    //let adt_implicit_generics = surf_get_adt_generic_params(tcx, def_id);
    //println!("ADT: {:?} IMPLICIT GENS: {:?}", def_id, adt_implicit_generics);
    let mut constructors = FxHashSet::<DefId>::default();
    if let Ok(adt_inherent_impls) = tcx.inherent_impls(def_id){
        //println!("\tType{:?}: ", def_id);
        for adt_inherent_impl in adt_inherent_impls{
            let impl_items = tcx.associated_items(adt_inherent_impl);
            for impl_item in impl_items.in_definition_order(){
                let impl_item_def_id = impl_item.def_id;
                //println!("\tItem{:?}: ", impl_item_def_id);
                if assoc_item_is_safe_fn(tcx, impl_item) 
                    && !fn_item_has_self(tcx, impl_item_def_id)
                    //&& !surf_fn_has_non_inferable_generics(tcx, impl_item_def_id)
                    && surf_fn_has_constructor_output(tcx, impl_item_def_id, def_id)
                    && surf_fn_has_constructor_inputs(tcx, impl_item_def_id, def_id)
                    && (item_is_exported(tcx, impl_item_def_id) || item_is_imported(tcx, impl_item_def_id)){
                    constructors.insert(impl_item_def_id);
                }
            }
        }
    }
    // Extend with APIs that return the appropriate type
    for (func_def_id, surf_func) in SURF_FNS.lock().unwrap().iter(){
        if !constructors.contains(func_def_id)
            //&& !surf_fn_has_non_inferable_generics(tcx, *func_def_id)
            //&& !surf_is_converter(tcx, surf_func)
            && !surf_is_default_constructor(tcx, surf_func)
            && !surf_is_deserializer(tcx, surf_func)
            && !surf_fn_is_unsafe(surf_func)
            && surf_fn_has_constructor_output(tcx, *func_def_id, def_id)
            && surf_fn_has_constructor_inputs(tcx, *func_def_id, def_id)
            && item_is_exported(tcx, *func_def_id){
            constructors.insert(*func_def_id);
        }
    }
    constructors
}

pub fn surf_fn_has_constructor_output<'tcx>(tcx: TyCtxt<'tcx>, func_def_id: DefId, complex_type_def_id: DefId) -> bool{
    let output = tcx.fn_sig(func_def_id).skip_binder().skip_binder().output();
    surf_inner_type_matches(tcx, output, complex_type_def_id)
}

pub fn surf_inner_type_matches<'tcx>(tcx: TyCtxt<'tcx>, inner_type: Ty<'tcx>, complex_type_def_id: DefId) -> bool{
    match inner_type.kind() {
        ty::Adt(adt, adt_args) => {
            let adt_def_id = adt.did();
            if surf_complex_types_cmp(adt_def_id, complex_type_def_id){
                return true
            }
            else{
                if surf_is_result(tcx, adt_def_id) || surf_is_option(tcx, adt_def_id){
                    let wrapped_type = adt_args.type_at(0);
                    return surf_inner_type_matches(tcx, wrapped_type, complex_type_def_id);
                }    
            }
        },
        ty::Tuple(tuple_args) => {
            for tuple_arg in tuple_args.iter(){
                if surf_inner_type_matches(tcx, tuple_arg, complex_type_def_id){
                    return true
                }
            }
        },
        _ => {},
    }
    false
}

pub fn surf_fn_has_constructor_inputs<'tcx>(tcx: TyCtxt<'tcx>, func_def_id: DefId, complex_type_def_id: DefId) -> bool{
    let inputs = tcx.fn_sig(func_def_id).skip_binder().skip_binder().inputs();
    for input_type in inputs{
        if surf_fn_input_uses_the_complex_type(tcx, *input_type, complex_type_def_id){
            return false
        }
    }
    return true
}

// Let's not support enums' fields unwrapping for now. In order to prevent recursion we need to use global hashset in the same
// manner that we did with the arguments analyzing. W/o it we will get stuck at a enum MyEnum{A, B, C(Box<MyEnum>, MyStruct)}.
pub fn surf_fn_input_uses_the_complex_type<'tcx>(tcx: TyCtxt<'tcx>, input_type: Ty<'tcx>, complex_type_def_id: DefId) -> bool{
    match input_type.kind(){
        ty::Adt(adt, adt_args) => {
            let adt_def_id = adt.did();
            if surf_complex_types_cmp(adt_def_id, complex_type_def_id){
                return true
            }
            else if surf_is_result(tcx, adt_def_id) || surf_is_option(tcx, adt_def_id) || surf_is_vec(tcx, adt_def_id) || adt.is_box(){
                for adt_arg_type in adt_args.types(){
                    return surf_fn_input_uses_the_complex_type(tcx, adt_arg_type, complex_type_def_id);
                }
            }
        },
        ty::Array(inner_type, _) 
        | ty::Slice(inner_type)
        | ty::Ref(_, inner_type, _) => return surf_fn_input_uses_the_complex_type(tcx, *inner_type, complex_type_def_id),
        ty::Tuple(tuple_args) => {
            for tuple_arg in tuple_args.iter(){
                if surf_fn_input_uses_the_complex_type(tcx, tuple_arg, complex_type_def_id){
                    return true
                }
            }
        }
        _ => {},
    }
    return false
}

pub fn surf_complex_types_cmp(canditate_def_id: DefId, target_def_id: DefId) -> bool{
    canditate_def_id == target_def_id
}

pub fn surf_update_self_flag<'tcx>(tcx: TyCtxt<'tcx>){
    // Update URAPIs self Flag
    let mut urapis_lock = SURF_APIS_REACH_UNSAFE.lock().unwrap();
    for (api_def_id, surf_urapi) in urapis_lock.iter_mut(){
        surf_urapi.flags.entry("has_self").or_insert(fn_item_has_self(tcx, *api_def_id));
    }
    // Update Constructors self Flag
    let mut constructors_lock = SURF_CONSTRUCTORS.lock().unwrap();
    for (constructor_def_id , surf_constructor) in constructors_lock.iter_mut(){
        surf_constructor.flags.entry("has_self").or_insert(fn_item_has_self(tcx, *constructor_def_id));
    }
    // Update Trait Functions self Flag
    let mut trait_fns_lock = SURF_TRAIT_FNS.lock().unwrap();
    for (trait_fn_def_id , surf_trait_fn) in trait_fns_lock.iter_mut(){
        surf_trait_fn.flags.entry("has_self").or_insert(fn_item_has_self(tcx, *trait_fn_def_id));
    }
}

pub fn surf_update_drop_flag<'tcx>(tcx: TyCtxt<'tcx>){
    // Update URAPIs self Flag
    let mut urapis_lock = SURF_APIS_REACH_UNSAFE.lock().unwrap();
    let surf_fns_lock = SURF_FNS.lock().unwrap();
    for (api_def_id, surf_urapi) in urapis_lock.iter_mut(){
        surf_urapi.flags.entry("is_drop").or_insert(surf_is_drop_impl(tcx, surf_fns_lock.get(api_def_id).unwrap()));
    }
}

pub fn surf_update_display_flag<'tcx>(tcx: TyCtxt<'tcx>){
    // Update URAPIs self Flag
    let mut urapis_lock = SURF_APIS_REACH_UNSAFE.lock().unwrap();
    let surf_fns_lock = SURF_FNS.lock().unwrap();
    for (api_def_id, surf_urapi) in urapis_lock.iter_mut(){
        surf_urapi.flags.entry("is_display").or_insert(surf_is_display_impl(tcx, surf_fns_lock.get(api_def_id).unwrap()));
    }
}

pub fn surf_update_debug_flag<'tcx>(tcx: TyCtxt<'tcx>){
    // Update URAPIs self Flag
    let mut urapis_lock = SURF_APIS_REACH_UNSAFE.lock().unwrap();
    let surf_fns_lock = SURF_FNS.lock().unwrap();
    for (api_def_id, surf_urapi) in urapis_lock.iter_mut(){
        surf_urapi.flags.entry("is_debug").or_insert(surf_is_debug_impl(tcx, surf_fns_lock.get(api_def_id).unwrap()));
    }
}

pub fn surf_is_iter_trait<'tcx>(tcx: TyCtxt<'tcx>, trait_def_id: DefId) -> bool{
    let iterator_paths = ["std::iter::Iterator", "core::iter::Iterator"];
    let trait_def_path_str = ty::print::with_no_trimmed_paths!(tcx.def_path_str(trait_def_id));
    tcx.is_lang_item(trait_def_id, LangItem::Iterator) || iterator_paths.contains(&trait_def_path_str.as_str())
}

pub fn surf_is_converter<'tcx>(tcx: TyCtxt<'tcx>, surf_func: &SurfFunction) -> bool{
    if let SurfFnType::Method(surf_trait) = &surf_func.fn_type{
        if let Some(impl_tr_def_id) = surf_trait.impl_tr_name{
            return surf_is_from(tcx, impl_tr_def_id);
        }
    }
    return false;
}

pub fn surf_is_default_constructor<'tcx>(tcx: TyCtxt<'tcx>, surf_func: &SurfFunction) -> bool{
    if let SurfFnType::Method(surf_trait) = &surf_func.fn_type{
        if let Some(impl_tr_def_id) = surf_trait.impl_tr_name{
            return surf_is_default(tcx, impl_tr_def_id);
        }
    }
    return false;
}

pub fn surf_is_deserializer<'tcx>(tcx: TyCtxt<'tcx>, surf_func: &SurfFunction) -> bool{
    if let SurfFnType::Method(surf_trait) = &surf_func.fn_type{
        if let Some(impl_tr_def_id) = surf_trait.impl_tr_name{
            return surf_is_deserialize(tcx, impl_tr_def_id);
        }
    }
    return false;
}

// pub fn surf_is_consumable_cmplx_type(){

// }

pub fn surf_is_display_impl<'tcx>(tcx: TyCtxt<'tcx>, surf_func: &SurfFunction) -> bool{
    if let SurfFnType::Method(surf_trait) = &surf_func.fn_type{
        if let Some(impl_tr_def_id) = surf_trait.impl_tr_name{
            return surf_is_display(tcx, impl_tr_def_id);
        }
    }
    return false;
}

pub fn surf_is_display<'tcx>(tcx: TyCtxt<'tcx>, trait_def_id: DefId) -> bool{
    let display_paths = ["std::fmt::Display", "core::fmt::Display"];
    let candidate_path = ty::print::with_no_trimmed_paths!(tcx.def_path_str(trait_def_id));
    //println!("{:?}", candidate_path);
    display_paths.contains(&candidate_path.as_str())
}

pub fn surf_is_debug_impl<'tcx>(tcx: TyCtxt<'tcx>, surf_func: &SurfFunction) -> bool{
    if let SurfFnType::Method(surf_trait) = &surf_func.fn_type{
        if let Some(impl_tr_def_id) = surf_trait.impl_tr_name{
            return surf_is_debug(tcx, impl_tr_def_id);
        }
    }
    return false;
}

pub fn surf_is_debug<'tcx>(tcx: TyCtxt<'tcx>, trait_def_id: DefId) -> bool{
    let debug_paths = ["std::fmt::Debug", "core::fmt::Debug"];
    let candidate_path = ty::print::with_no_trimmed_paths!(tcx.def_path_str(trait_def_id));
    //println!("{:?}", candidate_path);
    debug_paths.contains(&candidate_path.as_str())
}

pub fn surf_is_drop_impl<'tcx>(tcx: TyCtxt<'tcx>, surf_func: &SurfFunction) -> bool{
    if let SurfFnType::Method(surf_trait) = &surf_func.fn_type{
        if let Some(impl_tr_def_id) = surf_trait.impl_tr_name{
            return surf_is_drop(tcx, impl_tr_def_id);
        }
    }
    return false;
}

pub fn surf_is_drop<'tcx>(tcx: TyCtxt<'tcx>, trait_def_id: DefId) -> bool{
    let drop_paths = ["core::ops::Drop", "std::ops::Drop"];
    let candidate_path = ty::print::with_no_trimmed_paths!(tcx.def_path_str(trait_def_id));
    //println!("{:?}", candidate_path);
    drop_paths.contains(&candidate_path.as_str())
}

pub fn surf_is_default<'tcx>(tcx: TyCtxt<'tcx>, trait_def_id: DefId) -> bool{
    let default_paths = ["std::default::Default", "core::default::Default"];
    let candidate_path = ty::print::with_no_trimmed_paths!(tcx.def_path_str(trait_def_id));
    default_paths.contains(&candidate_path.as_str())
}

pub fn surf_is_deserialize<'tcx>(tcx: TyCtxt<'tcx>, trait_def_id: DefId) -> bool{
    let candidate_path = ty::print::with_no_trimmed_paths!(tcx.def_path_str(trait_def_id));
    candidate_path.ends_with("serde::Deserialize")
}

pub fn surf_is_from<'tcx>(tcx: TyCtxt<'tcx>, trait_def_id: DefId) -> bool{
    let from_paths = ["std::convert::From", "core::convert::From"];
    let candidate_path = ty::print::with_no_trimmed_paths!(tcx.def_path_str(trait_def_id));
    from_paths.contains(&candidate_path.as_str())
}

pub fn surf_is_result<'tcx>(tcx: TyCtxt<'tcx>, adt_def_id: DefId) -> bool{
    //return Some(adt_def_id) == tcx.get_diagnostic_item(sym::Result)
    let result_paths = ["std::result::Result", "core::result::Result"];
    let candidate_path = ty::print::with_no_trimmed_paths!(tcx.def_path_str(adt_def_id));
    result_paths.contains(&candidate_path.as_str()) || Some(adt_def_id) == tcx.get_diagnostic_item(sym::Result)
}

pub fn surf_is_option<'tcx>(tcx: TyCtxt<'tcx>, adt_def_id: DefId) -> bool{
    //return Some(adt_def_id) == tcx.get_diagnostic_item(sym::Option)
    let option_paths = ["std::option::Option", "core::option::Option"];
    let candidate_path = ty::print::with_no_trimmed_paths!(tcx.def_path_str(adt_def_id));
    option_paths.contains(&candidate_path.as_str()) || Some(adt_def_id) == tcx.get_diagnostic_item(sym::Option)
}

pub fn surf_is_string<'tcx>(tcx: TyCtxt<'tcx>, adt_def_id: DefId) -> bool{
    let string_paths = ["std::string::String", "core::string::String", "alloc::string::String"];
    let candidate_path = ty::print::with_no_trimmed_paths!(tcx.def_path_str(adt_def_id));
    string_paths.contains(&candidate_path.as_str()) || Some(adt_def_id) == tcx.get_diagnostic_item(sym::String)
}

pub fn surf_is_vec<'tcx>(tcx: TyCtxt<'tcx>, adt_def_id: DefId) -> bool {
    // return Some(adt_def_id) == tcx.get_diagnostic_item(sym::Vec)
    let vec_paths = ["std::vec::Vec", "alloc::vec::Vec"];
    let candidate_path = ty::print::with_no_trimmed_paths!(tcx.def_path_str(adt_def_id));
    vec_paths.contains(&candidate_path.as_str()) || Some(adt_def_id) == tcx.get_diagnostic_item(sym::Vec)
}

pub fn surf_is_maybeuinit<'tcx>(tcx: TyCtxt<'tcx>, adt_def_id: DefId) -> bool {
    let uinit_paths = ["std::mem::MaybeUninit", "core::mem::MaybeUninit"];
    let candidate_path = ty::print::with_no_trimmed_paths!(tcx.def_path_str(adt_def_id));
    uinit_paths.contains(&candidate_path.as_str())
}

pub fn surf_compute_stats(){
    // may get better but it works
    SURF_FNS_STATS.lock().unwrap().entry("PubFn")
                                         .or_insert(FxHashMap::<&'static str, u64>::default())
                                         .extend([("Total", 0), ("SafeFn", 0), ("UnsafeFn", 0), ("UnsafeBlockFn", 0), ("APIsReachUnsafe", 0)]);
    
    SURF_FNS_STATS.lock().unwrap().entry("PrvFn")
                                        .or_insert(FxHashMap::<&'static str, u64>::default())
                                        .extend([("Total", 0), ("SafeFn", 0), ("UnsafeFn", 0), ("UnsafeBlockFn", 0)]);

    for (_, surf_func) in SURF_FNS.lock().unwrap().iter(){
        let visiblity = match surf_func.visibility{
            SurfFnVisibility::PubFn => "PubFn",
            SurfFnVisibility::PrvFn => "PrvFn",
        };
        let safety = match surf_func.safety{
            SurfFnSafety::SafeFn => "SafeFn",
            SurfFnSafety::UnsafeFn => "UnsafeFn",
            SurfFnSafety::UnsafeBlockFn => "UnsafeBlockFn",
        };
        // Future Update: Create a handle so you dont lock twice?
        SURF_FNS_STATS.lock().unwrap().get_mut(visiblity).unwrap().entry(safety).and_modify(|count| *count += 1);
        SURF_FNS_STATS.lock().unwrap().get_mut(visiblity).unwrap().entry("Total").and_modify(|count| *count += 1);
    }

    // The APIs That Reach Unsafe Stats
    let count = SURF_APIS_REACH_UNSAFE.lock().unwrap().len() as u64;
    *SURF_FNS_STATS.lock().unwrap().get_mut("PubFn").unwrap().get_mut("APIsReachUnsafe").unwrap() = count;    
}



/* ---------------------------------------------------------------------------------------------------------------
                                            OUTPUT FUNCTIONS
-----------------------------------------------------------------------------------------------------------------*/

pub fn surf_create_report_dir() -> Option<String>{
    if let Ok(working_path) =  env::var("SURF_WORKING_PATH"){
        let report_dir_path = format!("{working_path}/deepSURF/report");
        let path = Path::new(&report_dir_path);
        if !path.exists() {
            match fs::create_dir_all(path) {
                Ok(_) => Some(report_dir_path),
                Err(e) => {
                    eprintln!("Failed to create folder: {}", e);
                    None
                },
            }
        } else {
            Some(report_dir_path)
        }

    }else{
        eprintln!("Please set the env variable 'SURF_WORKING_PATH'");
        None
    }
}

pub fn surf_export_cfg_to_dot(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.cfg.dot", &crate_name, crate_id);
        let mut file = File::create(report_path).unwrap();
        let binding = SURF_CFG.lock().unwrap().clone();
        let dot = Dot::with_config(&binding, &[Config::EdgeNoLabel]);
        file.write_all(format!("{:?}", dot).as_bytes()).expect("Unable to write to DOT file");
    }
}

pub fn surf_write_func_dict(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.funcs.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &SURF_FNS.lock().unwrap().clone()).unwrap();
        writer.flush().unwrap();
    }
}

pub fn surf_write_used_deps(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.deps.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &TARGET_CRATE_USED_DEPS.lock().unwrap().clone()).unwrap();
        writer.flush().unwrap();
    }
}

pub fn surf_write_urapi_dict(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.urapi.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &SURF_APIS_REACH_UNSAFE.lock().unwrap().clone()).unwrap();
        writer.flush().unwrap();
    } 
}

pub fn surf_write_macro_exp_urapis(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.macro-urapi.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &SURF_MACRO_URAPIS.lock().unwrap().clone()).unwrap();
        writer.flush().unwrap();
    } 
}

pub fn surf_write_urapi_dict_llm(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.urapi-llm.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        let mut surf_urapis_llm = FxHashMap::<DefId, SurfURAPILLM>::default();
        for (def_id, surf_urapi) in SURF_APIS_REACH_UNSAFE.lock().unwrap().iter(){
            surf_urapis_llm.insert(*def_id, SurfURAPILLM::new(surf_urapi));
        }
        serde_json::to_writer_pretty(&mut writer, &surf_urapis_llm).unwrap();
        writer.flush().unwrap();
    } 
}

pub fn surf_write_constructors_dict(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.constrs.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &SURF_CONSTRUCTORS.lock().unwrap().clone()).unwrap();
        writer.flush().unwrap();
    }
}

pub fn surf_write_constructors_dict_llm(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.constrs-llm.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        let mut surf_constructors_llm = FxHashMap::<DefId, SurfConstructorDataLLM>::default();
        for (def_id, surf_constructor_data) in SURF_CONSTRUCTORS.lock().unwrap().iter(){
            surf_constructors_llm.insert(*def_id, SurfConstructorDataLLM::new(surf_constructor_data));
        }
        serde_json::to_writer_pretty(&mut writer, &surf_constructors_llm).unwrap();
        writer.flush().unwrap();
    }
}

pub fn surf_write_enums_dict(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.enums.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &SURF_ENUMS.lock().unwrap().clone()).unwrap();
        writer.flush().unwrap();
    }
}

pub fn surf_write_trait_fns_dict(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.trait_fns.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &SURF_TRAIT_FNS.lock().unwrap().clone()).unwrap();
        writer.flush().unwrap();
    }
}

pub fn surf_write_traits_dict(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.traits.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &SURF_TRAITS.lock().unwrap().clone()).unwrap();
        writer.flush().unwrap();
    }
}

pub fn surf_write_complex_types_dict(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.cmplx_tys.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &SURF_COMPLEX_TYPES_TO_CONSTRUCTORS.lock().unwrap().clone()).unwrap();
        writer.flush().unwrap();
    }
}

pub fn surf_write_stats(crate_name: &str, crate_id: u64){
    if let Some(report_dir_path) = surf_create_report_dir(){
        let report_path = format!("{report_dir_path}/{}_{}.stats.json", &crate_name, crate_id);
        let file = File::create(report_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &SURF_FNS_STATS.lock().unwrap().clone()).unwrap();
        writer.flush().unwrap();
    }
}


/*
------------------------------------------------------------------------------------------------------------------
                        Complex types (Enums & Structs)
------------------------------------------------------------------------------------------------------------------
*/

/*
    Enum that stores the version (type and number) of a dependency.
    Used for dependencies of the targeting crate.
*/
#[derive(Debug, Serialize, Clone)]
pub enum CrateDepType{
    Version(String),
    Path(String),
    Empty
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum SurfFnSafety{
    UnsafeFn,
    UnsafeBlockFn,
    SafeFn,
}

#[derive(Debug, Clone)]
pub struct SurfClosureSig<'tcx>{
    inputs: Vec<Ty<'tcx>>,
    output: Option<Ty<'tcx>>,
    is_mutable: bool,
}

impl <'tcx> SurfClosureSig<'tcx>{
    pub fn new(is_mutable: bool) -> Self{
        Self{
            inputs: Vec::<Ty<'tcx>>::new(),
            output: None,
            is_mutable,
        }
    }
}



#[derive(Debug, Serialize, Clone)]
pub enum SurfFnType{
    Fn,
    Method(SurfTrait),
}

#[derive(Debug, Serialize, Clone, Eq, Hash, PartialEq)]
pub enum SurfFnVisibility{
    PubFn,
    PrvFn,
}

#[derive(Debug, Serialize, Clone)]
pub struct SurfURAPI{
    pub name: String,
    pub full_name: String,
    pub def_path_str: String,
    pub inputs: Vec<SurfFnArg>,
    pub output: Option<SurfFnArg>,
    pub implicit_generics: Vec<SurfFnArg>,
    pub flags: FxHashMap<&'static str, bool>,
    pub crate_name: String,
    pub reachable_unresolved_callees: FxHashMap<DefId, FxHashSet<DefId>>,
}

impl SurfURAPI{
    pub fn new(name: String, full_name: String, def_path_str: String, crate_name: String) -> SurfURAPI{
        Self {
            name,
            full_name,
            def_path_str,
            inputs: Vec::<SurfFnArg>::new(),
            output: None,
            implicit_generics: Vec::<SurfFnArg>::new(),
            flags: FxHashMap::<&'static str, bool>::default(),
            crate_name,
            reachable_unresolved_callees: FxHashMap::<DefId, FxHashSet<DefId>>::default(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SurfURAPILLM{
    pub full_name: String,
    pub inputs: Vec<SurfFnArgDistilled>,
    pub output: Option<SurfFnArgDistilled>,
    pub implicit_generics: Vec<SurfFnArgDistilled>,
}

impl SurfURAPILLM{
    pub fn new(surf_urapi: &SurfURAPI) -> SurfURAPILLM{
        let mut distilled_inputs = Vec::<SurfFnArgDistilled>::new();
        for surf_fn_arg in surf_urapi.inputs.iter(){
            distilled_inputs.push(surf_distill_fn_arg(&surf_fn_arg));
        }
        
        let distilled_output = match &surf_urapi.output{
            Some(surf_fn_arg) => Some(surf_distill_fn_arg(&surf_fn_arg)),
            _ => None,
        };

        let mut distilled_implicit_generics = Vec::<SurfFnArgDistilled>::new();
        for surf_fn_arg in surf_urapi.implicit_generics.iter(){
            distilled_implicit_generics.push(surf_distill_fn_arg(&surf_fn_arg));
        }
        
        Self {
            full_name: surf_urapi.full_name.clone(),
            inputs: distilled_inputs,
            output: distilled_output,
            implicit_generics: distilled_implicit_generics,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SurfConstructorData{
    pub name: String,
    pub full_name: String,
    pub def_path_str: String,
    pub inputs: Vec<SurfFnArg>,
    pub output: Option<SurfFnArg>,
    pub implicit_generics: Vec<SurfFnArg>,
    pub flags: FxHashMap<&'static str, bool>,
}

impl SurfConstructorData{
    pub fn new(name: String, full_name: String, def_path_str: String) -> SurfConstructorData{
        Self {
            name,
            full_name,
            def_path_str,
            inputs: Vec::<SurfFnArg>::new(),
            output: None,
            implicit_generics: Vec::<SurfFnArg>::new(),
            flags: FxHashMap::<&'static str, bool>::default(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SurfConstructorDataLLM{
    pub full_name: String,
    pub inputs: Vec<SurfFnArgDistilled>,
    pub output: Option<SurfFnArgDistilled>,
    pub implicit_generics: Vec<SurfFnArgDistilled>,
}

impl SurfConstructorDataLLM{
    pub fn new(surf_constructor_data: &SurfConstructorData) -> SurfConstructorDataLLM{
        let mut distilled_inputs = Vec::<SurfFnArgDistilled>::new();
        for surf_fn_arg in surf_constructor_data.inputs.iter(){
            distilled_inputs.push(surf_distill_fn_arg(&surf_fn_arg));
        }
        
        let distilled_output = match &surf_constructor_data.output{
            Some(surf_fn_arg) => Some(surf_distill_fn_arg(&surf_fn_arg)),
            _ => None,
        };

        let mut distilled_implicit_generics = Vec::<SurfFnArgDistilled>::new();
        for surf_fn_arg in surf_constructor_data.implicit_generics.iter(){
            distilled_implicit_generics.push(surf_distill_fn_arg(&surf_fn_arg));
        }

        Self {
            full_name: surf_constructor_data.full_name.clone(),
            inputs: distilled_inputs,
            output: distilled_output,
            implicit_generics: distilled_implicit_generics,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SurfTraitData{
    pub name: String,
    pub is_unsafe: bool,
    pub external: Option<SurfExternalData>,
}

impl SurfTraitData{
    pub fn new(name: String, is_unsafe: bool, external: Option<SurfExternalData>) -> SurfTraitData{
        Self {
            name,
            is_unsafe,
            external,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SurfExternalData{
    pub crate_name: String,
    pub trait_path: String,
}

impl SurfExternalData{
    pub fn new(crate_name: String, trait_path: String) -> SurfExternalData{
        Self {
            crate_name,
            trait_path,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SurfTraitFnData{
    pub name: String,
    pub full_name: String,
    pub is_unsafe: bool,
    pub span_str: String,
    pub inputs: Vec<SurfFnArg>,
    pub output: Option<SurfFnArg>,
    pub flags: FxHashMap<&'static str, bool>,
}

impl SurfTraitFnData{
    pub fn new(name: String, full_name: String, span_str: String, is_unsafe: bool) -> SurfTraitFnData{
        Self {
            name,
            full_name,
            is_unsafe,
            span_str,
            inputs: Vec::<SurfFnArg>::new(),
            output: None,
            flags: FxHashMap::<&'static str, bool>::default(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum SurfFnArg{
    Primitive(String),
    String,
    Generic{
        name: String,
        traits: FxHashMap<DefId, Box<SurfFnArg>>, // trait_def_id -> SurfFnArg::Trait
    },
    AssocType{
        def_id: DefId,
        trait_def_id: DefId,
        placeholder_name: String,
        assoc_type_id: String,
        traits: FxHashMap<DefId, Box<SurfFnArg>>,
    },
    Trait{
        def_id: DefId,
        types: Vec<Box<SurfFnArg>>, // vec![TraitType(...)]
        funcs: FxHashSet<DefId>
    },
    TraitType{
        def_id: DefId,
        placeholder_name: String,
        assoc_type_id: String,
        concrete_type_id: Option<String>,
        traits: FxHashMap<DefId, Box<SurfFnArg>>,
    },
    Closure{param_name: String, inputs: Vec<Box<SurfFnArg>>, output: Option<Box<SurfFnArg>>, is_mutable: bool},
    Struct{def_id: DefId, name: String, full_name: String, is_consumable: bool},
    Reference(Box<SurfFnArg>, bool),
    RawPointer(Box<SurfFnArg>, bool),
    DynTrait(Vec<Box<SurfFnArg>>),
    Slice(Box<SurfFnArg>),
    Array(Box<SurfFnArg>, String),
    Tuple(Vec<Box<SurfFnArg>>),
    Enum{def_id: DefId, name: String, full_name: String, is_consumable: bool},
    Vector(Box<SurfFnArg>),
    Uinit(Box<SurfFnArg>),
    Box(Box<SurfFnArg>),
    Option(Box<SurfFnArg>),
    Result{ok: Box<SurfFnArg>, err: String},
    Todo(String),
}

#[derive(Debug, Serialize, Clone)]
pub enum SurfFnArgDistilled{
    Primitive(String),
    String,
    Generic{
        name: String,
        #[serde(skip_serializing)]
        traits: Vec<Box<SurfFnArgDistilled>>,
    },
    AssocType{
        def_id: DefId,
        //trait_def_id: DefId,
        placeholder_name: String,
        //assoc_type_id: String,
        #[serde(skip_serializing)]
        traits: Vec<Box<SurfFnArgDistilled>>,
    },
    Trait{
        def_id: DefId,
        types: Vec<Box<SurfFnArgDistilled>>, // vec![TraitType(...)]
        funcs: FxHashSet<DefId>
    },
    TraitType{
        def_id: DefId,
        placeholder_name: String,
        //assoc_type_id: String,
        //concrete_type_id: Option<String>,
        traits: Vec<Box<SurfFnArgDistilled>>,
    },
    Closure{
        param_name: String,
        inputs: Vec<Box<SurfFnArgDistilled>>,
        output: Option<Box<SurfFnArgDistilled>>,
        is_mutable: bool
    },
    Struct{
        def_id: DefId,
        full_name: String
    },
    Reference(Box<SurfFnArgDistilled>),
    MutReference(Box<SurfFnArgDistilled>),
    RawPointer(Box<SurfFnArgDistilled>),
    MutRawPointer(Box<SurfFnArgDistilled>),
    DynTrait{
        candidates: Vec<Box<SurfFnArgDistilled>>
    },
    Slice(Box<SurfFnArgDistilled>),
    Array{
        inner_ty: Box<SurfFnArgDistilled>,
        size: String
    },
    Tuple(Vec<Box<SurfFnArgDistilled>>),
    Enum{
        def_id: DefId,
        full_name: String
    },
    Vector(Box<SurfFnArgDistilled>),
    Uinit(Box<SurfFnArgDistilled>),
    Box(Box<SurfFnArgDistilled>),
    Option(Box<SurfFnArgDistilled>),
    Result{ok: Box<SurfFnArgDistilled>, err: String},
    Todo(String),
}

pub fn surf_distill_fn_arg(surf_fn_arg: &SurfFnArg) -> SurfFnArgDistilled{
    match surf_fn_arg{
        SurfFnArg::Primitive(prim_ty) => SurfFnArgDistilled::Primitive(prim_ty.clone()),
        SurfFnArg::String => SurfFnArgDistilled::String,
        SurfFnArg::Generic { name, traits } => {
            let mut distilled_traits = Vec::<Box<SurfFnArgDistilled>>::new();
            for trait_fn_arg in traits.values(){
                distilled_traits.push(Box::new(surf_distill_fn_arg(trait_fn_arg)));
            }

            SurfFnArgDistilled::Generic{
                                            name: name.to_string(),
                                            traits: distilled_traits,
                                        }
        },
        SurfFnArg::AssocType { def_id, traits, placeholder_name, .. } => {
            
            let mut distilled_traits = Vec::<Box<SurfFnArgDistilled>>::new();
            for trait_fn_arg in traits.values(){
                distilled_traits.push(Box::new(surf_distill_fn_arg(trait_fn_arg)));
            }
            SurfFnArgDistilled::AssocType{
                                        def_id: *def_id,
                                        placeholder_name: placeholder_name.clone(),
                                        traits: distilled_traits
                                    }
        },
        SurfFnArg::TraitType { def_id, traits, placeholder_name, .. } => {
            
            let mut distilled_traits = Vec::<Box<SurfFnArgDistilled>>::new();
            for trait_fn_arg in traits.values(){
                distilled_traits.push(Box::new(surf_distill_fn_arg(trait_fn_arg)));
            }
            SurfFnArgDistilled::TraitType{
                                        def_id: *def_id,
                                        placeholder_name: placeholder_name.clone(),
                                        traits: distilled_traits
                                    }
        },
        SurfFnArg::Trait { def_id, types, funcs } => {
            /*
                types: Vec<Box<SurfFnArg>>, // vec![TraitType(...)]
                funcs: FxHashSet<DefId>
            */
            let mut distilled_types = Vec::<Box<SurfFnArgDistilled>>::new();
            for type_fn_arg in types.iter(){
                distilled_types.push(Box::new(surf_distill_fn_arg(type_fn_arg)));
            }

            let mut distilled_funcs = FxHashSet::<DefId>::default();
            for type_fn_arg in funcs.iter(){
                distilled_funcs.insert(*type_fn_arg);
            }

            SurfFnArgDistilled::Trait {
                                def_id: *def_id,
                                types: distilled_types,
                                funcs: distilled_funcs
                            }
        },
        SurfFnArg::Closure{ param_name, inputs, output, is_mutable } => {
            let mut distilled_inputs = Vec::<Box<SurfFnArgDistilled>>::default();
            for surf_fn_arg in inputs{
                distilled_inputs.push(Box::new(surf_distill_fn_arg(&surf_fn_arg)));
            }
            
            let distilled_output = match output{
                Some(surf_fn_arg) => Some(Box::new(surf_distill_fn_arg(&surf_fn_arg))),
                _ => None,
            };
            SurfFnArgDistilled::Closure {
                                    param_name: param_name.clone(),
                                    inputs: distilled_inputs,
                                    output: distilled_output,
                                    is_mutable: *is_mutable,
                                }
        },
        SurfFnArg::Struct { def_id, full_name, .. } => {
            SurfFnArgDistilled::Struct { def_id: *def_id, full_name: full_name.clone() }
        },
        SurfFnArg::Reference(inner_type, is_mutable) => {
            match *is_mutable{
                true => SurfFnArgDistilled::MutReference(Box::new(surf_distill_fn_arg(inner_type))),
                false => SurfFnArgDistilled::Reference(Box::new(surf_distill_fn_arg(inner_type))),
            }
        },
        SurfFnArg::RawPointer(inner_type, is_mutable) => {
            match *is_mutable{
                true => SurfFnArgDistilled::MutRawPointer(Box::new(surf_distill_fn_arg(inner_type))),
                false => SurfFnArgDistilled::RawPointer(Box::new(surf_distill_fn_arg(inner_type))),
            }        },
        SurfFnArg::DynTrait(candidates) => {
            let mut distilled_candidates = Vec::<Box<SurfFnArgDistilled>>::new();
            for candidate in candidates{
                distilled_candidates.push(Box::new(surf_distill_fn_arg(&candidate)));
            }
            SurfFnArgDistilled::DynTrait { candidates: distilled_candidates }
        },
        SurfFnArg::Slice(inner_ty) => {
            let distilled_inner_ty = Box::new(surf_distill_fn_arg(inner_ty));
            SurfFnArgDistilled::Slice(distilled_inner_ty)
        },
        SurfFnArg::Array(inner_ty, size) => {
            let distilled_inner_ty = Box::new(surf_distill_fn_arg(inner_ty));
            SurfFnArgDistilled::Array { inner_ty: distilled_inner_ty, size: size.clone() }
        },
        SurfFnArg::Tuple(surf_fn_args) => {
            let mut distilled_surf_fn_args = Vec::<Box<SurfFnArgDistilled>>::new();
            for surf_fn_arg in surf_fn_args{
                distilled_surf_fn_args.push(Box::new(surf_distill_fn_arg(&surf_fn_arg)));
            }
            SurfFnArgDistilled::Tuple(distilled_surf_fn_args)
        },
        SurfFnArg::Enum { def_id, full_name, .. } => {
            SurfFnArgDistilled::Enum { def_id: *def_id, full_name: full_name.clone() }
        },
        SurfFnArg::Vector(inner_ty) => {
            let distilled_inner_ty = Box::new(surf_distill_fn_arg(inner_ty));
            SurfFnArgDistilled::Vector(distilled_inner_ty)
        },
        SurfFnArg::Uinit(inner_ty) => {
            let distilled_inner_ty = Box::new(surf_distill_fn_arg(inner_ty));
            SurfFnArgDistilled::Uinit(distilled_inner_ty)
        },
        SurfFnArg::Box(inner_ty) => {
            let distilled_inner_ty = Box::new(surf_distill_fn_arg(inner_ty));
            SurfFnArgDistilled::Box(distilled_inner_ty)
        },
        SurfFnArg::Option(inner_ty) => {
            let distilled_inner_ty = Box::new(surf_distill_fn_arg(inner_ty));
            SurfFnArgDistilled::Option(distilled_inner_ty)
        },
        SurfFnArg::Result { ok, err } => {
            let distilled_ok_ty = Box::new(surf_distill_fn_arg(ok));
            SurfFnArgDistilled::Result { ok: distilled_ok_ty, err: err.clone() }
        },
        SurfFnArg::Todo(msg) => {
            SurfFnArgDistilled::Todo(msg.clone())
        }
    }
}


#[derive(Debug, Serialize, Clone)]
pub struct SurfFunction{
    pub name: String,
    pub safety: SurfFnSafety,
    pub crate_name: String,
    pub visibility: SurfFnVisibility,
    pub fn_type: SurfFnType,
    pub callers: FxHashSet<DefId>,
    pub public_predecessors: FxHashSet<DefId>, // Only for functions with unsafe blocks
    pub unresolved_callees: FxHashSet<DefId>,
}

impl SurfFunction{
    pub fn new(name: &str, safety: SurfFnSafety,
                crate_name: &str, visibility: SurfFnVisibility,
                fn_type: SurfFnType) -> SurfFunction{
        Self {
            name: name.to_owned(),
            safety,
            crate_name: crate_name.to_owned(),
            visibility,
            fn_type,
            callers: FxHashSet::<DefId>::default(),
            public_predecessors: FxHashSet::<DefId>::default(),
            unresolved_callees: FxHashSet::<DefId>::default(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SurfTrait{
    pub impl_name: String,
    pub impl_tr_name: Option<DefId>,
}

impl SurfTrait{
    pub fn new(impl_name: &str, impl_tr_name: Option<DefId>) -> SurfTrait{
        Self {
            impl_name: impl_name.to_owned(),
            impl_tr_name
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ArrayCandidateInnerTypeKind{
    Generic(ParamTy),
    // ...
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ArrayTraitCandidate{
    kind: ArrayCandidateInnerTypeKind,
    impl_def_id: DefId,
    len: u64,
}

impl ArrayTraitCandidate{
    pub fn new(kind: ArrayCandidateInnerTypeKind, impl_def_id: DefId, len: u64) -> Self{
        Self{
            kind,
            impl_def_id,
            len
        }
    }
}
