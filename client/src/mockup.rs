trait Loadable {
    fn load(value: &serde_json::Value) -> Self;
}

#[derive(Debug)]
pub struct Gun {
    pub offset: f64,
    pub direction: f64,
    pub width: f64,
    pub height: f64,
    pub angle: f64,
    pub aspect: f64,
    pub color: String,
    pub shape: u8
}

#[derive(Debug)]
pub struct Mockup {
    pub index: u16,
    pub color: String,
    pub label: String,
    pub shape: u8,
    pub width: f64,
    pub height: f64,
    pub health: u8,
    pub guns: Vec<Gun>,
}

impl Loadable for Gun {
    fn load(value: &serde_json::Value) -> Gun {
        Gun {
            offset: value["offset"].as_f64().unwrap(),
            direction: value["direction"].as_f64().unwrap(),
            width: value["width"].as_f64().unwrap(),
            height: value["height"].as_f64().unwrap(),
            angle: value["angle"].as_f64().unwrap(),
            aspect: value["aspect"].as_f64().unwrap(),
            color: value["color"].as_str().unwrap().to_string(),
            shape: value["shape"].as_u64().unwrap() as u8
        }
    }
}

impl Loadable for Mockup {
    fn load(value: &serde_json::Value) -> Mockup {
        Mockup {
            index: value["index"].as_u64().unwrap() as u16,
            color: value["color"].as_str().unwrap().to_string(),
            label: value["label"].as_str().unwrap().to_string(),
            shape: value["shape"].as_u64().unwrap() as u8,
            width: value["width"].as_f64().unwrap(),
            height: value["height"].as_f64().unwrap(),
            health: value["health"].as_u64().unwrap() as u8,
            guns: value["guns"].as_array().unwrap().iter().map(|g| Gun::load(g)).collect()
        }
    }
}

pub struct Mockups {
    mockups: Vec<Mockup>
}

impl Mockups {
    pub fn new() -> Self {
        Self { mockups: Vec::new() }
    }

    pub fn find(&self, index: u16) -> Option<&Mockup> {
        self.mockups.iter().find(|m| m.index == index)
    }

    pub fn get(&self, index: u16) -> &Mockup {
        self.find(index).unwrap()
    }

    pub fn load(&mut self, mockups: serde_json::Value) {
        mockups.as_array().unwrap().iter()
            .for_each(|m| self.mockups.push(Mockup::load(m)));
    }

    pub fn as_vec(&self) -> &Vec<Mockup> {
        &self.mockups
    }
}