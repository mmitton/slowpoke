use graphics::{
    math::{identity, Vec2d},
    Transformed,
};

use crate::{
    color_names::TurtleColor,
    command::{DrawRequest, InstantaneousDrawCmd, MotionCmd, RotateCmd, TimedDrawCmd},
};

pub(crate) struct LineInfo {
    pub begin: Vec2d<isize>,
    pub end: Vec2d<isize>,
}

pub(crate) enum DrawCommand {
    DrawLine(LineInfo),
    SetPenColor(TurtleColor),
    SetPenWidth(f64),
    SetFillColor(TurtleColor),
    // DrawPolygon(TurtlePolygon),
}

#[derive(Debug)]
pub(crate) struct CurrentTurtleState {
    transform: [[f64; 3]; 2],
    angle: f64,
}

pub(crate) trait TurtlePosition<T> {
    fn pos(&self) -> [T; 2];
}

impl TurtlePosition<f64> for CurrentTurtleState {
    fn pos(&self) -> [f64; 2] {
        [self.transform[0][2] as f64, self.transform[1][2] as f64]
    }
}

impl TurtlePosition<isize> for CurrentTurtleState {
    fn pos(&self) -> [isize; 2] {
        [self.transform[0][2] as isize, self.transform[1][2] as isize]
    }
}

impl Default for CurrentTurtleState {
    fn default() -> Self {
        Self {
            transform: identity(),
            angle: 0.,
        }
    }
}

impl CurrentTurtleState {
    pub fn angle(&self) -> f64 {
        self.angle
    }

    fn get_point(&mut self) -> Vec2d<isize> {
        let x = self.transform[0][2].round() as isize;
        let y = self.transform[1][2].round() as isize;
        [x, y]
    }

    pub(crate) fn apply(&mut self, cmd: &DrawRequest) -> Option<DrawCommand> {
        match cmd {
            DrawRequest::TimedDraw(td) => match td {
                TimedDrawCmd::Motion(motion) => {
                    let begin = self.get_point();
                    match motion {
                        MotionCmd::Forward(dist) => {
                            self.transform = self.transform.trans(*dist, 0.);
                        }
                        MotionCmd::Teleport(x, y) | MotionCmd::GoTo(x, y) => {
                            self.transform = identity().trans(*x, *y).rot_deg(self.angle);
                        }
                        MotionCmd::SetX(x) => {
                            let cur_y = self.transform[1][2];
                            self.transform = identity().trans(*x, cur_y).rot_deg(self.angle);
                        }
                        MotionCmd::SetY(y) => {
                            let cur_x = self.transform[0][2];
                            self.transform = identity().trans(cur_x, *y).rot_deg(self.angle);
                        }
                    }
                    let end = self.get_point();
                    return Some(DrawCommand::DrawLine(LineInfo { begin, end }));
                }
                TimedDrawCmd::Rotate(rotation) => match rotation {
                    RotateCmd::Right(angle) => {
                        self.transform = self.transform.rot_deg(*angle);
                        self.angle += angle;
                    }
                    RotateCmd::Left(angle) => {
                        self.transform = self.transform.rot_deg(-*angle);
                        self.angle -= angle;
                    }
                    RotateCmd::SetHeading(h) => {
                        self.transform = self.transform.rot_deg(h - self.angle);
                        self.angle = *h;
                    }
                },
            },
            DrawRequest::InstantaneousDraw(id) => match id {
                InstantaneousDrawCmd::Undo => {}
                InstantaneousDrawCmd::BackfillPolygon => {}
                InstantaneousDrawCmd::PenDown => {}
                InstantaneousDrawCmd::PenUp => {}
                InstantaneousDrawCmd::PenColor(pc) => {
                    return Some(DrawCommand::SetPenColor(*pc));
                }
                InstantaneousDrawCmd::FillColor(fc) => {
                    return Some(DrawCommand::SetFillColor(*fc));
                }
                InstantaneousDrawCmd::PenWidth(pw) => {
                    return Some(DrawCommand::SetPenWidth(*pw));
                }
                InstantaneousDrawCmd::Dot(_, _) => {}
                InstantaneousDrawCmd::Stamp(_) => {}
                InstantaneousDrawCmd::Fill(_) => {}
            },
        }
        None
    }
}
