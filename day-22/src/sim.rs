use std::ops::ControlFlow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    turn: Who,
    you: Fighter,
    boss: Fighter,
    effects: Effects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Who {
    You,
    Boss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Fighter {
    hp: u32,
    mana: u32,
    damage: u32,
    armor: u32,
}

/// Each field stores the number of turns remaining.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Effects {
    shield: usize,
    poison: usize,
    recharge: usize,
}

impl State {
    pub fn start() -> Self {
        let this = Self {
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
            effects: Effects {
                shield: 0,
                poison: 0,
                recharge: 0,
            },
        };
        debug_assert!(!this.is_losing());
        this
    }

    pub fn is_winning(&self) -> bool {
        assert!(!self.is_losing());
        self.boss.hp == 0
    }

    pub fn next_states(&self) -> Vec<Edge> {
        assert!(!self.is_losing());

        let mut this = self.clone();
        this.tick_effects();
        debug_assert!(!this.is_losing());
        if this.is_winning() {
            // Early return, since the boss died so players stop taking turns.
            return vec![Edge {
                state: this,
                cost: 0,
            }];
        }

        if this.turn == Who::Boss {
            // Boss hit.
            let hit = (this.boss.damage - this.you.armor).clamp(1, u32::MAX);
            this.you.hp = this.you.hp.saturating_sub(hit);
            this.turn = Who::You;

            if this.is_losing() {
                return vec![];
            } else {
                return vec![Edge {
                    state: this,
                    cost: 0,
                }];
            }
        }

        //
        // Player turn.
        //

        let mut out = vec![];
        for action in Self::SPELLS {
            let mut next = this.clone();
            if action(&mut next).is_break() {
                continue;
            }
            next.turn = Who::Boss;

            let cost = this.you.mana - next.you.mana;
            debug_assert!(!next.is_losing());
            out.push(Edge { cost, state: next });
        }
        out
    }

    /// Note that this may kill the boss.
    fn tick_effects(&mut self) {
        if self.effects.shield > 0 {
            self.effects.shield -= 1;
            if self.effects.shield == 0 {
                self.you.armor -= SHIELD_ARMOR;
            }
        }
        if self.effects.recharge > 0 {
            self.effects.recharge -= 1;
            self.you.mana += 101;
        }
        if self.effects.poison > 0 {
            self.effects.poison -= 1;
            self.boss.hp = self.boss.hp.saturating_sub(3);
        }
    }

    /// Note: the intention is to filter losing states out of the API.
    /// That is, never return a losing state to the user.
    ///
    /// That way the game-tree-search implementation only has to worry about
    /// traversal and finding a winning state.
    fn is_losing(&self) -> bool {
        self.you.hp == 0
    }

    const SPELLS: [fn(&mut Self) -> ControlFlow<()>; 5] = [
        Self::missile,
        Self::drain,
        Self::shield,
        Self::poison,
        Self::recharge,
    ];

    fn missile(&mut self) -> ControlFlow<()> {
        self.you.mana = match self.you.mana.checked_sub(53) {
            Some(m) => m,
            None => return ControlFlow::Break(()),
        };
        self.boss.hp = self.boss.hp.saturating_sub(4);
        ControlFlow::Continue(())
    }

    fn drain(&mut self) -> ControlFlow<()> {
        self.you.mana = match self.you.mana.checked_sub(73) {
            Some(m) => m,
            None => return ControlFlow::Break(()),
        };
        self.boss.hp -= 2;
        self.you.hp += 2; // assuming over-healing is allowed
        ControlFlow::Continue(())
    }

    fn shield(&mut self) -> ControlFlow<()> {
        self.you.mana = match self.you.mana.checked_sub(113) {
            Some(m) => m,
            None => return ControlFlow::Break(()),
        };
        if self.effects.shield > 0 {
            return ControlFlow::Break(());
        }
        self.effects.shield = 6;
        self.you.armor += SHIELD_ARMOR;
        ControlFlow::Continue(())
    }

    fn poison(&mut self) -> ControlFlow<()> {
        self.you.mana = match self.you.mana.checked_sub(173) {
            Some(m) => m,
            None => return ControlFlow::Break(()),
        };
        if self.effects.poison > 0 {
            return ControlFlow::Break(());
        }
        self.effects.poison = 6;
        ControlFlow::Continue(())
    }

    fn recharge(&mut self) -> ControlFlow<()> {
        self.you.mana = match self.you.mana.checked_sub(229) {
            Some(m) => m,
            None => return ControlFlow::Break(()),
        };
        if self.effects.recharge > 0 {
            return ControlFlow::Break(());
        }
        self.effects.recharge = 5;
        ControlFlow::Continue(())
    }
}

const SHIELD_ARMOR: u32 = 7;

pub struct Edge {
    pub state: State,
    pub cost: u32,
}
