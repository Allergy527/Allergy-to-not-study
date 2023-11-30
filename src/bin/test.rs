#[macro_use]
macro_rules! scanf {
    ($($x:ident),*) => {
        $(
            let mut $x =String::new();
            std::io::stdin().read_line(&mut $x).unwrap();
        )*
    };
}

use std::io::stdin;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut nums = input.trim().split(' ');
    let a: i32 = nums.next().unwrap().parse().unwrap();
    let b: i32 = nums.next().unwrap().parse().unwrap();
    println!("{}", a + b);
    let a_1 = String::new();
    let a_2 = String::new();
    scanf!(a_1, a_2);
    println!("{a_1},{a_2}");
}
// fn scanf<T, U>(a: T, b: U) -> (T, U) {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     let mut input = input.trim().split_whitespace();
//     let (x, y): (T, U) = (
//         input.next().unwrap().parse().unwrap(),
//         input.next().unwrap().parse().unwrap(),
//     );
//     (x, y)
// }
