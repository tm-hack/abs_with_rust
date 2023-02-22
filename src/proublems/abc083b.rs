use proconio::input;

pub fn main() {
    input! {
        input: [i32; 3],
    }

    let n = input[0];
    let a = input[1];
    let b = input[2];

    let mut res = 0;

    for n in 0..n + 1 {
        let n_string = n.to_string();
        let n_iter = n_string.chars().map(|n| n.to_digit(10).unwrap());
        let n_sum: i32 = n_iter.fold(0, |sum, n| sum + n as i32);

        if n_sum >= a && n_sum <= b {
            res = res + n;
        }
    }

    println!("{}", res);
}
