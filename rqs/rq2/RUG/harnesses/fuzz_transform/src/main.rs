#![feature(box_into_inner)]
use std::any;
use std::cmp::min;
use std::io::{Read, Write};
use std::str::FromStr;
use syn::{
    parse::Parser, visit::Visit, Expr, ExprLit, ItemFn, Lit, LitBool, LitChar, LitFloat,
    LitInt, LitStr, Stmt, visit, Ident, fold::{self, Fold},
    ExprMacro, MacroDelimiter, Item, ExprCall, Token, ItemMod, ExprRepeat, Type,
};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
fn lit_to_string(lit: &Lit) -> String {
    match lit {
        Lit::Str(lit_str) => lit_str.value(),
        Lit::ByteStr(lit_bytestr) => format!("b{:?}", & lit_bytestr.value()),
        Lit::Byte(lit_byte) => format!("b'{}'", lit_byte.value() as char),
        Lit::Char(lit_char) => format!("'{}'", lit_char.value()),
        Lit::Int(lit_int) => lit_int.to_string(),
        Lit::Float(lit_float) => lit_float.to_string(),
        Lit::Bool(lit_bool) => lit_bool.value().to_string(),
        Lit::Verbatim(lit_verbatim) => lit_verbatim.to_string(),
        &_ => todo!(),
    }
}
fn are_lits_equal(lit1: &Lit, lit2: &Lit) -> bool {
    match (lit1, lit2) {
        (Lit::Str(lit_str1), Lit::Str(lit_str2)) => lit_str1.value() == lit_str2.value(),
        (Lit::ByteStr(lit_byte_str1), Lit::ByteStr(lit_byte_str2)) => {
            lit_byte_str1.value() == lit_byte_str2.value()
        }
        (Lit::Byte(lit_byte1), Lit::Byte(lit_byte2)) => {
            lit_byte1.value() == lit_byte2.value()
        }
        (Lit::Char(lit_char1), Lit::Char(lit_char2)) => {
            lit_char1.value() == lit_char2.value()
        }
        (Lit::Int(lit_int1), Lit::Int(lit_int2)) => {
            lit_int1.base10_digits() == lit_int2.base10_digits()
                && lit_int1.suffix() == lit_int2.suffix()
        }
        (Lit::Float(lit_float1), Lit::Float(lit_float2)) => {
            lit_float1.base10_digits() == lit_float2.base10_digits()
                && lit_float1.suffix() == lit_float2.suffix()
        }
        (Lit::Bool(lit_bool1), Lit::Bool(lit_bool2)) => {
            lit_bool1.value == lit_bool2.value
        }
        (Lit::Verbatim(lit_verbatim1), Lit::Verbatim(lit_verbatim2)) => {
            lit_verbatim1.to_string() == lit_verbatim2.to_string()
        }
        _ => false,
    }
}
struct LiteralReplacer {
    primitives: Vec<(Lit, String, TokenStream)>,
    replacements: Vec<(Lit, Ident)>,
    in_mod: bool,
    in_fn: bool,
    target_mod: Option<String>,
    cur_fn: String,
}
impl LiteralReplacer {
    fn new() -> Self {
        LiteralReplacer {
            primitives: vec![],
            replacements: Vec::<(Lit, Ident)>::new(),
            in_mod: false,
            in_fn: false,
            target_mod: None,
            cur_fn: String::new(),
        }
    }
    fn new_with_target(t: String) -> Self {
        LiteralReplacer {
            primitives: vec![],
            replacements: Vec::<(Lit, Ident)>::new(),
            in_mod: false,
            in_fn: false,
            target_mod: Some(t),
            cur_fn: String::new(),
        }
    }
    fn clean(&mut self) {
        self.replacements.clear();
        self.primitives.clear();
    }
    fn generate_declarations(&self) -> (Vec<Stmt>, Stmt, Stmt) {
        let ans1 = self
            .primitives
            .iter()
            .map(|(lit, var_name, ty)| {
                let var_ident = syn::Ident::new(
                    var_name,
                    proc_macro2::Span::call_site(),
                );
                let declaration = quote! {
                    let # var_ident = # lit;
                };
                print!(
                    "{}~{}~{}~{}_rrrruuuugggg_", self.target_mod.clone().unwrap(), self
                    .cur_fn, var_name, lit.to_token_stream().to_string()
                );
                syn::parse2(declaration).unwrap()
            })
            .collect();
        let start_comment = syn::parse_str::<
            Stmt,
        >(
                &*format!(
                    "let _rug_st_{}_rrrruuuugggg_{}=0;", self.target_mod.clone()
                    .unwrap(), self.cur_fn
                ),
            )
            .unwrap();
        let end_comment = syn::parse_str::<
            Stmt,
        >(
                &*format!(
                    "let _rug_ed_{}_rrrruuuugggg_{}=0;", self.target_mod.clone()
                    .unwrap(), self.cur_fn
                ),
            )
            .unwrap();
        return (ans1, start_comment, end_comment);
    }
}
impl Fold for LiteralReplacer {
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        if !self.in_mod || !self.in_fn {
            return fold::fold_expr(self, expr);
        }
        match expr {
            Expr::Lit(expr_lit) => {
                let var_name = format!("rug_fuzz_{}", self.primitives.len());
                let i = expr_lit.lit;
                let (lit, ty) = match &i {
                    Lit::Str(_) => (i.clone(), quote!(& str)),
                    Lit::ByteStr(_) => (i.clone(), quote!(& [u8])),
                    Lit::Byte(_) => (i.clone(), quote!(u8)),
                    Lit::Char(_) => (i.clone(), quote!(char)),
                    Lit::Int(lit_int) => {
                        let ty = quote!(u8);
                        (i.clone(), ty)
                    }
                    Lit::Float(lit_float) => {
                        let ty = quote!(f32);
                        (i.clone(), ty)
                    }
                    Lit::Bool(_) => (i.clone(), quote!(bool)),
                    Lit::Verbatim(_) => (i.clone(), quote!()),
                    &_ => todo!(),
                };
                self.primitives.push((lit, var_name.clone(), ty));
                self.replacements
                    .push((
                        i.clone(),
                        Ident::new(&var_name.clone(), proc_macro2::Span::call_site()),
                    ));
                let var_ident = syn::Ident::new(
                    &*var_name,
                    proc_macro2::Span::call_site(),
                );
                let replaced_expr = Expr::Path(syn::ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: var_ident.into(),
                });
                return fold::fold_expr(self, replaced_expr);
            }
            _ => fold::fold_expr(self, expr),
        }
    }
    fn fold_expr_repeat(&mut self, i: ExprRepeat) -> ExprRepeat {
        let mut ans = i.clone();
        ans.expr = Box::new(self.fold_expr(*i.expr));
        ans
    }
    fn fold_type(&mut self, i: Type) -> Type {
        i
    }
    fn fold_expr_macro(&mut self, node: ExprMacro) -> ExprMacro {
        if !self.in_mod || !self.in_fn {
            return fold::fold_expr_macro(self, node);
        }
        let macro_name = node.mac.path.segments.last().unwrap().ident.to_string();
        let new_macro_name = match macro_name.as_str() {
            "assert" => String::from("debug_assert"),
            "assert_eq" => String::from("debug_assert_eq"),
            "assert_ne" => String::from("debug_assert_ne"),
            "assert_matches" => String::from("debug_assert_matches"),
            _ => macro_name.clone(),
        };
        if new_macro_name.starts_with("assert") {
            println!("missing {}", new_macro_name);
        }
        let mut new_macro = node.clone();
        let tmp = format!("f({})", node.mac.tokens.to_string());
        if let Ok(syntax_tree) = syn::parse_str::<ExprCall>(&tmp) {
            let mut first = true;
            let mut parsed = String::new();
            if macro_name.eq("format") || macro_name.eq("println")
                || macro_name.eq("print") || macro_name.eq("panic")
            {
                for ex in syntax_tree.args {
                    if first {
                        parsed += &*ex.to_token_stream().to_string();
                        first = false;
                    } else {
                        let nf = self.fold_expr(ex);
                        parsed += &*nf.to_token_stream().to_string();
                    }
                    parsed += ", ";
                }
            } else {
                for ex in syntax_tree.args {
                    if first {
                        let nf = self.fold_expr(ex);
                        parsed += &*nf.to_token_stream().to_string();
                        first = false;
                    } else {
                        parsed += &*ex.to_token_stream().to_string();
                    }
                    parsed += ", ";
                }
            }
            let t = &*parsed;
            let end = if parsed.len() > 2 { parsed.len() - 2 } else { parsed.len() };
            new_macro
                .mac
                .tokens = proc_macro2::TokenStream::from_str(&t[0..end]).unwrap();
        }
        if !macro_name.eq(&new_macro_name) {
            let new_ident = Ident::new(
                &*new_macro_name.clone(),
                proc_macro2::Span::call_site(),
            );
            new_macro.mac.path = new_ident.into();
        }
        return fold::fold_expr_macro(self, new_macro);
    }
    fn fold_item_mod(&mut self, i: ItemMod) -> ItemMod {
        if i.ident.to_string().starts_with("tests_llm_16")
            || i.ident.to_string().starts_with("tests_rug")
        {
            if let Some(tar) = &self.target_mod {
                if tar.eq(&i.ident.to_string()) {
                    self.in_mod = true;
                    let ans = fold::fold_item_mod(self, i);
                    self.in_mod = false;
                    return ans;
                }
                return fold::fold_item_mod(self, i);
            } else {
                println!("{}", i.ident.to_string());
                return fold::fold_item_mod(self, i);
            }
        } else {
            return fold::fold_item_mod(self, i);
        }
    }
    fn fold_item_fn(&mut self, i: ItemFn) -> ItemFn {
        self.cur_fn = i.sig.ident.to_string();
        if self.in_mod {
            self.clean();
            self.in_fn = true;
            let mut ans = fold::fold_item_fn(self, i);
            self.in_fn = false;
            let (t, st, ed) = self.generate_declarations();
            ans.block.stmts.splice(0..0, t);
            ans.block.stmts.insert(0, st);
            ans.block.stmts.push(ed);
            return ans;
        } else {
            return fold::fold_item_fn(self, i);
        }
    }
}
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if let Some(loc) = args.get(1) {
        let mut file = std::fs::File::open(loc)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let syntax_tree = syn::parse_file(&*contents).unwrap();
        let mut replacer = LiteralReplacer::new();
        if let Some(tar) = args.get(2) {
            replacer = LiteralReplacer::new_with_target(tar.clone());
            let n_tree = replacer.fold_file(syntax_tree);
            let formt = prettyplease::unparse(&n_tree);
            std::fs::write(loc, formt)?;
        } else {
            let _ = replacer.fold_file(syntax_tree);
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests_rug_1 {
    use super::*;
    use syn::Lit;
    #[test]
    fn test_lit_to_string() {
        let _rug_st_tests_rug_1_rrrruuuugggg_test_lit_to_string = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = "test_string";
        let rug_fuzz_2 = 0;
        let _rug_st_tests_rug_1_rrrruuuugggg_test_lit_to_string = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let p0: Lit = syn::Lit::Str(
            syn::LitStr::new(rug_fuzz_0, proc_macro2::Span::call_site()),
        );
        debug_assert_eq!(lit_to_string(& p0), "test_string".to_string());
        let _rug_ed_tests_rug_1_rrrruuuugggg_test_lit_to_string = rug_fuzz_2;
        let _rug_ed_tests_rug_1_rrrruuuugggg_test_lit_to_string = 0;
    }
}
#[cfg(test)]
mod tests_rug_2 {
    use super::*;
    use syn::Lit;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_2_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = "hello";
        let rug_fuzz_2 = "hello";
        let rug_fuzz_3 = 0;
        let _rug_st_tests_rug_2_rrrruuuugggg_test_rug = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let rug_fuzz_1 = rug_fuzz_2;
        let lit_str1 = syn::Lit::Str(
            syn::LitStr::new(rug_fuzz_0, proc_macro2::Span::call_site()),
        );
        let lit_str2 = syn::Lit::Str(
            syn::LitStr::new(rug_fuzz_1, proc_macro2::Span::call_site()),
        );
        debug_assert_eq!(crate ::are_lits_equal(& lit_str1, & lit_str2), true);
        let _rug_ed_tests_rug_2_rrrruuuugggg_test_rug = rug_fuzz_3;
        let _rug_ed_tests_rug_2_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    use std::io::Read;
    use std::fs;
    use syn;
    use prettyplease;
    use crate::LiteralReplacer;
    #[test]
    fn test_main() {
        let _rug_st_tests_rug_3_rrrruuuugggg_test_main = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = "test.rs";
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 0;
        let _rug_st_tests_rug_3_rrrruuuugggg_test_main = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let rug_fuzz_1 = rug_fuzz_2;
        let rug_fuzz_2 = rug_fuzz_3;
        let args: Vec<String> = vec![
            String::from(rug_fuzz_0), String::from("target_literal")
        ];
        let loc = args.get(rug_fuzz_1).unwrap();
        let mut file = std::fs::File::open(loc).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let syntax_tree = syn::parse_file(&*contents).unwrap();
        let mut replacer = LiteralReplacer::new();
        let tar = args.get(rug_fuzz_2).unwrap();
        replacer = LiteralReplacer::new_with_target(tar.clone());
        let n_tree = replacer.fold_file(syntax_tree);
        let formt = prettyplease::unparse(&n_tree);
        debug_assert_eq!((), main().unwrap());
        let _rug_ed_tests_rug_3_rrrruuuugggg_test_main = rug_fuzz_4;
        let _rug_ed_tests_rug_3_rrrruuuugggg_test_main = 0;
    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    use crate::LiteralReplacer;
    use syn::Lit;
    use syn::Ident;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_4_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let _rug_st_tests_rug_4_rrrruuuugggg_test_rug = rug_fuzz_0;
        LiteralReplacer::new();
        let _rug_ed_tests_rug_4_rrrruuuugggg_test_rug = rug_fuzz_1;
        let _rug_ed_tests_rug_4_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::LiteralReplacer;
    use syn::Lit;
    use syn::Ident;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_5_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = "target_module";
        let rug_fuzz_2 = 0;
        let _rug_st_tests_rug_5_rrrruuuugggg_test_rug = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let mut p0: String = rug_fuzz_0.to_string();
        LiteralReplacer::new_with_target(p0);
        let _rug_ed_tests_rug_5_rrrruuuugggg_test_rug = rug_fuzz_2;
        let _rug_ed_tests_rug_5_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    use crate::LiteralReplacer;
    #[test]
    fn test_clean() {
        let _rug_st_tests_rug_6_rrrruuuugggg_test_clean = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let _rug_st_tests_rug_6_rrrruuuugggg_test_clean = rug_fuzz_0;
        let mut p0 = LiteralReplacer::new();
        p0.clean();
        debug_assert!(p0.replacements.is_empty());
        debug_assert!(p0.primitives.is_empty());
        let _rug_ed_tests_rug_6_rrrruuuugggg_test_clean = rug_fuzz_1;
        let _rug_ed_tests_rug_6_rrrruuuugggg_test_clean = 0;
    }
}
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    use crate::LiteralReplacer;
    use syn::Stmt;
    use quote::quote;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_7_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = "sample_target_mod";
        let rug_fuzz_2 = 0;
        let _rug_st_tests_rug_7_rrrruuuugggg_test_rug = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let mut p0 = LiteralReplacer::new_with_target(String::from(rug_fuzz_0));
        let (ans1, start_comment, end_comment) = p0.generate_declarations();
        debug_assert_eq!(ans1.len(), 0);
        let _rug_ed_tests_rug_7_rrrruuuugggg_test_rug = rug_fuzz_2;
        let _rug_ed_tests_rug_7_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    use crate::LiteralReplacer;
    use syn::{ExprMacro, Expr};
    #[test]
    fn test_fold_expr_macro() {
        let _rug_st_tests_rug_11_rrrruuuugggg_test_fold_expr_macro = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = "sample_target_mod";
        let rug_fuzz_2 = 0;
        let _rug_st_tests_rug_11_rrrruuuugggg_test_fold_expr_macro = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let mut p0 = LiteralReplacer::new_with_target(String::from(rug_fuzz_0));
        let p1: ExprMacro = syn::parse_quote! {
            macro_rules! assert { ($cond : expr) => ({ if !$cond {
            panic!("Assertion failed: {:?}", stringify!($cond)); } }); }
        };
        <LiteralReplacer as syn::fold::Fold>::fold_expr_macro(&mut p0, p1);
        let _rug_ed_tests_rug_11_rrrruuuugggg_test_fold_expr_macro = rug_fuzz_2;
        let _rug_ed_tests_rug_11_rrrruuuugggg_test_fold_expr_macro = 0;
    }
}
#[cfg(test)]
mod tests_rug_13 {
    use super::*;
    use crate::LiteralReplacer;
    use syn::ItemFn;
    #[test]
    fn test_fold_item_fn() {
        let _rug_st_tests_rug_13_rrrruuuugggg_test_fold_item_fn = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = "sample_target_mod";
        let rug_fuzz_2 = 0;
        let _rug_st_tests_rug_13_rrrruuuugggg_test_fold_item_fn = rug_fuzz_0;
        let rug_fuzz_0 = rug_fuzz_1;
        let mut p0 = LiteralReplacer::new_with_target(String::from(rug_fuzz_0));
        let p1: ItemFn = syn::parse_quote! {
            fn example_function() {}
        };
        <LiteralReplacer as syn::fold::Fold>::fold_item_fn(&mut p0, p1);
        let _rug_ed_tests_rug_13_rrrruuuugggg_test_fold_item_fn = rug_fuzz_2;
        let _rug_ed_tests_rug_13_rrrruuuugggg_test_fold_item_fn = 0;
    }
}
