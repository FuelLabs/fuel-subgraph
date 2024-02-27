use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opt {
    /// Fuel node instance to connect to
    pub fuel_url: String,
    /// Block height to start from
    pub height: u32,
    /// Stop the latest block has been reached
    #[clap(long)]
    pub stop: bool,
    /// Polling interval
    #[clap(default_value = "1s")]
    pub poll: humantime::Duration,
}
