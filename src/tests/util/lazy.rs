use std::sync::{Arc, Mutex};
use util::lazy::Lazy;

#[test]
fn test_simple_evaluation() {
    let v = Lazy::new(|| 42);
    assert_eq!(42, *v);
}

#[test]
fn test_single_evaluation() {
    let c = Arc::new(Mutex::new(0));
    let counter = c.clone();

    let v = Lazy::new(|| {
        let mut counter = c.lock().unwrap();
        *counter += 1;
    });

    *v;
    *v;
    assert_eq!(1, *counter.lock().unwrap());
}

#[test]
fn test_no_evaluation() {
    let c = Arc::new(Mutex::new(0));
    let counter = c.clone();

    let _v = Lazy::new(|| {
        let mut counter = c.lock().unwrap();
        *counter += 1;
    });

    assert_eq!(0, *counter.lock().unwrap());
}

#[test]
fn test_into_inner() {
    let v = Lazy::new(|| 42);
    assert_eq!(42, v.into_inner());
}
