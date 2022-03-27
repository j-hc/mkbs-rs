# MKBS-rs

Multi-Key Binary Search Implementation in Rust  

## Usage
```toml
# Cargo.toml
mkbs = { git = "https://github.com/j-hc/mkbs-rs" }
```
<br>

Pseudo code:
```rust

use mkbs::MKBS;

let keys = &[123, 3131];
let vec_of_vals = (0..4123).collect::<Vec<_>>();

let results = vec_of_vals.mkbs(keys);
println!("{results:?}");
// [Ok(123), Ok(3131)]

/// If the value is not found then [`Result::Err`] is returned, containing
/// the index where a matching element could be inserted while maintaining
/// sorted order, just like in the std library binary search.
/// In the case of [`MKBS::mkbs()`], it mostly returns [`Option::None`] in the place
/// of possible indices.
/// If you want every not-found element to have a possible index then you should
/// use [`MKBS::mkbs_all()`], note that it is almost two times slower.
```