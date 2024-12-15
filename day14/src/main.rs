use std::{
    fs,
    io::{self, Write},
    process::Command,
    thread::sleep,
    time::Duration,
};

const TOTAL_SECS: isize = 100;
const BOUNDARY_X: isize = 101;
const BOUNDARY_Y: isize = 103;

fn main() {
    let input = fs::read_to_string("inputs/day14.txt").unwrap();

    let mut robots = Vec::new();
    for line in input.lines() {
        robots.push(Robot::from(line));
    }

    part1(&mut robots.clone());
    part2(&mut robots);
}

#[derive(Clone, Debug, PartialEq)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let mut pos = parts.next().unwrap().split(',');
        let mut vel = parts.next().unwrap().split(',');

        let pos_x: isize = pos.next().unwrap().get(2..).unwrap().parse().unwrap();
        let pos_y: isize = pos.next().unwrap().parse().unwrap();

        let vel_x: isize = vel.next().unwrap().get(2..).unwrap().parse().unwrap();
        let vel_y: isize = vel.next().unwrap().parse().unwrap();

        Self {
            position: (pos_x, pos_y),
            velocity: (vel_x, vel_y),
        }
    }
}

impl Robot {
    fn update(&mut self) {
        self.position = (
            (self.position.0 + self.velocity.0 + BOUNDARY_X) % BOUNDARY_X,
            (self.position.1 + self.velocity.1 + BOUNDARY_Y) % BOUNDARY_Y,
        );
    }
}

fn part1(robots: &mut Vec<Robot>) {
    println!("Day 14 - Part 1");

    let (mut q1_count, mut q2_count, mut q3_count, mut q4_count) = (0, 0, 0, 0);

    for sec in 0..TOTAL_SECS {
        for robot in &mut *robots {
            robot.update();

            if sec == TOTAL_SECS - 1 {
                match robot.position {
                    (x, y) if x < BOUNDARY_X / 2 && y < BOUNDARY_Y / 2 => {
                        q1_count += 1;
                    }
                    (x, y) if x < BOUNDARY_X / 2 && y > BOUNDARY_Y / 2 => {
                        q2_count += 1;
                    }
                    (x, y) if x > BOUNDARY_X / 2 && y < BOUNDARY_Y / 2 => {
                        q3_count += 1;
                    }
                    (x, y) if x > BOUNDARY_X / 2 && y > BOUNDARY_Y / 2 => {
                        q4_count += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    println!(
        "Safety factor: {}",
        q1_count * q2_count * q3_count * q4_count
    );
}

fn part2(robots: &mut Vec<Robot>) {
    println!("Day 14 - Part 2");

    let mut output_file = fs::File::create("day14/output.txt").unwrap();

    let mut grid = vec![vec![vec![]; BOUNDARY_Y as usize]; BOUNDARY_X as usize];
    for robot in robots {
        grid[robot.position.0 as usize][robot.position.1 as usize].push(robot.clone());
    }

    let mut sec = 0;
    loop {
        if (sec - 11) % BOUNDARY_X == 0 || (sec - 65) % BOUNDARY_Y == 0 {
            writeln!(output_file, "Sec {}", sec).unwrap();

            for i in 0..BOUNDARY_X {
                for j in 0..BOUNDARY_Y {
                    let displayable_char = match grid[i as usize][j as usize].len() {
                        0 => '.',
                        _ => '#',
                    };

                    write!(output_file, "{}", displayable_char).unwrap();
                }

                writeln!(output_file).unwrap();
            }
        }

        let mut new_grid = vec![vec![vec![]; BOUNDARY_Y as usize]; BOUNDARY_X as usize];

        for row in &mut grid {
            for cell in row {
                for robot in cell {
                    robot.update();
                    new_grid[robot.position.0 as usize][robot.position.1 as usize]
                        .push(robot.clone());
                }
            }
        }

        grid = new_grid;
        sec += 1;
        if sec > 10000 {
            break;
        }
    }

    output_file.flush().unwrap();
}
