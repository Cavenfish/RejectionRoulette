use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct RrArgs {
    #[clap(subcommand)]
    pub command: RrCommands,
}

#[derive(Subcommand)]
pub enum RrCommands {
    /// Add
    Add(AddCommand),

    /// Remove
    Remove(RemoveArgs),

    /// Update
    Update(UpdateArgs),

    /// Edit
    Edit(EditArgs),

    /// Show
    Show(ShowArgs),

    /// Stats
    Stats,
}

#[derive(Parser)]
pub struct AddCommand {
    #[clap(subcommand)]
    pub command: AddSubcommand,
}

#[derive(Subcommand)]
pub enum AddSubcommand {
    /// Applications
    Application(AddApplicationArgs),

    /// Interviews
    Interview(AddInterviewArgs),

    /// Offers
    Offer(AddOfferArgs),
}

#[derive(Args)]
pub struct AddApplicationArgs {
    /// Company
    #[arg(short)]
    pub company: String,

    /// Role
    #[arg(short)]
    pub role: String,

    // Location
    #[arg(short)]
    pub location: String,

    /// Application already rejected
    #[arg(long, action)]
    pub rejected: bool,

    /// Date in YEAR/MONTH/DAY (defaults to today)
    #[arg(long, default_value = "today")]
    pub date: String,
}

#[derive(Args)]
pub struct AddInterviewArgs {
    /// Application id
    #[arg(short)]
    pub id: i64,

    /// Interview date in YEAR/MONTH/DAY
    #[arg(short)]
    pub date: String,

    /// Interview type
    #[arg(short)]
    pub category: String,

    /// Notes
    #[arg(long, default_value = "")]
    pub notes: String,
}

#[derive(Args)]
pub struct AddOfferArgs {
    /// Application id
    #[arg(short)]
    pub id: i64,

    /// Salary
    #[arg(short)]
    pub salary: i64,

    /// Bonus
    #[arg(short)]
    pub bonus: i64,

    /// Equity details
    #[arg(short)]
    pub equity: String,

    /// Expiration date in YEAR/MONTH/DAY
    #[arg(short)]
    pub date: String,

    /// Accepted offer
    #[arg(long, action)]
    pub accepted: bool,
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

    /// Location
    #[arg(short)]
    pub location: String,

    /// Status
    #[arg(short)]
    pub status: String,

    /// Date in YEAR/MONTH/DAY (defaults to today)
    #[arg(long, default_value = "today")]
    pub date: String,
}

#[derive(Args)]
pub struct ShowArgs {
    /// Applications
    #[arg(long, action)]
    pub applications: bool,

    /// Interviews
    #[arg(long, action)]
    pub interviews: bool,

    /// Offers
    #[arg(long, action)]
    pub offers: bool,
}
