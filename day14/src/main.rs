use std::collections::HashMap;

type Reactions = HashMap<&'static str, Reaction>;

#[derive(Debug)]
struct Reaction {
    count: usize,
    reactants: Vec<(usize, &'static str)>,
}

fn main() {
    let input = include_str!("../input.txt");
    let reactions = parse_input(input);

    let mut bank = HashMap::new();
    let result1 = compute_ore("FUEL", 1, &reactions, &mut bank);
    println!("Part 1: {}", result1);

    let mut bank = HashMap::new();
    let result2 = part2(&reactions, &mut bank);
    println!("Part 2: {}", result2);
}

fn parse_input(input: &'static str) -> Reactions {
    let mut reactions = HashMap::new();
    for line in input.trim().lines() {
        let mut data = line.split("=>");
        let reactants = data
            .next()
            .unwrap()
            .split(",")
            .map(|chemical| parse_chemical(chemical.trim()))
            .collect();
        let resultant = parse_chemical(data.next().unwrap().trim());
        let reaction = Reaction {
            count: resultant.0,
            reactants,
        };
        reactions.insert(resultant.1, reaction);
    }
    reactions
}

fn parse_chemical(input: &'static str) -> (usize, &'static str) {
    let mut data = input.trim().split_ascii_whitespace();
    let count = data.next().unwrap().parse().unwrap();
    let chemical = data.next().unwrap();
    (count, chemical)
}

fn compute_ore(
    chemical: &'static str,
    mut count: usize,
    reactions: &Reactions,
    bank: &mut HashMap<&'static str, usize>,
) -> usize {
    if chemical == "ORE" {
        return count;
    }

    let mut used_all = false;
    if let Some(banked_amount) = bank.get_mut(chemical) {
        if *banked_amount > count {
            *banked_amount -= count;
            return 0;
        } else {
            count -= *banked_amount;
            used_all = true;
        }
    }
    if used_all {
        bank.remove(chemical);
    }

    let reaction = reactions.get(chemical).unwrap();

    let reaction_count = (count as f32 / reaction.count as f32).ceil() as usize;

    let mut sum = 0;

    for reactant in &reaction.reactants {
        if count > 0 {
            sum += compute_ore(reactant.1, reactant.0 * reaction_count, reactions, bank);
        }
    }

    let remaining = (reaction_count * reaction.count) as isize - count as isize;
    if remaining > 0 {
        bank.entry(chemical)
            .and_modify(|count| *count += remaining as usize)
            .or_insert(remaining as usize);
    }

    sum
}

fn part2(reactions: &Reactions, bank: &mut HashMap<&'static str, usize>) -> u128 {
    let max = 1000000000000u128;
    let min = 0u128;
    binary_search(min, max, reactions, bank)
}

fn binary_search(
    lower: u128,
    upper: u128,
    reactions: &Reactions,
    bank: &mut HashMap<&'static str, usize>,
) -> u128 {
    if lower == upper {
        return lower;
    }
    let midpoint = (lower + upper + 1) / 2;
    if compute_ore("FUEL", midpoint as usize, reactions, bank) as u128 > 1000000000000u128 {
        binary_search(lower, midpoint - 1, reactions, bank)
    } else {
        binary_search(midpoint, upper, reactions, bank)
    }
}
