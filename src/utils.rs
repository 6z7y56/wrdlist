use crate::generator::PatternToken;

/// Estimates the size of the wordlist based on the provided tokens.
pub fn estimate_size(tokens: &[PatternToken]) -> Result<u64, String> {
    let mut total = 1u64;

    for token in tokens {
        let count = match token {
            PatternToken::Char(_) => 1,
            PatternToken::Range(start, end) => {
                if start > end {
                    return Err("Invalid range: start must be less than or equal to end".to_string());
                }
                (*end - *start + 1) as u64
            }
            PatternToken::LowercaseAlphabet => 26,
            PatternToken::UppercaseAlphabet => 26,
            PatternToken::Digits => 10,
            PatternToken::Symbols => 9,
        };

        total = total.checked_mul(count).ok_or("Overflow: The estimated size is too large to handle".to_string())?;
    }

    Ok(total)
}

/// Formats the file size into human-readable units (B, KB, MB, GB).
pub fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size < KB {
        format!("{}B", size)
    } else if size < MB {
        format!("{:.2}KB", size as f64 / KB as f64)
    } else if size < GB {
        format!("{:.2}MB", size as f64 / MB as f64)
    } else {
        format!("{:.2}GB", size as f64 / GB as f64)
    }
}
