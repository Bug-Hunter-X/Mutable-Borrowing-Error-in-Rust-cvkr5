fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; // Modifying x through y
    let z = &x; //Trying to create a shared reference to x 
    println!("x = {}", x); 
}