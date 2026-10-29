use clap::CommandFactory;
use clap::{Parser, ValueEnum};
use clap_complete::{Shell, generate};
use std::io;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Specify shell you want autocompletion for
    #[arg(short, long)]
    completion: Option<String>,

    /// Enable verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Linux distrobution to update
    #[arg(short, long)]
    os: Option<OperatingSystems>,

    /// Package manager to update
    #[arg(short, long)]
    pkg: Option<PackageManagers>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[value(rename_all = "lower")]
enum OperatingSystems {
    Gentoo,
    Fedora,
    Aeryn,
    Debian,
    Arch,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[value(rename_all = "lower")]
enum PackageManagers {
    Emerge,
    Dnf,
    Moss,
    Apt,
    Pacman,
}

// Check for shell
fn get_supported_shells() -> Vec<String> {
    return vec![
        String::from("fish"),
        String::from("bash"),
        String::from("zsh"),
    ];
}

fn generate_completions(shell: Shell) {
    let mut cmd = Args::command();
    let bin_name = env!("CARGO_PKG_NAME");

    generate(shell, &mut cmd, bin_name, &mut io::stdout());
}

fn main() {
    // Get all arguments
    let args = Args::parse();

    // Parse arguments

    // Check for shell completion argument
    let supported = get_supported_shells();

    if let Some(shell_str) = args.completion {
        if !supported.iter().any(|s| s == &shell_str) {
            eprintln!("error: unsupported shell '{}'", shell_str);
            exit(1);
        }

        let shell = match shell_str.to_lowercase().as_str() {
            "fish" => Shell::Fish,
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            _ => {
                eprintln!("error: shell not supported");
                exit(1);
            }
        };

        generate_completions(shell);
        exit(0);
    }

    // Check for operating system arguments
    match args.os {
        Some(OperatingSystems::Gentoo) => println!("Gentoo"),
        Some(OperatingSystems::Fedora) => println!("Fedora"),
        Some(OperatingSystems::Aeryn) => println!("Aeryn"),
        Some(OperatingSystems::Debian) => println!("Debian"),
        Some(OperatingSystems::Arch) => println!("Arch"),
        None => {}
    }

    // Check for package manager arguments
    match args.pkg {
        Some(PackageManagers::Emerge) => println!("emerge"),
        Some(PackageManagers::Dnf) => println!("dnf"),
        Some(PackageManagers::Moss) => println!("moss"),
        Some(PackageManagers::Apt) => println!("apt"),
        Some(PackageManagers::Pacman) => println!("pacman"),
        None => {}
    }
}
