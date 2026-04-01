use clap::{Parser, Subcommand};
use std::net::Ipv4Addr;

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
        repo_path: String,
    },
    /// Sets the HEAD branch for a repository
    SetHead {
        /// Repository to set HEAD for
        #[arg(value_name = "REPOSITORY", required = true)]
        repo_path: String,

        /// Branch name to set as HEAD
        #[arg(value_name = "BRANCH", required = true)]
        branch: String,
    },
}
