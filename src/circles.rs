use nannou::prelude::*;

const LINE_WIDTH: f32 = 2.0;
const MIN_RADIUS: f32 = 2.0;
const MAX_RADIUS: f32 = 185.0;
const N_CIRCLES: usize = 4000;
const CIRCLE_ATTEMPTS: usize = 500;

struct Model;

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app.new_window().view(view).build().unwrap();
    Model
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

impl Circle {
    fn collides(&self, other: &Circle) -> bool {
        let a = self.radius + other.radius;
        let x = self.x - other.x;
        let y = self.y - other.y;

        if a >= (x.powi(2) + y.powi(2)).sqrt() {
            true
        } else {
            false
        }
    }
    fn any_collision(&self, others: &Vec<Circle>) -> bool {
        for other in others {
            if self.collides(other) {
                return true;
            }
        }
        false
    }
}

fn main() {
    nannou::app(model).run()
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw();

    draw.background().color(POWDERBLUE);

    let mut circles = Vec::<Circle>::with_capacity(N_CIRCLES);

    for _ in 0..=N_CIRCLES {
        for _ in 0..=CIRCLE_ATTEMPTS {
            let x = random_range(window.left(), window.right());
            let y = random_range(window.top(), window.bottom());
            let radius = random_range(MIN_RADIUS, MAX_RADIUS);

            let c = Circle {
                x: x,
                y: y,
                radius: radius,
            };
            if c.any_collision(&circles) {
                continue;
            }

            circles.push(c);
            break;
        }
    }

    for c in circles {
        let line_points = (0..360).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x_ = c.x + radian.sin() * c.radius;
            let y_ = c.y + radian.cos() * c.radius;

            (pt2(x_, y_), WHITE)
        });
        draw.polyline()
            .weight(LINE_WIDTH)
            .points_colored(line_points);
    }
    draw.to_frame(app, &frame).unwrap();
}
