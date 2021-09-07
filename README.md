bit_combi_iter
==============

bit_combi_iter is a small dependency-free crate to enumerate all bit combinations less than given unsigned integer value keeping the number of bits.

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

## License

Distributed under [the MIT License](./LICENSE.txt).
