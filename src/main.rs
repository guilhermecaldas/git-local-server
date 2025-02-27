use clap::{Parser, Subcommand};
use git2::Repository;
use indicatif::{FormattedDuration, ProgressBar, ProgressStyle};
use local_ip_address::local_ip;
use std::{net::Ipv4Addr, ops::SubAssign, process::exit, time::Duration};
mod git_helper;
use git_helper::{init_repo, list_repos, serve_repos, set_head, update_server_info};

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Serves Git repositories inside of a specified directory
    Serve {
        /// Root directory of server
        #[arg(value_name = "PATH", default_value = ".")]
        dir: String,

        /// Port number
        #[arg(short, long, value_name = "PORT", default_value_t = 5005)]
        port: u16,

        /// IPv4 address. Set to 127.0.0.1 to serve only localhost
        #[arg(short, long, value_name = "ADDR", default_value_t= Ipv4Addr::from([0,0,0,0]))]
        addr: Ipv4Addr,

        /// Disable server timeout. (not recommended)
        #[arg(long, value_name = "TIMEOUT_DISABLED", default_value_t = false)]
        no_timeout: bool,
    },
    /// Initializes a Git repository in the specified path
    Init {
        /// Name of repository directory to be initialized. eg. my_repo.git
        #[arg(value_name = "REPO_NAME", required = true)]
        repository: String,
    },
    /// Sets the HEAD branch for a repository
    SetHead {
        /// Repository to set HEAD for
        #[arg(value_name = "REPOSITORY", required = true)]
        repository: String,

        /// Branch name to set as HEAD
        #[arg(value_name = "BRANCH", required = true)]
        branch: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Serve {
            dir,
            port,
            addr,
            no_timeout,
        }) => match list_repos(&dir) {
            Ok(repos) => {
                println!("Serving repositories:");
                for repo in repos {
                    println!("http://{:?}:{}/{}", local_ip().unwrap(), port, repo);
                }

                // By default, enable session timeout to avoid leaving the server open
                if !no_timeout {
                    tokio::spawn(async move {
                        let timeout = 60 * 5;
                        let mut duration = Duration::from_secs(timeout);

                        let pb = ProgressBar::new(timeout);

                        pb.set_style(
                            ProgressStyle::default_bar()
                                .template("\nEnding session in {msg} [{bar:20.yellow}]")
                                .unwrap()
                                .progress_chars("#-"),
                        );

                        pb.set_message(FormattedDuration(duration).to_string());
                        pb.set_position(timeout);

                        for _ in (0..timeout).rev() {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            duration.sub_assign(Duration::from_secs(1));
                            pb.set_message(FormattedDuration(duration).to_string());
                            pb.dec(1);
                        }
                        println!("Server session expired");
                        exit(0)
                    });
                }
                serve_repos(&dir, &addr, &port).await;
            }
            Err(_) => {
                eprintln!("No repository available. Initialize a repository before serving.");
                exit(1);
            }
        },
        Some(Commands::Init { repository }) => {
            println!("Initializing repository {}", repository);
            init_repo(&repository, None);
            update_server_info(&repository);
            println!("Repository HEAD set to \"develop\"");
            println!("To change HEAD, use set-head <REPOSITORY> <BRANCH>");
        }
        Some(Commands::SetHead { repository, branch }) => {
            let repo = Repository::open_bare(repository).unwrap_or_else(|err| {
                eprintln!("Error opening repository: {}", err.message());
                exit(1);
            });
            set_head(&branch, repo);
            println!("New HEAD set to {}", branch);
        }
        None => {
            eprintln!("No command specified. Use --help for usage information.");
            exit(1);
        }
    }
}
