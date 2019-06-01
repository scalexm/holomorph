use crate::types::game::friend::AbstractContactInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 499)]
pub struct ArenaRankInfos<'a> {
    pub ranking: ArenaRanking<'a>,
    pub league_ranking: ArenaLeagueRanking<'a>,
    #[protocol(var)]
    pub victory_count: u16,
    #[protocol(var)]
    pub fightcount: u16,
    pub num_fight_needed_for_ladder: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 555)]
pub struct LeagueFriendInformations<'a> {
    pub base: AbstractContactInformations<'a>,
    #[protocol(var)]
    pub player_id: u64,
    pub player_name: &'a str,
    pub breed: i8,
    pub sex: bool,
    #[protocol(var)]
    pub level: u16,
    #[protocol(var)]
    pub league_id: i16,
    #[protocol(var)]
    pub total_league_points: i16,
    pub ladder_position: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 553)]
pub struct ArenaLeagueRanking<'a> {
    #[protocol(var)]
    pub rank: u16,
    #[protocol(var)]
    pub league_id: u16,
    #[protocol(var)]
    pub league_points: i16,
    #[protocol(var)]
    pub total_league_points: i16,
    pub ladder_position: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 554)]
pub struct ArenaRanking<'a> {
    #[protocol(var)]
    pub rank: u16,
    #[protocol(var)]
    pub best_rank: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
