use clap::Parser;
use local_ip_address::local_ip;
use std::{net::IpAddr, path::Path, process::exit};

mod cli;
mod git;
mod server;
mod ui;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    match args.command {
        Some(cli::Commands::Serve {
            dir,
            port,
            addr,
            no_timeout,
        }) => match git::list_bare_repositories(Path::new(&dir)) {
            Ok(repos) => {
                println!("Serving repositories:");
                let local_ip = local_ip().unwrap_or(IpAddr::from([0, 0, 0, 0]).into());
                for repo in repos {
                    println!("http://{:?}:{}/{}", local_ip, port, repo);
                }
                ui::display_timer(no_timeout);
                server::serve_repositories(&dir, &addr, &port).await;
            }
            Err(_) => {
                eprintln!("No repository available. Initialize a repository before serving.");
                exit(1);
            }
        },
        Some(cli::Commands::Init { repo_path }) => {
            println!("Initializing repository {}", repo_path);
            match git::init_bare_repository(Path::new(&repo_path)) {
                Ok(_) => println!("Repository initialized with success!"),
                Err(err) => eprintln!("Error: {err}"),
            }
        }
        Some(cli::Commands::SetHead { repo_path, branch }) => {
            match git::set_repository_head(Path::new(&repo_path), &branch) {
                Ok(_) => println!("New HEAD set to {branch}"),
                Err(err) => eprintln!("Error: {err}"),
            }
        }
        None => {
            eprintln!("No command specified. Use --help for usage information.");
            exit(1);
        }
    }
}
