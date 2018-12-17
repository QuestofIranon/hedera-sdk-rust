#![feature(uniform_paths)]

extern crate structopt;

pub(crate) mod crypto;
mod proto;

use crypto::SecretKey;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="hedera keygen", about = "Private and Public key generator for Ħedera")]
enum HederaKeygen{

    /// Generate a mnemonic and Public/Private key pair
    #[structopt(name="generate")]
    Generate {

        /// include a password for mnemonic generation (defaults to "")
        #[structopt(long = "password", short = "p")]
        password: Option<String>,
    },

    /// Recover a Public/Private key pair by providing the mnemonic
    #[structopt(name="recover")]
    Recover {
        /// The required mnemonic
        mnemonic: Vec<String>,

        /// provide the password used for generation (defaults to "")
        #[structopt(long = "password", short = "p")]
        password: Option<String>
    }
}

fn command_generate(password: Option<String>) {
    let (secret_key, mnemonic) = SecretKey::generate(&password.unwrap_or_default());

    println!("Secret Key: {}", secret_key.to_string());
    println!("Public Key: {}", secret_key.public().to_string());
    println!("Mnemonic: {}", mnemonic);
}

fn command_recover(mnemonic: Vec<String>, password: Option<String>) {
    let secret_key = SecretKey::from_mnemonic(
        &mnemonic.join(" "),
        &password.unwrap_or_default()
    ).unwrap_or_else(|_| {
        panic!("Something went wrong, please try again. You can use hedera-keygen --help for more info.");
    });

    println!("Secret Key: {}", secret_key.to_string());
    println!("Public Key: {}", secret_key.public().to_string());
}


fn main() {
    let command = HederaKeygen::from_args();

    match command {
        HederaKeygen::Generate { password } => command_generate(password),
        HederaKeygen::Recover { mnemonic, password } => command_recover(mnemonic, password),
    }
}
