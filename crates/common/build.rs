use std::process::Command;

fn main() {
    let git_tag = Command::new("git")
        .args(["describe", "--exact-match", "--tags", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        // Ignore "nightly" tag - it's not a version tag
        .filter(|tag| tag != "nightly");

    let version = if let Some(tag) = git_tag {
        // Use the git tag for releases
        tag
    } else {
        // Use nightly-<git short hash>
        let git_hash = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout).ok()
                } else {
                    None
                }
            })
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        format!("nightly-{}", git_hash)
    };

    println!("cargo:rustc-env=ALLIUM_VERSION={}", version);
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/tags");
}
