//@author    Allergy
//@email     Allergy527@gmail.com
//@workspace src\bin\b2090.rs
//@data      2023/11/30 18:57:39
//
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().split_whitespace();
    let (a, b): (f64, f64) = (
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
    );
    let capable = 3.14 * b * b * a;
    println!("{:.0}", (20000.0 / capable).ceil());
}
