#![allow(warnings, unused)]

use tetra::{Context, ContextBuilder, State};
use tetra::graphics::{self, Texture, Color, DrawParams, Camera, Rectangle};
use tetra::math::Vec2;

use tetra::time::{self, Timestep};

use tetra::error::TetraError;

mod json_0_8_1;
mod ldtk;

use json_0_8_1::*;

pub const SCREEN_WIDTH : f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 576.0;

pub const BACKGROUND_ID: &str = "BACKGROUND";

pub const TITLE_SCREEN_ID: &str = "TITLE_SCREEN";
pub const GAME_SCREEN_ID: &str = "GAME_SCREEN";
pub const START_TEXT_ID: &str = "START_TEXT";
pub const PREGAME_TEXT_ID: &str = "PREGAME_TEXT";
pub const GAME_TEXT_ID: &str = "GAME_TEXT";
pub const PREGAME_BANNER_ID: &str = "PREGAME_BANNER";

pub enum RunState{
    PreRun,
    Run,
}

pub struct GameState{
    pub runstate: RunState,
    pub project: json_0_8_1::Project,
    pub tex: Texture,
}
impl GameState{
    fn new(ctx: &mut Context) -> tetra::Result::<GameState> {
        let runstate = RunState::PreRun;
        let file_path = "resources\\ldtk_practice.ldtk".to_string();
        let project: Project = Project::new(file_path);
        let tex = {
            match Texture::new(ctx, "./resources/background.png") {
                Err(e)=>{ return Err(e); },
                Ok(texture)=>{ texture },
            }
        };
        
        
        Ok(GameState{ runstate, project, tex })
    }
}
impl State for GameState{
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.runstate {
            RunState::PreRun=>{
                graphics::clear(ctx, Color::rgb8(0,0,0));
                /*
                println!("First level pixel height is {}!", self.project.levels[0].px_hei);
                
                
                println!("background color: {}", self.project.bg_color);
                println!("worldGridHeight: {}", self.project.world_grid_height);
                println!("worldGridWidth: {}", self.project.world_grid_width);
                match self.project.world_layout {
                    WorldLayout::Free=>{ println!("Free Layout."); },
                    WorldLayout::GridVania=>{ println!("GridVania Layout."); },
                    WorldLayout::LinearHorizontal=>{ println!("LinearHorizontal Layout."); },
                    WorldLayout::LinearVertical=>{ println!("LinearVertical Layout."); },
                    _=>{ println!("World Layout is something else."); },
                }
                for level in &self.project.levels {
                    println!("\nlevel: {}", level.identifier);
                    println!("bgcolor: {}", level.bg_color);
                    for n in &level.neighbours {
                        println!("neighbour dir/uid: {}, {}", n.dir, n.level_uid);
                    }
                    for fi in &level.field_instances {
                        println!("field instance:\ntype: {},\ndefuid: {},",
                        fi.field_instance_type, fi.def_uid);
                        match &fi.value {
                            None=>{ println!("None"); },
                            Some(value)=>{
                                match value {
                                    serde_json::Value::Null=>{},
                                    serde_json::Value::Bool(bool)=>{},
                                    serde_json::Value::Number(num)=>{
                                        println!("value: {},", num);
                                    },
                                    _=>{}
                                }
                            }
                        }
                    }
                    match &level.layer_instances {
                        None=>{ println!("Layer Instances: None"); }
                        Some(li)=>{
                            for i in li {
                                println!("layer_instance identifier: {}", i.identifier);
                                println!("c_hei: {}", i.c_hei);
                                println!("c_wid: {}", i.c_wid);
                                println!("grid_size: {}", i.grid_size);
                                match &i.tileset_rel_path {
                                    None=>{ println!("Tileset rel path: None"); }
                                    Some(ts_path)=>{
                                        println!("tileset path: {}", ts_path);
                                    }
                                }
                                for tiles in &i.grid_tiles {
                                    println!("tiles px: {},{}", tiles.px[0], tiles.px[1]);
                                    println!("tileset id: {}", tiles.t);
                                    
                                }
                            }
                        }
                    }
                }
                */
                /*
                for ts in &self.project.defs.tilesets {
                    println!("Tileset identifier: {}", ts.identifier);
                    println!("Tileset pxhei: {}", ts.px_hei);
                    println!("Tileset pxwid: {}", ts.px_wid);
                    println!("Tileset relpath: {}", ts.rel_path);
                    println!("Tileset tilegridsize: {}", ts.tile_grid_size);

                }
                */
                /*
                for level in &self.project.levels {
                    match &level.layer_instances {
                        None=>{},
                        Some(layers)=>{
                            println!("layer count: {}", layers.len());
                            for layer in layers.iter().rev() {
                                match layer.identifier.as_str() {
                                    _=>{}
                                }
                                println!("identifier: {}", layer.identifier);
                                for entity in &layer.entity_instances {
                                    println!("found one!");
                                }
                            }
                        }
                    }
                }
                */
                let mut test = String::new();
                for level in &self.project.levels {
                    for fi in &level.field_instances {
                        println!("identifier: {}", fi.identifier);
                        match &fi.value {
                            None=>{},
                            Some(value)=>{
                                match value {
                                    serde_json::Value::Null=>{},
                                    serde_json::Value::Bool(bool)=>{
                                        println!("bool");
                                    },
                                    serde_json::Value::Number(num)=>{
                                        println!("number");
                                    },
                                    serde_json::Value::String(s)=>{
                                        println!("string");
                                        println!("value: {}", s);
                                        test = s.clone();
                                        
                                    },
                                    serde_json::Value::Array(a)=>{
                                        println!("array");
                                        
                                    },
                                    serde_json::Value::Object(o)=>{
                                        println!("object!!!!");

                                    },
                                    _=>{},

                                }
                            }
                        }
                    }
                }
                for d in &self.project.defs.enums {
                    match d.icon_tileset_uid {
                        None=>{},
                        Some(uid)=>{
                            println!("tilesetuid: {}", uid);
                        }
                    }
                    for v in &d.values {
                        println!("id: {}", v.id);
                        match v.tile_id {
                            None=>{},
                            Some(id)=>{
                                println!("tile_id: {}", id);
                            }
                        }
                        for t in &v.tile_src_rect {
                            println!("rect: {}", t);
                        }

                    }
                }
                for t in &self.project.defs.tilesets {
                    println!("tileset def uid: {}", t.uid);
                }
                
                
            }
            RunState::Run=>{
                graphics::clear(ctx, Color::rgb8(55,55,55));
                let params = DrawParams::default()
                .position(Vec2::new(0.0, 0.0));

            }
            _=>{},
        }
        Ok(())
        
    }
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.runstate {
            RunState::PreRun=>{
                
                self.runstate = RunState::Run;
            }
            RunState::Run=>{
                
            }
            _=>{},
        }
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
