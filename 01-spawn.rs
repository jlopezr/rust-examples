use std::io::println;

fn main() {
    for num in range(0, 10) {
        spawn(proc() {
            let greeting_message = "Hello ";
            println(greeting_message);
        });
    }
}
