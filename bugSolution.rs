fn main() {
    let mut v = vec![1, 2, 3];
    let v_clone = v.clone(); // Create a copy
    let ptr = v_clone.as_mut_ptr();
    unsafe {
        *ptr = 10;
    }
    println!("Value at the pointer: {}", *ptr);
    println!("Original vector: {:?}", v); 
    // Alternatively, use Vec::get_mut to avoid potential memory problems.
}