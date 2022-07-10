use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser)]
    address: Option<String>,
    #[clap(short, long, value_parser)]
    block: Option<u64>,
    #[clap(short, long, value_parser, default_value = "")]
    cursor: String,
    #[clap(short, long, action)]
    owners: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.address.as_deref() {
        Some(address) => {
            let balance = nft_shared::balance(address).await;
            match balance {
                Ok(result) => println!("{}", serde_json::to_string(&result).unwrap()),
                Err(error) => {
                    eprintln!("{}", error.to_json())
                }
            };
        }
        _ => {}
    };

    match cli.block {
        Some(block) => {
            let events = nft_shared::events(block).await;
            match events {
                Ok(result) => println!("{}", serde_json::to_string(&result).unwrap()),
                Err(error) => {
                    eprintln!("{}", error.to_json())
                }
            };
        }
        _ => {}
    };

    if cli.owners {
        let query_result = nft_shared::owners(&cli.cursor).await;
        match query_result {
            Ok(result) => println!("{}", serde_json::to_string(&result).unwrap()),
            Err(error) => {
                eprintln!("{}", error.to_json())
            }
        };
    }
    Ok(())
}
