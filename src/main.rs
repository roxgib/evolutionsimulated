use evolution_simulated::world::World;

fn main() {
    let mut world = World::new();
    world.spawn_random_organisms(100);
    for _ in 0..1000 {
        world.tick();
    }
}