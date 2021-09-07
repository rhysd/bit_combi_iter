bit_combi_iter
==============
[![crates.io][crate-badge]][crate]
[![CI][ci-badge]][ci]

[bit_combi_iter][crate] is a small dependency-free crate to enumerate all bit combinations less than given unsigned integer value keeping the number of bits.

```rust
use bit_combi_iter::BitCombinations;

fn main() {
    let u = 0b00010100u8;

    let mut c = BitCombinations::new(u);

    println!("{:#b}", c.next().unwrap()); // => 0b00010010
    println!("{:#b}", c.next().unwrap()); // => 0b00010001
    println!("{:#b}", c.next().unwrap()); // => 0b00001100
    println!("{:#b}", c.next().unwrap()); // => 0b00001010
    println!("{:#b}", c.next().unwrap()); // => 0b00001001
    println!("{:#b}", c.next().unwrap()); // => 0b00000110
    println!("{:#b}", c.next().unwrap()); // => 0b00000101
    println!("{:#b}", c.next().unwrap()); // => 0b00000011
    println!("{}", c.next().is_none()); // => true
}
```

## Install

Add the crate to dependencies in your `Cargo.toml`. Using [cargo-edit][]:

```
cargo add bit_combi_iter
```

## API

### `BitCombinations<U>` (`U` can be `u8`, `u16`, `u32`, `u64`, `u128`)

A struct implementing `Iterator<Item=U>`. Size of this struct is the same as size of `U`. No additional allocation is necessary.

### `BitCombinations::<U>::new(init: U) -> Self`

`new` generates a `BitCombinations` instance initializing with state `init`.

### `BitCombinations::<U>::next(&mut self) -> Option<U>`

`next` method generates a next bit combination of the current state. The generated value is always smaller than the next state.

### `BitCombinations::<U>::peek(self) -> Option<U>`

`peek` method returns the current state. When the iterator finished iterating all combinations, this method returns `None`.

## Special Thanks

The algorithm was borrowed from [the blog post][thanks-herumi] by [@herumi](https://github.com/herumi).

## License

Distributed under [the MIT License](./LICENSE.txt).

[crate]: https://crates.io/crates/bit_combi_iter
[ci-badge]: https://github.com/rhysd/bit_combi_iter/actions/workflows/ci.yaml/badge.svg
[ci]: https://github.com/rhysd/bit_combi_iter/actions/workflows/ci.yaml
[doc]: https://docs.rs/crate/bit_combi_iter
[crate-badge]: https://img.shields.io/crates/v/bit_combi_iter.svg
[cargo-edit]: https://github.com/killercup/cargo-edit
[thanks-herumi]: https://github.com/herumi/blog/blob/main/bit-operation.md#%E3%83%93%E3%83%83%E3%83%88%E7%B5%84%E3%81%BF%E5%90%88%E3%82%8F%E3%81%9B%E3%81%AE%E3%83%91%E3%82%BF%E3%83%BC%E3%83%B3
