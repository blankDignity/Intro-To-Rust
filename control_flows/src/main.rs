fn main() {
    let (a, b, c) = (10, 20, 10);

    if a < b {
        println!("{} is less than {}", a, b);
    } else {
        println!("{} is less than {}", b, a);
    }

    if a < b && a == c {
        println!("a is less than b but equal to c");
    }

    let mut x = 0;

    let res = loop {
        x += 1;

        if x == 5 {
            break x * 3;
        }
    };

    println!("The value is {res}");

    let a = [1, 2, 3, 4, 5];

    for val in a {
        println!("{val}");
    }

    // = makes the ending value inclusive
    for val in (1..=10).rev() {
        println!("{val}");
    }
}

// lookup loop label for breaking out of specific nested loop
