
use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x: {}", x);
    x = 6;
    println!("The value of x: {}", x);

    // shadowing
    // using let to redeclare variable
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    println!("========= Floating point type ========");
    let x = 2.0;  // f64
    let y: f32 = 3.0; // f32
    println!("X: {}", x);
    println!("Y: {}", y);

    println!("========= Numeric Operations ========");
    // addition
    let sum = 5 + 10;

    // substraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;  // result in 0

    // remainder
    let remainder = 43 % 5;

    println!("Addition: {}", sum);
    println!("Substraction: {}", difference);
    println!("Multiplication: {}", product);
    println!("Division, quotient: {}, floored: {}", quotient, floored);
    println!("Remainder: {}", remainder);

    println!("========= Boolean type ========");
    let t = true;
    let f: bool = false;
    println!("True: {}", t);
    println!("False: {}", f);


    println!("========= Character type ========");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("common char: {}", c);
    println!("Uncommon char: {}", z);
    println!("Emoticon: {}", heart_eyed_cat);

    // Compound type

    // Tuple
    println!("========= Tuple type ========");
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuple value is: {:?}", tup);
    println!("THe value of y is: {}", y);

    // Array
    println!("========= Array type ========");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array a: {:?}", a);

    let a = [3, 5];
    println!("array a: {:?}", a);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first: {}, second: {}", first, second);

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read index.");
    
        let index: usize = index
            .trim().parse()
            .expect("Index entered was not number");
    
    let element = a[index];

    println!(
        "The value of elemet at index {} is : {}",
        index, element
    );


}