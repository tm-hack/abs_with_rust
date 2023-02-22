use proconio::input;

pub fn main() {
    input! {
        n: i32,
        txy: [[i32;3];n],
    }

    let mut t_0 = 0;
    let mut x_0 = 0;
    let mut y_0 = 0;
    let mut res = "Yes";

    for txy_i in txy {
        let (t_1, x_1, y_1) = (txy_i[0], txy_i[1], txy_i[2]);

        let elapsed_time = t_1 - t_0;
        let distance = (x_1 - x_0).abs() + (y_1 - y_0).abs();

        if (elapsed_time - distance < 0) || ((elapsed_time - distance) % 2 != 0) {
            res = "No";
            break;
        }

        t_0 = t_1;
        x_0 = x_1;
        y_0 = y_1;
    }

    println!("{}", res);
}
