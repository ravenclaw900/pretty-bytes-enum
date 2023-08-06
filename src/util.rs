pub fn round_float(num: f64, round_places: u8) -> f64 {
    let exponent = 10_f64.powi(round_places.into());
    (num * exponent).round() / exponent
}
