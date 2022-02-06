
fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisble by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("NUmber is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {}", number);

    repetition_with_loop();
    returning_value_from_loop();
    conditional_with_while();
    looping_through_collection_with_for();
    loop_using_in();
    loop_in_range();
}

fn repetition_with_loop() {
    let mut count = 0;
    'counting_up: loop {
         println!("Count = {}", count);
         let mut remaining = 10;

         loop {
             println!("remaining = {}", remaining);
             if remaining == 9 {
                 break;
             }
             if count == 2 {
                 break 'counting_up;
             }
             remaining -= 1;
         }
         count += 1;
    }
    println!("End count {}", count);
}

fn returning_value_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {}", result);
}

fn conditional_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn looping_through_collection_with_for() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is {}", a[index]);
        index += 1;
    }
}

fn loop_using_in() {
    let  a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {}", element);
    }
}

fn loop_in_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF");
}