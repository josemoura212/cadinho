#[tauri::command]
pub async fn install_dependencies(
    deps: Vec<String>,
) -> Result<std::collections::HashMap<String, String>, String> {
    use std::collections::HashMap;

    let mut results = HashMap::new();

    for dep in deps {
        let result = match dep.as_str() {
            "ffmpeg" => install_ffmpeg().await,
            "pandoc" => install_pandoc().await,
            _ => Err(format!("Dependência desconhecida: {}", dep)),
        };

        results.insert(
            dep,
            match result {
                Ok(msg) => format!("Sucesso: {}", msg),
                Err(err) => format!("Erro: {}", err),
            },
        );
    }

    Ok(results)
}

async fn install_ffmpeg() -> Result<String, String> {
    use tokio::process::Command;

    if cfg!(target_os = "windows") {
        // Tentar winget primeiro, depois chocolatey
        let winget_result = Command::new("winget")
            .args(&[
                "install",
                "--id",
                "Gyan.FFmpeg",
                "--accept-source-agreements",
                "--accept-package-agreements",
            ])
            .output()
            .await;

        if winget_result.is_ok() && winget_result.as_ref().unwrap().status.success() {
            return Ok("FFmpeg instalado via Winget".to_string());
        }

        let choco_result = Command::new("choco")
            .args(&["install", "ffmpeg", "-y"])
            .output()
            .await;

        if choco_result.is_ok() && choco_result.as_ref().unwrap().status.success() {
            return Ok("FFmpeg instalado via Chocolatey".to_string());
        }

        Err("Falha ao instalar FFmpeg. Instale manualmente via Winget ou Chocolatey.".to_string())
    } else if cfg!(target_os = "linux") {
        // Tentar apt primeiro
        let apt_result = Command::new("sudo")
            .args(&[
                "apt", "update", "&&", "sudo", "apt", "install", "-y", "ffmpeg",
            ])
            .output()
            .await;

        if apt_result.is_ok() && apt_result.as_ref().unwrap().status.success() {
            return Ok("FFmpeg instalado via apt".to_string());
        }

        Err(
            "Falha ao instalar FFmpeg. Execute: sudo apt update && sudo apt install ffmpeg"
                .to_string(),
        )
    } else if cfg!(target_os = "macos") {
        let brew_result = Command::new("brew")
            .args(&["install", "ffmpeg"])
            .output()
            .await;

        if brew_result.is_ok() && brew_result.as_ref().unwrap().status.success() {
            return Ok("FFmpeg instalado via Homebrew".to_string());
        }

        Err("Falha ao instalar FFmpeg. Instale Homebrew e execute: brew install ffmpeg".to_string())
    } else {
        Err("Sistema operacional não suportado para instalação automática".to_string())
    }
}

async fn install_pandoc() -> Result<String, String> {
    use tokio::process::Command;

    if cfg!(target_os = "windows") {
        // Tentar winget primeiro, depois chocolatey
        let winget_result = Command::new("winget")
            .args(&[
                "install",
                "--id",
                "JohnMacFarlane.Pandoc",
                "--accept-source-agreements",
                "--accept-package-agreements",
            ])
            .output()
            .await;

        if winget_result.is_ok() && winget_result.as_ref().unwrap().status.success() {
            return Ok("Pandoc instalado via Winget".to_string());
        }

        let choco_result = Command::new("choco")
            .args(&["install", "pandoc", "-y"])
            .output()
            .await;

        if choco_result.is_ok() && choco_result.as_ref().unwrap().status.success() {
            return Ok("Pandoc instalado via Chocolatey".to_string());
        }

        Err("Falha ao instalar Pandoc. Instale manualmente via Winget ou Chocolatey.".to_string())
    } else if cfg!(target_os = "linux") {
        // Tentar apt primeiro
        let apt_result = Command::new("sudo")
            .args(&[
                "apt", "update", "&&", "sudo", "apt", "install", "-y", "pandoc",
            ])
            .output()
            .await;

        if apt_result.is_ok() && apt_result.as_ref().unwrap().status.success() {
            return Ok("Pandoc instalado via apt".to_string());
        }

        Err(
            "Falha ao instalar Pandoc. Execute: sudo apt update && sudo apt install pandoc"
                .to_string(),
        )
    } else if cfg!(target_os = "macos") {
        let brew_result = Command::new("brew")
            .args(&["install", "pandoc"])
            .output()
            .await;

        if brew_result.is_ok() && brew_result.as_ref().unwrap().status.success() {
            return Ok("Pandoc instalado via Homebrew".to_string());
        }

        Err("Falha ao instalar Pandoc. Instale Homebrew e execute: brew install pandoc".to_string())
    } else {
        Err("Sistema operacional não suportado para instalação automática".to_string())
    }
}
