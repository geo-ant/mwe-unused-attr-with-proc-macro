pub use mwe_macros::dummy;

#[allow(clippy::eq_op)]

#[test]
fn test1() {
    assert_eq!(true,true);
}

// this is fine
#[dummy]
#[test]
#[should_panic]
fn test2() {
    assert_eq!(true,false);
}

// this produces a warning / clippy error
#[test]
#[dummy]
#[should_panic]
fn test3() {
    assert_eq!(true,false);
}

// this produces a warning / clippy error
#[test]
#[should_panic]
#[dummy]
fn test4() {
    assert_eq!(true,false);
}