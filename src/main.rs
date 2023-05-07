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
    iterations: u32,
    x_lower_limit: f32,
    x_upper_limit: f32
}

impl EulersFormulaTest {
    pub fn new(_context: &mut Context) -> EulersFormulaTest {
        EulersFormulaTest {
            e_replacement: Complex::new(E, 0.0),
            iterations: 400,
            x_lower_limit: 0.0,
            x_upper_limit: TAU
        }
    }
}

impl EventHandler for EulersFormulaTest {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let context_keyboad = &context.keyboard;
        let change = 0.025; // TODO: In terms of units per second (hertz!) multiplied by delta time
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
        return Ok(());
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, Color::BLACK);
        for iteration in 0..self.iterations {
            let iteration_lerp =  iteration as f32 / self.iterations as f32;
            let x = self.x_lower_limit + (self.x_upper_limit - self.x_lower_limit) * iteration_lerp;
            let ix = Complex::new(0.0, x); // TODO: Multiply x by numbers other than i, draw a hollow red circle for that number
            let output = self.e_replacement.powc(ix);
            let circle = Mesh::new_circle(
                context,
                DrawMode::fill(),
                mint::Point2{
                    x: output.re * SCALE + WINDOW_WIDTH as f32 / 2.0,
                    y: output.im * -SCALE + WINDOW_HEIGHT as f32 / 2.0
                },
                1.0,
                0.1,
                Color::WHITE, // TODO: Draw colour interpolated between green and blue by a t of iteration as f32 / self.iterations as f32
            )?;
            canvas.draw(&circle, DrawParam::default());
        }
        println!("{}", self.e_replacement);
        let circle = Mesh::new_circle(
            context,
            DrawMode::fill(),
            mint::Point2{
                x: self.e_replacement.re * SCALE + WINDOW_WIDTH as f32 / 2.0,
                y: self.e_replacement.im * -SCALE + WINDOW_HEIGHT as f32 / 2.0
            },
            3.0,
            0.1,
            Color::RED,
        )?;
        // TODO: Draw small red dot at x = e
        // TODO: Draw axis lines
        canvas.draw(&circle, DrawParam::default());
        return canvas.finish(context);
    }
}
