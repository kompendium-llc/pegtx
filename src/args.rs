use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pegtx", about = "An accounting tool for use with pegnet")]
pub struct Config {
    /// FCT address
    // #[structopt(short, long)]
    pub address: String,

    /// The list of transactions will include transfers
    #[structopt(short, long)]
    pub transfer: bool,

    /// The list of transactions will include conversions
    #[structopt(short, long)]
    pub conversion: bool,

    /// The list of transactions will include miner payouts
    #[structopt(short, long)]
    pub mining: bool,

    /// The list of transactions will include transfers
    #[structopt(short, long)]
    pub burn: bool,

    /// Retrieve newest transactions first
    #[structopt(short, long)]
    pub descending: bool,

    /// Specify output file, defaults to <ADDRESS>.csv
    #[structopt(short, long)]
    pub output: Option<String>,

    /// Sets a custom pegnetd node rather than using the public node
    #[structopt(short, long)]
    pub node: Option<String >,

}

