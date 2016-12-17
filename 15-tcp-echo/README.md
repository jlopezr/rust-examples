# A TCP echo example in Rust

## Setup

Install Rust and netcat

## Usage

The program uses env_logger so requires setting the RUST_LOG environment variable. Open the terminal and type:

    env RUST_LOG=info cargo run

Then in another terminal type:

    echo -n 'Hello World' | nc 127.0.0.1 8080

After entering the above JSON string, press `enter`. That will send the text
over the wire to our example program. Our example program will echo that to you.

## License

MIT
