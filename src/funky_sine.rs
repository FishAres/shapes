use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}



fn view(app: &App, frame: Frame) {

    let draw = app.draw();

    

    draw.background().color(AZURE);

    let window = app.window_rect();
    let radius = 20.0;

    for ind in (0..200).map(|y| y as f32) {

        let x = map_range((2.0 * (app.time) + ind*5.0) % window.w(), 0.0, window.w(), window.left(), window.right());
        // let x: Vec<_> = (-300i8..=300).map(f32::from).collect();

        let y = map_range((app.time / 22.0 * ind).sin(), -1.0, 1.0, window.bottom()-25.0, window.top()-25.0 );

        draw.ellipse()
            .color(PAPAYAWHIP)
            .x_y(x, y)
            .w_h(radius, radius);

    }

    draw.to_frame(app, &frame).unwrap();


}
