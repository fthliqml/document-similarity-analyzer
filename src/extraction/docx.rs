//! DOCX text extraction module

/// Extract text from DOCX file bytes
///
/// Uses docx-rs library to parse DOCX (Open XML format) and extract text content.
/// Returns concatenated text from all paragraphs.
pub fn extract_docx(file_bytes: &[u8]) -> Result<String, String> {
    docx_rs::read_docx(file_bytes)
        .map_err(|e| format!("Failed to extract DOCX: {}", e))
        .map(|docx| {
            // Extract text from all paragraphs
            let text_parts: Vec<String> = docx
                .document
                .children
                .iter()
                .filter_map(|child| {
                    match child {
                        docx_rs::DocumentChild::Paragraph(para) => {
                            let para_text: String = para
                                .children
                                .iter()
                                .filter_map(|p_child| {
                                    match p_child {
                                        docx_rs::ParagraphChild::Run(run) => {
                                            let run_text: String = run
                                                .children
                                                .iter()
                                                .filter_map(|r_child| {
                                                    match r_child {
                                                        docx_rs::RunChild::Text(text) => {
                                                            Some(text.text.clone())
                                                        }
                                                        _ => None,
                                                    }
                                                })
                                                .collect::<Vec<_>>()
                                                .join("");
                                            if run_text.is_empty() {
                                                None
                                            } else {
                                                Some(run_text)
                                            }
                                        }
                                        _ => None,
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(" ");
                            
                            if para_text.trim().is_empty() {
                                None
                            } else {
                                Some(para_text)
                            }
                        }
                        _ => None,
                    }
                })
                .collect();

            text_parts.join("\n")
        })
}
