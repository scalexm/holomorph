use postgres::rows::Row;

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
}

impl MapData {
    pub fn from_sql<'a>(row: Row<'a>) -> (i32, MapData) {
        let id = row.get("id");
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
            sub_area_id: row.get("sub_area_id"),
            left: row.get("left"),
            right: row.get("right"),
            top: row.get("top"),
            bottom: row.get("bottom"),
            cells: cells,
        })
    }
}

pub struct SubAreaData {
    id: i16,
    area_id: i16,
    monsters: Vec<i16>,
}

impl SubAreaData {
    pub fn from_sql<'a>(row: Row<'a>) -> (i16, SubAreaData) {
        use std::fmt::Write;

        let id = row.get("id");
        let monsters: String = row.get("monsters");

        let mut error = String::new();
        write!(&mut error, "bad monsters data, sub area id {}", id).unwrap();

        (id, SubAreaData {
            id: id,
            area_id: row.get("area_id"),
            monsters: if monsters.is_empty() { Vec::new() } else {
                monsters.split(",").map(|s| s.parse().ok().expect(&error)).collect()
            },
        })
    }
}

pub struct AreaData {
    id: i16,
    priority: i16,
}

impl AreaData {
    pub fn from_sql<'a>(row: Row<'a>) -> (i16, AreaData) {
        let id = row.get("id");

        (id, AreaData {
            id: id,
            priority: row.get("priority"),
        })
    }
}
