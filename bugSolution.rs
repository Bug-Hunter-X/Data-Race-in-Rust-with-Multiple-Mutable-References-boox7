fn main() {
    let mut x = 5;
    { // create a new scope
        let y = &mut x; 
        *y = 6; 
    }
    let z = &mut x; 
    *z = 7; 
    println!("x = {}", x);
}
//Alternative Solution using clone
fn main() {
    let mut x = 5;
    let y = x.clone();
    let z = x.clone();
    let mut y1 = y;
    let mut z1 = z;
    y1 = 6;
    z1 = 7;
    println!("x = {}", x);
    println!("y = {}", y1);
    println!("z = {}", z1);
}