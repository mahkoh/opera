use crate::{PhantomNotSend, PhantomNotSync};
use std::mem;
use trybuild::TestCases;

fn assert_send<T: Send>() {}

fn assert_sync<T: Sync>() {}

#[test]
fn compile_fail() {
    let tc = TestCases::new();
    tc.compile_fail("compile-fail/not_send_01.rs");
    tc.compile_fail("compile-fail/not_sync_01.rs");
}

#[test]
fn marker_types() {
    assert_eq!(mem::size_of::<PhantomNotSend>(), 0);
    assert_eq!(mem::size_of::<PhantomNotSync>(), 0);

    assert_eq!(mem::align_of::<PhantomNotSend>(), 1);
    assert_eq!(mem::align_of::<PhantomNotSync>(), 1);
}

#[test]
fn phantom_not_send_sync() {
    assert_sync::<PhantomNotSend>();
}

#[test]
fn phantom_not_sync_send() {
    assert_send::<PhantomNotSync>();
}
