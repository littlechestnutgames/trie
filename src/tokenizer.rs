use std::sync::Arc;

use unicode_segmentation::UnicodeSegmentation;

pub enum Tokenizer {
    /// A slice based `Tokenizer`
    ///
    /// Will tokenize `String` and detokenize `Vec<String>` by `usize` length.
    Slice(usize),
    /// A delimiter based `Tokenizer`
    ///
    /// Will tokenize `String` and detokenize `Vec<String>` using a `String` delimiter.
    Delimiter(String),
    /// A custom user defined `Tokenizer`
    ///
    /// Arguments
    ///
    /// `Box<dyn Fn(String) -> Vec<String>>` - A function that will be used to `tokenize` a key into tokens.
    /// `Box<dyn Fn(Vec<String>) -> String` - A function that will be used to `detokenize` a `Vec<String>` of tokens into a `String`.
    ///
    /// Will tokenize and detokenize in a user defined way.
    Custom(Arc<dyn Fn(String) -> Vec<String>>, Arc<dyn Fn(Vec<String>) -> String>)
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
            Self::Custom(tokenize_fn, _) => tokenize_fn(key)
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
            Self::Custom(_, detokenize_fn) => detokenize_fn(tokens)
        }
    }
}

