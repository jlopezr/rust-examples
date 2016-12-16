use std::thread;

fn process() {
    let handles: Vec<_> = (0..10).map(|n| {
        thread::spawn(move || {
            println!("Thread start {}!",n);
            let mut _x = 0;
            for _ in 0..5_000_001 {
                _x += 1
            }
            println!("Thread end {}!",n);
        })
    }).collect();

    for h in handles {
        h.join().ok().expect("Could not join a thread!");
    }
}

fn main() {
    println!("Starting...");
    process();
    println!("Ending...");
}
