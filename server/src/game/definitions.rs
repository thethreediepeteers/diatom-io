use serde_json::{json, Value};

enum Definition {
    #[allow(dead_code)]
    Building(Building),
    Unit(Unit)
}

struct Building {
    label: String,
    body: Body,
    color: String,
    size: u8,
    guns: Vec<Gun>
}

struct Unit {
    label: String,
    body: Body,
    color: String,
    shape: u8,
    width: f64,
    height: f64,
    guns: Vec<Gun>
}

struct Body {
    health: u8
}

struct Gun {
    color: String,
    shape: u8,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    angle: f64,
    aspect: f64
}

fn create_defs() -> Vec<Definition> {
    let mut definitions = Vec::new();
    let gray = "#808080".to_string();
    let time = std::time::Instant::now();

    definitions.push(Definition::Unit(Unit {
        label: "Aggressor".to_string(),
        body: Body { health: 100 },
        color: "#00B0E1".to_string(), //?
        shape: 0,
        width: 65.0,
        height: 65.0,
        guns: vec![Gun {
            color: gray,
            shape: 4, // not used
            x: 0.0,
            y: 0.0,
            width: 40.0,
            height: 42.5,
            angle: 0.0,
            aspect: 1.0
        }]
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
                    guns.push(json!({"color": gun.color, "shape": gun.shape, "x": gun.x, "y": gun.y, "width": gun.width, "height": gun.height, "angle": gun.angle, "aspect": gun.aspect}));
                }
                mockups.push(json!({"index": index, "label": b.label, "color": b.color, "size": b.size, "health": b.body.health, "guns": guns}));
            }
            Definition::Unit(u) => {
                for gun in &u.guns {
                    guns.push(json!({"color": gun.color, "shape": gun.shape, "x": gun.x, "y": gun.y, "width": gun.width, "height": gun.height, "angle": gun.angle, "aspect": gun.aspect}));
                }
                mockups.push(json!({"index": index, "label": u.label, "color": u.color, "shape": u.shape, "width": u.width, "height": u.height, "health": u.body.health, "guns": guns}));
            }
        }

        index += 1;
    }

    json!(mockups)
}
