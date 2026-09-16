use chrono::Utc;

pub const DIRNAME: &'static str = "issues";

pub fn generate_datetime_id() -> String {
    // Generates YYYYMMDD-HHMMSS-microseconds: 20260917-002514
    Utc::now().format("%Y%m%d-%H%M%S").to_string()
}

pub mod lexer;

pub mod parser;

pub mod compiler;

pub mod vm;

pub mod task;
