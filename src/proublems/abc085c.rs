use proconio::input;

pub fn main() {
    input! {
        ny: [i32;2],
    }

    let n = ny[0];
    let y = ny[1];

    let mut mai_10000 = -1;
    let mut mai_5000 = -1;
    let mut mai_1000 = -1;

    for i in 0..n + 1 {
        for j in 0..n + 1 - i {
            let sum = 10000 * i + 5000 * j + 1000 * (n - i - j);

            if y == sum {
                mai_10000 = i;
                mai_5000 = j;
                mai_1000 = n - i - j;
                break;
            }
        }
    }

    println!("{} {} {}", mai_10000, mai_5000, mai_1000);
}
