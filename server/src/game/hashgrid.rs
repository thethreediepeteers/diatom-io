use core::ops::AddAssign;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct XY {
    pub x: f32,
    pub y: f32,
}

impl AddAssign for XY {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Box {
    pub entity_index: i32,
    pub min: XY,
    pub max: XY,
}

impl Box {
    pub fn new(index: i32, x: f32, y: f32, size: f32) -> Self {
        Self {
            entity_index: index,
            min: XY { x, y },
            max: XY {
                x: x + size,
                y: y + size,
            },
        }
    }
}

struct Cell {
    objects: Vec<Box>,
}

pub struct HashGrid {
    cells: HashMap<(i32, i32), Cell>,
    cell_size: i32,
    cell_size_log2: i32,
}

impl HashGrid {
    pub fn new(cell_size: i32) -> Self {
        Self {
            cells: HashMap::new(),
            cell_size,
            cell_size_log2: cell_size.trailing_zeros() as i32,
        }
    }

    pub fn insert(&mut self, object: Box) {
        let x1 = object.min.x as i32 >> self.cell_size_log2;
        let y1 = object.min.y as i32 >> self.cell_size_log2;
        let x2 = object.max.x as i32 >> self.cell_size_log2;
        let y2 = object.max.y as i32 >> self.cell_size_log2;

        for i in x1..=x2 {
            for j in y1..=y2 {
                let pos = (i, j);
                self.cells
                    .entry(pos)
                    .or_insert(Cell {
                        objects: Vec::new(),
                    })
                    .objects
                    .push(object);
            }
        }
    }

    pub fn remove(&mut self, object: Box) {
        let x1 = object.min.x as i32 >> self.cell_size_log2;
        let y1 = object.min.y as i32 >> self.cell_size_log2;
        let x2 = object.max.x as i32 >> self.cell_size_log2;
        let y2 = object.max.y as i32 >> self.cell_size_log2;

        for i in x1..=x2 {
            for j in y1..=y2 {
                if let Some(cell) = self.cells.get_mut(&(i, j)) {
                    cell.objects.retain(|&obj| obj != object);
                    if cell.objects.is_empty() {
                        self.cells.remove(&(i, j));
                    }
                }
            }
        }
    }

    pub fn query(&self, region: Box) -> Vec<Box> {
        let mut result = Vec::new();

        let x1 = region.min.x as i32 >> self.cell_size_log2;
        let y1 = region.min.y as i32 >> self.cell_size_log2;
        let x2 = region.max.x as i32 >> self.cell_size_log2;
        let y2 = region.max.y as i32 >> self.cell_size_log2;

        for i in x1..=x2 {
            for j in y1..=y2 {
                if let Some(cell) = self.cells.get(&(i, j)) {
                    for obj in &cell.objects {
                        if obj.min.x <= region.max.x
                            && obj.max.x >= region.min.x
                            && obj.min.y <= region.max.y
                            && obj.max.y >= region.min.y
                            && obj != &region
                        {
                            result.push(*obj);
                        }
                    }
                }
            }
        }

        result
    }
}
