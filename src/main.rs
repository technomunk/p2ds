//! To be able to build the project for web-assembly it is structured like a library.

mod lib;
use lib::game;

fn main() {
    game();
}
