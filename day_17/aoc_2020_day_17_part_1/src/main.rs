use std::{cmp::{max, min}, process::Command};

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../test_input.txt");
    let mut dimension = PocketDimension::from(input);

    for _ in 0..6 {
        dimension.display();
        println!("{}", dimension.count_active_cubes());
        let _ = Command::new("pause").status();
        dimension = dimension.next();
    }

    let result = dimension.count_active_cubes();

    println!("{}", result);

    Ok(())
}

#[derive(Clone, Copy, Debug)]
struct Cube {
    active: bool,
}

#[derive(Clone, Copy, Debug)]
struct PocketDimension {
    // 3D array of 20 by 20 by 13
    //
    // 20 is the maximum possible size for the world since the original
    // dimensions are 8 x 8 x 1 and the active cube pattern can only grow of 1
    // unit in each direction (2 units by dimension) by cycle. Over 6 cycles,
    // this amounts to 12 more units by dimensions. (8 + 12 = 20, 1 + 12 = 13)
    cubes: [[[Cube; 13]; 20]; 20],
}

impl PocketDimension {
    fn new() -> PocketDimension {
        PocketDimension {
            cubes: [[[Cube { active: false }; 13]; 20]; 20],
        }
    }

    fn from(input: &str) -> PocketDimension {
        let input_lines = input.lines().collect::<Vec<_>>();
        if input_lines.len() != 8 || input_lines[0].len() != 8 {
            panic!("This input doesn't have the right dimensions.")
        }

        let mut result = PocketDimension::new();

        for i in 0..8 {
            for j in 0..8 {
                result.cubes[6 + i][6 + j][6].active = match input_lines[i].chars().nth(j) {
                    Some('#') => true,
                    Some('.') => false,
                    _ => panic!("Malformed input"),
                }
            }
        }

        result
    }

    fn count_active_neighbours(&self, x: usize, y: usize, z: usize) -> u8 {
        if x >= 20 || y >= 20 || z >= 13 {
            panic!("Can't access this cube");
        }

        let mut active_neighbours = 0;

        for i in max(0, x as isize - 1) as usize..=min(19, x + 1) {
            for j in max(0, y as isize - 1) as usize..=min(19, y + 1) {
                for k in max(0, z as isize - 1) as usize..=min(12, z + 1) {
                    if self.cubes[i][j][k].active {
                        active_neighbours += 1;
                    }
                }
            }
        }

        active_neighbours
    }

    fn next(&self) -> PocketDimension {
        let mut next = PocketDimension::new();

        for x in 0..self.cubes.len() {
            for y in 0..self.cubes[0].len() {
                for z in 0..self.cubes[0][0].len() {
                    next.cubes[x][y][z].active = match (
                        self.cubes[x][y][z].active,
                        self.count_active_neighbours(x, y, z),
                    ) {
                        (true, 2) | (true, 3) | (false, 3) => true,
                        _ => false,
                    }
                }
            }
        }

        next
    }

    fn count_active_cubes(&self) -> u32 {
        let mut count = 0;
        for x in 0..self.cubes.len() {
            for y in 0..self.cubes[0].len() {
                for z in 0..self.cubes[0][0].len() {
                    if self.cubes[x][y][z].active {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn display(&self) {
        for z in 0..self.cubes[0][0].len() {
            println!("\nz = {}", z as i32 - 6);
            for x in 0..self.cubes[0].len() {
                println!("");
                for y in 0..self.cubes.len() {
                    let symbol =  match self.cubes[x][y][z].active {
                        true => '#',
                        false => '.',
                    };
                    print!("{}", symbol);
                }
            }
            println!("");
        }
    }
}
