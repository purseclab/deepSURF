#[macro_use]
extern crate afl;
extern crate algorithmica;

fn test_function1() {
    let _local0: algorithmica::tree::bst::BST<String> = algorithmica::tree::bst::BST::new();
    algorithmica::tree::bst::BST::is_empty(&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function1();
    });
}
