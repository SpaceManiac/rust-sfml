//! Example from SFML: Custom drawable

extern crate sfml;

use sfml::graphics::{RenderWindow, Color, CircleShape, RectangleShape, Shape,
                      RenderTarget, RenderStates, Drawable, Transformable};
use sfml::window::{VideoMode, ContextSettings, Event, window_style, Key};
use sfml::system::Vector2f;

// Create a struct who contains two drawable for the example
struct CustomDrawable<'s> {
    circle:    CircleShape<'s>,
    rect:      RectangleShape<'s>
}

impl<'s> CustomDrawable<'s> {
    pub fn new() -> CustomDrawable<'s> {
        let mut c = CircleShape::new_init(50., 50).expect("Cannot create the CircleShape");
        c.set_position2f(100., 100.);
        c.set_fill_color(Color::red());
        let mut r = RectangleShape::new_init(Vector2f::new(100., 200.)).expect("Cannot create the RectangleShape");
        r.set_position2f(100., 150.);
        r.set_fill_color(Color::blue());

        CustomDrawable {
            circle: c,
            rect:   r
        }
    }
}

// Implements the drawable trait, only this function is mendatory.
impl<'s> Drawable for CustomDrawable<'s> {
    fn draw(&self, render_target: &mut RenderTarget, states: &RenderStates) -> () {
        self.circle.draw(render_target, states);
        self.rect.draw(render_target, states);
    }
}

fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(
        VideoMode::new(800, 600),
        "Custom Drawable - SFML Examples",
        window_style::CLOSE,
        ContextSettings::default()).expect("Cannot create a new Render Window.");
    window.set_vertical_sync_enabled(true);

    // create our Drawable
    let custom_drawable = CustomDrawable::new();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code: Key::Escape , .. } => window.close(),
                _ => {}
            }
        }

        // Clear the window
        window.clear(Color::black());
        // Draw it ! yeah you create a custome shape!
        window.draw(&custom_drawable);
        // Display things on screen
        window.display();
    }
}

