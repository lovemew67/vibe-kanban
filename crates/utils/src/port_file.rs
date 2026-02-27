use std::{env, path::PathBuf};

use tokio::fs;

fn resolve_port_file_path(app_name: &str) -> PathBuf {
    if let Ok(data_dir) = env::var("VIBE_KANBAN_DATA_DIR") {
        PathBuf::from(data_dir).join(format!("{app_name}.port"))
    } else {
        env::temp_dir().join(app_name).join(format!("{app_name}.port"))
    }
}

pub async fn write_port_file(port: u16) -> std::io::Result<PathBuf> {
    let path = resolve_port_file_path("vibe-kanban");
    tracing::debug!("Writing port {} to {:?}", port, path);
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).await?;
    }
    fs::write(&path, port.to_string()).await?;
    Ok(path)
}

pub async fn read_port_file(app_name: &str) -> std::io::Result<u16> {
    let path = resolve_port_file_path(app_name);
    tracing::debug!("Reading port from {:?}", path);

    let content = fs::read_to_string(path).await?;
    let port: u16 = content
        .trim()
        .parse()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(port)
}
