//@author    Allergy
//@email     Allergy527@gmail.com
//@workspace Prectice_Contest/p5743.rs
//@data      2024/10/28 03:01:21
fn cin() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn main() {
    let t = 1;
    //let t = cin().parse::<i32>().unwrap();
    (0..t).for_each(|_| solve());
}
fn solve() {
    let n = cin().parse::<usize>().unwrap();
    println!("{}", (1..n).fold(1, |ans, _| (ans + 1) << 1))
}