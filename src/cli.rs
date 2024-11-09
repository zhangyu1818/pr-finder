use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// GitHub personal access token
    #[clap(short, long)]
    pub token: String,

    /// Repository to query (e.g. owner/repo)
    #[clap(short, long)]
    pub repo: String,

    /// Filter by author
    #[clap(short, long)]
    pub author: Option<String>,

    /// status of PRs: open, closed, merged
    #[clap(short, long)]
    pub status: Option<String>,

    /// Grouping mode: day, week, month
    #[clap(short, long, default_value = "day")]
    pub group_by: String,

    /// Start date for the query (e.g. 2024-01-01)
    #[clap(long)]
    pub start_date: Option<String>,

    /// End date for the query (e.g. 2024-01-31)
    #[clap(long)]
    pub end_date: Option<String>,
}
