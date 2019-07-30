use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Point {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
    size : f32,
    color: Rgb
}

struct Model {
    points : Vec<Point>
}

fn model(_app: &App) -> Model {

    let mut points = Vec::new();
    let mut rng = rand::thread_rng();
    let win = _app.window_rect();

    for _count in 0..100 {
        let point = Point{
            x: rng.gen_range(win.left(), win.right()),
            y: rng.gen_range(win.bottom(), win.top()),
            x_speed: rng.gen_range(-8.0, 8.0),
            y_speed: rng.gen_range(-6.0, 6.0),
            size: rng.gen_range(10.0, 100.0),
            color: Rgb::new(
                rng.gen_range(0.0, 1.0), 
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0))
        };
        points.push(point);
    }
    return Model{ points : points };
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

    let win = _app.window_rect();

    for point in _model.points.iter_mut() {
        point.x += point.x_speed;
        point.y += point.y_speed;

        if point.x < win.left() || point.x > win.right() {
            point.x_speed = -point.x_speed;
        }
        if point.y < win.bottom() || point.y > win.top() {
            point.y_speed = -point.y_speed;
        }
    }
}

fn view(_app: &App, _model: &Model, frame: &Frame){

    let draw = _app.draw();
    draw.background().color(WHITE);
    for point in _model.points.iter() {
        draw.ellipse()
            .x_y(point.x, point.y)
            .w_h(point.size, point.size)
            .color(point.color);
    }

    draw.to_frame(_app, &frame).unwrap();
}