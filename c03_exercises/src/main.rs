fn main() {
    let c = 97.0;
    let f = 100.0;

    println!("{c} Celsius is {:0.2} Fahrenheit", c_to_f(c));
    println!("{f} Fahrenheit is {:0.2} Celsius", f_to_c(f));

    let n = 100;

    println!("The {n}th fibonacci number is {}", fib(n));
    carol();
}

fn c_to_f(c: f32) -> f32 {
    (9.0 * c / 5.0) + 32.0
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}

fn fib(n: i128) -> i128 {
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    } else {
        let mut minus2 = 0;
        let mut minus1 = 1;
        let mut current = minus2 + minus1;

        for _idx in 1..n - 1 {
            minus2 = minus1;
            minus1 = current;
            current = minus2 + minus1;
        }
        return current;
    }
}

fn carol() {
    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves, and",
        "Three french hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese-a-laying,",
        "Seven swans-a-swimming,",
        "Eight maids-a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for n in 0..12 {
        println!("On the {} day of Christmas,", ordinals[n]);
        println!("My true love sent to me");
        for k in (0..n + 1).rev() {
            println!("{}", gifts[k]);
        }
        println!("")
    }
}
