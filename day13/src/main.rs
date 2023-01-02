pub mod structs;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();

    let bits = bit_parser::parse(&file_contents).unwrap();

    match part.as_str() {
        "p1" => p1(bits),
        // "p2" => p2(map),
        _ => println!(""),
    }
}

fn p1(packets: Vec<structs::Pair>) {
    let mut total = 0;

    for (idx, packet_pair) in packets.iter().enumerate() {
        if !packet_pair.is_sorted() {
            continue;
        }

        total += (idx as u32) + 1;
    }

    println!("total = {}", total);
}

mod bit_parser {

    use crate::structs::{self, Packet, Pair};

    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{newline, u8},
        combinator::map,
        multi::{separated_list0, separated_list1},
        sequence::{delimited, separated_pair},
        Finish, IResult,
    };

    fn integer(s: &str) -> IResult<&str, structs::Packet> {
        map(u8, structs::Packet::Bit)(s)
    }

    fn list(s: &str) -> IResult<&str, structs::Packet> {
        let list_contents = separated_list0(tag(","), packet);
        map(
            delimited(tag("["), list_contents, tag("]")),
            structs::Packet::Packets,
        )(s)
    }

    fn packet(s: &str) -> IResult<&str, structs::Packet> {
        alt((integer, list))(s)
    }

    fn packet_pair(s: &str) -> IResult<&str, Pair> {
        let (s, (first, second)) = separated_pair(packet, newline, packet)(s)?;
        Ok((
            s,
            structs::Pair {
                packet1: first,
                packet2: second,
            },
        ))
    }

    pub fn parse(s: &str) -> Result<Vec<structs::Pair>> {
        let result = separated_list1(tag("\n\n"), packet_pair)(s).finish();
        let (_, pair_list) = result.map_err(|e| anyhow!("{e}"))?;
        Ok(pair_list)
    }
}
