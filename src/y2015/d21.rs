use std::ops::Add;

struct Boss {
    hp: u16,
    damage: u16,
    armor: u16,
}

#[derive(Clone, Copy)]
struct Item {
    cost: u16,
    damage: u16,
    armor: u16,
}

impl Add for Item {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

fn loadouts_vs_boss(boss: Boss) -> (u16, u16) {
    let weapons = [
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    let armors = [
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ];

    let rings = [
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];

    let mut ring_combos = Vec::with_capacity(22);
    ring_combos.push(Item {
        cost: 0,
        damage: 0,
        armor: 0,
    });

    for (i, &first) in rings.iter().enumerate() {
        ring_combos.push(first);
        for &second in &rings[i + 1..] {
            ring_combos.push(first + second);
        }
    }

    let mut min_gold = u16::MAX;
    let mut max_gold = u16::MIN;

    for weapon in weapons {
        for armor in armors {
            for &third in &ring_combos {
                let loadout = weapon + armor + third;

                let hero_damage = (loadout.damage).saturating_sub(boss.armor).max(1);
                let boss_death = (boss.hp).div_ceil(hero_damage);

                let boss_damage = (boss.damage).saturating_sub(loadout.armor).max(1);
                let hero_death = 100_u16.div_ceil(boss_damage);

                if boss_death <= hero_death {
                    min_gold = min_gold.min(loadout.cost);
                } else {
                    max_gold = max_gold.max(loadout.cost);
                }
            }
        }
    }

    (min_gold, max_gold)
}

pub fn one_star() {
    let boss = Boss {
        hp: 104,
        damage: 8,
        armor: 1,
    };

    let (min_gold, _) = loadouts_vs_boss(boss);

    println!("Minimum gold to defeat boss: {min_gold}");
}

pub fn two_star() {
    let boss = Boss {
        hp: 104,
        damage: 8,
        armor: 1,
    };

    let (_, max_gold) = loadouts_vs_boss(boss);

    println!("Maximum gold to lose to boss: {max_gold}");
}
