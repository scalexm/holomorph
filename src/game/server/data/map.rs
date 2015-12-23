use std::collections::HashMap;

/*fn init_grid() -> Vec<(i8, i8)> {
    let mut cells = Vec::new();
    let mut loc1 = 0;
	let mut loc2 = 0;
	for _ in 0..20 {
		for loc4 in 0..14 {
            cells.push((loc1 + loc4, loc2 + loc4));
		}
		loc1 += 1;
		for loc4 in 0..14 {
            cells.push((loc1 + loc4, loc2 + loc4));
		}
		loc2 += 1;
	}
    cells
}

lazy_static! { pub static ref GRID: Vec<(i8, i8)> = init_grid(); }*/

#[derive(Queriable)]
pub struct MapData {
    id: i32,
    pos_x: i16,
    pos_y: i16,
    outdoor: bool,
    capabilities: i32,
    sub_area_id: i16,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    cells: Vec<u8>,

    client_top: i32,
    client_bottom: i32,

    custom_left_cell: i16,
    custom_right_cell: i16,
    custom_top_cell: i16,
    custom_bottom_cell: i16,
}

impl MapData {
    pub fn verif_cells(&self) {
        if self.cells.len() != 1120 {
            panic!("map id {} has an invalid field `cells`", self.id);
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn sub_area_id(&self) -> i16 {
        self.sub_area_id
    }

    pub fn get_free_cell(&self) -> Option<i16> {
        for i in 0..560 {
            if self.cells[i * 2] & 1 == 1 {
                return Some(i as i16)
            }
        }
        None
    }

    pub fn get_cell_data(&self, cell: i16) -> (u8, u8) {
        (self.cells[(cell * 2) as usize], self.cells[(cell * 2 + 1) as usize])
    }

    pub fn left(&self) -> i32 {
        self.left
    }

    pub fn right(&self) -> i32 {
        self.right
    }

    pub fn top(&self) -> i32 {
        self.top
    }

    pub fn bottom(&self) -> i32 {
        self.bottom
    }

    pub fn client_top(&self) -> i32 {
        self.client_top
    }

    pub fn client_bottom(&self) -> i32 {
        self.client_bottom
    }

    pub fn custom_left_cell(&self) -> i16 {
        self.custom_left_cell
    }

    pub fn custom_right_cell(&self) -> i16 {
        self.custom_right_cell
    }

    pub fn custom_top_cell(&self) -> i16 {
        self.custom_top_cell
    }

    pub fn custom_bottom_cell(&self) -> i16 {
        self.custom_bottom_cell
    }
}

#[derive(Queriable)]
pub struct SubAreaData {
    id: i16,
    area_id: i16,
    monsters: Vec<i16>,
}

impl SubAreaData {
    pub fn verif_area(&self, areas: &HashMap<i16, AreaData>) {
        if !areas.contains_key(&self.area_id) {
            panic!("sub area id {} has an incorrect field `area_id`", self.id);
        }
    }

    pub fn area_id(&self) -> i16 {
        self.area_id
    }

    pub fn id(&self) -> i16 {
        self.id
    }
}

#[derive(Queriable)]
pub struct AreaData {
    id: i16,
    priority: i16,
}

impl AreaData {
    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn priority(&self) -> i16 {
        self.priority
    }
}
