use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();

    /* Design is based on MVC and a little bit of FRP
    Model -> Internal state
    View -> How model is presented 
    Controller -> How model is updated by certain events
    */
}

struct Model {
    // this is a lot like a Julia struct
    // we can then define functions relating to its fields
    joints: usize,
    line_length: f32,
    speed_relation: f32,
    center: Vector2,
    pendulum_paths: Vec<Vec<Point2>>, // wot
    start_positions: Vec<Point2>,
    angle: f32,
    max_angle: f32,
    speed: f32,
    show_pendulum: bool,
    show_pendulum_path: bool,
}

fn model(app: &App) -> Model {
  /* The "app" is a helper type that provides support for stuff like
    IO, event loops, etc, and the model can have it as an argument
    */
    app.new_window()
        // .size(1280, 720)
        .view(view) // takes "view" parameters from the "view" function?
        .key_released(key_released) // k
        .build()
        .unwrap(); // i don't know what this does

    let joints = 5;
    
    // returns the model i guess
    Model {
        joints,
        line_length: 500.0,
        speed_relation: 2.0,
        center: vec2(0.0, 0.0),
        pendulum_paths: vec![Vec::new(); joints],
        start_positions: vec![pt2(0.0, 0.0); joints],
        angle: 0.0,
        max_angle: 360.0,
        speed: 1.0,
        show_pendulum: true,
        show_pendulum_path: true,
    }
}

fn start_drawing(model: &mut Model) {
    // apparently this just affects some states in the model
    // w/ keyboard control written later
    model.start_positions = vec![pt2(0.0, 0.0); model.joints];
    model.pendulum_paths = vec![Vec::new(); model.joints];

    for i in 0..model.pendulum_paths.len() {
        model.pendulum_paths[i].clear();
    }

    model.angle = 0.0;
    model.speed = 9.0 / 1.75.powf(model.joints as f32 - 1.0) / 2.0.powf(model.speed_relation - 1.0);
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.angle += model.speed;

    if model.angle <= model.max_angle + model.speed {
        let mut pos = model.center;

        for i in 0..model.joints {
            let mut a = model.angle * model.speed_relation.powf(i as f32);
            if i % 2 == 1 {
                a = -a;
            }

            let vx = a.to_radians().cos();
            let vy = a.to_radians().sin();
            let mut next_pos = pt2(vx, vy);

            next_pos = next_pos.with_magnitude(
                (model.joints - i) as f32
            );
            next_pos += pos;

            model.start_positions[i] = pos;
            model.pendulum_paths[i].push(next_pos); // vec
            pos = next_pos;
            // this is kind of ugly
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    if model.angle <= model.max_angle + model.speed {

        // this draws the pendulums
        for i in 0..model.joints {
            if model.show_pendulum {
                let pos = model.start_positions[i];
                let next_pos = model.pendulum_paths[i].last().unwrap(); // hmm
                draw.ellipse()
                    .x_y(pos.x, pos.y)
                    .radius(3.0)
                    .rgba(0.0, 0.0, 0.0, 0.5); // alpha

                draw.line() 
                    .start(pt2(pos.x, pos.y))
                    .end(pt2(next_pos.x, next_pos.y))
                    .rgba(0.0, 0.0, 0.0, 0.5);
            }
        }
    }

    if model.show_pendulum_path {
        let weight = 2.5;
        for i in 0..model.pendulum_paths.len() {
            let hue = map_range(i, 0, model.joints, 0.0, 1.0);
            let hsla = hsla(hue, 0.8, 0.6, 0.5);

            let vertices = model.pendulum_paths[i]
                .iter()
                .map(|p| pt2(p.x, p.y))
                .map(|p| (p, hsla));

            draw.polyline()
                .weight(weight)
                .join_round()
                .points_colored(vertices);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_released(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png"); // how you save stuff
        }
        Key::Up => { // increase line length by 2 (reference length 100)
            model.line_length += 2.0;
            start_drawing(model); 
        }
        Key::Down => {
            model.line_length -= 2.0;
            start_drawing(model);
        }
        Key::Left => { // fewer joints
            if model.joints > 1 {
                model.joints -= 1;
                start_drawing(model);
            }
        }
        Key::Right => {
            if model.joints < 10 {
                model.joints += 1;
                start_drawing(model);
            }
        }
        Key::Equals => {
            if model.speed_relation < 5.0 {
                model.speed_relation += 0.5;
                start_drawing(model);
            }
        }
        Key::Minus => {
            if model.speed_relation > 2.0 {
                model.speed_relation -= 0.5;
                start_drawing(model);
            }
        }
        Key::Key1 => { // make pendulum invisiburu
            model.show_pendulum = !model.show_pendulum; 
        }
        _other_key => {}
    }
}