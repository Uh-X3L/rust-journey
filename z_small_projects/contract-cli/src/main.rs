use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

mod contract;
use contract::Contract;

#[derive(Parser)]
#[command(name = "Contract CLI")]
#[command(about = "Interact with your contract", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show contract balance and owner
    Status,
    /// Deposit funds into the contract
    Deposit {
        #[arg(long)]
        amount: u64,
    },
    /// Withdraw funds from the contract
    Withdraw {
        #[arg(long)]
        amount: u64,
    },
    /// Show last 5 transactions
    History,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = Connection::open("contract.db")?;

    let mut contract = Contract::load_or_create(&conn, 1, "alice")?;

    match cli.command {
        Commands::Status => contract.status(),
        Commands::Deposit { amount } => contract.deposit(&conn, amount)?,
        Commands::Withdraw { amount } => contract.withdraw(&conn, amount)?,
        Commands::History => contract.show_history(&conn)?,
    }

    Ok(())
}
