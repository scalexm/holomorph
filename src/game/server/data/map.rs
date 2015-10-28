use postgres::rows::Row;
use std::collections::HashMap;

fn init_grid() -> Vec<(i8, i8)> {
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

lazy_static! { pub static ref GRID: Vec<(i8, i8)> = init_grid(); }

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
    pub fn from_sql<'a>(subareas: &HashMap<i16, SubAreaData>, row: Row<'a>) -> (i32, Self) {
        let id = row.get("id");
        let sub_area_id = row.get("sub_area_id");

        if !subareas.contains_key(&sub_area_id) {
            panic!("map id {} has an unknown sub area id", id);
        }

        let cells: Vec<u8> = row.get("cells");
        if cells.len() != 1120 {
            panic!("bad cell data, map id {}", id);
        }

        (id, MapData {
            id: id,
            pos_x: row.get("pos_x"),
            pos_y: row.get("pos_y"),
            outdoor: row.get("outdoor"),
            capabilities: row.get("capabilities"),
            sub_area_id: sub_area_id,
            left: row.get("left"),
            right: row.get("right"),
            top: row.get("top"),
            bottom: row.get("bottom"),
            cells: cells,

            client_top: row.get("client_top"),
            client_bottom: row.get("client_bottom"),

            custom_left_cell: row.get("custom_left_cell"),
            custom_right_cell: row.get("custom_right_cell"),
            custom_top_cell: row.get("custom_top_cell"),
            custom_bottom_cell: row.get("custom_bottom_cell"),
        })
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

pub struct SubAreaData {
    id: i16,
    area_id: i16,
    monsters: Vec<i16>,
}

impl SubAreaData {
    pub fn from_sql<'a>(areas: &HashMap<i16, AreaData>, row: Row<'a>) -> (i16, Self) {
        let id = row.get("id");
        let area_id = row.get("area_id");

        if !areas.contains_key(&area_id) {
            panic!("sub area id {} has an unknown area id", id);
        }

        let monsters: String = row.get("monsters");
        let error = format!("bad monsters data, sub area id {}", id);

        (id, SubAreaData {
            id: id,
            area_id: area_id,
            monsters: if monsters.is_empty() {
                Vec::new()
            } else {
                monsters.split(",").map(|s| s.parse().ok().expect(&error)).collect()
            },
        })
    }

    pub fn area_id(&self) -> i16 {
        self.area_id
    }

    pub fn id(&self) -> i16 {
        self.id
    }
}

pub struct AreaData {
    id: i16,
    priority: i16,
}

impl AreaData {
    pub fn from_sql<'a>(row: Row<'a>) -> (i16, Self) {
        let id = row.get("id");

        (id, AreaData {
            id: id,
            priority: row.get("priority"),
        })
    }

    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn priority(&self) -> i16 {
        self.priority
    }
}
