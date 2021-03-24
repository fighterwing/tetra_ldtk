#![allow(warnings, unused)]

use tetra::{Context, ContextBuilder, State};
use tetra::graphics::{self, Texture, Color, DrawParams, Camera};

use tetra::time::{self, Timestep};

use tetra::error::TetraError;

const SCREEN_WIDTH : f32 = 640.0;
const SCREEN_HEIGHT: f32 = 576.0;

pub enum RunState{
    PreRun,
    Run,
}

pub struct GameState{
    pub runstate: RunState,
}
impl GameState{
    fn new(ctx: &mut Context) -> tetra::Result::<GameState> {
        Ok(GameState{ runstate: RunState::Run })
    }
}
impl State for GameState{
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.runstate {
            RunState::PreRun=>{
                graphics::clear(ctx, Color::rgb8(0,0,0));
            }
            RunState::Run=>{
                graphics::clear(ctx, Color::rgb8(55,55,55));
            }
            _=>{},
        }
        Ok(())

    }
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        Ok(())
    }
}

fn main() -> tetra::Result{
    ContextBuilder::new("LDTK", SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32,)
    .quit_on_escape(true)
    .show_mouse(true)
    .timestep(Timestep::Fixed(60.0))
    .build()?
    .run(GameState::new)
}
