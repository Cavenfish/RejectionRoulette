use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct RrArgs {
    #[clap(subcommand)]
    pub command: RrCommands,
}

#[derive(Subcommand)]
pub enum RrCommands {
    /// Add
    Add(AddArgs),

    /// Remove
    Remove(RemoveArgs),

    /// Update
    Update(UpdateArgs),

    /// Edit
    Edit(EditArgs),

    /// Show
    Show,

    /// Stats
    Stats,
}

#[derive(Args)]
pub struct AddArgs {
    /// Company
    #[arg(short)]
    pub company: String,

    /// Role
    #[arg(short)]
    pub role: String,

    /// Status
    #[arg(short)]
    pub status: String,

    /// Date in YEAR/MONTH/DAY (defaults to today)
    #[arg(long, default_value = "today")]
    pub date: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Item ID
    #[arg(short)]
    pub id: i64,
}

#[derive(Args)]
pub struct UpdateArgs {
    /// Item ID
    #[arg(short)]
    pub id: i64,

    /// Status
    #[arg(short)]
    pub status: String,
}

#[derive(Args)]
pub struct EditArgs {
    /// Item ID
    #[arg(short)]
    pub id: i64,

    /// Company
    #[arg(short)]
    pub company: String,

    /// Role
    #[arg(short)]
    pub role: String,

    /// Status
    #[arg(short)]
    pub status: String,

    /// Date in YEAR/MONTH/DAY (defaults to today)
    #[arg(long, default_value = "today")]
    pub date: String,
}
