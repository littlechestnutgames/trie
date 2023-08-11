# LittleChestnutGames Trie

I didn't find a trie implementation that fit my needs on crates.io, so I wrote one.
This library provides a generalized Trie data structure.

## Features

- Add, remove, and search keys and data.
- Optional data (Option<T>).
- Lightweight and easy to use.

## Installation

Add this crate as a dependency in your `Cargo.toml`:

```toml
[dependencies]
trie = { git = "https://github.com/littlechestnutgames/trie" }
```

## Usage
```rust
crate trie;
use trie::Trie;

fn your_fn() {
    // Instance your Trie.
    let mut trie = Trie::<String>::default();

    // Fill it with keys and/or data.
    trie.add("my_first_cool_key", Some("This key has data associated with it.".to_string()));
    trie.add("my_second_cool_key", None);
    trie.add("my_first_cool_key_1", Some("Last key didn't have data.".to_string()));
    trie.add("how_about_something_different", "I'm different.".to_string());

    // Search your Trie. The keys come back in a Vec<String>.
    let keys = trie.get_keys_under_prefix("my");
    println!("{:?}\n", keys);
    // outputs [ "my_first_cool_key", "my_second_cool_key", "my_first_cool_key_1" ]

    // Remove keys too!
    trie.remove("my_first_cool_key");
    let keys = trie.get_keys_under_prefix("my");
    println!("{:?}\n", keys);
    // outputs [ "my_second_cool_key", "my_first_cool_key_1" ]
}
```
## Thanks for checking out Trie.

