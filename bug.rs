fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 10;
    }
    println!("Value at the pointer: {}", *ptr);
    // This will cause undefined behavior because v is dropped and it will modify memory that is no longer valid. 
}