use proconio::input;

pub fn main() {
    input! {
        n: i32,
        mut aa: [i32;n],
    }

    aa.sort();
    let mut sum_a = 0;
    let mut sum_b = 0;

    loop {
        if aa.is_empty() {
            break;
        }
        sum_a = sum_a + aa.pop().unwrap();

        if aa.is_empty() {
            break;
        }
        sum_b = sum_b + aa.pop().unwrap();
    }

    println!("{}", sum_a - sum_b);
}
