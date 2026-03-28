use std::{ops::SubAssign, process::exit, time::Duration};

/// Displays a countdown timer for the server session.
///
/// By default, a session timeout is enabled to prevent leaving the server open
/// indefinitely. This function spawns a Tokio task that runs a progress bar
/// indicating the remaining time. If the `no_timeout` flag is true, this
/// timer will not be displayed and the session will not time out.
///
/// # Arguments
///
/// * `no_timeout` - A boolean flag. If `true`, the session timeout will be disabled.
///                  If `false`, the session timeout will be enabled and the timer
///                  will be displayed.
pub fn display_timer(no_timeout: bool) {
    // By default, enable session timeout to avoid leaving the server open
    if !no_timeout {
        tokio::spawn(async move {
            let timeout = 60 * 5;
            let mut duration = Duration::from_secs(timeout);

            let pb = indicatif::ProgressBar::new(timeout);

            pb.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("\nEnding session in {msg} [{bar:20.yellow}]")
                    .unwrap()
                    .progress_chars("#-"),
            );

            pb.set_message(indicatif::FormattedDuration(duration).to_string());
            pb.set_position(timeout);

            for _ in (0..timeout).rev() {
                tokio::time::sleep(Duration::from_secs(1)).await;
                duration.sub_assign(Duration::from_secs(1));
                pb.set_message(indicatif::FormattedDuration(duration).to_string());
                pb.dec(1);
            }
            println!("Server session expired");
            exit(0)
        });
    }
}
