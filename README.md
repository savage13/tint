tint
====
Color creation and manipulation in rust

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
tint = "1.0.0"
```

and this to your crate root:

```rust
extern crate tint;
```

### Example
```rust
extern crate tint;
use tint::Color;

fn main() {
    let purple = Color::from("purple");
    println!("purple: {}", purple);
    // purple: (1.000, 0.000, 1.000, 1.000)

    let green = Color::from("#00ff00");
    println!("green: {}", green);
    // green: (0.000, 1.000, 0.000, 1.000)
}
```

### Color names

Basic and Extended Colors from W3C and SVG are supported, along with colors from the XKCD color database through tint::xkcd().

