/// toml parser and config for keybindings, etc.
use toml::{Parser,Value};
use std::fs::File;
use std::io::Read;

use glium::glutin::VirtualKeyCode;
use std::collections::{HashMap,BTreeMap};

pub type Bindings = HashMap<String,VirtualKeyCode>;
pub trait Default{
    fn default() -> Self;
}
pub type Config = BTreeMap<String,Value>;
fn load (path: &str) -> Config {
    let mut input = String::new();
    if let Some(mut file) = File::open(path).ok() {
        file.read_to_string(&mut input);
    }

    Parser::new(&input).parse().unwrap_or(BTreeMap::new())
}

impl Default for Bindings {
    fn default() -> Bindings {
        let r = load("assets/config.toml");
        let mut bindings = HashMap::new();

        {
            if let Some(table) = r.get("keys") {
                match table {
                    &Value::Table(ref keys) => {
                        for key in keys.iter() {
                            let vkey = {
                                match key.0 as &str {
                                    "A" => VirtualKeyCode::A,
                                    "S" => VirtualKeyCode::S,
                                    "D" => VirtualKeyCode::D,
                                    "W" => VirtualKeyCode::W,
                                    "R" => VirtualKeyCode::R,
                                    "Tab" => VirtualKeyCode::Tab,
                                    _ => VirtualKeyCode::F12,
                                }
                            };
                            if let Some(action) = Actions::parse_string(key.1) {
                                bindings.insert(action,vkey);
                            }
                            
                        }
                    }
                    _ => {},
                }
            }
        }

        bindings
    }
}

struct Actions;
impl Actions {
    fn parse_string(action: &Value) -> Option<String> {
        match action {
            &Value::String(ref action) => Some(action.clone()),
            _ => None,
        }
    }
}
