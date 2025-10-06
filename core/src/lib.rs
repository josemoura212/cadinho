use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;
mod images;
mod media;
use media::convert_media;

use crate::images::convert_image;

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

fn convert_doc(job: &Job) -> Result<()> {
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

pub use crate::images::supported_image_formats;
pub use crate::media::supported_media_formats;
