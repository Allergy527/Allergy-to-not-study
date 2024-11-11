//@author    Allergy
//@email     Allergy527@gmail.com
//@workspace Prectice_Contest/test.rs
//@data      2024/11/10 22:47:37
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

struct TreeList {
    data: Vec<i64>,
    n: usize,
}

impl TreeList {
    fn new(size: usize) -> Self {
        TreeList {
            data: vec![0; size + 1],
            n: size,
        }
    }

    fn add(&mut self, idx: usize, value: i64) {
        let mut i = idx as isize;
        while i <= self.n as isize {
            self.data[i as usize] += value;
            i += i & -i;
        }
    }

    fn sum(&self, idx: usize) -> i64 {
        let mut i = idx as isize;
        let mut result = 0;
        while i > 0 {
            result += self.data[i as usize];
            i -= i & -i;
        }
        result
    }

    fn range_add(&mut self, left: usize, right: usize, value: i64) {
        self.add(left, value);
        self.add(right + 1, -value);
    }

    fn range_sum(&self, left: usize, right: usize) -> i64 {
        self.sum(right) - self.sum(left - 1)
    }
}

fn solve() {
    let (n, m) = cin!(usize, i64);
    let arr = cin!([i64; n]);
    let mut qwq = TreeList::new(n);
    for i in 0..n {
        qwq.add(i + 1, arr[i]);
    }
    for i in 0..m {
        let (op, a, b) = cin!(i64, i64, i64);
        if op == 1 {
            qwq.add(a as usize, b);
        } else {
            println!("{}", qwq.range_sum(a as usize, b as usize));
        }
    }
}
