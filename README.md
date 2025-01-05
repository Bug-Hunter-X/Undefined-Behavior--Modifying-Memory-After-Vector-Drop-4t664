# Rust Undefined Behavior Example

This repository demonstrates a common error in Rust: undefined behavior resulting from modifying memory after the owning vector is dropped. The `bug.rs` file contains the erroneous code. The `bugSolution.rs` file provides a corrected version.

The bug arises from obtaining a raw pointer to the vector's data and then modifying the data after the vector goes out of scope. This leads to undefined behavior since the memory the pointer points to might be reallocated or reused. The solution avoids this by either cloning the vector before operating on the raw pointer or using approaches like `Vec::get_mut` which properly handles memory management.