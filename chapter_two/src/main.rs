use std::io::Cursor;

use image::io::Reader;
use render::{
    math::tuple::Tuple,
    scene::{canvas::Canvas, color::Color},
};

fn main() {
    let start = Tuple::point(0.0, 1.0, 0.0);
    let velocity = Tuple::direction(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut projectile = Projectile {
        position: start,
        velocity,
    };

    let gravity = Tuple::direction(0.0, -0.1, 0.0);
    let wind = Tuple::direction(-0.01, 0.0, 0.0);
    let environment = Environment { gravity, wind };

    let width = 900;
    let height = 550;
    let mut canvas = Canvas::new(width, height);

    let mut counter = 0;
    while projectile.position.y > 0.0 {
        projectile = tick(environment, projectile);
        canvas.write_pixel(
            projectile.position.x as usize,
            (height as f64 - projectile.position.y) as usize,
            Color::simple_green(),
        );
        counter = counter + 1;
        println!("Current counter is {}", counter);
    }

    let mut reader = Reader::new(Cursor::new(canvas.to_ppm()))
        .with_guessed_format()
        .expect("Failed to create image reader from ppm data");
    let img = reader.decode().expect("Decoding image data failed");
    img.save("./chapter_two.png").expect("Failed to save decoded image");

    println!("done!");
}

#[derive(Debug, Copy, Clone)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Debug, Copy, Clone)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}
