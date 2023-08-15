use std::{collections::HashMap, sync::Arc};

use super::tokenizer::Tokenizer;

pub struct Trie<T> {
    count: u64,
    pub children: HashMap<String, Trie<T>>,
    pub data: Option<T>,
    is_key_end: bool,
    tokenizer: Tokenizer,
}

impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            count: 0,
            data: None,
            is_key_end: false,
            tokenizer: Tokenizer::Slice(1)
        }
    }
}

impl<T> Trie<T> {
    /// Creates a new `Trie<T>` that has a `Tokenizer::Slice` set to `usize` `length`.
    ///
    /// Arguments
    ///
    /// `length` - A `usize` that represents the length in utf8 glyphs to split the key into.
    ///
    /// Returns
    ///
    /// `Trie<T>` - A `Trie<T>` configured to split given keys into `length` glyphs during operations.
    pub fn with_slice(length: usize) -> Self {
        Self {
            children: HashMap::new(),
            count: 0,
            data: None,
            is_key_end: false,
            tokenizer: Tokenizer::Slice(length)
        }
    }

    /// Creates a new `Trie<T>` that has a `Tokenizer::Delimiter` with splitting
    /// delimiter `delimiter`.
    ///
    /// Arguments
    ///
    /// `delimiter` - A `String` representing how we want to segment keys through
    /// the `Trie` structure.
    ///
    /// Returns
    ///
    /// `Trie<T>` - A new `Trie` with `tokenizer` set to `Tokenizer::Delimiter(delimiter)`
    pub fn with_delimiter(delimiter: String) -> Self {
        Self {
            children: HashMap::new(),
            count: 0,
            data: None,
            is_key_end: false,
            tokenizer: Tokenizer::Delimiter(delimiter)
        }
    }

    /// Creates a new `Trie<T>` that has a `Tokenizer::Custom` which the library
    /// user specifies their own tokenize and detokenize functions.
    ///
    /// Arguments
    ///
    /// `tokenize_fn` - `Arc<dyn Fn(String) -> Vec<String>>`, a function that takes in a `String` and returns a `Vec<String>`, wrapped in an `Arc`. This function is run on each key operation to split keys into different `Trie` levels.
    /// `detokenize_fn` - `Arc<dyn Fn(Vec<String>) -> String>`, a function that takes in a `Vec<String>`, wrapped in an `Arc`. This function is run to reassemble `Trie` levels into keys.
    ///
    /// Returns
    ///
    /// `Trie<T>` - A new `Trie` with `tokenizer` set to `Tokenizer::Custom`.
    pub fn with_custom_tokenization(
        tokenize_fn: Arc<dyn Fn(String) -> Vec<String>>,
        detokenize_fn: Arc<dyn Fn(Vec<String>) -> String>
    ) -> Self {
        Self {
            children: HashMap::new(),
            count: 0,
            data: None,
            is_key_end: false,
            tokenizer: Tokenizer::Custom(
                tokenize_fn.clone(),
                detokenize_fn.clone()
            )
        }
    }

    /// Creates a new nearly blank `Trie<T>`, clones the `tokenizer` field from the original `Trie<T>`.
    ///
    /// Returns
    ///
    /// `Trie<T>`
    pub fn new_from_current(&self) -> Self {
        let tokenizer = match &self.tokenizer {
            Tokenizer::Slice(length) => Tokenizer::Slice(*length),
            Tokenizer::Delimiter(delimiter) => Tokenizer::Delimiter(delimiter.clone()),
            Tokenizer::Custom(tokenize_fn, detokenize_fn) => Tokenizer::Custom(
                Arc::clone(tokenize_fn),
                Arc::clone(detokenize_fn)
            )
        };
        Self {
            children: HashMap::new(),
            count: 0,
            data: None,
            is_key_end: false,
            tokenizer
        }
    }
    /// Adds a complete `key` to the `Trie` structure.
    ///
    /// Arguments
    ///
    /// `key` - A `&str` which is a complete key.
    /// `data` - An `Option<T>` which is stored in the Trie at the end of the key.
    pub fn add(&mut self, key: &str, data: Option<T>) {
        let keystr = String::from(key);
        let mut trie = self;
        for token in trie.tokenizer.tokenize(keystr) {
            let new_child = trie.new_from_current();
            trie = trie.children.entry(token).or_insert_with(|| new_child);
            trie.count += 1;
        }
        trie.data = data;
        trie.is_key_end = true;
    }

    /// Removes a `key` from the `Trie` structure.
    ///
    /// Arguments
    ///
    /// `key` - A `&str` that you're removing from the Trie
    pub fn remove(&mut self, key: &str) {
        if !self.exists(key) {
            return;
        }
        let mut tokens = self.tokenizer.tokenize(String::from(key));
        let mut is_first = true;
        while tokens.len() > 0 {
            let detokenized = self.tokenizer.detokenize(tokens.clone());
            let token = detokenized.as_str();
            if let Some(trie) = self.get_mut(token) {
                trie.count -= 1;
                if is_first {
                    is_first = false;
                    trie.is_key_end = false;
                }

                trie.prune_unused_children();
            } else {
                return;
            }
            tokens.pop();
        }

        self.prune_unused_children();
    }

    /// Removes all children from the `Trie` that have a 0 count.
    fn prune_unused_children(&mut self) {
        let unused_children: Vec<String> = self.children
            .iter()
            .filter(|(_, v)| v.count == 0)
            .map(|(k, _)| k.clone())
            .collect();
        for key in unused_children {
            self.children.remove(&key);
        }
    }

    /// Checks if a given key exists in the Trie hierarchy.
    ///
    /// Arguments
    /// `key` - A `&str` which represent a full key in the hierarchy.
    ///
    /// Returns
    /// `bool` - true if the key exists.
    pub fn exists(&self, key: &str) -> bool {
        if let Some(trie) = self.get(key) {
            return trie.is_key_end;
        }
        false
    }

    /// Get an immutable Trie from the Trie queried.
    ///
    /// Arguments:
    /// `key` - A `&str` representing the full path to the Trie you're querying.
    ///
    /// Returns:
    /// `Option<&Trie<T>>`
    /// * When the `key` doesn't exist in Trie's children, `None` is returned.
    /// * When the `key` exists in the Trie's children, the last child will be returned as
    /// `Some(&Trie<T>)`
    pub fn get(&self, key: &str) -> Option<&Trie<T>> {
        let mut trie = self;
        let tokens = self.tokenizer.tokenize(String::from(key));
        let mut iter = tokens.iter();

        while let Some(token) = iter.next() {
            if let Some(t) = trie.children.get(token) {
                trie = t;
            } else {
                return None;
            }
        }
        Some(trie)
    }

    /// Get a mutable Trie from the Trie queried.
    ///
    /// Arguments:
    /// `key` - A `&str` representing the full path to the Trie you're querying.
    ///
    /// Returns:
    /// `Option<&mut Trie<T>>`
    /// * When the `key` doesn't exist in Trie's children, `None` is returned.
    /// * When the `key` exists in the Trie's children, the last child will be returned as
    /// `Some(&mut Trie<T>)`
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Trie<T>> {
        let mut trie = self;
        let tokens = trie.tokenizer.tokenize(String::from(key));
        let mut iter = tokens.iter();

        while let Some(token) = iter.next() {
            if let Some(t) = trie.children.get_mut(token) {
                trie = t;
            } else {
                return None;
            }
        }

        Some(trie)
    }

    /// Gets one or more Tries from the Trie queried, and returns it in vector.
    ///
    /// Arguments:
    /// `key` - A `&str` representing a partial or full path to the `Trie` you're querying.
    ///
    /// * If the exact Trie path `key` exists, this will return a single `Trie`
    /// * If the last path segment doesn't contain an exact match on the penultimate `Trie`
    ///   it will try a `fuzzy_get` to return any Trie children that match.
    ///
    /// Returns
    /// `Vec<&Trie<T>>
    pub fn fuzzy_get(&self, key: &str) -> Vec<&Trie<T>> {
        let mut trie = self;
        let mut tokens = self.tokenizer.tokenize(String::from(key));
        let last_token = tokens.pop();
        let mut iter = tokens.iter();
        let mut items: Vec<&Trie<T>> = Vec::new();
        while let Some(token) = iter.next() {
            if let Some(t) = trie.children.get(token) {
                trie = t;
            } else {
                return items;
            }
        }

        if let Some(token) = last_token {
            let fuzzy_keys = trie.children.keys().filter(|k| k.contains(token.as_str()));

            for k in fuzzy_keys {
                if let Some(t) = trie.get(k) {
                    items.push(t);
                }
            }
        }
        items
    }

    pub fn get_keys_by_partial_path(&self, key: &str) -> Vec<String> {
        let mut trie = self;
        let mut tokens = self.tokenizer.tokenize(String::from(key));
        let last_token = tokens.pop().unwrap_or(String::from(""));
        let mut iter = tokens.iter();
        let items: Vec<String> = Vec::new();
        while let Some(token) = iter.next() {
            if let Some(t) = trie.children.get(token) {
                trie = t;
            } else {
                return items;
            }
        }
        if let Some(_) = trie.get(last_token.as_str()) {
            return vec![String::from(key)];
        }

        // These are fuzzy matches.
        trie.children
            .iter()
            .filter(|(k, _)| k.starts_with(last_token.trim()))
            .map(|(k, _)| {
                let mut t = tokens.clone();
                t.push(String::from(k));
                trie.tokenizer.detokenize(t)
            })
            .collect()
    }


    /// Collects all keys of children under a given prefix.
    ///
    /// Arguments:
    /// `key` - A `&str` representing the prefix you're searching under.
    ///
    /// Returns:
    /// `Vec<String>` - A vector of strings containing the collected keys under the prefix.
    pub fn get_keys_under_prefix(&self, key: &str) -> Vec<String> {
        let mut keys = Vec::new();
        let search_keys = self.get_keys_by_partial_path(key);
        for k in search_keys {
            if let Some(trie) = self.get(k.as_str()) {
                trie.get_keys_recursive(k.as_str(), &mut keys);
            }
        }
        keys
    }

    fn get_keys_recursive(&self, key: &str, keys: &mut Vec<String>) {
        if self.is_key_end {
            keys.push(String::from(key));
        }
        for (token, trie) in &self.children {
            let new_key = self.tokenizer.detokenize(
                vec![
                    String::from(key),
                    String::from(token)
                ]
            );
            trie.get_keys_recursive(&new_key, keys);
        }
    }
}
