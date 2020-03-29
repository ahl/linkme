use linkme::distributed_slice;

#[distributed_slice]
pub static SLICE1: [fn()] = [..];

#[distributed_slice(SLICE1)]
fn foo() {}

#[distributed_slice]
pub static SLICE2: [for<'a, 'b> fn(&'a &'b ())] = [..];

#[distributed_slice(SLICE2)]
fn bar<'a, 'b>(_: &'a &'b ()) {}

#[distributed_slice]
pub static SLICE3: [unsafe extern "C" fn() -> i32] = [..];

#[distributed_slice(SLICE3)]
unsafe extern "C" fn baz() -> i32 {
    42
}

#[test]
fn test_slices() {
    assert!(!SLICE1.is_empty());
    assert!(!SLICE2.is_empty());
    assert!(!SLICE3.is_empty());
}