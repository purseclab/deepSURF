create_static_stor!(foo: Vec<u8>);

#[test]
fn test() {
    foo::with(|e| assert_eq!(e, &vec![]) );
    foo::with_mut(|e| e.push(1) );
    foo::with(|e| assert_eq!(e, &vec![1]) );
    foo::with_mut(|e| e.push(2) );
    foo::with(|e| assert_eq!(e, &vec![1,2]) );
    foo::with_mut(|e| e.push(4) );
    foo::with(|e| assert_eq!(e, &vec![1,2,4]) );
    foo::with_mut(|e| e.push(8) );
    foo::with(|e| assert_eq!(e, &vec![1,2,4,8]) );
}