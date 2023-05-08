use std::f32::consts::E;
use std::f32::consts::TAU;
use ggez::*;
use ggez::graphics::*;
use ggez::input::keyboard::*;
use ggez::event::*;
use num::complex::Complex;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const SCALE: f32 = 100.0;

fn main() {
    let (mut context, event_loop) = ContextBuilder::new("eulers_formula_test", "Tachytaenius")
        .build()
        .expect("Could not create ggez context");
    let eulers_formula_test = EulersFormulaTest::new(&mut context);
    event::run(context, event_loop, eulers_formula_test);
}

struct EulersFormulaTest {
    e_replacement: Complex<f32>,
    i_replacement: Complex<f32>,
    iterations: u32,
    x_lower_limit: f32,
    x_upper_limit: f32
}

impl EulersFormulaTest {
    pub fn new(_context: &mut Context) -> EulersFormulaTest {
        EulersFormulaTest {
            e_replacement: Complex::new(E, 0.0),
            i_replacement: Complex::new(0.0, 1.0),
            iterations: 100,
            x_lower_limit: 0.0,
            x_upper_limit: TAU
        }
    }
}

impl EventHandler for EulersFormulaTest {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let context_keyboad = &context.keyboard;
        let change = 0.025; // TODO: In terms of units per second (hertz!) multiplied by delta time

        // Update e_replacement with arrow keys
        if context_keyboad.is_key_pressed(KeyCode::Left) {
            self.e_replacement.re -= change;
        }
        if context_keyboad.is_key_pressed(KeyCode::Right) {
            self.e_replacement.re += change;
        }
        if context_keyboad.is_key_pressed(KeyCode::Up) {
            self.e_replacement.im += change;
        }
        if context_keyboad.is_key_pressed(KeyCode::Down) {
            self.e_replacement.im -= change;
        }

        // Update i_replacement with WASD
        if context_keyboad.is_key_pressed(KeyCode::A) {
            self.i_replacement.re -= change;
        }
        if context_keyboad.is_key_pressed(KeyCode::D) {
            self.i_replacement.re += change;
        }
        if context_keyboad.is_key_pressed(KeyCode::W) {
            self.i_replacement.im += change;
        }
        if context_keyboad.is_key_pressed(KeyCode::S) {
            self.i_replacement.im -= change;
        }

        return Ok(());
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, Color::BLACK);

        // Draw axis lines
        let x_axis = Mesh::new_line(
            context,
            &[
                mint::Point2 {x: 0.0, y: WINDOW_HEIGHT as f32 / 2.0},
                mint::Point2 {x: WINDOW_WIDTH as f32, y: WINDOW_HEIGHT as f32 / 2.0}
            ],
            1.0,
            Color::WHITE
        )?;
        canvas.draw(&x_axis, DrawParam::default());
        let y_axis = Mesh::new_line(
            context,
            &[
                mint::Point2 {x: WINDOW_WIDTH as f32 / 2.0, y: 0.0},
                mint::Point2 {x: WINDOW_WIDTH as f32 / 2.0, y: WINDOW_HEIGHT as f32}
            ],
            1.0,
            Color::WHITE
        )?;
        canvas.draw(&y_axis, DrawParam::default());

        // Draw white dots at 1, i, -1, and -i
        let circle = Mesh::new_circle(
            context,
            DrawMode::fill(),
            mint::Point2 {x: SCALE + WINDOW_WIDTH as f32 / 2.0, y: WINDOW_HEIGHT as f32 / 2.0},
            3.0,
            0.1,
            Color::WHITE
        )?;
        canvas.draw(&circle, DrawParam::default());
        let circle = Mesh::new_circle(
            context,
            DrawMode::fill(),
            mint::Point2 {x: WINDOW_WIDTH as f32 / 2.0, y: -SCALE + WINDOW_HEIGHT as f32 / 2.0},
            3.0,
            0.1,
            Color::WHITE
        )?;
        canvas.draw(&circle, DrawParam::default());
        let circle = Mesh::new_circle(
            context,
            DrawMode::fill(),
            mint::Point2 {x: -SCALE + WINDOW_WIDTH as f32 / 2.0, y: WINDOW_HEIGHT as f32 / 2.0},
            3.0,
            0.1,
            Color::WHITE
        )?;
        canvas.draw(&circle, DrawParam::default());
        let circle = Mesh::new_circle(
            context,
            DrawMode::fill(),
            mint::Point2 {x: WINDOW_WIDTH as f32 / 2.0, y: SCALE + WINDOW_HEIGHT as f32 / 2.0},
            3.0,
            0.1,
            Color::WHITE
        )?;
        canvas.draw(&circle, DrawParam::default());

        // Draw small green dot at y = i
        let circle = Mesh::new_circle(
            context,
            DrawMode::fill(),
            mint::Point2 {x: WINDOW_WIDTH as f32 / 2.0, y: -SCALE + WINDOW_HEIGHT as f32 / 2.0},
            1.5,
            0.1,
            Color::GREEN,
        )?;
        canvas.draw(&circle, DrawParam::default());
        // Draw small red dot at x = e
        let circle = Mesh::new_circle(
            context,
            DrawMode::fill(),
            mint::Point2 {x: E * SCALE + WINDOW_WIDTH as f32 / 2.0, y: WINDOW_HEIGHT as f32 / 2.0},
            1.5,
            0.1,
            Color::RED,
        )?;
        canvas.draw(&circle, DrawParam::default());

        // Draw i_replacement
        let circle = Mesh::new_circle(
            context,
            DrawMode::fill(),
            mint::Point2 {
                x: self.i_replacement.re * SCALE + WINDOW_WIDTH as f32 / 2.0,
                y: self.i_replacement.im * -SCALE + WINDOW_HEIGHT as f32 / 2.0
            },
            3.0,
            0.1,
            Color::GREEN
        )?;
        canvas.draw(&circle, DrawParam::default());

        // Draw e_replacement
        let circle = Mesh::new_circle(
            context,
            DrawMode::fill(),
            mint::Point2 {
                x: self.e_replacement.re * SCALE + WINDOW_WIDTH as f32 / 2.0,
                y: self.e_replacement.im * -SCALE + WINDOW_HEIGHT as f32 / 2.0
            },
            3.0,
            0.1,
            Color::RED
        )?;
        canvas.draw(&circle, DrawParam::default());

        // Draw graph
        for iteration in 0..self.iterations {
            let iteration_lerp =  iteration as f32 / self.iterations as f32;
            let x = self.x_lower_limit + (self.x_upper_limit - self.x_lower_limit) * iteration_lerp;
            let output = self.e_replacement.powc(self.i_replacement * x);
            let circle = Mesh::new_circle(
                context,
                DrawMode::fill(),
                mint::Point2 {
                    x: output.re * SCALE + WINDOW_WIDTH as f32 / 2.0,
                    y: output.im * -SCALE + WINDOW_HEIGHT as f32 / 2.0
                },
                1.0,
                0.1,
                Color::WHITE, // TODO: Draw colour interpolated between two colours (white and black?) by a t of iteration as f32 / self.iterations as f32
            )?;
            canvas.draw(&circle, DrawParam::default());
        }

        println!("e replacement: {}, i replacement: {}", self.e_replacement, self.i_replacement);
        
        return canvas.finish(context);
    }
}
