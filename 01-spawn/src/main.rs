use std::thread;

fn main() {
    for num in 0..10 {
        thread::spawn(move || {
            let greeting_message = "Hello World";
            println!("{} {}",greeting_message, num);
        });
    }
}
