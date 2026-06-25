use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // direct panic using macro
    // panic!("Crash and burn!");

    // code bug that causes panic
    // let v = vec![1, 2, 3];
    // v[99];

    // Result<T, E> returns either T on success or E on failure
    // let greeting_file_result = File::open("hello.txt");

    // unwrap method
    // let greeting_unwrap = File::open("hello2.txt").unwrap();

    // expect to write a custom panic message
    let greeting_expect =
        File::open("hello3.txt").expect("hello3.txt must be apart of this package");

    // closures to act on T and E differently
    // cleaner than the nested match expressions that would otherwise be needed
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            // create file if the error is that it does not exist
            File::create("hello.txt").unwrap_or_else(|error| {
                // abort if issues with creating the file, such as lack of permissions
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            // abort if file exists but cannot be opened (some other error)
            panic!("Problem opening the file: {error:?}");
        }
    });
}
