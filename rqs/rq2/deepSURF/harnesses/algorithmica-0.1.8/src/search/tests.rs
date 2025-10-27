use crate::search::binary;
use std::cmp::{Ordering, PartialEq, PartialOrd};

#[derive(Debug, Clone)]
struct Employee {
    id: i32,
    name: String,
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Employee) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Employee {
    fn eq(&self, other: &Employee) -> bool {
        self.id == other.id
    }
}

impl Ord for Employee {
    fn cmp(&self, other: &Employee) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Eq for Employee {}

#[test]
fn test_int() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7];
    assert_eq!(false, binary::search(&arr, &12));
    assert_eq!(true, binary::search(&arr, &1));
}

#[test]
fn test_str() {
    let arr = vec!["a", "b", "c", "d", "e", "f"];
    assert_eq!(true, binary::search(&arr, &"f"));
    assert_eq!(false, binary::search(&arr, &"g"));
}

#[test]
fn test_struct() {
    let arr = vec![
        Employee {
            id: 1,
            name: "Abrar".to_string(),
        },
        Employee {
            id: 2,
            name: "Abrar".to_string(),
        },
        Employee {
            id: 3,
            name: "Abrar".to_string(),
        },
        Employee {
            id: 4,
            name: "Abrar".to_string(),
        },
        Employee {
            id: 6,
            name: "Abrar".to_string(),
        },
        Employee {
            id: 10,
            name: "Abrar".to_string(),
        },
    ];
    assert_eq!(
        true,
        binary::search(
            &arr,
            &Employee {
                id: 6,
                name: "Abrar".to_string(),
            }
        )
    );

    assert_eq!(
        true,
        binary::search(
            &arr,
            &Employee {
                id: 10,
                name: "Abrar".to_string(),
            }
        )
    );

    assert_eq!(
        false,
        binary::search(
            &arr,
            &Employee {
                id: 0,
                name: "Abrar".to_string(),
            }
        )
    );
}
