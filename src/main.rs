use clap::Parser;
use clap_complete::Shell;
use std::process::exit;
use std::sync::{LazyLock, Mutex};

mod shell;
use shell::{Args, OperatingSystems, PackageManagers, generate_completions, get_supported_shells};

static _SHOULD_ENCRYPT: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

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

    if *&args.verbose {
        println!("Enabling Verbose {}", &args.verbose);
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
