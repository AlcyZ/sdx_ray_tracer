use render::math::tuple::Tuple;

fn main() {
    let position = Tuple::point(0.0, 1.0, 0.0);
    let velocity = Tuple::direction(1.0, 1.0, 0.0).normalize();
    let mut projectile = Projectile { position, velocity };

    let wind = Tuple::direction(0.0, -0.1, 0.0);
    let gravity = Tuple::direction(-0.01, 0.0, 0.0);
    let envrironment = Environment { gravity, wind };

    let mut ticks = 0;
    let mut proj_y = projectile.position.y;

    while proj_y > 0.0 {
        projectile = tick(&envrironment, &projectile);
        let proj_x = projectile.position.x;
        proj_y = projectile.position.y;

        println!("Projectile pos_x: {proj_x}");
        println!("Projectile pos_y: {proj_y}");
        eprintln!("projectile.velocity = {:#?}", projectile.velocity);

        ticks += 1;
        println!("this was the {ticks} tick");
    }
}

fn tick(env: &Environment, projectile: &Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + env.gravity + env.wind;

    Projectile { position, velocity }
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}
