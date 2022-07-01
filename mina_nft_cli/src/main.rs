use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser)]
    address: Option<String>,
    #[clap(short, long, value_parser)]
    block: Option<u64>,
}

fn main() {
    let cli = Cli::parse();

    match cli.address.as_deref() {
        Some(address) => {
            let balance = nft_shared::balance(address);
            match balance {
                Ok(result) => println!("{}", result),
                Err(error) => {
                    eprintln!("{}", error.to_json())
                }
            };
        }
        _ => {}
    };

    match cli.block {
        Some(block) => {
            let events = nft_shared::events(block);
            match events {
                Ok(result) => println!("{}", result),
                Err(error) => {
                    eprintln!("{}", error.to_json())
                }
            };
        }
        _ => {}
    };
}
