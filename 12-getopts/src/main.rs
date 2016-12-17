extern crate getopts;

use std::env;
use std::io;
use std::path::Path;
use std::io::Write;
use std::process;
use getopts::Options;

static VERSION: &'static str = "1.0.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[0]);
    let file = path.file_name();
    let program  = file.unwrap().to_str().unwrap();

    // Set possible flags.
    // The first argument to `optflag` is the short flag name.
    // The second argument is the long flag name.
    // The third argument is the help text.
    let mut opts = Options::new();
    opts.optflag("n", "", "do not output the trailing newline");
    opts.optflag("h", "help", "display this help and exit");
    opts.optflag("V", "version", "output version information and exit");


    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            process::exit(1);
            // The exit code is 0 (success) by default.
            // Any exit code other than 0 indicates failure.
        }
    };

    if matches.opt_present("help") {
        //^ We could as well have used the short name: "h"
        println!("{} {} - display a line of text", program, VERSION);
        println!("");
        println!("Usage:");
        println!(" {} [SHORT-OPTION]... [STRING]...", program);
        println!(" {} LONG-OPTION", program);
        println!("");
        let usage = opts.usage("Echo the STRING(s) to standard output.");
        println!("{}", usage);
        return;
    }

    if matches.opt_present("version") {
        println!("echo version: {}", VERSION);
        return;
    }

    if !matches.free.is_empty() {
        //^ `matches.free` contains all the arguments that are not options.
        let string = matches.free.join(" ");
        println!("{}", string);
    }

    if !matches.opt_present("n") {
        println!("")
    } else {
        let _ = io::stdout().flush();
    }
}
