use render::math::tuple::Tuple;
use render::scene::canvas::Canvas;
use render::scene::color::Color;

fn main() {
    // this is the sampe sample code like for chapter one, but plotted this time
    let start = Tuple::point(0.0, 1.0, 0.0);
    let velocity = Tuple::direction(1.0, 1.8, 0.0).normalize();

    let gravity = Tuple::direction(1.0, 1.0, 1.0);
    let wind = Tuple::direction(0.01, 1.0, 1.0);

    let mut projectile = Projectile {
        position: start,
        velocity,
    };

    // Todo: Check how to get method parameter hints/descriptions (maybe in INSERT-Mode only)

    let environment = Environment {
        gravity: Tuple::direction(0.0, -0.01, 0.0),
        wind: Tuple::direction(-0.01, 0.0, 0.0),
    };

    let width = 900;
    let height = 550;
    let mut canvas = Canvas::new(width, height);

    let mut counter = 0;
    while projectile.position.y > 0.0 {
        projectile = tick(environment, projectile);
        canvas.write_pixel(
            projectile.position.x as usize,
            projectile.position.y as usize,
            Color::simple_green(),
        );
        counter = counter + 1;
        println!("Current counter is {}", counter);
    }

    println!("Saving raw ppm data to file raw_data.txt");
    let ppm = canvas.to_ppm();
    std::fs::write("./raw_data.txt", ppm).unwrap();

    // println!("Saving contents of canvas into sample txt file");
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
