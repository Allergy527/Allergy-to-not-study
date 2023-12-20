//@author    Allergy
//@email     Allergy527@gmail.com
//@workspace luogu/p1307.rs
//@data      2023/12/13 21:36:04
fn main() {
    let t = 1;
    //let mut input = String::new();std::io::stdin().read_line(&mut input).unwrap();let t:i32=input.parse().unwrap();
    let _ = (0..t).for_each(|_| solve());
}
fn solve() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().chars();
    let res = if input.clone().next() == Some('-' as char) {
        input.next();
        '-'.to_string()
    } else {
        String::new()
    };
    let ans: String = res
        .chars()
        .chain(input.rev().skip_while(|&x| x == '0'))
        .collect();
    println!("{}", if ans.len() > 0 { ans } else { 0.to_string() });
}
