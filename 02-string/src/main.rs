fn main() {
    let s = "Hello there!";
    println!("{}",s);
    let mut s2 = "Hello".to_string();
    println!("{}",s2);
    s2.push_str(" world!");
    println!("{}",s2);
    let x = 100;
    let y = 200;
    let z = sum(x,y);
    println!("{}",z);
}

fn sum(a:u32, b:u32)->u32 {
    return a+b;
}
