
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
    size : f32,
    color: Rgb
}

impl Default for Point {
    fn default()-> Point {
        Point {
            x:0.0, 
            y:0.0,
            size: 1.0,
            color: Rgb::new(0.0, 0.0, 0.0)
        }
    }
}

struct Model {
    points : [Point; 64]
}

fn model(_app: &App) -> Model {

    let mut points : [Point; 64] = Default::default();
    
    for point in points.iter_mut() {

        let mut rng = rand::thread_rng();

        point.x = rng.gen_range(-480.0, 480.0);
        point.y = rng.gen_range(-320.0, 320.0);
        point.size = rng.gen_range(5.0, 120.0);
        point.color = Rgb::new(
            rng.gen_range(0.0, 1.0), 
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0));
    }

    let model = Model{ points : points};

    return model;
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: &Frame){

    let draw = _app.draw();
    for point in _model.points.iter() {

        draw.ellipse()
            .x_y(point.x, point.y)
            .w_h(point.size, point.size)
            .color(point.color);
    }

    draw.to_frame(_app, &frame).unwrap();
}