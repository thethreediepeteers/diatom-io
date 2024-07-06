use serde_json::{json, Value};
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub enum Definition {
    #[allow(dead_code)]
    Building(Building),
    Unit(Unit),
}

#[derive(Debug, Clone)]
struct Building {
    label: String,
    body: Body,
    color: String,
    size: u8,
    guns: Vec<Gun>,
}

#[derive(Debug, Clone)]
struct Unit {
    label: String,
    body: Body,
    color: String,
    shape: u8,
    width: f64,
    height: f64,
    guns: Vec<Gun>,
}

#[derive(Debug, Clone)]
struct Body {
    health: u8,
}

#[derive(Debug, Clone)]
struct Gun {
    color: String,
    shape: u8,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    angle: f64,
    aspect: f64,
}

fn create_defs() -> Vec<Definition> {
    let mut definitions = Vec::new();
    let gray = "#808080".to_string();
    let time = std::time::Instant::now();

    definitions.push(Definition::Unit(Unit {
        label: "Aggressor".to_string(),
        body: Body { health: 100 },
        color: "#00B0E1".to_string(),
        shape: 0,
        width: 65.0,
        height: 65.0,
        guns: vec![
            Gun {
                color: gray.clone(),
                shape: 4,
                x: 0.0,
                y: 0.0,
                width: 42.5,
                height: 57.5,
                angle: 0.0,
                aspect: 1.0,
            },
        ],
    }));

    println!("Mockups loaded in {:?}", time.elapsed());

    definitions
}

pub fn generate_mockups() -> Value {
    let defs = create_defs();

    let mut index = 0;

    let mut mockups: Vec<Value> = Vec::new();
    for def in &defs {
        let mut guns: Vec<Value> = Vec::new();

        match def {
            Definition::Building(b) => {
                for gun in &b.guns {
                    let off = Vector::new(gun.x, gun.y);
                    let direction = off.direction();
                    let offset = off.length();

                    guns.push(json!({"color": gun.color, "shape": gun.shape, "offset": offset, "direction": direction, "width": gun.width, "height": gun.height, "angle": gun.angle * PI / 180.0, "aspect": gun.aspect}));
                }
                mockups.push(json!({"index": index, "label": b.label, "color": b.color, "size": b.size, "health": b.body.health, "guns": guns}));
            }
            Definition::Unit(u) => {
                for gun in &u.guns {
                    let off = Vector::new(gun.x, gun.y);
                    let direction = off.direction();
                    let offset = off.length();

                    guns.push(json!({"color": gun.color, "shape": gun.shape, "offset": offset, "direction": direction, "width": gun.width, "height": gun.height, "angle": gun.angle * PI / 180.0, "aspect": gun.aspect}));
                }
                mockups.push(json!({"index": index, "label": u.label, "color": u.color, "shape": u.shape, "width": u.width, "height": u.height, "health": u.body.health, "guns": guns}));
            }
        }

        index += 1;
    }

    json!(mockups)
}

struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn direction(&self) -> f64 {
        self.y.atan2(self.x)
    }
}
