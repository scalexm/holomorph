mod map;
mod breed;

use std::sync::Arc;
use config::Config;
use shared::{net, database};
use diesel::*;
use character::CharacterMinimal;
use self::map::*;
use self::breed::*;
use std::collections::{HashMap, LinkedList};
use server::{self, Server};
use std::sync::mpsc;

#[derive(Clone)]
pub struct GameServerData {
    pub server: server::Sender,
    pub io_loop: net::Sender,
    pub cnf: Arc<Config>,
    pub auth_db: database::Sender,
    pub db: database::Sender,
    pub maps: Arc<HashMap<i32, MapData>>,
    pub sub_areas: Arc<HashMap<i16, SubAreaData>>,
    pub areas: Arc<HashMap<i16, AreaData>>,
    pub breeds: Arc<HashMap<i16, BreedData>>,
    pub heads: Arc<HashMap<i16, HeadData>>,
    shutdown: mpsc::Sender<()>,
}

impl GameServerData {
    pub fn new(server: server::Sender, io_loop: net::Sender, cnf: Config, db: database::Sender,
               auth_db: database::Sender, shutdown: mpsc::Sender<()>) -> Self {
        GameServerData {
            server: server,
            io_loop: io_loop,
            cnf: Arc::new(cnf),
            auth_db: auth_db,
            db: db,
            maps: Arc::new(HashMap::new()),
            sub_areas: Arc::new(HashMap::new()),
            areas: Arc::new(HashMap::new()),
            breeds: Arc::new(HashMap::new()),
            heads: Arc::new(HashMap::new()),
            shutdown: shutdown,
        }
    }

    pub fn load(&mut self, conn: &Connection) {
        use shared::database::schema::{
            areas,
            sub_areas,
            maps,
            map_positions,
            breeds,
            breed_heads
        };

        self.areas = Arc::new(
            areas::table.load::<AreaData>(conn)
                        .unwrap()
                        .map(|a| (a.id(), a))
                        .collect()
        );
        info!("loaded {} areas", self.areas.len());

        self.sub_areas = Arc::new(
            sub_areas::table.load::<SubAreaData>(conn)
                            .unwrap()
                            .map(|s| {
                                s.verif_area(&*self.areas);
                                (s.id(), s)
                            })
                            .collect()
        );
        info!("loaded {} sub areas", self.sub_areas.len());

        self.maps = Arc::new(
            maps::table.inner_join(map_positions::table)
                       .select((
                           map_positions::id,
                           map_positions::pos_x,
                           map_positions::pos_y,
                           map_positions::outdoor,
                           map_positions::capabilities,
                           map_positions::sub_area_id,
                           maps::left,
                           maps::right,
                           maps::top,
                           maps::bottom,
                           maps::cells,
                           maps::client_top,
                           maps::client_bottom,
                           maps::custom_left_cell,
                           maps::custom_right_cell,
                           maps::custom_top_cell,
                           maps::custom_bottom_cell,
                       ))
                       .load::<MapData>(conn)
                       .unwrap()
                       .map(|m| {
                          m.verif_cells();
                          (m.id(), m)
                       })
                       .collect()
        );
        info!("loaded {} maps", self.maps.len());

        self.breeds = Arc::new(
            breeds::table.select((
                breeds::id,
                breeds::male_look,
                breeds::female_look,
                breeds::spawn_map
            ))
            .load::<BreedData>(conn)
            .unwrap()
            .map(|b| (b.id(), b))
            .collect()
        );
        info!("loaded {} breeds", self.breeds.len());

        self.heads = Arc::new(
            breed_heads::table.load::<HeadData>(conn)
                              .unwrap()
                              .map(|h| (h.id(), h))
                              .collect()
        );
        info!("loaded {} breed heads", self.heads.len());
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown.send(());
    }
}

impl Server {
    pub fn load(&mut self, conn: &Connection) {
        use shared::database::schema::character_minimals;

        self.characters = character_minimals::table.load::<CharacterMinimal>(conn)
                                                   .unwrap()
                                                   .map(|ch| (ch.id(), ch))
                                                   .collect();

        let (nicknames, names_plus_accounts): (_, LinkedList<(_, _)>) =
            self.characters.iter().map(|(id, ch)| {
                ((ch.account_nickname().to_lowercase(), *id),
                ((ch.name().to_lowercase(), *id), (ch.account_id(), *id)))
            }).unzip();
        self.character_nicknames = nicknames;
        let (names, accounts) = names_plus_accounts.into_iter().unzip();
        self.character_names = names;
        self.character_accounts = accounts;
        info!("loaded {} characters", self.characters.len());
    }
}
