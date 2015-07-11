fn main() {
    let plus_one = |x: i32| x + 1;
    let func = plus_one;
    println!("{}",func(1));
}
