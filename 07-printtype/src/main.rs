//#![feature(core)]

fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            std::intrinsics::type_name::<T>()
        };
    println!("{}", type_name);
}

fn do_loop(v:&[u16]) -> () {
    println!("START");
    println!("LENGTH {}",v.len());
    for n in 0..v.len() {
        println!("a {}",v[n]);
    }
    for n in v.iter() {
        println!("b {}",n);
    }
    println!("END");
}

fn main() -> () {
    print_type_of(&32.90);           // prints "f64"
    print_type_of(&(vec!(1, 2, 4))); // prints "collections::vec::Vec<i32>"
    let a:[u8; 2] = [1,2];
    let mut b = [0u16; 10];

    for n in 0..b.len() {
        print!("*");
        b[n]=n as u16;
    }
    println!("");

    for j in 10..20 {
        println!("N {}",j);
    }

    print_type_of(&a);
    print_type_of(&b);
    let c = &b[0..5];
    print_type_of(&c);
    println!("{:?}",c);
    do_loop(&c);
}
