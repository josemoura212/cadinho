use std::collections::HashMap;
use std::process::Command;

#[tauri::command]
pub fn check_dependencies() -> Result<std::collections::HashMap<String, bool>, String> {
    let mut results = HashMap::new();

    let ffmpeg_check = if cfg!(target_os = "windows") {
        Command::new("where").arg("ffmpeg").output()
    } else {
        Command::new("which").arg("ffmpeg").output()
    };

    results.insert(
        "ffmpeg".to_string(),
        ffmpeg_check
            .map(|output| output.status.success())
            .unwrap_or(false),
    );

    let pandoc_check = if cfg!(target_os = "windows") {
        Command::new("where").arg("pandoc").output()
    } else {
        Command::new("which").arg("pandoc").output()
    };

    results.insert(
        "pandoc".to_string(),
        pandoc_check
            .map(|output| output.status.success())
            .unwrap_or(false),
    );

    Ok(results)
}
