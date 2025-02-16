use poloto::formatm;
fn main() {
    let x = (0..500).map(|x| (x as f32 / 500.0) * 10.0);

    let mut plotter = poloto::plot("cos per year", "year", "cows");

    plotter.line(formatm!("test {}", 1), x.clone().map(|x| [x, x.cos()]));

    plotter.line(formatm!("test {}", 2), x.clone().map(|x| [x, x.sin()]));

    plotter.simple_theme_dark(poloto::upgrade_write(std::io::stdout()));
}
