use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone)]
pub enum Tokenizer {
    /// A slice based `Tokenizer`
    ///
    /// Will tokenize `String` and detokenize `Vec<String>` by `usize` length.
    Slice(usize),
    /// A delimiter based `Tokenizer`
    ///
    /// Will tokenize `String` and detokenize `Vec<String>` using a `String` delimiter.
    Delimiter(String)
}

impl Tokenizer {
    /// Breaks a `String` into pieces based on `Tokenizer` type.
    ///
    /// * A `Tokenizer::Slice(length)` will split the String by `length`.
    /// * A `Tokenizer::Delimiter(characters)` will split the String by `characters`.
    ///
    /// Arguments
    /// `key` - A `String` that you want to be broken into pieces.
    ///
    /// Returns
    /// `Vec<String>`
    pub fn tokenize(&self, key: String) -> Vec<String> {
        match self {
            Self::Slice(length) => {
                let mut slices = Vec::new();
                let mut current_slice = String::new();

                for grapheme in key.graphemes(true) {
                    if current_slice.len() + grapheme.len() <= *length {
                        current_slice.push_str(grapheme);
                    } else {
                        slices.push(current_slice.clone());
                        current_slice.clear();
                        current_slice.push_str(grapheme);
                    }
                }

                if !current_slice.is_empty() {
                    slices.push(current_slice);
                }
                slices
            }
            Self::Delimiter(delimiter) => {
                key.split(delimiter).map(|s| s.to_string()).collect()
            }
        }
    }

    /// Joins pieces of a `String` together based on `Tokenizer` type.
    ///
    /// * A `Tokenizer::Slice` will join elements together without a delimiter.
    /// * A `Tokenizer::Delimiter` will join elements together with a delimiter.
    /// Arguments
    /// `tokens` - A `Vec<String>` that you'd like to be a single String.
    ///
    /// Returns
    /// `String`
    pub fn detokenize(&self, tokens: Vec<String>) -> String {
        match self {
            Self::Slice(_) => {
                tokens.join("")
            }
            Self::Delimiter(delimiter) => {
                tokens.join(delimiter)
            }
        }
    }
}

