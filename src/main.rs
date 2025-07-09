use clap::Parser;

mod args;
mod mods;
mod utils;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args: args::Args = args::Args::parse();
    match args.command {
        args::Commands::Fs(param) => mods::fs::main(param).await,
        args::Commands::Loop(param) => mods::looper::main(param).await,
        args::Commands::Limit(param) => limit::main(param),
        args::Commands::RBS(param) => mods::rbs::main(param).await,
        args::Commands::Install(param) => mods::install::main(param).await,
    }
}
