use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("|=================== Guseesing Game ==========================|");
    game_loop_start();
    println!("|=================== End of the Game ==========================|");
}

fn secret_number() -> u32 {
    // Y: in this function we are returning only and only u32 value for that value to return we are not even adding variable.
    // G: In rust anythign which chanses its internel state must be mutable. RNG changes its internel state every time its called.

    let mut rng = rand::rng();
    rng.random_range(1..=1000)
}

// Y: This function get stdin from user.
fn guess_input() -> String {
    println!("| Guess The Number |");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("⛔Falure to read line !!!");
    guess
}

fn check_guess_number(n: String) -> u32 {
    let n: u32 = match n.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => 0,
    };
    n
}

enum GuessResualt {
    GuessIsSmaller,
    GuessIsGreater,
    GuessIsEqual,
}
// Y: work on this part now
struct RangeOfNumbers {
    lower_bound: u32,
    upper_bound: u32,
}

fn update_after_guess() {}
fn display_range() {}
fn is_within_range() {}

fn match_guess(sec_num: u32, guess: u32) -> GuessResualt {
    match guess.cmp(&sec_num) {
        Ordering::Less => GuessResualt::GuessIsSmaller,
        Ordering::Greater => GuessResualt::GuessIsGreater,
        Ordering::Equal => GuessResualt::GuessIsEqual,
    }
}

fn game_loop_start() {
    let sec_num = secret_number();
    let range: RangeOfNumbers = RangeOfNumbers {
        lower_bound: 1,
        upper_bound: 1000,
    }; // Y : Decleared : range -> we will lower it for more accurate op.

    loop {
        let mut guess_num = check_guess_number(guess_input());
        if (guess_num == 0) {
            println!("🚫 Only Numbers between 1-1000 are allowed !!!");
            continue;
        }
        // RangeOfNumbers.lower_bound = 1;
        // RangeOfNumbers.upper_bound = 1000;

        match match_guess(sec_num, guess_num) {
            GuessResualt::GuessIsSmaller => smaller_guess(guess_num, &mut RangeOfNumbers),
            GuessResualt::GuessIsGreater => greater_guess(guess_num, &mut RangeOfNumbers),
            GuessResualt::GuessIsEqual => {
                additional_functionality_additioon();
                break;
            }
        }
    }
}

fn smaller_guess(n: u32, range: RangeOfNumbers) {
    let range = Point { lower_bound: n };
    println!(
        "{}, {}",
        "Your guess is too small".red(),
        "Please Input Higher Numbers"
    );
}
fn greater_guess(n: u32, range: RangeOfNumbers) {
    let range = Point { upper_bound: n };
    println!(
        "{}, {}",
        "Your guess is too big".red(),
        "Please Input Lower Numbers"
    );
}

fn additional_functionality_additioon() {
    println!("{}", "You Won !!! 💛💙🩶🩷🤍💖💛💙🩶🩷🤍💖".green());
    println!("next functionallitye is to add :
========================================================================================================
1. everyt time uer inout we are allready checking if it big or small , now we have to check how close the users guess is.
2. we will be needing 2 different mutable variables, a. for closest smaller guess and b. closest biger guess.
3. each time the guess is compaired with the exciting one guess if the guess is closer the newer will replace the older one.
4. and with the output we will be showing the result to choose in the newer 'shringked' range  with the help of a and b.
5. with this method we can increase the guess range drastically and add levels like easy, modaret, hard, and assested game or unassisted.
6. add what every you want to add just do it fast.
")
}
