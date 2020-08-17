use nannou::prelude::*;

const stringus: [&str; 5] = ["Pawel", "like", "pig", "good", "oh no! is worm :("];
// )

fn main() {
    nannou::sketch(view).run()
}


fn view(app: &App, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    let w = app.window_rect();

    let draw = draw.scale((app.time * 0.1).cos());

    let max_side = w.right().max(w.top()); // wot
    let gap = 30.0;
    let len = (max_side / gap) as u32;
    for i in 1..=len {
        let f = i as f32 / len as f32;

        let rotate = (app.time * 0.5).sin() * (app.time * 0.25 + f * PI * 2.0).cos(); // wot

        let draw = draw.rotate(rotate);

        let hue = app.time + f * 2.0 * PI;
        let dive = (f + app.time * 0.1) % 1.0;
        let color = hsla(hue, 0.5, 0.5, 1.0 - dive.powi(3) );
        let rect_scale = dive.powi(2) * max_side * 2.0;

        draw.scale(rect_scale)
            .rect()
            .stroke(color)
            .stroke_weight(1.0 / len as f32)
            .no_fill()
            .w_h(1.0, 1.0);

        let text_scale = rect_scale * 0.001; // smol
        draw.scale(text_scale)
            .text(stringus[(i % 5) as usize])
            .wh(w.wh() * 0.8)
            .align_text_bottom()
            .color(color)
            .font_size(96);
    }
    draw.to_frame(app, &frame).unwrap();
}