//@author    Allergy
//@email     Allergy527@gmail.com
//@workspace PrecticeContest/cf_988_div3_g.rs
//@data      2024/11/17 23:41:14
#[macro_export]
macro_rules! cin {
    ()=>{{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }};
    ([$type:ty;$n:expr]) => {{
        let mut input = String::new();
        let mut s = loop{
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let s = input.trim();
            if !s.is_empty() {
                break s.split_whitespace();
            }
        };
        let mut arr = Vec::with_capacity($n as usize);
        for _ in 0..$n {
            arr.push((s.next().unwrap()).parse::<$type>().unwrap());
        }
        arr
    }};
    ($type:ty) => {{
        let mut input = String::new();
        let mut s = loop{
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let s = input.trim();
            if !s.is_empty() {
                break s.split_whitespace();
            }
        };
        (s.next().unwrap()).parse::<$type>().unwrap()
    }};

    ($($type:ty),*) => {{
        let mut input = String::new();
        let mut s = loop{
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let s = input.trim();
            if !s.is_empty() {
                break s.split_whitespace();
            }
        };
        ($(s.next().unwrap().parse::<$type>().unwrap()),*)
    }}
}
fn main() {
    let t = 1;
    //let t = cin!(i64);
    (0..t).for_each(|_| solve());
}
use std::collections::{HashMap, HashSet};

fn solve() {
    let n = cin!(usize);
    let a = cin!([usize; n]);
    const MOD: i64 = 998244353;
    let max_val = *a.iter().max().unwrap();

    // Step 1: 莫比乌斯函数预处理
    let mut mu = vec![1; max_val + 1];
    let mut is_prime = vec![true; max_val + 1];
    for i in 2..=max_val {
        if is_prime[i] {
            for j in (i..=max_val).step_by(i) {
                is_prime[j] = false;
                mu[j] *= -1;
            }
            for j in (i * i..=max_val).step_by(i * i) {
                mu[j] = 0; // 非平方自由数
            }
        }
    }

    // Step 2: 初始化路径计数器 count
    let mut count = vec![0; max_val + 1];
    let mut dp = vec![0; n];
    dp[0] = 1; // 初始条件：到达第一个城市的路径数为 1

    // Step 3: 计算 dp[i] 并更新 count 数组
    for i in 0..n {
        let val = a[i];
        let mut dp_i = 0;

        // 利用莫比乌斯函数计算 dp[i]
        for d in 1..=(val as f64).sqrt() as usize {
            if val % d == 0 {
                dp_i = (dp_i + mu[d] * count[d]) % MOD;
                if d != val / d {
                    dp_i = (dp_i + mu[val / d] * count[val / d]) % MOD;
                }
            }
        }
        dp_i = (dp_i + MOD) % MOD; // 防止负数出现
        dp[i] = dp_i;

        // 更新 count 数组
        for d in 1..=(val as f64).sqrt() as usize {
            if val % d == 0 {
                count[d] = (count[d] + dp[i]) % MOD;
                if d != val / d {
                    count[val / d] = (count[val / d] + dp[i]) % MOD;
                }
            }
        }
    }

    println!("{:?}", dp);
    println!("{}", dp[n - 1]);
}