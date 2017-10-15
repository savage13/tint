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

| Color | Name |
|-------|------|
| ![#00000](https://placehold.it/100x15/000000?text=+) | Black |
| ![#c0c0c0](https://placehold.it/100x15/c0c0c0?text=+) | Silver |
| ![#808080](https://placehold.it/100x15/808080?text=+) | Gray |
| ![#fffff](https://placehold.it/100x15/ffffff?text=+) | White |
| ![#800000](https://placehold.it/100x15/800000?text=+) | Maroon |
| ![#ff0000](https://placehold.it/100x15/ff0000?text=+) | Red |
| ![#800080](https://placehold.it/100x15/800080?text=+) | Purple |
| ![#ff00ff](https://placehold.it/100x15/ff00ff?text=+) | Fuchsia |
| ![#008000](https://placehold.it/100x15/008000?text=+) | Green |
| ![#00ff00](https://placehold.it/100x15/00ff00?text=+) | Lime |
| ![#808000](https://placehold.it/100x15/808000?text=+) | Olive |
| ![#ffff00](https://placehold.it/100x15/ffff00?text=+) | Yellow |
| ![#000080](https://placehold.it/100x15/000080?text=+) | Navy |
| ![#0000ff](https://placehold.it/100x15/0000ff?text=+) | Blue |
| ![#008080](https://placehold.it/100x15/008080?text=+) | Teal |
| ![#00ffff](https://placehold.it/100x15/00ffff?text=+) | Aqua |

