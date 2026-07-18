use crate::Scene;
use engine_core::{Color, Vec2};

#[test]
fn create_and_move_rect() {
    let mut scene = Scene::new();
    let id = scene.create_rect(
        "R1",
        Vec2::new(10.0, 20.0),
        Vec2::new(100.0, 50.0),
        Color::WHITE,
    );
    assert!(scene.is_alive(id));
    scene.translate(id, Vec2::new(5.0, -5.0));
    let t = scene.transforms[id.index()].position;
    assert_eq!(t, Vec2::new(15.0, 15.0));
}

#[test]
fn hit_bounds() {
    let mut scene = Scene::new();
    let id = scene.create_rect(
        "R1",
        Vec2::new(0.0, 0.0),
        Vec2::new(100.0, 100.0),
        Color::WHITE,
    );
    let bounds = scene.world_bounds(id).unwrap();
    assert!(bounds.contains_point(Vec2::new(50.0, 50.0)));
    assert!(!bounds.contains_point(Vec2::new(150.0, 50.0)));
}
