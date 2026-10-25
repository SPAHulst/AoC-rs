#[derive(Clone, Copy, Debug)]
struct CombatState {
    wizard_hp: i16,
    mana: u16,
    boss_hp: i16,
    shield_timer: u8,
    poison_timer: u8,
    recharge_timer: u8,
    mana_spent: u16,
}

#[derive(Clone, Copy)]
enum Spell {
    MagicMissle,
    Drain,
    Shield,
    Poison,
    Recharge,
}

const ALL_SPELLS: [Spell; 5] = [
    Spell::MagicMissle,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

impl Spell {
    fn mana_cost(self) -> u16 {
        match self {
            Spell::MagicMissle => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn can_cast(self, state: &CombatState) -> bool {
        if state.mana < self.mana_cost() {
            return false;
        }

        match self {
            Spell::Shield => state.shield_timer == 0,
            Spell::Poison => state.poison_timer == 0,
            Spell::Recharge => state.recharge_timer == 0,
            Spell::MagicMissle | Spell::Drain => true,
        }
    }
}

fn apply_effects(state: &mut CombatState) {
    if state.shield_timer != 0 {
        state.shield_timer -= 1;
    }
    if state.poison_timer != 0 {
        state.poison_timer -= 1;
        state.boss_hp -= 3;
    }
    if state.recharge_timer != 0 {
        state.recharge_timer -= 1;
        state.mana += 101;
    }
}

fn cast_spell(mut state: CombatState, spell: Spell) -> CombatState {
    let mana_cost = spell.mana_cost();

    state.mana -= mana_cost;
    state.mana_spent += mana_cost;

    match spell {
        Spell::MagicMissle => state.boss_hp -= 4,
        Spell::Drain => {
            state.boss_hp -= 2;
            state.wizard_hp += 2;
        }
        Spell::Shield => state.shield_timer = 6,
        Spell::Poison => state.poison_timer = 6,
        Spell::Recharge => state.recharge_timer = 5,
    }

    state
}

fn find_min_mana(mut state: CombatState, wizards_turn: bool, hard_mode: bool, min_mana: &mut u16) {
    if state.mana_spent >= *min_mana {
        return;
    }

    if wizards_turn && hard_mode {
        state.wizard_hp -= 1;
        if state.wizard_hp <= 0 {
            return;
        }
    }

    apply_effects(&mut state);

    if state.boss_hp <= 0 {
        *min_mana = (*min_mana).min(state.mana_spent);
        return;
    }
    if state.wizard_hp <= 0 {
        return;
    }

    if wizards_turn {
        for spell in ALL_SPELLS {
            if spell.can_cast(&state) {
                let next_state = cast_spell(state, spell);
                find_min_mana(next_state, false, hard_mode, min_mana);
            }
        }
    } else {
        let boss_damage = match state.shield_timer {
            0 => 10,
            _ => 3,
        };
        state.wizard_hp -= boss_damage;
        if state.wizard_hp <= 0 {
            return;
        }
        find_min_mana(state, true, hard_mode, min_mana);
    }
}

pub fn one_star() {
    let start = CombatState {
        wizard_hp: 50,
        mana: 500,
        boss_hp: 71,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        mana_spent: 0,
    };

    let mut min_mana = u16::MAX;
    find_min_mana(start, true, false, &mut min_mana);
    println!("Minimum mana spend to win the fight: {min_mana}");
}

pub fn two_star() {
    let start = CombatState {
        wizard_hp: 50,
        mana: 500,
        boss_hp: 71,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        mana_spent: 0,
    };

    let mut min_mana = u16::MAX;
    find_min_mana(start, true, true, &mut min_mana);
    println!("Minimum mana spend to win the fight: {min_mana}");
}
