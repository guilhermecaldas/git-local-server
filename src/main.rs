use dav_server::{fakels::FakeLs, localfs::LocalFs, warp::dav_handler, DavHandler};
use git2::{Repository, RepositoryInitMode, RepositoryInitOptions};
use local_ip_address::local_ip;
use std::{env, fs, net::SocketAddr, path::Path, process::exit};

/// Initializes a new bare Git repository at the specified path.
////// # Parameters
/// * `path` - The path where the repository should be created
////// # Effects
/// - Creates a new bare Git repository
/// - Sets up a post-update hook for server info updates
fn init_repo(path: &String) {
    let mut options = RepositoryInitOptions::new();
    let repo = match Repository::init_opts(
        path,
        options.bare(true).mode(RepositoryInitMode::SHARED_ALL),
    ) {
        Ok(repo) => repo,
        Err(err) => {
            eprintln!("Error initializing repository: {}", err.message());
            exit(1);
        }
    };

    let hooks_dir = repo.path().join("hooks");
    let post_update_file = hooks_dir.join("post-update");
    fs::write(post_update_file, "#!/bin/sh\nexec git update-server-info").unwrap();
}

/// Serves a Git repository using WebDAV protocol.
///
/// # Parameters
/// * `path` - Path to the repository directory to serve via WebDAV. The path should be relative or
///           absolute to the current working directory.
/// * `port` - Port number to listen on for incoming WebDAV connections. Must be a valid port number
///           between 1-65535.
///
/// # Effects
/// - Binds to the specified port on all network interfaces (0.0.0.0)
/// - Initializes a WebDAV handler with filesystem access and locking support
/// - Uses LocalFs for filesystem operations with support for special macOS handling
/// - Implements a fake locking system via FakeLs
/// - Creates a warp HTTP server serving WebDAV requests
/// - Blocks the current thread and handles requests asynchronously
///
/// # Examples
/// ```
/// serve_repo("./repo", &8080).await; // Serves from ./repo on port 8080
/// ```
async fn serve_repo(path: &str, port: &u16) {
    let addr: SocketAddr = ([0, 0, 0, 0], *port).into();
    let handler = DavHandler::builder()
        .filesystem(LocalFs::new(path, true, false, cfg!(target_os = "macos")))
        .locksystem(FakeLs::new())
        .build_handler();

    let warpdav = dav_handler(handler);
    warp::serve(warpdav).run(addr).await;
}

/// Updates server information files for the Git repository.
///
/// # Parameters
/// * `path` - Path to the bare Git repository
///
/// # Effects
/// - Creates/updates refs file containing reference information
/// - Creates/updates packs file containing pack information
fn update_server_info(path: &str) {
    let repo = Repository::open_bare(path).unwrap();
    let repo_path = repo.path();
    let info_dir = repo_path.join("info");

    // Create info directory if it doesn't exist
    fs::create_dir_all(&info_dir).unwrap();

    // Update refs file
    let refs_file = info_dir.join("refs");
    let mut refs_content = String::new();
    for reference in repo.references().unwrap() {
        let reference = reference.unwrap();
        if let Some(name) = reference.name() {
            if let Some(target) = reference.target() {
                refs_content.push_str(&format!("{}\t{}\n", target, name));
            }
        }
    }
    fs::write(refs_file, refs_content).unwrap();

    // Update packs file
    let packs_file = info_dir.join("packs");
    let mut packs_content = String::new();
    let pack_dir = repo_path.join("objects/pack");
    if pack_dir.exists() {
        for entry in fs::read_dir(pack_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "pack") {
                if let Some(name) = path.file_name() {
                    packs_content.push_str(&format!("P {}\n", name.to_string_lossy()));
                }
            }
        }
    }
    fs::write(packs_file, packs_content).unwrap();
}

/// Parses command line arguments and updates port and repository values.
///
/// # Parameters
/// * `cmd` - The command line flag (--repo or --port)
/// * `val` - The value associated with the flag
/// * `port` - Mutable reference to port number
/// * `repo` - Mutable reference to repository name
///
/// # Effects
/// Updates port and repo values based on command line arguments
fn parse_args(cmd: &String, val: &String, port: &mut u16, repo: &mut String) {
    match cmd.as_str() {
        "--repo" => {
            *repo = val.to_string();
        }
        "--port" => {
            *port = match val.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid value: \"{}\"", val);
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("Invalid parameter: \"{}\"", cmd);
            exit(1);
        }
    }
}

/// Displays help information about program usage.
///
/// # Effects
/// Prints usage information to stdout and exits the program
fn show_help() {
    println!("usage: [--repo <repository>] [--port <port>]\n");
    println!("   --repo   Defines repository name. (defaults: demo.git)");
    println!("   --port   Port to serve Git. (defaults: 5005)\n");
    println!("eg.: git-local-server --repo my-app.git --port 5008");
    exit(0);
}

/// Displays version information about the program.
///
/// # Effects
/// Prints package name, version, operating system, and architecture information to stdout
///
/// # Example Output
/// ```text
/// git-local-server 1.0.0 linux_x86_64
/// ```
fn show_version() {
    println!(
        "{} {} {}_{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env::consts::OS,
        env::consts::ARCH
    );
}

/// Main entry point for the Git server application.
///
/// # Effects
/// - Processes command line arguments
/// - Initializes repository if needed
/// - Starts the Git server
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut port: u16 = 5005;
    let mut repo = String::from("demo.git");

    match args.len() {
        1 => {
            println!("Using default repository \"{}\" and port {}", &repo, &port);
        }
        2 => {
            let param: &str = &args[1];
            if vec!["--version", "-v"].contains(&param) {
                show_version();
                exit(0);
            }
            show_help();
            exit(0);
        }
        3 => {
            parse_args(&args[1], &args[2], &mut port, &mut repo);
        }
        5 => {
            if args[1] == args[3] {
                eprintln!("Parameters cannot be duplicated");
                exit(1);
            }
            parse_args(&args[1], &args[2], &mut port, &mut repo);
            parse_args(&args[3], &args[4], &mut port, &mut repo);
        }
        _ => {
            show_help();
        }
    }

    if !Path::new(&repo).exists() {
        println!("Initializing repository {}", repo);
        init_repo(&repo);
    }

    println!("Serving at:");
    println!("http://{:?}:{}/{}", local_ip().unwrap(), port, repo);
    update_server_info(&repo);
    serve_repo(".", &port).await;
}
