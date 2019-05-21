//! Basic hello world example.
extern crate ggez;
extern crate mint;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;

trait State {
    fn get_name(&self) -> &'static str;
    //add default for this function 
    fn change_state(&mut self) -> bool;
    fn update(&mut self, _ctx: &mut Context) -> GameResult; 
    fn draw(&mut self, ctx: &mut Context) -> GameResult; 
}

// First we make a structure to contain the game's state
struct MainState {
    name: &'static str,
    frames: usize,
    text: graphics::Text,
    to_next_state: bool, 
    count: u32
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        let text = graphics::Text::new(("Hello world!", font, 48.0));

        let s = MainState { name: "Main state", frames: 0, text, to_next_state: false, count: 0};
        Ok(s)
    }
}

impl State for MainState{
    fn get_name(&self) -> &'static str{
        self.name
    }

    fn change_state(&mut self) -> bool{
        if self.to_next_state {
            self.to_next_state = false;
            return true;  
        }
        false 
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Drawables are drawn from their top-left corner.
        let offset = self.frames as f32 / 10.0;
        let dest_point = mint::Point2{x: offset, y: offset};
        // let dest_point = cgmath::Point2::new(0.5, -0.5);
        graphics::draw(ctx, &self.text, (dest_point,))?;
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
            println!("FRAME NUMBER: {:?}", self.frames);
            self.count += 1;
        }

        if self.count > 10{
            self.to_next_state = true;
            self.count = 0;
        }

        Ok(())
    }
}

// First we make a structure to contain the game's state
struct GameState {
    name: &'static str,
    frames: usize,
    text: graphics::Text,
    to_next_state: bool
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        let text = graphics::Text::new(("Game State!", font, 48.0));

        let s = GameState { name: "Game state", frames: 0, text, to_next_state: false };
        Ok(s)
    }
}

impl State for GameState{
    fn get_name(&self) -> &'static str{
        self.name
    }

    fn change_state(&mut self) -> bool{
        if self.to_next_state {
            self.to_next_state = false;
            return true;    
        }
        false
        
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Drawables are drawn from their top-left corner.
        let offset = self.frames as f32 / 10.0;
        let dest_point = mint::Point2{x: offset, y: offset};
        // let dest_point = cgmath::Point2::new(0.5, -0.5);
        graphics::draw(ctx, &self.text, (dest_point,))?;
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        Ok(())
    }
}

struct StateManager {
    resource_dir: path::PathBuf,
    states: Vec<Box<dyn State>>,
    current_state: Option<Box<dyn State>>

}

impl StateManager {
    pub fn new(resource_dir: path::PathBuf) -> GameResult<StateManager> {
        let states = Vec::new(); 

        Ok(StateManager{resource_dir: resource_dir, states: states, current_state: None})
    }

    pub fn add_state(&mut self, state: Box<State>) {
        self.states.push(state);
    }

    pub fn change_state(&mut self){
        //Add error handler 
        self.current_state = Some(self.states.pop().unwrap());
    }
}

impl event::EventHandler for StateManager {
    fn update(&mut self, _ctx: &mut Context) -> GameResult { 
        match &mut self.current_state {
              Some(current_state) => {
                current_state.update(_ctx)?;
                println!("CURRENT STATE: {}", current_state.get_name());
                //always at the end of the statement 
                if current_state.change_state(){
                    self.change_state();
                }
              },
              None => (),
        }  

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.current_state {
              Some(current_state) => {current_state.draw(ctx)?;},
              None => (),
        }  
        Ok(())
    }
}

pub fn main() {
    //add match to prevent error from GameResult
    let title = "My Awesome Game";
    let author = "Thewrath";
    let resource_path = "./resources";
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push(resource_path);
        path
    } else {
        path::PathBuf::from(resource_path)
    };
    let cb = ggez::ContextBuilder::new(title, author).add_resource_path(&resource_dir);
    let mut context: Context;
    let mut event_loop: event::EventsLoop; 
    match cb.build() {
        Ok((ctx, e_l)) => {
            context = ctx; 
            event_loop = e_l; 
        },
        Err(_) => panic!("Context cannot be initialize"),
    }
    let mut state_manager = StateManager::new(resource_dir).unwrap();
    
    //add match to prevent error from GameResult
    
    let main_state = MainState::new(&mut context).unwrap();
    let game_state = GameState::new(&mut context).unwrap();
    
    state_manager.add_state(Box::new(game_state));
    state_manager.add_state(Box::new(main_state));
    state_manager.change_state();

    match event::run(&mut context, &mut event_loop, &mut state_manager) {
        Err(_) => panic!("Run exception"),
        _ => (), 
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//TODO 
/*
    -Move FPS management in the stateManager and restrained at 60 FPS all state 
    -Put the name and the fps display system in the stateManager 
*/