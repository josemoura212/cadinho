use crate::Job;
use anyhow::Result;
use std::process::Command;

pub fn convert_media(job: &Job) -> Result<()> {
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-i").arg(&job.input);

    match job.output_format.as_deref() {
        Some("mp4") | None => {
            cmd.arg("-c:v").arg("libx264").arg("-c:a").arg("aac");
        }
        Some("webm") => {
            cmd.arg("-c:v").arg("libvpx-vp9").arg("-c:a").arg("libopus");
        }
        Some("avi") => {
            cmd.arg("-c:v").arg("mpeg4").arg("-c:a").arg("mp3");
        }
        Some("mkv") => {
            cmd.arg("-c:v").arg("libx264").arg("-c:a").arg("aac");
        }
        Some("mov") => {
            cmd.arg("-c:v").arg("libx264").arg("-c:a").arg("aac");
        }
        Some("flv") => {
            cmd.arg("-c:v").arg("flv").arg("-c:a").arg("mp3");
        }
        Some("wmv") => {
            cmd.arg("-c:v").arg("wmv2").arg("-c:a").arg("wmav2");
        }
        Some("mp3") => {
            cmd.arg("-vn").arg("-c:a").arg("mp3");
        }
        Some("wav") => {
            cmd.arg("-vn").arg("-c:a").arg("pcm_s16le");
        }
        Some("flac") => {
            cmd.arg("-vn").arg("-c:a").arg("flac");
        }
        Some("aac") => {
            cmd.arg("-vn").arg("-c:a").arg("aac");
        }
        Some("ogg") => {
            cmd.arg("-vn").arg("-c:a").arg("libvorbis");
        }
        // Para formatos desconhecidos, tentar codecs padrão
        Some(_) => {
            // Assumir vídeo + áudio com codecs comuns
            cmd.arg("-c:v").arg("libx264").arg("-c:a").arg("aac");
        }
    }

    cmd.arg(&job.output);

    let status = cmd.status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Falha na conversão de mídia com ffmpeg"))
    }
}
