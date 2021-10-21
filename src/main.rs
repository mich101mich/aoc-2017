#![allow(unused_imports, clippy::while_let_on_iterator)]

#[macro_use]
mod utils;
mod days {
    pub mod day_23;
}
use days::day_23;

fn main() {
    day_23::run();
}
