use std::{cmp::min, io};

use anyhow::{Context, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
    let deer = read_input()?;
    // let ans = farthest_distance(&deer);
    let ans = most_points(&deer);
    dbg!(ans);
    Ok(())
}

fn read_input() -> Result<Vec<ReindeerStats>> {
    io::stdin().lines().map(|l| parse_line(&l?)).try_collect()
}

fn parse_line(l: &str) -> Result<ReindeerStats> {
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

    Ok(ReindeerStats {
        speed,
        flight_duration,
        cooldown,
    })
}

#[derive(Debug, Clone, Copy)]
struct ReindeerStats {
    speed: u32,
    flight_duration: u32,
    cooldown: u32,
}

impl ReindeerStats {
    fn position_at_time(&self, mut time: u32) -> u32 {
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

const END_TIME: u32 = 2503;

/// Part 1.
#[allow(dead_code)]
fn farthest_distance(deer: &[ReindeerStats]) -> u32 {
    deer.iter()
        .map(|d| d.position_at_time(END_TIME))
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Reindeer {
    stats: ReindeerStats,
    pos: u32,
    phase: Phase,
    phase_time_remaining: u32,
    points: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Flying,
    Waiting,
}

impl Reindeer {
    fn new(stats: ReindeerStats) -> Self {
        Self {
            stats,
            pos: 0,
            phase: Phase::Flying,
            phase_time_remaining: stats.flight_duration,
            points: 0,
        }
    }

    /// Simulate the passing of 1 seconds.
    fn tick(&mut self) {
        assert_ne!(self.phase_time_remaining, 0);

        if self.phase == Phase::Flying {
            self.pos += self.stats.speed;
        }

        self.phase_time_remaining -= 1;
        if self.phase_time_remaining == 0 {
            (self.phase, self.phase_time_remaining) = match self.phase {
                Phase::Flying => (Phase::Waiting, self.stats.cooldown),
                Phase::Waiting => (Phase::Flying, self.stats.flight_duration),
            }
        }
    }
}

/// Part 2.
fn most_points(deer: &[ReindeerStats]) -> u32 {
    let mut deer = deer.iter().map(|d| Reindeer::new(*d)).collect_vec();

    for _ in 0..END_TIME {
        // Update positions.
        for d in &mut deer {
            d.tick();
        }

        // Award a point to the current leader(s).
        //
        // BUG: initially, I didn't think about ties (or read the problem
        // statement very closely, apparently). My original code broke ties
        // arbitrarily, instead of giving points to *all* leaders.
        let max_pos = deer.iter().map(|d| d.pos).max().unwrap();
        for d in &mut deer {
            if d.pos == max_pos {
                d.points += 1;
            }
        }
    }

    deer.into_iter().map(|d| d.points).max().unwrap()
}
