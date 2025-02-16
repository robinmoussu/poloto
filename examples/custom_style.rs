//PIPE me to a file!
fn main() {
    let x = (0..50).map(|x| (x as f32 / 50.0) * 10.0);

    let mut s = poloto::plot(
        "Demo: you can change the style of the svg file itself!",
        "x",
        "y",
    );

    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));

    let mut e = tagger::new(tagger::upgrade_write(std::io::stdout()));

    poloto::default_svg(&mut e, tagger::no_attr(), |d| {
        d.put_raw(format_args!(
            "<style>{}</style>",
            poloto::STYLE_CONFIG_LIGHT_DEFAULT
        ));
        d.put_raw(
            r###"
        <defs>
            <pattern id="pattern" patternUnits="userSpaceOnUse" width="50" height="50">
                <circle cx="25" cy="25" r="25" fill="black" fill-opacity="0.2"/>
            </pattern>
            <pattern id="pattern2" patternUnits="userSpaceOnUse" width="10" height="10">
                <line x1="0" y1="5" x2="10" y2="5" stroke="red" stroke-width="5"/>
                </pattern> 
        </defs>
        <style>
        .poloto_background.poloto_background{
            fill: url(#pattern);
        }
        .poloto0stroke.poloto0stroke{
            stroke-dasharray:10 2 2;
        }
        .poloto1fill.poloto1fill{
            fill: url(#pattern2);
        }
        </style>
        "###,
        );
        s.render(d.writer());
    });
}
