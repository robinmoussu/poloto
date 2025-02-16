// PIPE me to a file!
fn main() {
    // https://mathworld.wolfram.com/HeartCurve.html
    let heart = |t: f64| {
        [
            16.0 * t.sin().powi(3),
            13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos(),
        ]
    };

    let range = (0..100).map(|x| x as f64 / 100.0).map(|x| x * 6.0 - 3.0);

    poloto::plot("Heart Graph", "x", "y")
        .line_fill("heart", range.map(|x| heart(x)))
        .preserve_aspect()
        .simple_theme(poloto::upgrade_write(std::io::stdout()));
}
