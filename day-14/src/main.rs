use std::{cmp::min, io};

use anyhow::{Context, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
    let deer = read_input()?;
    let ans = farthest_distance(&deer);
    dbg!(ans);
    Ok(())
}

fn read_input() -> Result<Vec<Reindeer>> {
    io::stdin().lines().map(|l| parse_line(&l?)).try_collect()
}

fn parse_line(l: &str) -> Result<Reindeer> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        let re =
            r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$";
        Regex::new(re).unwrap()
    });

    let caps = RE.captures(l).with_context(|| l.to_string())?;
    let _name = &caps[1];
    let speed = caps[2].parse()?;
    let flight_duration = caps[3].parse()?;
    let cooldown = caps[4].parse()?;

    Ok(Reindeer {
        speed,
        flight_duration,
        cooldown,
    })
}

struct Reindeer {
    speed: u32,
    flight_duration: u32,
    cooldown: u32,
}

impl Reindeer {
    fn position(&self, mut time: u32) -> u32 {
        let mut curr_pos = 0;

        // Alternate b/w flying and waiting.
        while time != 0 {
            // Flying.
            let segment = min(self.flight_duration, time);
            time -= segment;
            curr_pos += segment * self.speed;

            // Waiting.
            let segment = min(self.cooldown, time);
            time -= segment;
        }

        curr_pos
    }
}

fn farthest_distance(deer: &[Reindeer]) -> u32 {
    let end_time = 2503;
    deer.iter().map(|d| d.position(end_time)).max().unwrap()
}
