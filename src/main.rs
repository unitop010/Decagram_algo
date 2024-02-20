use std::io;
// mod utils;

fn main() {
    println!("Enter the value of s: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s: i32 = input.trim().parse().expect("Please enter a valid integer");
    let a = 10;
    let b = -10;
    let c = 1 - s;

    let a_f64 = a as f64;
    let b_f64 = b as f64;
    let c_f64 = c as f64;

    let discriminant = b_f64 * b_f64 - 4.0 * a_f64 * c_f64;
    if discriminant < 0.0 {
        println!("No real solutions");
    } else {
        let n = (-b_f64 + discriminant.sqrt()) / (2.0 * a_f64);
        println!("Decagrams({})", n);
    }
}