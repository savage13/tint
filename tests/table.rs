extern crate tint;

#[test]
fn table() {
    println!("| Name                 | Color                                                  |");
    println!("|----------------------|--------------------------------------------------------|");
    for name in tint::names() {
        let color = tint::Color::from(&name);
        println!(
            "| {name:20} | ![{hex}](https://placehold.it/100x15/{hex}?text=+) |",
            name = name,
            hex = color.to_hex()
        );
    }
}
