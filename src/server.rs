use dav_server::{fakels, localfs};
use std::net::{Ipv4Addr, SocketAddr};

/// Serves Git repositories using WebDAV protocol.
///
/// # Arguments
/// * `path` - Directory path containing Git repos to serve
/// * `addr` - IPv4 address to bind server to
/// * `port` - Port number to listen on
///
/// # Example
/// ```rust
/// serve_repositories("./repos", &Ipv4Addr::LOCALHOST, &8080).await;
/// ```
pub async fn serve_repositories(path: &str, addr: &Ipv4Addr, port: &u16) {
    let addr: SocketAddr = (*addr, *port).into();
    let handler = dav_server::DavHandler::builder()
        .filesystem(localfs::LocalFs::new(
            path,
            true,
            false,
            cfg!(any(target_os = "macos", target_os = "windows")),
        ))
        .locksystem(fakels::FakeLs::new())
        .build_handler();

    let warpdav = dav_server::warp::dav_handler(handler);
    warp::serve(warpdav).run(addr).await;
}
