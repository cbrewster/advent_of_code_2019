use std::io::Write;

#[derive(Debug)]
struct Image {
    layers: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

fn main() {
    let input = include_str!("../input.txt");
    let image = parse_input(input, 25, 6);

    let result = image.checksum();
    println!("Part 1: {}", result);

    println!("Part 2:");
    image.print_image();
}

fn parse_input(input: &str, width: usize, height: usize) -> Image {
    let mut layers = vec![];
    let mut current_layer = vec![];
    let mut counter = 0;

    for c in input.chars() {
        current_layer.push(c.to_digit(10).unwrap() as u8);
        counter += 1;
        if counter >= width * height {
            counter = 0;
            layers.push(current_layer);
            current_layer = vec![];
        }
    }

    Image {
        layers,
        width,
        height,
    }
}

impl Image {
    fn checksum(&self) -> usize {
        let (layer, _) = self
            .layers
            .iter()
            .map(|l| (l, l.iter().filter(|n| **n == 0).count()))
            .min_by_key(|(_, count)| *count)
            .unwrap();

        let ones_count = layer.iter().filter(|n| **n == 1).count();
        let twos_count = layer.iter().filter(|n| **n == 2).count();
        ones_count * twos_count
    }

    fn print_image(&self) {
        let image = self.render_image();
        for index in 0..self.width * self.height {
            if index != 0 && index % self.width * self.height == 0 {
                print!("\n");
            }
            if image[index] == 0 {
                print!(" ");
            } else {
                print!("X");
            }
        }
        print!("\n");
        std::io::stdout().flush().unwrap();
    }

    fn render_image(&self) -> Vec<u8> {
        let mut image = vec![];
        for index in 0..self.width * self.height {
            image.push(self.compute_pixel(index));
        }
        image
    }

    fn compute_pixel(&self, index: usize) -> u8 {
        let mut result = 0;
        for layer in &self.layers {
            result = layer[index];
            // Find first non-transparent pixel
            if result != 2 {
                break;
            }
        }
        result
    }
}
