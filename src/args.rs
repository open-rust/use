use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug, Clone)]
pub enum Commands {
    Fs(crate::mods::fs::Param),
    Loop(crate::mods::looper::Param),
    Limit(limit::args::Args),
    RBS(crate::mods::rbs::Param),
    RBC(crate::mods::rbc::Param),
    Oneport(crate::mods::oneport::Param),
    Toast(crate::mods::toast::Param),
    Install(crate::mods::install::Param),
}
