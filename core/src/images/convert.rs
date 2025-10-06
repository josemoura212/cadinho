use crate::Job;
use anyhow::Result;
use image::ImageFormat;

pub fn convert_image(job: &Job) -> Result<()> {
    let img = image::open(&job.input)?;

    let format = match job.output_format.as_deref() {
        Some("png") | None => ImageFormat::Png,
        Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
        Some("bmp") => ImageFormat::Bmp,
        Some("tiff") => ImageFormat::Tiff,
        Some("gif") => ImageFormat::Gif,
        Some("webp") => ImageFormat::WebP,
        Some("ico") => ImageFormat::Ico,
        Some("tga") => ImageFormat::Tga,
        Some("pnm") => ImageFormat::Pnm,
        Some("dds") => ImageFormat::Dds,
        Some("hdr") => ImageFormat::Hdr,
        Some("openexr") | Some("exr") => ImageFormat::OpenExr,
        Some("farbfeld") | Some("ff") => ImageFormat::Farbfeld,
        Some("avif") => ImageFormat::Avif,
        Some("qoi") => ImageFormat::Qoi,
        Some(fmt) => {
            return Err(anyhow::anyhow!("Formato de imagem não suportado: {}", fmt));
        }
    };

    // Salvar no formato escolhido
    img.save_with_format(&job.output, format)?;

    Ok(())
}
