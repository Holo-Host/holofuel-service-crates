use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Opt {
    /// Gets your balance, fees, promised and available Fuel
    #[structopt(name = "b")]
    Ledger,
    /// Gets the list of your pending transactions
    #[structopt(name = "p")]
    Pending,
    /// Gets the list of your actionable transactions
    #[structopt(name = "a")]
    Actionable,
    /// Gets the list of your completed transactions
    #[structopt(name = "c")]
    Completed,
}
impl Opt {
    /// Run this command
    pub async fn run(self) -> Result<()> {
        match self {
            Opt::Ledger => hf::actions::ledger::get().await?,
            Opt::Pending => hf::actions::pending::get().await?,
            Opt::Actionable => hf::actions::actionable::get().await?,
            Opt::Completed => hf::actions::completed::get().await?,
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    opt.run().await
}
