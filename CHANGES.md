# Changes to littlechestnutgames-trie

## 2.0.0
### Breaking Changes
* `tokenizer` field added to `Trie<T>` struct.
* `Debug` removed from derives.

### Dependencies
* `unicode-segmentation` to uniformly tokenize keys.

### Modifications**
* `Trie<T>` code moved from lib.rs into `trie.rs`.
* `trie` module imported and its contents rexported in lib.
* `Trie<T>` as stated before has a new field, `tokenizer`.
    * This field allows the user to configure how key strings are broken apart.
* `Trie<T>` methods `String` and `Tokenizer` instead of `char` for tokenizing.
* `Default` `Tokenization` method set to `Tokenizer::Slice(1)` to emulate 1.0.0 style `Trie` mapping.
* Changes to the `README.md`.

### New Features**
* `Tokenizer` enum in file `tokenizer.rs`.
* `Tokenizer::Slice(usize)` lets a user slice strings at a specified length.
* `Tokenizer::Delimiter(String)` lets a user slice strings by `delimiter`.
* `Tokenizer::tokenize` breaks a given `key` into parts using the specified tokenizer strategy.
    * This uses `unicode-segmentation::UnicodeSegmentation` to parse the strings into graphemes, to ensure we don't split in the middle of a double width character.
* `Tokenizer::detokenize` joins `tokens` back together using the specified tokenizer strategy.
* `with_slice` method to `Trie<T>` impl.
    * This function creates a new `Trie<T>` that allows the user to choose the maximum length they'd like keys to be split into.
* `with_delimiter` method to `Trie<T>` impl.
    * This function creates a new `Trie<T>` that makes the keys split tokenize and detokenize at a specific `delimiter`.
* `new_from_current` method that creates a new blank `Trie<T>` that has the same tokenization method as the calling `Trie<T>`.
* `fuzzy_get`, which returns `Trie<T>` that are on a similar level, but begin with the same prefix.
* `get_keys_by_partial_path` which returns a `Vec<String>` of keys that match are present which contain the prefix specified in `key`.
* `CHANGES.md`. Hello.

## 1.0.0
* **New Features**
* struct `Trie<T>`
* `Default` implementation.
* `add` method to add keys to the trie structure.
* `remove` method to remove keys from the trie structure.
    * `prune_unused_children` implemented to help `remove` function.
* `exists` method to check if keys are present.
* `get` and `get_mut` methods to get a `Trie` by it's fully qualified key, immutable and mutable respectively.
* `get_keys_under_prefix` to get all the keys starting with a specified prefix.
    * `get_keys_recursive` as a helper to the `get_keys_recursive` function.
* Setup `Cargo.toml`

