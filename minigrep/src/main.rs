use minigrep::search;
use minigrep::search_case_insensitive;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    // anonymous function with single argument err, to be called when unwrap finds an
    // Err value
    // env::args() is an iterator, that is the input to Config::build
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // eprintln will not show in the stdout stream, which can be redirected to a file
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });

    // similar to above, but with nothing to do when run returns the unit ()
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}

// in the failure case, returns a type that implements the Error trait
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    // standard way of indicating that the function is not returning a meaningful value on success
    Ok(())
}
// these two variables are now grouped together into a struct
struct Config {
    query: String,
    file_path: String,
    // environment variable that can be set once per terminal session
    ignore_case: bool,
}

impl Config {
    // this fn is now an associated fn of Config
    // cleaner error messages with a Result enum instead of just calling panic
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // The first item in env::args is the name of the program, which is of no use
        args.next();

        // run through the iterator, keeping in mind it returns an Option<String>
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // use environment variable checking to assign the ignore_case boolean
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
