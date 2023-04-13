extern crate molecular
use molecular::*;
use nalgebra::Vector3

fn main() {
    let mut scene = scene::Scene::default();
    scene.add_atom(atom::Atom::new(Vector3::new(0.0, 0.0, 0.0)));
    scene.add_atom(atom::Atom::new(Vector3::new(1920.0, 1080.0, 0.0)));
    scene.add_atom(atom::Atom::new(Vector3::new(200.0, 500.0, 0.0)));
    scene.add_atom(atom::Atom::new(Vector3::new(250.0, 0.0, 0.0)));
    scene.add_bond(bond::Bond::new([
        std::cell::RefCell::new(scene.atoms[0]),
        std::cell::RefCell::new(scene.atoms[1]),
    ]));
    scene.add_bond(bond::Bond::new([
        std::cell::RefCell::new(scene.atoms[3]),
        std::cell::RefCell::new(scene.atoms[2]),
    ]));

    let mut renderer = renderer::Renderer::new((1920, 1080));
    renderer.draw_scene(&scene).save("line.png").unwrap();
}
