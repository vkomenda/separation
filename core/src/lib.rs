use std::borrow::BorrowMut;

pub type HelloWorldArray = [u8; 11];

pub fn say_hello_world(arr: &mut [u8; 11]) {
    let hw = b"hello world";
    let slice: &mut [u8] = arr.borrow_mut();
    slice.copy_from_slice(&hw[..]);
}
