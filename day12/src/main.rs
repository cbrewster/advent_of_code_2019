use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Planet {
    position: Vector,
    velocity: Vector,
}

impl FromStr for Planet {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // strip leading and trailing '<' '>'
        let mut components = input[1..input.len() - 1]
            .split(",")
            .map(|comp| comp.trim().split("=").nth(1).unwrap());

        let x = components.next().unwrap().parse().unwrap();
        let y = components.next().unwrap().parse().unwrap();
        let z = components.next().unwrap().parse().unwrap();

        Ok(Planet {
            position: Vector { x, y, z },
            velocity: Vector { x: 0, y: 0, z: 0 },
        })
    }
}

impl Planet {
    fn apply_gravity(&mut self, other: Vector) {
        self.apply_gravity_x(other.x);
        self.apply_gravity_y(other.y);
        self.apply_gravity_z(other.z);
    }

    fn apply_gravity_x(&mut self, other: isize) {
        match self.position.x.cmp(&other) {
            Ordering::Greater => self.velocity.x -= 1,
            Ordering::Less => self.velocity.x += 1,
            Ordering::Equal => {}
        }
    }

    fn apply_gravity_y(&mut self, other: isize) {
        match self.position.y.cmp(&other) {
            Ordering::Greater => self.velocity.y -= 1,
            Ordering::Less => self.velocity.y += 1,
            Ordering::Equal => {}
        }
    }

    fn apply_gravity_z(&mut self, other: isize) {
        match self.position.z.cmp(&other) {
            Ordering::Greater => self.velocity.z -= 1,
            Ordering::Less => self.velocity.z += 1,
            Ordering::Equal => {}
        }
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn apply_velocity_x(&mut self) {
        self.position.x += self.velocity.x;
    }

    fn apply_velocity_y(&mut self) {
        self.position.y += self.velocity.y;
    }

    fn apply_velocity_z(&mut self) {
        self.position.z += self.velocity.z;
    }

    fn potential_energy(&self) -> isize {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> isize {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct System {
    planets: Vec<Planet>,
}

impl System {
    fn new(planets: Vec<Planet>) -> System {
        System { planets }
    }

    fn step(&mut self) {
        for i in 0..self.planets.len() {
            for j in 0..self.planets.len() {
                if i == j {
                    continue;
                }
                let other = self.planets[j].position;
                self.planets[i].apply_gravity(other);
            }
        }

        for planet in &mut self.planets {
            planet.apply_velocity();
        }
    }

    fn step_x(&mut self) {
        for i in 0..self.planets.len() {
            for j in 0..self.planets.len() {
                if i == j {
                    continue;
                }
                let other = self.planets[j].position;
                self.planets[i].apply_gravity_x(other.x);
            }
        }

        for planet in &mut self.planets {
            planet.apply_velocity_x();
        }
    }

    fn step_y(&mut self) {
        for i in 0..self.planets.len() {
            for j in 0..self.planets.len() {
                if i == j {
                    continue;
                }
                let other = self.planets[j].position;
                self.planets[i].apply_gravity_y(other.y);
            }
        }

        for planet in &mut self.planets {
            planet.apply_velocity_y();
        }
    }

    fn step_z(&mut self) {
        for i in 0..self.planets.len() {
            for j in 0..self.planets.len() {
                if i == j {
                    continue;
                }
                let other = self.planets[j].position;
                self.planets[i].apply_gravity_z(other.z);
            }
        }

        for planet in &mut self.planets {
            planet.apply_velocity_z();
        }
    }

    fn total_energy(&self) -> isize {
        self.planets
            .iter()
            .map(|planet| planet.potential_energy() * planet.kinetic_energy())
            .sum()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let planets = parse_input(input);
    let system = System::new(planets);

    let total_energy = part1(system.clone());
    println!("Part 1: {}", total_energy);

    let loops = part2(system);
    println!("Part 2: {}", loops);
}

fn parse_input(input: &str) -> Vec<Planet> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part1(mut system: System) -> isize {
    for _ in 0..1000 {
        system.step();
    }
    system.total_energy()
}

fn part2(mut system: System) -> usize {
    // Find x pattern
    let starting_x = system.clone();
    let mut x_loop = 1;
    system.step_x();
    while system != starting_x {
        system.step_x();
        x_loop += 1;
    }

    let starting_y = system.clone();
    let mut y_loop = 1;
    system.step_y();
    while system != starting_y {
        system.step_y();
        y_loop += 1;
    }

    let starting_z = system.clone();
    let mut z_loop = 1;
    system.step_z();
    while system != starting_z {
        system.step_z();
        z_loop += 1;
    }

    lcm(lcm(x_loop, y_loop), z_loop)
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
