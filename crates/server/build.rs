use chrono::{SecondsFormat, Utc};

fn main() {
    let git_hash = git_short_hash().unwrap_or_else(|| "unknown".to_string());
    let build_time = build_time_rfc3339();

    println!("cargo:rustc-env=VIBEKANBAN_GIT_HASH={git_hash}");
    println!("cargo:rustc-env=VIBEKANBAN_BUILD_TIME={build_time}");
}

fn git_short_hash() -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let hash = String::from_utf8(output.stdout).ok()?;
    let trimmed = hash.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn build_time_rfc3339() -> String {
    if let Ok(epoch) = std::env::var("SOURCE_DATE_EPOCH") {
        if let Ok(secs) = epoch.parse::<i64>() {
            if let Some(dt) = chrono::NaiveDateTime::from_timestamp_opt(secs, 0) {
                let utc = chrono::DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc);
                return utc.to_rfc3339_opts(SecondsFormat::Secs, true);
            }
        }
    }

    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}
