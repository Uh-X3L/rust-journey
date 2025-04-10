use clap::{Parser, Subcommand};
use contract_cli::{Contract, establish_connection,utils}; // from lib.rs





#[derive(Parser)]
#[command(name = "Contract CLI")]
#[command(about = "Interact with your contract", long_about = None)]
struct Cli {
    /// Contract owner (defaults to 'alice')
    #[arg(long, default_value = "alice")]
    owner: String,

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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let conn = establish_connection()?; // new helper from db.rs
    let contract_id = utils::hash::hash_owner(&cli.owner);

    let mut contract = Contract::load_or_create(&conn, contract_id, &cli.owner)?;

    match cli.command {
        Commands::Status => contract.status(),
        Commands::Deposit { amount } => contract.deposit(&conn, amount)?,
        Commands::Withdraw { amount } => contract.withdraw(&conn, amount)?,
        Commands::History => contract.show_history(&conn)?,
    }

    Ok(())
}
