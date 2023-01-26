pub mod intree;

use nom::{
    bytes::complete::tag,
    character::complete::i32 as parse_i32,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse_coord(inp: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), parse_i32),
        tag(", "),
        preceded(tag("y="), parse_i32),
    )(inp)
}

pub fn parse_line(inp: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    preceded(
        tag("Sensor at "),
        separated_pair(parse_coord, tag(": closest beacon is at "), parse_coord),
    )(inp)
}

pub fn intersects_with(sensor: (i32, i32), beacon: (i32, i32), row_num: i32) -> Option<(i32, i32)> {
    // dbg!((sensor, beacon));
    let dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

    let row_dist = (sensor.1 - row_num).abs();

    if row_dist > dist {
        None
    } else {
        let off = dist - row_dist;
        Some((sensor.0 - off, sensor.0 + off))
    }
}
