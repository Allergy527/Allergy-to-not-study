//@author    Allergy
//@email     Allergy527@gmail.com
//@workspace src\bin\p5707.rs
//@data      2023/11/30 18:37:18
//
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().split_whitespace();
    let (a, b): (i32, i32) = (
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
    );
    let res = if a % b > 0 { 1 } else { 0 };
    let mut go_time = 470 - a / b - res;
    if go_time < 0 {
        go_time += 1440;
    }
    if go_time % 60 < 10 {
        if go_time / 60 < 10 {
            println!("0{}:0{}", go_time / 60, go_time % 60);
        } else {
            println!("{}:0{}", go_time / 60, go_time % 60);
        }
    } else {
        if go_time / 60 < 10 {
            println!("0{}:{}", go_time / 60, go_time % 60);
        } else {
            println!("{}:{}", go_time / 60, go_time % 60);
        }
    }
}
