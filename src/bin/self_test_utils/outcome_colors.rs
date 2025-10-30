
pub fn apply_color_spectrum(value: f64, color_points: &[(f64, (f64, f64, f64))]) -> (f64, f64, f64) {
    for i in 0..color_points.len()-1 {
        let (min, (r1, g1, b1)) = color_points[i];
        let (max, (r2, g2, b2)) = color_points[i+1];

        if min <= value && value <= max {
            let fraction = (value - min) / (max - min);

            return (
                r1 + (r2 - r1) * fraction,
                g1 + (g2 - g1) * fraction,
                b1 + (b2 - b1) * fraction,
            );
        }
    }

    // If we're still here, the value must be either less than the first point
    // or greater than the last.

    if value < color_points[0].0 {
        color_points[0].1
    } else {
        color_points[color_points.len()-1].1
    }
}

pub fn ansi_truecolor((r, g, b): (f64, f64, f64)) -> String {
    format!(
        "\x1b[38;2;{};{};{}m",
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    )
}

