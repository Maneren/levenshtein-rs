# levenshtein-distance

Optimized implementation of levenshtein distance algorithm using only one extra
row. Has no external dependencies and supports `no_std`.

## Features

- calculating levenshtein distance
- custom weights
- higly optimized - uses only `O(b.length)` extra memory
- short-circuiting `levenshtein_is_within_limit`

## Usage

Add it to dependencies in `Cargo.toml`.

```toml
[dependencies]
levenshtein-distance = "1.0.0"
```

```rust
use levenshtein_distance::levenshtein;

assert_eq!(levenshtein("kitten", "kitten"), 0);
assert_eq!(levenshtein("kitten", "kittens"), 1);
assert_eq!(levenshtein("kitten", "sitting"), 3);
```
