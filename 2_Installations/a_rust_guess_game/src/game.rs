use crate::input;
use crate::logic;
use crate::random;
use colored::*;

pub fn set_dificulty() {
    println!(
        "|==================> Please Select The Level {} vs {} <==================|",
        "'Easy'".green(),
        "'Hard'".red()
    );
    let level: logic::Dificulty;
    let level = input::read_dificulty();
}

pub fn start() {
    set_dificulty();
    let secret = random::get_secret();
    loop {
        let guess = input::read_new_guess();
        match logic::compaire(secret, guess) {}
    }
}
