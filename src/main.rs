mod commands;

use clap::{command, Arg};
use commands::cd;

fn main() {

    cd::execute_cd();
}