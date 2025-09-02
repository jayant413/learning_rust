/**
 *  Statement & Expression
 *  Statement is a piece of code that does not return a value.
 *  Expression is a piece of code that returns a value.
 */

fn main() {
    // even_fn for if else statement
    even_fn();

    // for_loop for for loop
    for_loop();

    // loop label
    loop_label();
}

// if else statement
fn even_fn() {
    let mut number = String::new();
    println!("Enter a number: ");
    std::io::stdin().read_line(&mut number).unwrap();

    let number = number.trim().parse::<i32>().unwrap();

    let is_even = is_even(number);  

    let result = if is_even { "even" } else { "odd" };

    println!("result: {result}");

    
}

// is_even for if else statement
fn is_even(number: i32) -> bool {
    if number % 2 == 0 {
       return true;
    }

    false
}


// for loop
fn for_loop() {
    for i in 1..10 {
        println!("i: {i}");
    }
}



// loop label 
fn loop_label() {
    let mut count = 0;
    'outer:loop {
        count += 1;
        println!("count: {count}");
        loop {
            count += 1;
            println!("count: {count}");
            if count == 10 {
                break 'outer;
            }
        }
    }
}