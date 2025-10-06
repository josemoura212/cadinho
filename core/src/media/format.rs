/// Retorna uma lista de formatos de mídia suportados para conversão.
pub fn supported_media_formats() -> Vec<&'static str> {
    vec![
        "mp4", "webm", "avi", "mkv", "mov", "flv", "wmv", "mp3", "wav", "flac", "aac", "ogg",
    ]
}
