use enum_map::{Enum, EnumMap};
use fxhash::FxHashSet;
use rayon::prelude::*;

pub struct Blueprint {
    costs: EnumMap<Resource, Vec<Cost>>,
    max_costs: EnumMap<Resource, i32>,
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
struct Inventory {
    items: EnumMap<Resource, i32>,
    generators: EnumMap<Resource, i32>,
}

#[derive(Debug)]
struct Cost(Resource, i32);

#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug)]
enum Resource {
    Geode,
    Ore,
    Clay,
    Obsidian,
}

impl Blueprint {
    fn affordable_by(&self, inventory: &Inventory) -> EnumMap<Resource, bool> {
        self.costs
            .iter()
            .filter(|cost| inventory.can_afford(cost.1))
            .map(|t| (t.0, true))
            .collect()
    }

    fn purchase(&self, inventory: &mut Inventory, resource: Resource) {
        let costs = &self.costs[resource];
        for cost in costs {
            inventory.items[cost.0] -= cost.1;
        }
        inventory.generators[resource] += 1;
    }

    fn search_recurse(
        &self,
        mut inventory: Inventory,
        minute: i32,
        explored: &mut FxHashSet<(Inventory, i32)>,
        max: &mut i32,
    ) {
        let geode_count = inventory.items[Resource::Geode];
        if minute <= 0 {
            *max = (*max).max(geode_count);
            return;
        }
        let next_new_geodes = minute * inventory.generators[Resource::Geode];
        // check the sum of all previous minutes to calculate an estimate of
        // how many geodes we'd get if we kept generating one robot every minute
        if (minute - 1) * minute / 2 + geode_count + next_new_geodes < *max {
            return;
        }
        if explored.contains(&(inventory.clone(), minute)) {
            return;
        }
        explored.insert((inventory.clone(), minute));
        let before_purchase = inventory.generators;
        let mut had_geode = false;
        let mut skip_branch = false;
        for (resource, _) in self
            .affordable_by(&inventory)
            .into_iter()
            .filter(|(_, b)| *b)
        {
            if had_geode {
                return;
            }
            if resource != Resource::Geode {
                if inventory.generators[resource] >= self.max_costs[resource] {
                    skip_branch = true;
                    continue;
                }
            } else {
                had_geode = true;
                skip_branch = true;
            }
            let mut clone = inventory.clone();
            self.purchase(&mut clone, resource);
            clone.generate(&before_purchase);
            self.search_recurse(clone, minute - 1, explored, max);
        }
        if !skip_branch {
            inventory.generate(&before_purchase);
            self.search_recurse(inventory, minute - 1, explored, max);
        }
    }
}

impl Inventory {
    fn generate(&mut self, generators: &EnumMap<Resource, i32>) {
        for (res, amt) in generators {
            self.items[res] += amt;
        }
    }

    fn can_afford(&self, costs: &[Cost]) -> bool {
        costs.iter().all(|cost| self.items[cost.0] >= cost.1)
    }
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .flat_map(|w| w.parse::<i32>())
                .collect::<Vec<_>>();
            let costs: EnumMap<Resource, Vec<Cost>> = [
                (Resource::Ore, vec![Cost(Resource::Ore, numbers[0])]),
                (Resource::Clay, vec![Cost(Resource::Ore, numbers[1])]),
                (
                    Resource::Obsidian,
                    vec![
                        Cost(Resource::Ore, numbers[2]),
                        Cost(Resource::Clay, numbers[3]),
                    ],
                ),
                (
                    Resource::Geode,
                    vec![
                        Cost(Resource::Ore, numbers[4]),
                        Cost(Resource::Obsidian, numbers[5]),
                    ],
                ),
            ]
            .into_iter()
            .collect();
            let mut max_costs: EnumMap<Resource, i32> = EnumMap::default();
            for cost in costs.values().flatten() {
                max_costs[cost.0] = max_costs[cost.0].max(cost.1);
            }
            Blueprint { max_costs, costs }
        })
        .collect()
}

#[aoc(day19, part1)]
pub fn part1(input: &[Blueprint]) -> i32 {
    input
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let mut inventory = Inventory::default();
            inventory.generators[Resource::Ore] = 1;
            let mut memo = FxHashSet::default();
            let mut max = 0;
            blueprint.search_recurse(inventory, 24, &mut memo, &mut max);
            (i as i32 + 1) * max
        })
        .sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &[Blueprint]) -> i32 {
    input
        .par_iter()
        .take(3)
        .map(|blueprint| {
            let mut inventory = Inventory::default();
            inventory.generators[Resource::Ore] = 1;
            let mut memo =
                FxHashSet::with_capacity_and_hasher(2_900_000, fxhash::FxBuildHasher::default());
            let mut max = 0;
            blueprint.search_recurse(inventory, 32, &mut memo, &mut max);
            max
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;
        assert_eq!(part1(&parse(input)), 33);
    }

    #[test]
    fn part2_example() {
        let input = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;
        assert_eq!(part2(&parse(input)), 56 * 62);
    }
}
