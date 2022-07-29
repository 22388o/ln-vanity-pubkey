extern crate core;

use std::time::Instant;
use crate::derivation::node_keys2;
use crate::multithreading::{guess_pubkey_threaded};
use clap::{arg, command, value_parser};

mod derivation;
mod multithreading;


fn main() {
    let num_cores = num_cpus::get();

    let matches = command!() // requires `cargo` feature
        .about("Guesses millions of combinations to find a Lightning private key so the public key starts with the right prefix.")
        .arg(
            arg!([prefix] "Prefix in HEX.")
        )

        .arg(
            arg!(
                -t --threads <NUMBER> "Set the number of threads used. Default is the number of cores of the machine."
            )
                .required(false)
                .default_value(&num_cores.to_string())
                .value_parser(value_parser!(u8)),
        ).get_matches();

    let prefix = matches.get_one::<String>("prefix").expect("No prefix provided.");
    let thread_count = matches.get_one::<u8>("threads").expect("No thread provided.");

    println!("Start guessing pubkey with prefix {}.", prefix);
    println!("Use {} threads", thread_count);

    let start = Instant::now();
    let res = guess_pubkey_threaded(prefix, thread_count.clone());
    let duration = start.elapsed();

    match res {
        Some(guess_result) => {
            println!("Guessing took {duration:?}, {} guesses", guess_result.guesses);
            println!("{} guesses per second", guess_result.guesses/(duration.as_secs() as u128));
            let mnemonic = guess_result.mnemonic.expect("No mnemonic found.");
            let (pubkey, _) = node_keys2(mnemonic.to_entropy().as_slice());
            println!("Matched {} -> {}", prefix, pubkey.to_string().to_uppercase());
            println!("Mnemonic: {}", mnemonic.to_string())
        },
        None => println!("Didn't find mnemonic")
    }
}
