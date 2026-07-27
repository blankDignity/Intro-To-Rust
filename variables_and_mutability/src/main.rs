use std::io;

fn main() {
    // variables
    let mut a = 10;
    println!("The value is {a}");
    a = 20;
    println!("The value is {a}");
    const THREE_HOUSRES: u32 = 123;
    println!("{THREE_HOUSRES}");

    // Tuples
    let tup = (10.1, 10, 'c');

    let x = tup.0;
    let (k, l, m) = tup;
    println!("{k}, {l}, {m}");
    println!("{x}");

    let mut inp = String::new();

    io::stdin()
        .read_line(&mut inp)
        .expect("Enter a valid index:");

    // Arrays
    // array of random data and length
    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[0]);
    // array of type signed integer 32 of size 2
    let arr: [i32; 2] = [4, 30];
    println!("{}", arr[0]);
    // array with 3 2's
    let arr = [3; 2];
    println!("{}, {}", arr[0], arr[1]);

    let indx: usize = inp.trim().parse().expect("Index was not a number");

    println!("The value at index {indx} is {}", arr[indx])
}
