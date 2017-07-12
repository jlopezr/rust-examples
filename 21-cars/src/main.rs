// https://mobiarch.wordpress.com/2015/07/08/understanding-lifetime-in-rust-part-ii-3/

#[derive(Debug)]
struct Car {
    model : String
}

// car has the same lifetime as the Person that holds it
#[derive(Debug)]
struct Person<'a> {
    car:Option<&'a Car>
}

impl <'a> Person<'a> {
    fn new() -> Person<'a> {
        Person{
            car: None
        }
    }

    fn buy_car(&mut self, c : &'a Car) {
        self.car = Some(c);
    }

    fn sell_car(&mut self) {
        self.car = None;
    }

    fn trade_with(&mut self, other : &mut Person<'a>) {
        let tmp = other.car;

        other.car = self.car;
        self.car = tmp;
    }
}

fn main() {
        let civic = Car{model: "Honda Civic".to_string()};
        let ghibli = Car{model: "Maserati Ghibli".to_string()};

       let mut bob = Person::new();
       let mut alice = Person::new();

       bob.buy_car(&civic);
       alice.buy_car(&ghibli);

       bob.trade_with(&mut alice);

    println!("{:?}", bob);
    println!("{:?}", alice);
}
