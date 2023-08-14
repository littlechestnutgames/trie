# LittleChestnutGames Trie

I didn't find a trie implementation that fit my needs on crates.io, so I wrote one.
This library provides a generalized Trie data structure.

## Features

- Add, remove, and search keys and data.
- Optional data (Option<T>).
- Tokenization by delimiters or slice length.
- Lightweight and easy to use.

## Installation

Add this crate as a dependency in your `Cargo.toml`, do one of the following:

```toml
# For crates.io released version.
[dependencies]
littlechestnutgames-trie = "2.0.0"
```

```toml
# For the bleeding edge.
[dependencies]
littlechestnutgames-trie = { git = "https://github.com/littlechestnutgames/trie.git" }
```

```shell
cargo add littlechestnutgames-trie
```

## Usage
```rust
use trie::Trie;

fn main() {
    basic_trie();
    delimiters();
    slicing();
}

// The following example sets up a classic trie with single characters per Trie.
fn basic_trie() {
    // Instance your Trie.
    let mut trie = Trie::<String>::default();

    // Fill it with keys and/or data.
    trie.add("my_first_cool_key", Some(String::from("This key has data associated with it.")));
    // The underlying trie structure looks a lot like this.
    // m -> y -> _ -> f -> i -> r -> s -> t -> _ -> c -> o -> o -> l -> _ -> k
    // e -> y

    trie.add("my_second_cool_key", None);
    trie.add("my_first_cool_key_1", Some(String::from("Last key didn't have data.")));
    trie.add("how_about_something_different", Some(String::from("I'm different.")));

    // Search your Trie. The keys come back in a Vec<String>.
    println!("{:?}\n", trie.get_keys_under_prefix("my"));
    // Output: ["my_first_cool_key", "my_first_cool_key_1", "my_second_cool_key"]

    // Remove keys too!
    trie.remove("my_first_cool_key");
    println!("{:?}\n", trie.get_keys_under_prefix("my"));
    // Output: ["my_first_cool_key_1", "my_second_cool_key"]
}

// This example sets up a trie that splits keys using a delimiter, resulting in better storage.
fn delimiters() {
    let mut trie = Trie::<String>::with_delimiter(String::from("_"));
    trie.add("a_trie_with_delimiters", None);
    // The underlying trie structure looks a lot this this.
    // a -> trie -> with -> delimiters

    trie.add("a_trie_with_more_levels", None);
    trie.add("a_trie_that_diverges_at_the_word_that", None);
    trie.add("different_trie", None);

    println!("{:?}\n", trie.get_keys_under_prefix("a_trie_with"));
    // Output: ["a_trie_with_more_levels", "a_trie_with_delimiters"]
}

// You can customize slicing length as well.
fn slicing() {
    let mut trie = Trie::<String>::with_slice(2);

    trie.add("thisisashortkey", None);
    // The underlying structure looks a lot like this.
    // th -> is -> is -> as -> ho -> rt -> ke -> y

    trie.add("thisisalittlebitlongerofakey", None);

    println!("{:?}", trie.get_keys_under_prefix("thisisa"));
    // Output: ["thisisalittlebitlongerofakey", "thisisashortkey"]

    println!("{:?}", trie.get_keys_under_prefix("thisisas"));
    // Output: ["thisisashortkey"]
}
```
## Read CHANGES.md for changes between versions.

## Thanks for checking out Trie.

