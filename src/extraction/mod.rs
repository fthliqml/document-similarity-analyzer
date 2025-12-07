//! File extraction modules for PDF, DOCX, and TXT files

pub mod pdf;
pub mod docx;
pub mod txt;

pub use self::pdf::extract_pdf;
pub use self::docx::extract_docx;
pub use self::txt::extract_txt;

use std::path::Path;

/// Supported file types for extraction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    Pdf,
    Docx,
    Txt,
}

impl FileType {
    /// Detect file type from extension
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension.to_lowercase().as_str() {
            "pdf" => Some(FileType::Pdf),
            "docx" => Some(FileType::Docx),
            "txt" => Some(FileType::Txt),
            _ => None,
        }
    }

    /// Detect file type from filename
    pub fn from_filename(filename: &str) -> Option<Self> {
        Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }
}

/// Extract text from file bytes based on file type
pub fn extract_text(file_bytes: &[u8], file_type: FileType) -> Result<String, String> {
    match file_type {
        FileType::Pdf => extract_pdf(file_bytes),
        FileType::Docx => extract_docx(file_bytes),
        FileType::Txt => extract_txt(file_bytes),
    }
}
