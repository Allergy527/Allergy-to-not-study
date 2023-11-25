use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let array: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s: &str| s.parse().unwrap())
        .collect();
    io::stdin().read_line(&mut input).unwrap();
    let capabl: i32 = input.trim().parse().unwrap();
    let mut sum: i32 = 0;
    for num in array {
        if capabl + 30 <= num {
            sum += 1;
        }
    }
    println!("{}", sum);
}
