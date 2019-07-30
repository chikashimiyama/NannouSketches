use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Joint {
    angle: f32,
    length: f32,
    phase: f32,
    speed: f32
}

struct Model {
    trees : Vec<Vec<Joint>>
}

fn model(_app: &App) -> Model {
    let mut rng = rand::thread_rng();
    let mut trees = Vec::new();
    for _c in 0..10{
        let mut joints = Vec::new();
        let origin = Joint{ angle: 0.0, length: 10.0, phase: 0.0, speed: 0.1};
        joints.push(origin);
        for _d in 0..20 {
            let joint = Joint{
                angle: rng.gen_range(-std::f32::consts::PI, std::f32::consts::PI),
                length: rng.gen_range(10.0, 100.0),
                phase: rng.gen_range(-std::f32::consts::PI, std::f32::consts::PI),
                speed: rng.gen_range(-0.1, 0.1),
            };
            joints.push(joint);
        }
        trees.push(joints);
    }
    return Model{ trees : trees };
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for joints in _model.trees.iter_mut() {
        for joint in joints.iter_mut() {
            joint.angle += joint.phase.sin() * 0.01;
            joint.phase += joint.speed;
        }
    }
}

fn view(_app: &App, _model: &Model, frame: &Frame){
    let draw = _app.draw();
    draw.background().color(BLACK);
    for tree in _model.trees.iter() {
        let mut prev_point = Point2{x: 0.0, y: 0.0};
        for joint in tree {
            let point = Point2{
                x: joint.angle.sin() * joint.length + prev_point.x,
                y: joint.angle.cos() * joint.length + prev_point.y
            };

            draw.line()
                .start(prev_point)
                .end(point);
            prev_point = point;
        }
    }
    draw.to_frame(_app, &frame).unwrap();
}
