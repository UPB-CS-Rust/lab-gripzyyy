fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn smallest(a: i32, b: i32) -> i32 {
    if a > b {
        b
    } else {
        a
    }
}

fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut max = input[0];
    let mut min = input[0];

    for i in input {
        let x: i32 = i as i32;

        max = bigger(max, x);
        min = smallest(min, x);
    }

    println!("{} is largest and {} is smallest", max, min);
}
