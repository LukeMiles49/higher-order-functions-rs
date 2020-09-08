# higher_order_functions

[Crate](https://crates.io/crates/higher_order_functions)

[Documentation](https://docs.rs/higher_order_functions)

[Repository](https://github.com/LukeMiles49/higher-order-functions-rs)

[Changelog](https://github.com/LukeMiles49/higher-order-functions-rs/blob/master/CHANGELOG.md)

A small collection of traits for implementing higher order functions.

## Init:

```rust
use higher_order_functions::Init;

struct House { number: usize }

// [T; N]: Init<T, usize>
let road = <[House; 3]>::init(|i| House { number: i + 1 });

assert_eq!(road[0].number, 1);
assert_eq!(road[1].number, 2);
assert_eq!(road[2].number, 3);
```

## Map:

```rust
use higher_order_functions::Map;

let arr = [1, 4, 6, -3, 6].map(|x| x * 2);

assert_eq!(arr, [2, 8, 12, -6, 12]);
```

```rust
use higher_order_functions::Map;

let arr = [1, 4, 6, -3, 6].map(f64::from);

assert_eq!(arr, [1.0, 4.0, 6.0, -3.0, 6.0]);
```

## Zip:

```rust
use higher_order_functions::Zip;

let a = [1, 2, 3];
let b = ["a", "b", "c"];

let arr = a.zip(b, |ax, bx| (ax, bx));

assert_eq!(arr, [(1, "a"), (2, "b"), (3, "c")]);
```

```rust
use higher_order_functions::Zip;

let a = [1, 2, 3];
let b = [4, 5, 6];

let arr = a.zip(b, |ax, bx| ax * bx);

assert_eq!(arr, [4, 10, 18]);
```

## Section:

```rust
use higher_order_functions::Section;

let a: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

let arr: [u32; 4] = a.section(3); // Extracts 4 elements starting at a[3]

assert_eq!(arr, [4, 5, 6, 7]);
```

To use this, add it as a dependency to your Cargo.toml:
```toml
[dependencies]
higher_order_functions = "^0.1.1"
```
