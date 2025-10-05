use anyhow::Result;
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use std::process::Command;
mod media;
use media::convert_media;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum JobKind {
    Image,
    Media,
    Doc,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub input: String,
    pub output: String,
    pub kind: JobKind,
    pub output_format: Option<String>,
}

pub fn convert(job: &Job) -> Result<()> {
    match job.kind {
        JobKind::Image => convert_image(job),
        JobKind::Media => convert_media(job),
        JobKind::Doc => convert_doc(job),
    }
}

fn convert_image(job: &Job) -> Result<()> {
    // Carregar a imagem
    let img = image::open(&job.input)?;

    // Determinar formato de saída
    let format = match job.output_format.as_deref() {
        Some("png") | None => ImageFormat::Png,
        Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
        Some("bmp") => ImageFormat::Bmp,
        Some("tiff") => ImageFormat::Tiff,
        Some(fmt) => {
            return Err(anyhow::anyhow!("Formato de imagem não suportado: {}", fmt));
        }
    };

    // Salvar no formato escolhido
    img.save_with_format(&job.output, format)?;

    Ok(())
}

fn convert_doc(job: &Job) -> Result<()> {
    // Usar pandoc para converter documentos
    let status = Command::new("pandoc")
        .arg(&job.input)
        .arg("-o")
        .arg(&job.output)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Falha na conversão de documento com pandoc"
        ))
    }
}
