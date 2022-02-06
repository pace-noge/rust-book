
fn main() {
    println!("Hello, World");

    another_function();
    function_with_parameters(5);
    multiple_parameters(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {:?}", y);
    
    let x = return_value();
    println!("Value of x: {}", x);

    let x = plus_one(5);
    println!("the value of x is : {}", x);
}

fn another_function() {
    println!("Another function");
}

fn function_with_parameters(x: i32) {
    println!("the value of x: {}", x);
}

fn multiple_parameters(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn return_value() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}