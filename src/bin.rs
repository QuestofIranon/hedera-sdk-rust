#![feature(uniform_paths)]

extern crate structopt;
extern crate colored;
extern crate openssl;

pub(crate) mod crypto;
mod proto;

use crypto::SecretKey;
use structopt::StructOpt;
use std::io;
use colored::Colorize;
use std::io::Read;
use std::io::Write;
use openssl::bn::BigNumContext;
use openssl::ec::*;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use std::fs::File;

macro_rules! prompt {
   ($($arg:tt),*) => ({
            let mut i = String::new();

            print!($($arg,)*);
            std::io::stdout().flush();

            std::io::stdin().read_line(&mut i);

            i.trim().to_string()
    })
}

#[derive(StructOpt, Debug)]
#[structopt(name="hedera keygen", about = "Private and Public key generator for Ħedera")]
enum HederaKeygen{

    /// Generate a mnemonic and Public/Private key pair
    #[structopt(name="generate")]
    Generate {

        /// Print the unencrypted keys and mnemonic phrase  to the terminal
        #[structopt(long = "unencrypted", short = "u")]
        unencrypted: bool,
    },

    /// Recover a Public/Private key pair by providing the mnemonic
    #[structopt(name="recover")]
    Recover {
        /// Print the unencrypted keys and mnemonic phrase  to the terminal
        #[structopt(long = "unencrypted", short = "u")]
        unencrypted: bool,
    },

    /// inspect your key
    #[structopt(name="inspect")]
    Inspect {
        /// the pub or pem file you want to inspect
        file: String,
    },
}


fn command_generate(unencrypted: bool) {

    println!("Generating public/private {} key pair.", "ed25519".color("red"));

    let out_file = prompt!("Enter file name in which to save the key to: ");

    let passphrase = prompt!("Enter passphrase (empty for no passphrase): ");

    if !passphrase.is_empty() {

        let mut conf_passphrase = prompt!("Enter your passphrase again: ");

        while conf_passphrase != passphrase {
            conf_passphrase = prompt!("Passphrase did not match, try again: ");
        }
    }

    let (secret_key, mnemonic) = SecretKey::generate(&passphrase);

    if unencrypted {

        println!("Secret Key: {}", secret_key.to_string().color("green"));
        println!("Public Key: {}", secret_key.public().to_string().color("yellow"));
        println!("Mnemonic: {}", mnemonic.color("magenta"));

        return
    }

    println!("Your public key has been saved in {}{}", out_file.color("blue"), ".pub".color("blue"));
    println!("Your private key has been saved in {}", out_file.color("blue"));

    println!("You can use this phrase to recover your keys: ");
    println!("{}", mnemonic.color("magenta"));
}

fn command_recover(unencrypted: bool) {
    let recovery_phrase = prompt!("Enter your recovery phrase: ");

    let out_file = prompt!("Enter file name in which to save the key to: ");

    let passphrase = prompt!("Enter passphrase (empty for no passphrase): ");

    if !passphrase.is_empty() {

        let mut conf_passphrase = prompt!("Enter your passphrase again: ");

        while conf_passphrase != passphrase {
            conf_passphrase = prompt!("Passphrase did not match, try again: ");
        }
    }

    let secret_key = SecretKey::from_mnemonic(
        &recovery_phrase,
        &passphrase,
    ).unwrap_or_else(|_| {
        panic!("{}, please try again. You can use hedera-keygen --help for more info.", "Something went wrong".color("red"));
    });

    if unencrypted {
        println!("Secret Key: {}", secret_key.to_string().color("green"));
        println!("Public Key: {}", secret_key.public().to_string().color("yellow"));
        return
    }

    println!("Your public key has been saved in {}{}", out_file.color("blue"), ".pub".color("blue"));
    println!("Your private key has been saved in {}", out_file.color("blue"));
}

fn command_inspect(file: String) {

    //let mut key_file = File::open(file)?;

    // todo: open the file
    if file.ends_with(".pub") {
        unimplemented!()



    } else if file.ends_with(".pem") {
        unimplemented!()


    }




}

fn main() {
    let command = HederaKeygen::from_args();

    match command {
        HederaKeygen::Generate { unencrypted } => command_generate(unencrypted),
        HederaKeygen::Recover { unencrypted } => command_recover(unencrypted),
        HederaKeygen::Inspect { file } => command_inspect(file),
    }
}

