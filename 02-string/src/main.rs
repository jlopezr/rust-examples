use std::thread;

fn main() {
    for num in 0..10 {
        thread::spawn(move || {
            let mut greeting_message = "Hello " + num.to_string();
            println!("{}", greeting_message);
            greeting_message = "pepito";
            println!("{}", greeting_message);
        });
    }
}
