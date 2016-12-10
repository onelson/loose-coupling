use time;
use specs;

use systems;
use components;

use std::sync::Arc;
use radiant_rs::{Layer, Renderer, Sprite};

pub type Delta = f64;

pub struct Game {
    pub world: specs::World,
    pub planner: specs::Planner<Delta>,
    pub layer: Layer,
    last_time: u64
}


impl Game {
    pub fn new(renderer: &Renderer) -> Game
    {
        let (width, height) = (300, 300);
        let layer = Layer::new(width, height);

        // The world is in charge of component storage, and as such contains all the game state.
        let world = specs::World::new();
        world.register::<components::Sprited>();
        world.register::<components::Body>();

        let spinner_sys = systems::Spinner { factor: 2.5 };
        let render_sys = systems::Renderer { layer: Arc::new(&layer) };

        // entities are created by combining various components via the world
        world.create_now()
            .with(components::Sprited { sprite: Arc::new(Sprite::from_file(&renderer.context(), r"assets/rust_128x128x1.png")) })
            .with(components::Body { x: width /  2, y: height / 2, scale_x: 1., scale_y: 1., rotation: 0. })
            .build();

        // systems are registered with a planner, which manages their execution
        let mut plan = specs::Planner::new(world, 2);
        plan.add_system(spinner_sys, "spinner", 10);
        plan.add_system(render_sys, "render_layer", 20);

        Game {
            planner: plan,
            layer: layer,
            world: world,
            last_time: time::precise_time_ns()
        }
    }

    pub fn tick(&mut self) -> bool {

        let new_time = time::precise_time_ns();
        let delta = (new_time - self.last_time) as Delta / 1e9;
        self.last_time = new_time;

        // dispatch() tells the planner to run the registered systems in a
        // thread pool.
        self.planner.dispatch(delta);

        // the wait() is like a thread.join(), and will block until the systems
        // have completed their work.
        self.planner.wait();
        true
    }
}

