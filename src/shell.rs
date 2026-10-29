use clap::CommandFactory;
use clap::{Parser, ValueEnum};
use clap_complete::{Shell, generate};
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Specify shell you want autocompletion for
    #[arg(short, long)]
    pub completion: Option<String>,

    /// Enable verbose mode
    #[arg(short, long)]
    pub verbose: bool,

    /// Linux distrobution to update
    #[arg(short, long)]
    pub os: Option<OperatingSystems>,

    /// Package manager to update
    #[arg(short, long)]
    pub pkg: Option<PackageManagers>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[value(rename_all = "lower")]
pub enum OperatingSystems {
    Gentoo,
    Fedora,
    Aeryn,
    Debian,
    Arch,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[value(rename_all = "lower")]
pub enum PackageManagers {
    Emerge,
    Dnf,
    Moss,
    Apt,
    Pacman,
}

// Check for shell
pub fn get_supported_shells() -> Vec<String> {
    return vec![
        String::from("fish"),
        String::from("bash"),
        String::from("zsh"),
    ];
}

pub fn generate_completions(shell: Shell) {
    let mut cmd = Args::command();
    let bin_name = env!("CARGO_PKG_NAME");

    generate(shell, &mut cmd, bin_name, &mut io::stdout());
}
