/*
boss stats
----------
Hit Points: 103
Damage: 9
Armor: 2

the plan:
* iterate thru all combinations of equipment
    * details tbd
* for each, sim the fight & see who wins
    * (this is fast; at most 200 steps)

*/

use std::cmp::min;

fn main() {
    let boss = Fighter {
        hp: 103,
        damage: 9,
        armor: 2,
    };

    let mut best = u32::MAX;
    for (cost, you) in possible_stats() {
        if fight(you, boss) {
            best = min(best, cost);
        }
    }
    dbg!(best);
}

fn possible_stats() -> Vec<(u32, Fighter)> {
    let weapons: Vec<(u32, i32)> = vec![
        (8, 4),
        (10, 5),
        (25, 6),
        (40, 7),
        (74, 8),
    ];
    let armor: Vec<(u32, i32)> = vec![
        (0, 0),
        (13, 1),
        (31, 2),
        (53, 3),
        (75, 4),
        (102, 5),
    ];
    let rings: Vec<(u32, i32, i32)> = vec![
        (0, 0, 0),
        (0, 0, 0),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];
    
    let mut out = vec![];
    for w in weapons {
        for &a in &armor {
            for &r1 in &rings {
                for &r2 in &rings {
                    if r1 == r2 {
                        continue;
                    }

                    let cost = w.0 + a.0 + r1.0 + r2.0;
                    let damage = w.1 + r1.1 + r2.1;
                    let armor = a.1 + r1.2 + r2.2;
                    out.push((cost, Fighter { hp: 100, damage, armor }));
                }
            }
        }
    }
    out
}

#[derive(Debug, Clone, Copy)]
struct Fighter {
    hp: i32,
    damage: i32,
    armor: i32,
}

/// Return true if you win.
fn fight(mut you: Fighter, mut boss: Fighter) -> bool {
    loop {
        let hit = (you.damage - boss.armor).clamp(1, i32::MAX);
        boss.hp -= hit;
        if boss.hp <= 0 {
            return true;
        }

        let hit = (boss.damage - you.armor).clamp(1, i32::MAX);
        you.hp -= hit;
        if you.hp <= 0 {
            return false;
        }
    }
}
