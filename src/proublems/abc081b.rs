use proconio::input;

pub fn main() {
    input! {
        n: i32,
        aa: [i32;n],
    }

    let mut cnt = 1;

    loop {
        let mut break_flag = false;
        for a in &aa {
            if a % (2_i32.pow(cnt)) != 0 {
                break_flag = true;
            }
        }
        if break_flag {
            break;
        };

        cnt = cnt + 1;
    }

    println!("{}", cnt - 1);
}
