/*
boss stats
==========
Hit Points: 51
Damage: 9

plan:
[x] impl cost-first-search
    * it's a little hard for me to track all the details;
         maybe it could be simplified? (todo: come back to it after testing)
[x] impl next_state() fxn
[ ] try it out! :)
    * fix compiler errs
[ ] impl effects
[ ] test on the examples they give
[ ] review the code
    * todo: must State be ord? .. think about heap API details

*/

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet}, mem,
};

const PART_2: bool = true;

fn main() {
    let ans = min_win();
    dbg!(ans); // part1: 900
    // 1189 < part2 < 1242
}

/// Helper type to make to_visit a min-heap.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ToVisit {
    mana_spent: u32,
    state: State,
}

impl ToVisit {
    /// Sort decreasing by mana_spent.
    fn cmp_key(&self) -> (Reverse<u32>, &State) {
        (Reverse(self.mana_spent), &self.state)
    }
}

impl Ord for ToVisit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_key().cmp(&other.cmp_key())
    }
}

impl PartialOrd for ToVisit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Find the winning state which is 'closest' -- in the sense of spending the
/// least mana on spells.
fn min_win() -> u32 {
    let mut seen = HashMap::new();
    let mut to_visit = BinaryHeap::new(); // min-heap

    let start = State::game_start();
    seen.insert(start.clone(), 0);
    to_visit.push(ToVisit { mana_spent: 0, state: start });

    while let Some(curr) = to_visit.pop() {
        if curr.mana_spent > seen[&curr.state] {
            continue;
        }

        match curr.state.is_game_over() {
            GameOver::Loss => continue,
            GameOver::Win => return curr.mana_spent,
            GameOver::Ongoing => (),
        }

        for (mana_cost, next) in curr.state.next_states() {
            let mana_spent = curr.mana_spent + mana_cost;
            if !seen.contains_key(&next) || mana_spent < seen[&next] {
                seen.insert(next.clone(), mana_spent);
                to_visit.push(ToVisit { mana_spent, state: next });
            }
        }
    }

    panic!("no winning state found");
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    turn: Who,
    you: Fighter,
    boss: Fighter,
    active_effects: Vec<(&'static str, u32)>, // turns remaining
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Fighter {
    hp: i32,
    mana: i32,
    damage: i32,
    armor: i32,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Who {
    You,
    Boss,
}

enum GameOver {
    Win,
    Loss,
    Ongoing,
}

impl State {
    fn game_start() -> Self {
        Self {
            turn: Who::You,
            you: Fighter {
                hp: 50,
                mana: 500,
                damage: 0,
                armor: 0,
            },
            boss: Fighter {
                hp: 51,
                mana: 0,
                damage: 9,
                armor: 0,
            },
            active_effects: vec![],
        }
    }

    fn is_game_over(&self) -> GameOver {
        if self.you.hp <= 0 {
            GameOver::Loss
        } else if self.boss.hp <= 0 {
            GameOver::Win
        } else {
            GameOver::Ongoing
        }
    }

    fn tick_effects(&mut self) {
        let mut out = vec![];
        for (e, mut dur) in mem::take(&mut self.active_effects) {
            match e {
                "shield" => (),
                "poison" => self.boss.hp -= 3,
                "recharge" => self.you.mana += 101,
                _ => panic!("not an effect: {e:?}"),
            }

            dur -= 1;
            if dur <= 0 {
                // effect expires.
                if e == "shield" {
                    self.you.armor -= 7;
                }
            } else {
                out.push((e, dur));
            }
        }
        self.active_effects = out;
    }

    fn next_states(mut self) -> Vec<(u32, Self)> {
        // todo: if the boss dies to a poison tick, we'll spend 53 mana that we didn't have to.
        self.tick_effects();

        match self.turn {
            Who::Boss => {
                let mut next = self.clone();
                next.boss_hit();
                next.turn = Who::You;
                return vec![(0, next)];
            }
            Who::You => (),
        }

        let mut out = vec![];
        for action in [
            Self::missile, Self::drain, Self::shield, Self::poison, Self::recharge
        ] {
            let mut next = self.clone();

            if PART_2 {
                next.you.hp -= 1;
            }
            action(&mut next);
            next.turn = Who::Boss;

            if next.you.mana < 0 {
                continue; // not enough mana
            }
            if duplicate_effect(&next.active_effects) {
                continue; // can't re-apply an already-active effect
            }
            let mana_cost = (self.you.mana - next.you.mana).try_into().unwrap();
            out.push((mana_cost, next));
        }
        out
    }

    fn boss_hit(&mut self) {
        self.you.hp -= (self.boss.damage - self.you.armor).clamp(1, i32::MAX);
    }

    fn missile(&mut self) {
        self.you.mana -= 53;
        self.boss.hp -= 4;
    }

    fn drain(&mut self) {
        self.you.mana -= 73;
        self.boss.hp -= 2;
        self.you.hp += 2; // assuming over-healing is allowed
    }

    fn shield(&mut self) {
        self.you.mana -= 113;
        self.active_effects.push(("shield", 6));
        self.you.armor += 7;
    }

    fn poison(&mut self) {
        self.you.mana -= 173;
        self.active_effects.push(("poison", 6));
    }

    fn recharge(&mut self) {
        self.you.mana -= 229;
        self.active_effects.push(("recharge", 5));
    }
}

fn duplicate_effect(effects: &[(&'static str, u32)]) -> bool {
    let set: HashSet<_> = effects.iter().map(|(e, _)| e).collect();
    let all_good = set.len() == effects.len();
    !all_good
}
