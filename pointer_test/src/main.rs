fn main() {
    let x = 1;
    println!("{}", hoge(x));

    let mut y: i32 = 2;
    println!("{}", fuga(&mut y));
}

fn hoge(x: i32) -> i32 {
    x
}

fn fuga(y: &mut i32) -> i32 {
    *y += 1;
    *y
}
