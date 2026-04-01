use clap::Parser;
use local_ip_address::local_ip;
use std::{error::Error, net::IpAddr, path::Path};

mod cli;
mod git;
mod server;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    match args.command {
        cli::Commands::Serve {
            dir,
            port,
            addr,
            no_timeout,
        } => {
            let repos = git::list_bare_repositories(Path::new(&dir))?;
            if repos.len() == 0 {
                return Err(Box::<dyn Error>::from("no bare repository available"));
            }

            let local_ip = local_ip().unwrap_or(IpAddr::from([0, 0, 0, 0]));

            println!("Serving repositories:");
            for repo in repos {
                println!("http://{:?}:{}/{}", local_ip, port, repo);
            }

            if !no_timeout {
                ui::display_timer();
            }

            server::serve_repositories(&dir, &addr, &port).await;
            Ok(())
        }
        cli::Commands::Init { repo_path } => {
            println!("Initializing repository {}", repo_path);
            git::init_bare_repository(Path::new(&repo_path))?;
            println!("Repository initialized with success!");
            Ok(())
        }
        cli::Commands::SetHead { repo_path, branch } => {
            git::set_repository_head(Path::new(&repo_path), &branch)?;
            println!("New HEAD set to {branch}");
            Ok(())
        }
    }
}
