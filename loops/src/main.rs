fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // value to be returned after the loop ends
            break counter * 2;
            // no semicolon here since this is to be an expression
        }
    };
    println!("The result is: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // instead of breaking the innermost loop
                // break the labeled outer loop
                break 'counting_up;

                // this becomes an infinite loop, since the outer loop is never
                // broken
                // break;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut countdown = 3;
    while countdown != 0 {
        println!("number = {countdown}");
        countdown -= 1;
    }
    println!("BLASTOFF");

    // iterating directly through array elements
    let v = [1, 2, 3, 4, 5];
    for element in v {
        println!("The square of {} is {}", element, element * element);
    }

    // countdown using for and range
    for number in (1..4).rev() {
        println!("number = {number}")
    }
    println!("SECOND LAUNCH")
}
