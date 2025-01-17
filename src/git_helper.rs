use dav_server::{fakels::FakeLs, localfs::LocalFs, warp::dav_handler, DavHandler};
use git2::{Repository, RepositoryInitMode, RepositoryInitOptions};
use std::{
    fs,
    io::{Error, ErrorKind},
    net::{Ipv4Addr, SocketAddr},
    process::exit,
};

/// Initializes a new bare Git repository at the specified path.
////// # Parameters
/// * `path` - The path where the repository should be created
////// # Effects
/// - Creates a new bare Git repository
/// - Sets up a post-update hook for server info updates
pub fn init_repo(path: &str) {
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
/// * `path` - Directory path containing repositories to serve via WebDAV. Should
///           be relative or absolute path.
/// * `ip` - IPv4 address to bind server to. Use 127.0.0.1 for localhost or
///          0.0.0.0 for all interfaces.
/// * `port` - Port number to listen on, in range 1-65535.
///
/// # Details
/// Creates a WebDAV server to host Git repositories over HTTP. Uses
/// filesystem-based access control with basic locking features. Includes special
/// handling for macOS clients.
///
/// The server runs asynchronously and will block the current thread while serving
/// requests.
///
/// # Examples
/// ```
/// // Serve on localhost port 8080
/// serve_repo("./repos", &Ipv4Addr::LOCALHOST, &8080).await;
///
/// // Serve on all interfaces port 5005
/// serve_repo(".", &Ipv4Addr::UNSPECIFIED, &5005).await;
/// ```
pub async fn serve_repos(path: &str, addr: &Ipv4Addr, port: &u16) {
    let addr: SocketAddr = (*addr, *port).into();
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
pub fn update_server_info(path: &str) {
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

pub fn list_repos(dir: &str) -> Result<Vec<String>, Error> {
    let mut repos = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Ok(repo) = Repository::open_bare(&path) {
                let name = repo
                    .path()
                    .file_name()
                    .and_then(|p| p.to_str())
                    .map(String::from)
                    .unwrap();

                if name != ".git" {
                    repos.push(name);
                }
            }
        }
    }

    if repos.is_empty() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "No bare Git repositories found",
        ));
    }

    Ok(repos)
}
