use rand::prelude::*;
use std::process::Command;

// Shades: █ ▓ ▒ ░
const ALIVE: char = '█';
const DEAD: char = ' ';

fn main() {
    let mut current_gen: Vec<char> = vec![];
    loop {
        // Enforce sensible framerate
        wait();

        // Keep checking terminal size to account for resizing
        let termsize::Size { rows, cols } =
            termsize::get().expect("Error: Failed to read terminal dimensions.");

        // Calculate next generation
        let next_generation = build_gen(rows as i16, cols as i16, &current_gen);

        // Apply and draw generation
        current_gen = next_generation;
        draw_gen(&current_gen);
    }
}

fn draw_gen(world: &Vec<char>) {
    // Convert into one big string. This should autowrap
    let world_string: String = world.iter().collect();
    print!("{esc}c", esc = 27 as char);
    print!("{}", world_string);
}

fn build_gen(rows: i16, cols: i16, current_gen: &Vec<char>) -> Vec<char> {
    let mut next_gen: Vec<char> = vec![];
    for i in 0..(rows * cols) {
        next_gen.push(if current_gen.is_empty() {
            // Random plotting of first gen
            rand_cell()
        } else {
            // Get neighbours
            let neighbours = get_neighbours(i, rows, cols, current_gen);

            // Apply the rules of life for the next gen
            let idx = i as usize;
            if current_gen.len() <= idx {
                rand_cell()
            } else if current_gen[idx] == ALIVE && (neighbours.len() > 3 || neighbours.len() < 2) {
                DEAD
            } else if current_gen[idx] == DEAD && neighbours.len() == 3 {
                ALIVE
            } else {
                current_gen[idx]
            }
        });
    }
    next_gen
}

fn get_neighbours(i: i16, rows: i16, cols: i16, current_gen: &Vec<char>) -> Vec<usize> {
    // Convert cell index to xy
    let cell_2d = (i % cols, i / cols);

    // Check neighbours and add if alive
    let mut neighbours: Vec<usize> = vec![];
    for xi in -1..=1 {
        for yi in -1..=1 {
            if (xi, yi) != (0, 0) {
                // Calculate neighbour position, with wrapping
                let neigh_x = (cell_2d.0 + xi + cols) % cols;
                let neigh_y = (cell_2d.1 + yi + rows) % rows;
                let neigh_idx = (neigh_x + (neigh_y * cols)) as usize;

                // If alive, add to the return
                if neigh_idx < current_gen.len() && current_gen[neigh_idx] == ALIVE {
                    neighbours.push(neigh_idx);
                }
            }
        }
    }

    neighbours
}

fn wait() {
    let mut sleep = Command::new("sleep").arg("0.1").spawn().unwrap();
    let _ = sleep.wait().unwrap();
}

fn rand_cell() -> char {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (1..3).collect();
    nums.shuffle(&mut rng);
    if nums[0] == 2 { ALIVE } else { DEAD }
}
