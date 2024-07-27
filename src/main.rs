use clap::{command, Arg};

fn main() {

    // Allows you to build the Command instance from your Cargo.toml at compile time.
    let match_result = command!().arg(
        Arg::new("firstname").short('f').long("firstname")
    )
    .arg(
        Arg::new("lastname").short('l')
    )
    .arg(
        Arg::new("fluffy").short('F')
    )
    .get_matches();
}