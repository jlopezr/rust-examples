use std::io::println;

fn main() {
    for num in range(0, 10) {
        spawn(proc() {
            let mut greeting_message = ~"Hello " + num.to_str();
            println(greeting_message);
            greeting_message = ~"pepito";
            println(greeting_message);
        });
    }
}
