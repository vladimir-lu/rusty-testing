// Vanilla rust tests

#[test]
fn it_works() {
    assert!(true);
}

#[test]
#[should_panic]
fn it_shouldnt_work() {
    panic!("We failed, good job!");
}

#[test]
fn assertions_rock() {
    assert_eq!(1u8, 0x01);
    assert_eq!("hello", concat!("he", "llo"));
}

