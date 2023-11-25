use std::io;
fn main() {
    let mut input: String = String::new();
    let mut input2: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let array: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s: &str| s.parse().unwrap())
        .collect();
    io::stdin().read_line(&mut input2).unwrap();
    let capabl: i32 = input2.trim().parse::<i32>().expect("crazy!!");
    let mut sum: i32 = 0;
    for num in array {
        if capabl + 30 >= num {
            sum += 1;
        }
    }
    println!("{}", sum);
}
