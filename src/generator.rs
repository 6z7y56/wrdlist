use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, Write, BufWriter};

#[derive(Debug)]
pub enum PatternToken {
    Char(char),
    Range(u32, u32),
    LowercaseAlphabet,
    UppercaseAlphabet,
    Digits,
    Symbols,
}

#[derive(Debug)]
pub struct WordlistGenerator {
    pub tokens: Vec<PatternToken>,
}

impl WordlistGenerator {
    pub fn new(pattern: &str) -> Result<Self, String> {
        let tokens = Self::parse_pattern(pattern)?;
        Ok(Self { tokens })
    }

    fn parse_pattern(pattern: &str) -> Result<Vec<PatternToken>, String> {
        let mut tokens = Vec::new();
        let chars = pattern.chars().collect::<Vec<_>>();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '.' => {
                    if i + 1 < chars.len() {
                        tokens.push(PatternToken::Char(chars[i + 1]));
                        i += 1;
                    } else {
                        return Err("Invalid pattern: '.' at the end".to_string());
                    }
                }
                '[' => {
                    if i + 2 < chars.len() && chars[i + 1].is_digit(10) {
                        let mut j = i + 1;
                        while j < chars.len() && chars[j].is_digit(10) {
                            j += 1;
                        }
                        if j < chars.len() && chars[j] == '-' {
                            let start_str: String = chars[i + 1..j].iter().collect();
                            let start = start_str.parse::<u32>().map_err(|_| "Invalid range start".to_string())?;
                            
                            let mut k = j + 1;
                            while k < chars.len() && chars[k].is_digit(10) {
                                k += 1;
                            }
                            if k < chars.len() && chars[k] == ']' {
                                let end_str: String = chars[j + 1..k].iter().collect();
                                let end = end_str.parse::<u32>().map_err(|_| "Invalid range end".to_string())?;
                                
                                if start >= end {
                                    return Err("Invalid range: start must be less than end".to_string());
                                }
                                tokens.push(PatternToken::Range(start, end));
                                i = k;
                            } else {
                                return Err("Invalid range syntax".to_string());
                            }
                        } else {
                            return Err("Invalid range syntax".to_string());
                        }
                    } else {
                        return Err("Invalid range syntax".to_string());
                    }
                }
                '!' => tokens.push(PatternToken::LowercaseAlphabet),
                '@' => tokens.push(PatternToken::UppercaseAlphabet),
                '#' => tokens.push(PatternToken::Digits),
                '%' => tokens.push(PatternToken::Symbols),
                _ => {
                    if chars[i].is_numeric() {
                        return Err("Static numbers are not allowed. Use ranges like [0-9] instead.".to_string());
                    } else if !chars[i].is_alphanumeric() && chars[i] != '.' {
                        return Err(format!("Invalid character: '{}'. Allowed characters: !, @, #, %, ., [0-9]", chars[i]));
                    } else if chars[i] != '.' && (i == 0 || chars[i - 1] != '.') {
                        return Err(format!("Static character '{}' must be preceded by a '.'", chars[i]));
                    }
                }
            }
            i += 1;
        }

        Ok(tokens)
    }

    pub fn generate(&self) -> Vec<String> {
        let mut wordlist = vec![String::new()];

        for token in &self.tokens {
            let mut new_wordlist = Vec::new();

            for word in &wordlist {
                match token {
                    PatternToken::Char(c) => new_wordlist.push(word.clone() + &c.to_string()),
                    PatternToken::Range(start, end) => {
                        for num in *start..=*end {
                            new_wordlist.push(word.clone() + &num.to_string());
                        }
                    }
                    PatternToken::LowercaseAlphabet => {
                        for c in 'a'..='z' {
                            new_wordlist.push(word.clone() + &c.to_string());
                        }
                    }
                    PatternToken::UppercaseAlphabet => {
                        for c in 'A'..='Z' {
                            new_wordlist.push(word.clone() + &c.to_string());
                        }
                    }
                    PatternToken::Digits => {
                        for c in '0'..='9' {
                            new_wordlist.push(word.clone() + &c.to_string());
                        }
                    }
                    PatternToken::Symbols => {
                        let symbols = ['!', '@', '#', '$', '%', '^', '&', '*', '?'];
                        for &c in &symbols {
                            new_wordlist.push(word.clone() + &c.to_string());
                        }
                    }
                }
            }

            wordlist = new_wordlist;
        }

        wordlist
    }

    pub fn shuffle(&self, wordlist: &mut Vec<String>) {
        let mut rng = rand::thread_rng();
        wordlist.shuffle(&mut rng);
    }

    pub fn reverse(&self, wordlist: &mut Vec<String>) {
        wordlist.reverse();
    }
}

/// Save the wordlist to a file.
pub fn save_to_file(wordlist: Vec<String>, path: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for word in wordlist {
        writeln!(writer, "{}", word)?;
    }

    writer.flush()?;
    Ok(())
}
