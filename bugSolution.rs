fn main() {
    let mut x = 5;
    { // Create a new scope for the mutable borrow
        let y = &mut x; 
        *y = 10; // Modifying x through y
    } // Mutable borrow goes out of scope here
    let z = &x; // Now it's safe to create a shared reference
    println!("x = {}", x); 
}
//Alternatively, if you need to modify and share simultaneously, consider cloning or using interior mutability (RefCell, Mutex).