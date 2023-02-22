use proconio::input;

pub fn main() {
    input! {
        n: i32,
        mut dd: [[i32;1]; n],
    }

    let mut dd = dd.concat();
    dd.sort();
    dd.reverse();

    let mut x = 0;
    let mut cnt = 0;

    loop {
        if dd.is_empty() {
            break;
        }
        let y = dd.pop().unwrap();

        if x != y {
            cnt = cnt + 1;
        }

        x = y;
    }

    println!("{}", cnt);
}
