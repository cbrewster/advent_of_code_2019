use std::collections::HashMap;

struct Universe<'a> {
    planets: HashMap<&'a str, &'a str>,
}

fn main() {
    let input = include_str!("../input.txt");
    let planets = parse_input(input);
    let mut universe = Universe::new();

    for (orbiting, orbiter) in planets {
        universe.add_planet(orbiting, orbiter);
    }

    let result = universe.checksum();
    println!("Part 1: {}", result);

    let result = universe.santa_jumps();
    println!("Part 2: {}", result);
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let mut planets = line.split(")");
            (planets.next().unwrap(), planets.next().unwrap())
        })
        .collect()
}

impl<'a> Universe<'a> {
    fn new() -> Universe<'a> {
        Universe {
            planets: HashMap::new(),
        }
    }

    fn add_planet(&mut self, orbiting: &'a str, orbiter: &'a str) {
        self.planets.insert(orbiter, orbiting);
    }

    fn checksum(&self) -> usize {
        self.planets.keys().map(|p| self.orbit_count(p)).sum()
    }

    fn orbit_count(&self, planet: &'a str) -> usize {
        let mut orbits = 0;
        let mut current_planet = planet;
        while current_planet != "COM" {
            orbits += 1;
            current_planet = self.planets.get(current_planet).unwrap();
        }
        orbits
    }

    fn orbit_list(&self, planet: &'a str) -> Vec<&'a str> {
        let mut orbits = vec![];
        let mut current_planet = planet;
        while current_planet != "COM" {
            current_planet = self.planets.get(current_planet).unwrap();
            orbits.push(current_planet);
        }
        orbits
    }

    fn santa_jumps(&self) -> usize {
        let mut you_list = self.orbit_list("YOU");
        let mut santa_list = self.orbit_list("SAN");

        while you_list.last().unwrap() == santa_list.last().unwrap() {
            you_list.pop();
            santa_list.pop();
        }

        you_list.len() + santa_list.len()
    }
}
