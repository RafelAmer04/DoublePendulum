extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use graphics::ellipse::circle;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::process::Command;

//REFERENCE: https://www.myphysicslab.com/pendulum/double-pendulum-en.html


struct Game {
    gl: GlGraphics,

    r1: f64,
    r2: f64,
    m1: f64,
    m2: f64,

    a1: f64,
    a2: f64,
    a1_v: f64,
    a2_v: f64,
    a1_a: f64,
    a2_a: f64,


    x1: f64,
    y1: f64,

    x2: f64,
    y2: f64,
}

impl Game{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        self.x1 = self.r1 * self.a1.sin();
        self.y1 = self.r1 * self.a1.cos();

        self.x2 = self.x1 + (self.r2 * self.a2.sin());
        self.y2 = self.y1 + (self.r2 * self.a2.cos());

        let white: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 3.0);




        self.gl.draw(args.viewport(), |c, gl|{
            graphics::clear(white, gl);

            let transform = c
                .transform
                .trans(x, y)
                .trans(-25.0, -25.0);

            let circle1 = circle(self.x1, self.y1, self.m1);
            let circle2 = circle(self.x2, self.y2, self.m2);

            line(black, 0.5, [0.0, 0.0, self.x1, self.y1], transform, gl);
            line(black, 0.5, [self.x1, self.y1, self.x2, self.y2], transform, gl);
            ellipse(black, circle1, transform, gl);
            ellipse(black, circle2, transform, gl);

        });
    }
    fn update(&mut self, _args: &UpdateArgs) {

        let g = 1.0;

        let mut num1 = -g * (2.0*self.m1 + self.m2) * self.a1.sin();
        let mut num2 = -self.m2 * g * (self.a1 - 2.0 * self.a2).sin();
        let mut num3 = -2.0 * (self.a1 - self.a2).sin() * self.m2;
        let mut num4 = self.a2_v * self.a2_v * self.r2 + self.a1_v * self.a1_v * self.r1 * (self.a1 - self.a2).cos();

        let mut den = self.r1 * (2.0 * self.m1 + self.m2 - self.m2 * (2.0 * self.a1 - 2.0 * self.a2).cos());

        self.a1_a = 0.1 *((num1 + num2 + num3 * num4) / den);

        num1 = 2.0 * (self.a1 - self.a2).sin();
        num2 = self.a1_v * self.a1_v * self.r1 * (self.m1 + self.m2);
        num3 = g * (self.m1 + self.m2) * self.a1.cos();
        num4 = self.a2_v *  self.a2_v * self.r2 * self.m2 * (self.a1 - self.a2).cos();

        den = self.r2 * (2.0 * self.m1 + self.m2 - self.m2 * (2.0 * self.a1 - 2.0 *self.a2).cos());

        self.a2_a =  0.1* ((num1)*(num2+num3+num4)) / den;


        self.a1_v += self.a1_a;
        self.a2_v += self.a2_a;
        self.a1 += self.a1_v;
        self.a2 += self.a2_v;



    }


}

fn main() {
    let opengl = OpenGL::V3_2;
    let pi = std::f64::consts::PI;

    let mut window: GlutinWindow = WindowSettings::new("Double Pendulum", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut game = Game {
        gl: GlGraphics::new(opengl),
        r1: 100.0,
        r2: 100.0,
        m1: 10.0,
        m2: 10.0,

        a1: pi/2.0,
        a2: pi/2.0,
        a1_v: 0.0,
        a2_v: 0.0,
        a1_a: 0.0,
        a2_a: 0.0,

        x1: 0.0,
        y1: 0.0,

        x2: 0.0,
        y2: 0.0,

    };


    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {



        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

    }
}
