use rand::rngs::ThreadRng;
use rand::Rng;
use raylib::prelude::*;

const NUM_FIREWORKS: usize = 100;
const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;
const LIFETIME: f32 = 250.0;

fn main() {
    let mut rng = rand::rng();
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("screensaver")
        .build();

    rl.set_target_fps(60);

    let start_pos: Vector2 = Vector2::new(
        rng.random_range(120.0..=1800.0),
        rng.random_range(80.0..=1000.0),
    );
    let mut fireworks: Fireworks = Fireworks::with_capacity(NUM_FIREWORKS);
    for _ in 0..NUM_FIREWORKS {
        fireworks.push(Firework::new(start_pos, LIFETIME, &mut rng));
    }

    let mut timer: f32 = 0.0;
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if timer >= LIFETIME {
            let start_pos: Vector2 = Vector2::new(
                rng.random_range(120.0..=1800.0),
                rng.random_range(80.0..=1000.0),
            );
            fireworks.set_start_pos(start_pos);
            fireworks.generate_new_velocities(&mut rng);
            timer = 0.0;
        }

        fireworks.iter_mut().for_each(|f| f.update(dt, timer));
        fireworks.iter_mut().for_each(|f| f.render(&mut d));

        timer += 1.0;
    }
}

struct Fireworks(Vec<Firework>);

impl Fireworks {
    fn with_capacity(cap: usize) -> Self {
        Fireworks(Vec::with_capacity(cap))
    }

    fn set_start_pos(&mut self, start_pos: Vector2) {
        self.0.iter_mut().for_each(|f| f.set_start_pos(start_pos));
    }

    fn generate_new_velocities(&mut self, rng: &mut ThreadRng) {
        self.0.iter_mut().for_each(|f| f.generate_new_velocity(rng));
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<Firework> {
        self.0.iter_mut()
    }

    fn push(&mut self, firework: Firework) {
        self.0.push(firework);
    }
}

struct Firework {
    pos: Vector2,
    color: Color,
    vel: Vector2,
    lifetime: f32,
}

impl Firework {
    fn new(pos: Vector2, lifetime: f32, rng: &mut ThreadRng) -> Self {
        let color = Color {
            r: rng.random_range(100..=255),
            g: rng.random_range(100..=255),
            b: rng.random_range(100..=255),
            a: 255,
        };

        Self {
            pos,
            color,
            vel: random_velocity(rng),
            lifetime,
        }
    }

    fn update(&mut self, dt: f32, timer: f32) {
        self.pos.x = self.pos.x + dt * self.vel.x;
        self.pos.y = self.pos.y + dt * self.vel.y;
        self.color.a = (((self.lifetime - timer * 5.0) / self.lifetime) * 255.0 + 1000.0)
            .clamp(0.0, 255.0) as u8;
    }

    fn generate_new_velocity(&mut self, rng: &mut ThreadRng) {
        self.vel = random_velocity(rng)
    }

    fn render(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.pos.x as i32, self.pos.y as i32, 3, 3, self.color);
    }

    fn set_start_pos(&mut self, start_pos: Vector2) {
        self.pos = start_pos
    }
}

fn random_velocity(rng: &mut ThreadRng) -> Vector2 {
    let mag = random_velocity_component(rng);
    let x = random_velocity_component(rng);
    let y = random_velocity_component(rng);
    let mut v: Vector2 = Vector2::new(x, y);

    v.normalize();
    v = v * mag;
    v
}

fn random_velocity_component(rng: &mut ThreadRng) -> f32 {
    if rng.random_bool(0.5) {
        rng.random_range(-70.0..=40.0)
    } else {
        rng.random_range(40.0..=70.0)
    }
}
