pub fn offset_hex(hex_color: &str, offset: u8) -> String {
    let mut r = u8::from_str_radix(&hex_color[1..3], 16).unwrap();
    let mut g = u8::from_str_radix(&hex_color[3..5], 16).unwrap();
    let mut b = u8::from_str_radix(&hex_color[5..7], 16).unwrap();

    r = r.saturating_sub(offset);
    g = g.saturating_sub(offset);
    b = b.saturating_sub(offset);

    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn lerp(a: f64, b: f64, c: f64) -> f64 {
    a + (b - a) * c
}
