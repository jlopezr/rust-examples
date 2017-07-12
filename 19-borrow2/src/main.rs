// http://pro.theta.eu.org/2016/03/12/learn-you-a-rust-for-great-good.html
// http://pro.theta.eu.org/2016/03/18/lyar-borrows.html
// http://pro.theta.eu.org/2016/04/16/lyar-lifetimes.html
// http://pro.theta.eu.org/2016/07/11/lyar-lifetime-fenangling.html

fn main() {
    main1();
    println!("-------------");
    main2();
    println!("-------------");
    main3();
    println!("-------------");
    main4();
    println!("-------------");
    main5();
    println!("-------------");
    main6();
    println!("-------------");
    main7();
    println!("-------------");
    main8();
    println!("-------------");
    main9();
    println!("-------------");
    main10();
    println!("-------------");
    main11();
    println!("-------------");
}

#[derive(Debug)]
struct ImportantData(u32, u32);

fn do_stuff_with_data(a: ImportantData, b: ImportantData, c: ImportantData) -> ImportantData {
    let ImportantData(a1,a2) = a;
    let ImportantData(b1,b2) = b;
    let ImportantData(c1,c2) = c;
    ImportantData(a1+b1+c1,a2+b2+c2)
}
fn main1() {
    let (a, b, c) = (ImportantData(0,1), ImportantData(1,2), ImportantData(2,3));
    let result = do_stuff_with_data(a, b, c);
    println!("{:?}", result);
}

fn do_stuff_with_data_2(a: &ImportantData, b: &ImportantData, c: &mut ImportantData) -> ImportantData {
    let &ImportantData(a1,a2) = a;
    let &ImportantData(b1,b2) = b;
    let &mut ImportantData(c1,c2) = c;
    c.0 = 6;
    c.1 = 9;
    ImportantData(a1+b1+c1,a2+b2+c2)
}

fn main2() {
    let (a, b) = (ImportantData(0,1), ImportantData(1,2));
    let mut c = ImportantData(2,3);
    let result = do_stuff_with_data_2(&a, &b, &mut c);
    println!("{:?}", result);
    println!("{:?}", a);
    println!("{:?}", c);
}

fn add_4_to_vec(vec: &mut Vec<i32>) { /* `Vec` is an 'array' type, short for 'vector' */
    vec.push(4);
}
fn main3() {
    let mut my_little_vector: Vec<i32> = vec![1, 2, 3];
    add_4_to_vec(&mut my_little_vector);
    println!("{:?}", my_little_vector);
}

#[derive(Debug, Copy, Clone)]
struct Foo;

fn main4() {
    let x = Foo;
    let y = x;
    // `y` is a copy of `x`
    println!("{:?}", x);
    println!("{:?}", y);
}

fn main5() {
    let mut x = 5;

    {
        let y = &mut x;
        *y += 1; /* magic dereferencing asterisk modifying `x` */
    }

    println!("{}", x);
}

struct Data(i32);

fn main6() {
    let vec2: Vec<Data> = vec![Data(0), Data(1), Data(2)];
    let mut vec1: Vec<&Data> = Vec::new();

    for data in &vec2 {
        vec1.push(data);
    }
}

fn main7() {
    let vec2: Vec<Data> = vec![Data(0), Data(1), Data(2)];
    let mut vec1: Vec<&Data> = Vec::new();

    for ref data in &vec2 {
        vec1.push(data);
    }
}

struct Object {
    number: u32
}

struct Multiplier<'a> {
    object: &'a Object,
    mult: u32
}

fn print_borrower_number(mu: Multiplier) {
    println!("Result: {}", mu.object.number * mu.mult);
}

fn main8() {
    let obj = Object { number: 5 };
    let obj_times_3 = Multiplier { object: &obj, mult: 3 };
    print_borrower_number(obj_times_3);
}

fn object_combinator<'a>(a: &'a mut Object, b: &Object) -> &'a mut Object {
    a.number = a.number + b.number;
    a
}

fn main9() {
   let mut a = Object { number: 3 };
   let b = Object { number: 4 };
   println!("Result: {}", object_combinator(&mut a, &b).number);
}

// This is an object which owns a borrow to something,
// rather like our `Multiplier` from before.
// You can ignore the strange <'x> notation for now -
// don't worry, I'll cover it later.
struct RefObject<'x>(&'x u32);

fn steal_a_var<'x>(o: RefObject<'x>) {
    println!("{}", o.0);
}

fn main10() {
    // a is created in main()s scope
    let a = 3;

    // b is created in main()s scope
    let mut b = &a;

    // c is created in main()s scope
    let c = RefObject(&b);

    // c is moved out of main()s scope.
    // c now lives as long as steal_a_var()s scope.
    steal_a_var(c);

    // steal_a_var()s scope ends, killing all the variables inside it...
    // c goes away

    // d is created in main()s scope
    let _d = &mut b;

}
// main()s scope ends, killing all the variables inside it...
// d goes away, as it was declared last
// b goes away, as it was declared second-last
// a goes away, as it was declared third-last

struct Firework {
    strength: i32
}

impl Drop for Firework {
    fn drop(&mut self) {
        println!("BOOM times {}!!!", self.strength);
    }
}
struct FireworkShow<'a> {
    start: Option<&'a Firework>,
    middle: Option<&'a Firework>,
    finale: Option<&'a Firework>
}
impl<'a> Drop for FireworkShow<'a> {
    fn drop(&mut self) {
        println!("Welcome ladies and gentlemen! Now, without further ado...");
        ::std::mem::drop(self.start);
        println!("Now, for our second item...");
        ::std::mem::drop(self.middle);
        println!("And now, for the grand finale...!");
        ::std::mem::drop(self.finale);
    }
}
fn main11() {
    let tnt = Firework { strength: 100 };
    let firecracker = Firework { strength: 1 };
    let mut show = FireworkShow { start: None, middle: None, finale: None };
    show.start = Some(&firecracker);
    show.finale = Some(&tnt);
}
