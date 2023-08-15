# LittleChestnutGames Trie

I didn't find a trie implementation that fit my needs on crates.io, so I wrote one.
This library provides a generalized Trie data structure.

## Features

- Add, remove, and search keys and data.
- Optional data (Option<T>).
- Tokenization by delimiters or slice length.
- Custom tokenization and detokenization injection!
- Lightweight and easy to use.

## Installation

Add this crate as a dependency in your `Cargo.toml`, do one of the following:

```toml
# For crates.io released version.
[dependencies]
littlechestnutgames-trie = "3.0.0"
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
    custom_tokenization_example();
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

// You can also write your own custom tokenize and detokenize functions!
fn custom_tokenization_example() {
    // Store our tokenize and detokenize functions to pass into the Trie.
    let tokenize_function = Arc::new(my_tokenize);
    let detokenize_function = Arc::new(my_detokenize);

    // Creating the Trie with custom tokenizer.
    let mut trie = Trie::<String>::with_custom_tokenization(
        tokenize_function,
        detokenize_function
    );

    // Adding Trie data.
    trie.add("aaabbcccccd", Some(String::from("This is cool, right?")));
    // The underlying structure would look a lot like this.
    // aaa -> bb -> ccccc -> d

    trie.add("aaaccbbbbdbdd", None);
    trie.add("bbbbcccca1111", None);

    // Printing out the keys for our queries.
    println!("{:?}", trie.get_keys_under_prefix("aaab"));
    // Output: ["aaabbcccccd"]
    println!("{:?}", trie.get_keys_under_prefix("aa"));
    // Output: ["aaabbcccccd", "aaaccbbbbdbdd"]
    println!("{:?}", trie.get_keys_under_prefix("bbbbccc"));
    // Output: ["bbbbcccca1111"]
    println!("{:?}", trie.get_keys_under_prefix("bbccc"));
    // Output: [] <- This is correct because we don't have a bb key, only bbbb.
}

/// Tokenizes `key` to Vec<String> by grouping repeating characters together.
fn my_tokenize(key: String) -> Vec<String> {
    // The buffer to save all our keys to.
    let mut tokens = vec![];

    // The current token we're building in the loop.
    let mut current_token = String::from("");

    // The character from the last loop.
    let mut last_character = String::from("");

    // The iterator we're looping through.
    let mut keyclone = key.chars();

    while let Some(c) = keyclone.next() {
        let chstr = c.to_string();

        // If the new character we're seeing matches what we had last time, or
        // this is the first iteration, push the character onto the current_token.
        if chstr == last_character || last_character.is_empty() {
            current_token.push(c);
        }
        // We completed a token. Push it onto the tokens vec and clear the current_token.
        else {
            tokens.push(current_token.clone());
            current_token = chstr.clone();
        }
        // Mark the last character we saw.
        last_character = chstr;
    }
    // If we've still got characters left on the current_token, they're not
    // pushed to the tokens vec. Let's take care of that.
    if current_token.len() > 0 {
        tokens.push(current_token.clone());
    }

    tokens
}

/// Joins the repeating character sequences back together.
fn my_detokenize(tokens: Vec<String>) -> String {
    tokens.join("")
}
```
## Read CHANGES.md for changes between versions.

## Thanks for checking out Trie.

