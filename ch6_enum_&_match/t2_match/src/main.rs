use rand::Rng;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // coin
    let coin = Coin::Quarter;
    println!("The value of the coin is {} cents", value_in_cents(coin));

    // Addition of two numbers
    let num = 5;
    let num2 = Some(5);
    println!("The value of the addition is {} ", add_num(num, num2));

    // Dice roll
    let num = rand::rng().random_range(1..=10);
    dice_roll(num);
}




fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }


}

fn add_num(num: i32, num2: Option<i32>) -> i32 {
    match num2 {
        Some(i) => i + num,
        None => num,
    }
}


fn dice_roll(num: i32) {
    match num {
        1 => println!("You rolled a 1"),
        2 => println!("You rolled a 2"),
        3 => println!("You rolled a 3"),
        4 => println!("You rolled a 4"),
        5 => println!("You rolled a 5"),
        6 => println!("You rolled a 6"),
        other => println!("You rolled a {}", other),
    }
}
