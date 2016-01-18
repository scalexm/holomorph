use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(AlignmentRankUpdateMessage, 6058, alignment_rank| i8, verbose| bool);
impl_type!(SetEnableAVARequestMessage, 6443, enable| bool);
impl_type!(SetEnablePVPRequestMessage, 1810, enable| bool);
impl_type!(UpdateMapPlayersAgressableStatusMessage, 6454, player_ids| Vec<VarLong>, enable| Vec<u8>);
impl_type!(UpdateSelfAgressableStatusMessage, 6456, status| i8, probation_time| i32);
