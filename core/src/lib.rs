use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum JobKind {
    Image,
    Media,
    Doc,
    Text,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub input: String,
    pub output: String,
    pub kind: JobKind,
}

pub fn convert(job: &Job) -> Result<()> {
    // TODO: pipelines reais
    std::fs::copy(&job.input, &job.output)?;
    Ok(())
}
