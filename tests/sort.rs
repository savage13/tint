
extern crate tint;

fn by_rgb(a: &str, b: &str) -> std::cmp::Ordering {
    let ca = tint::Color::from(a);
    let cb = tint::Color::from(b);
    tint::compare_by_rgb(&ca, &cb)
}
fn by_hsv(a: &str, b: &str) -> std::cmp::Ordering {
    let ca = tint::Color::from(a);
    let cb = tint::Color::from(b);
    tint::compare_by_hsv(&ca, &cb)
}

#[test]
fn sort_rgb() {
    let mut keys = tint::names();
    keys.sort_by(|a, b| by_rgb(a,b));
    for k in keys.iter() {
        let c = tint::Color::from(k);
        println!("{:20}: {} {}", k, c, c.to_hex());
    }
}

#[test]
fn sort_hue() {
    let mut keys = tint::names();
    keys.sort_by(|a, b| by_hsv(a,b));
    for k in keys.iter() {
        let c = tint::Color::from(k);
        let hsv = c.to_hsv();
        println!("{:20}: {} {} {:.3} {:.3} {:.3}", k, c, c.to_hex(), hsv.0,hsv.1,hsv.2);
    }
}
