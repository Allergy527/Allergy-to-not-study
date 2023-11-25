use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace();
    let m: usize = input.next().unwrap().parse().unwrap();
    let n: i32 = input.next().unwrap().parse().unwrap();
    let mut qwq = [1; 10002];
    let mut count = 0;
    //下面是处理部分
    for _i in 0..=n - 1 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.split_whitespace();
        let left: usize = nums.next().unwrap().parse().unwrap();
        let right: usize = nums.next().unwrap().parse().unwrap();
        for j in left..=right {
            qwq[j] = 0;
        }
    }
    for i in 0..=m {
        if qwq[i] == 1 {
            count += 1;
        }
    }
    println!("{}", count);
}
