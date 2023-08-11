use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie<T: std::fmt::Debug> {
    count: u64,
    children: HashMap<char, Trie<T>>,
    data: Option<T>,
    is_key_end: bool,
}

impl<T: std::fmt::Debug> Default for Trie<T> {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            count: 0,
            data: None,
            is_key_end: false,
        }
    }
}

impl<T: std::fmt::Debug> Trie<T> {
    /// Adds a complete `key` to the `Trie` structure.
    ///
    /// Arguments
    ///
    /// `key` - A `&str` which is a complete key.
    /// `data` - An `Option<T>` which is stored in the Trie at the end of the key.
    pub fn add(&mut self, key: &str, data: Option<T>) {
        let mut trie = self;
        for working_char in key.chars() {
            trie = trie.children.entry(working_char).or_insert_with(|| Trie::default());
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
        let mut key_chars = key;
        if !self.exists(key) {
            return;
        }

        let original_length = key_chars.len();
        while key_chars.len() > 0 {
            if let Some(trie) = self.get_mut(key_chars) {
                trie.count -= 1;
                if original_length == key_chars.len() {
                    println!("Setting is_key_end to false on final Trie in key.");
                    trie.is_key_end = false;
                }

                trie.prune_unused_children();
            } else {
                return;
            }
            key_chars = &key_chars[..key_chars.len()-1];
        }

        self.prune_unused_children();
    }

    /// Removes all children from the `Trie` that have a 0 count.
    fn prune_unused_children(&mut self) {
        let unused_children: Vec<char> = self.children
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
        let mut chars = key.chars();
        while let Some(working_char) = chars.next() {
            if let Some(t) = trie.children.get(&working_char) {
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
        let mut chars = key.chars();
        while let Some(working_char) = chars.next() {
            if let Some(t) = trie.children.get_mut(&working_char) {
                trie = t;
            } else {
                return None;
            }
        }

        Some(trie)
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
        if let Some(trie) = self.get(key) {
            trie.get_keys_recursive(key, &mut keys);
        }
        keys
    }

    fn get_keys_recursive(&self, key: &str, keys: &mut Vec<String>) {
        if self.is_key_end {
            keys.push(key.to_string());
        }
        for (working_char, trie) in &self.children {
            let mut new_key = key.to_string();
            new_key.push(*working_char);
            trie.get_keys_recursive(&new_key, keys);
        }
    }
}
