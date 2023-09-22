use std::sync::mpsc::{self, Receiver, Sender};

use glutin_window::GlutinWindow;
use graphics::types::Vec2d;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    EventSettings, Events, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, WindowSettings,
};

mod draw;

pub struct TurtleTask {
    gl: GlGraphics, // OpenGL drawing backend.
    cmds: Vec<Command>,
    current_command: Option<Command>,
    percent: f64,
    is_pen_down: bool,
    pos: Vec2d<isize>,
    size: Vec2d<f64>,
}

pub struct Turtle {
    issue_command: Sender<Command>,
    command_complete: Receiver<Vec2d<isize>>,
    current_pos: Vec2d<isize>,
}

// transform is [[f64; 3]; 2]
// which looks like
// | a b c |
// | d e f |
//
// to transform (x, y), multiply by the transform like this
// | a*x + b*y + c| -> new x coordinate
// | d*x + e*y + f| -> new y coordinate
//
impl Turtle {
    pub fn start<F: FnOnce(&mut Turtle) + Send + 'static, S: Into<f64>>(
        xsize: S,
        ysize: S,
        func: F,
    ) {
        let (issue_command, receive_command) = mpsc::channel();
        let (finished, command_complete) = mpsc::channel();
        let xsize: f64 = xsize.into();
        let ysize: f64 = ysize.into();

        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Create a Glutin window.
        let mut window: GlutinWindow = WindowSettings::new("spinning-square", [xsize, ysize])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut tt = TurtleTask {
            gl: GlGraphics::new(opengl),
            cmds: Vec::new(),
            current_command: None,
            percent: 0.,
            is_pen_down: true,
            pos: [0, 0],
            size: [xsize, ysize],
        };

        let mut turtle = Self {
            issue_command,
            command_complete,
            current_pos: [0, 0],
        };

        let _ = std::thread::spawn(move || func(&mut turtle));

        let mut events = Events::new(EventSettings::new());
        let mut command_complete = true;
        while let Some(e) = events.next(&mut window) {
            if let Ok(cmd) = receive_command.try_recv() {
                match cmd {
                    Command::ClearScreen => {
                        tt.cmds.clear();
                        let _ = finished.send(tt.pos);
                    }
                    _ => {
                        tt.current_command = Some(cmd);
                        tt.percent = 0.;
                        command_complete = false;
                    }
                }
            }

            if !command_complete && tt.current_command.is_none() {
                command_complete = true;
                let _ = finished.send(tt.pos);
            }

            if let Some(args) = e.render_args() {
                tt.render(&args);
            }

            if let Some(args) = e.update_args() {
                tt.update(&args);
            }
        }
    }
}

impl TurtleTask {
    pub fn run(&mut self) {}

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let mut transform = c.transform.trans(x, y).rot_deg(-90.);
            let mut pct = 1.;
            let mut full = false;
            let mut done = false;
            let mut deg: f64 = -90.;

            let mut index = 0;
            while !done {
                let cmd = if index < self.cmds.len() {
                    index += 1;
                    let cmd = self.cmds[index - 1];
                    deg += cmd.get_rotation();
                    Some(cmd)
                } else {
                    pct = self.percent.min(1.);
                    full = pct >= 1.;
                    done = true;
                    self.current_command
                };

                let Some(cmd) = cmd else {
                    break;
                };

                if full {
                    self.cmds.push(cmd);
                    self.current_command = None;
                }

                match cmd {
                    Command::Forward(dist) => {
                        if self.is_pen_down {
                            line_from_to(BLACK, 1.0, [0., 0.], [dist * pct, 0.], transform, gl);
                        }
                        transform = transform.trans(dist * pct, 0.);
                    }
                    Command::Right(deg) => transform = transform.rot_deg(deg * pct),
                    Command::Left(deg) => transform = transform.rot_deg(-deg * pct),
                    Command::PenDown => self.is_pen_down = true,
                    Command::PenUp => self.is_pen_down = false,
                    Command::GoTo(xpos, ypos) => {
                        transform = c.transform.trans(xpos + x, ypos + y).rot_deg(deg);
                    }
                    Command::ClearScreen => panic!("{cmd:?} is not a drawing command"),
                }
            }

            self.pos = [
                (transform[0][2] * self.size[0] / 2.) as isize,
                (transform[1][2] * self.size[1] / 2.) as isize,
            ];

            let square = rectangle::square(0.0, 0.0, 10.0);
            ellipse(BLACK, square, transform.trans(-5., -5.), gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if self.percent < 1. {
            self.percent += args.dt * 30.;
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Command {
    Forward(f64),
    Right(f64),
    Left(f64),
    PenDown,
    PenUp,
    GoTo(f64, f64),
    ClearScreen,
}

impl Command {
    fn get_rotation(&self) -> f64 {
        match self {
            Command::Right(deg) => *deg,
            Command::Left(deg) => -*deg,
            _ => 0.,
        }
    }
}
