
//PIPE me to a file!
fn main() ->core::fmt::Result{
   
    let mut w=tagger::upgrade(std::io::stdout());
    let mut svg=tagger::new_element!(
        &mut w,
        "<svg class='poloto' height='{h}' width='{w}' viewBox='0 0 {w} {h}' xmlns='http://www.w3.org/2000/svg'>",
        w=poloto::WIDTH,
        h=poloto::HEIGHT)?;
    
    tagger::empty_element!(svg,"{}",
    r###"
    <defs>
        <pattern id="pattern" patternUnits="userSpaceOnUse" width="10" height="10">
            <circle cx="5" cy="5" r="5" fill="black" fill-opacity="0.2"/>
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
    "###)?;

    let mut s = poloto::plot(
        "Demo: you can change the style of the svg file itself!",
        "x",
        "y",
    );

    let x = (0..50).map(|x| (x as f32 / 50.0) * 10.0);

    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));
    s.render(&mut svg)?;
    
    tagger::end!(svg,"</svg>")?;
    Ok(())
    
}
