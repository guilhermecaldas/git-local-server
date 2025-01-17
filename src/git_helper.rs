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

    let mut config = repo.config().unwrap();
    config
        .set_bool("receive.denyNonFastforwards", false)
        .unwrap();
    config.set_bool("http.receivepack", true).unwrap();
    config.set_bool("http.uploadpack", true).unwrap();
    config.set_bool("receive.denyDeletes", false).unwrap();
    config.set_bool("receive.denyCurrentBranch", false).unwrap();

    let hooks_dir = repo.path().join("hooks");
    let post_update_file = hooks_dir.join("post-update");
    fs::write(post_update_file, "#!/bin/sh\nexec git update-server-info").unwrap();
}

/// Serves Git repositories using WebDAV protocol.
///
/// # Parameters
/// * `path` - Directory path containing Git repos to serve
/// * `ip` - IPv4 address to bind server to
/// * `port` - Port number to listen on
///
/// # Example
/// ```
/// serve_repo("./repos", &Ipv4Addr::LOCALHOST, &8080).await;
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
