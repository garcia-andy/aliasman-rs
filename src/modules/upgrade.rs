use crate::utils::shell_utils::SHELLS_REMOTE;

/// Enabled to auto upgrade teh final binary
/// # Panics
/// Panic if can´t build or upgrade the binary
pub fn upgrade_release() -> String {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("garcia-andy")
        .repo_name("aliasman-rs")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(SHELLS_REMOTE.stable.as_str())
        .build()
        .expect("Error building the new upgrade")
        .update()
        .expect("Fail to update");

    format!("[{}] latest: {}", status.version(), status.uptodate())
}
