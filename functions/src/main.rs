fn main() {
    println!("Hello, world!");
    another_function(5);

    let x = {
        let y = 3;
        // note that the return won't work here
        y + 1
    };

    println!("{x}");
    println!("Function five() returns: {}", five());
}

// don't add semi-colon to make it an expression, and it will return as such
// or just do return;

fn five() -> i32 {
    4 + 1
    // or just
    // 5
}

fn another_function(x: i32) {
    println!("Another function: {x}");
}
