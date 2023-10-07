use std::cell::RefCell;

use model::{model::Model, vectors::quantity_vector::QuantityVector};
use nannou::prelude::*;

use crate::{model::particles::particle::Particle, constants::G};



mod model;
mod constants;

use std::iter::zip;



fn main() {
    nannou::app(model).update(update).run();
}


fn model(app: &App) -> Model {

    let screen_dimensions = (800, 800);

    app.new_window()
        .title("Particle Simulation")
        .size(screen_dimensions.0, screen_dimensions.1)
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .view(view).build().unwrap();
    Model::new(screen_dimensions)
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // calculate the net force per regular particle
    let mut resultant_force: Vec<QuantityVector> = Vec::with_capacity(_model.regular_particles().len());
    for regular_particle in _model.regular_particles() {
        let mut force = QuantityVector::new();
        for gravity_particles in _model.gravity_particles() {
            force = &force + &gravity_particles.calculate_force(regular_particle);
        }
        force.clean();
        println!("{:?} {:?}", &force.magnitude(), &force.angle());
        resultant_force.push(force);
    }
    
    for (regular_particles, force) in  zip(_model.regular_particles_mut(), resultant_force) {
        regular_particles.apply_force(&force);
        regular_particles.update();
    }

}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    // Render all regular particles
    for particle in _model.regular_particles() {
        draw.ellipse().color(BLACK)
            .w(10.0 * particle.mass())
            .h(10.0 * particle.mass())
            .x(particle.position().x)
            .y(particle.position().y);
    }

    for gravity_particle in _model.gravity_particles() {
        let particle = gravity_particle.particle();
        draw.ellipse().color(BLUE)
            .w(2.0)
            .h(2.0)
            .x(particle.position().x)
            .y(particle.position().y);
    }


    draw.to_frame(&app, &frame).unwrap()
}

fn mouse_pressed(app: &App, model: &mut Model, mouse: MouseButton) {
    match mouse {
        MouseButton::Left => model.add_gravity_particle(
            0.0,
            0.0
            // app.mouse.x,
            // app.mouse.y
        ),
        _ => ()
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::A => model.add_regular_particles(1),
        _ => () 
    }
}
