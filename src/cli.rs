extern crate shop_db;
use failure::Error;

use clap::{Parser, Subcommand};
use shop_db::Conn;

#[derive(Parser)]
#[command(name = "shop_cli")]
#[command(about = "Edit the shop_base contents")]
#[command(version = "0.1.0")]
#[command(author = "jack")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Put an item on the database
    Put {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        description: String,
        #[arg(short, long)]
        price: i32,
        #[arg(short, long, default_value = "10")]
        instock: i32,
    },
    /// Find items matching a given name part
    Find {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value = "5")]
        limit: i64,
    },
}

fn main() -> Result<(), Error> {
    let mut conn = Conn::new()?;
    let cli = Cli::parse();

    match cli.command {
        Commands::Put {
            name,
            description,
            price,
            instock,
        } => {
            let r = conn.put_itmes(&name, &description, price, instock)?;
            println!("Added Item: {:?}", r);
        }
        Commands::Find { name, limit } => {
            let r = conn.find_item(&name, limit)?;
            for p in r {
                println!("\n---------Entry---------\n");
                println!("{:?}", p);
            }
        }
    }
    Ok(())
}
