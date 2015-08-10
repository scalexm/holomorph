use std::io::{Read, Write};
use io::Result;
use super::*;

impl_type!(Version, 11,
    major| i8,
    minor| i8,
    release| i8,
    revision| i32,
    patch| i8,
    build_type| i8);

impl_type!(VersionExtended, 393,
    base| Version,
    install| i8,
    technology| i8);
