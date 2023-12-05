//@author    Allergy
//@email     Allergy527@gmail.com
//@workspace src\bin\p5710.rs
//@data      2023/12/04 16:33:33
fn main() {
    let t = 1;
    //let mut input = String::new();std::io::stdin().read_line(&mut input).unwrap();let t:i32=input.parse().unwrap();
    for _ in 0..t {
        solve()
    }
}
fn solve() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let x: i32 = input.trim().parse().unwrap();
    match (x % 2 == 0, x > 4 && x <= 12) {
        (true, true) => println!("{} {} {} {}", 1, 1, 0, 0),
        (true, false) => println!("{} {} {} {}", 0, 1, 1, 0),
        (false, true) => println!("{} {} {} {}", 0, 1, 1, 0),
        (false, false) => println!("{} {} {} {}", 0, 0, 0, 1),
    }
}
