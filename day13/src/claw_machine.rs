use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

const A_COST: isize = 3;
const B_COST: isize = 1;

#[derive(Debug)]
pub struct ClawMachine {
    a_step: (usize, usize),
    b_step: (usize, usize),
    pub prize: (usize, usize),
}

impl From<&str> for ClawMachine {
    fn from(s: &str) -> Self {
        let mut m = Self {
            a_step: (0, 0),
            b_step: (0, 0),
            prize: (0, 0),
        };

        let mut lines = s.lines();

        let mut a_spec = lines.next().unwrap().split_whitespace().skip(2).take(2);
        let mut a_step_x = a_spec.next().unwrap().get(2..).unwrap().to_owned();
        a_step_x.pop();
        m.a_step.0 = a_step_x.parse().unwrap();
        m.a_step.1 = a_spec.next().unwrap().get(2..).unwrap().parse().unwrap();

        let mut b_spec = lines.next().unwrap().split_whitespace().skip(2).take(2);
        let mut b_step_x = b_spec.next().unwrap().get(2..).unwrap().to_owned();
        b_step_x.pop();
        m.b_step.0 = b_step_x.parse().unwrap();
        m.b_step.1 = b_spec.next().unwrap().get(2..).unwrap().parse().unwrap();

        let mut prize_spec = lines.next().unwrap().split_whitespace().skip(1).take(2);
        let mut prize_x = prize_spec.next().unwrap().get(2..).unwrap().to_owned();
        prize_x.pop();
        m.prize.0 = prize_x.parse().unwrap();
        m.prize.1 = prize_spec
            .next()
            .unwrap()
            .get(2..)
            .unwrap()
            .parse()
            .unwrap();

        m
    }
}

#[derive(Debug)]
struct MachinePoint {
    x: usize,
    y: usize,
    cost: isize,
    a_presses: isize,
    b_presses: isize,
}

impl MachinePoint {
    fn zero() -> Self {
        Self {
            x: 0,
            y: 0,
            cost: 0,
            a_presses: 0,
            b_presses: 0,
        }
    }
}

impl PartialEq for MachinePoint {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for MachinePoint {}

impl PartialOrd for MachinePoint {
    fn lt(&self, other: &Self) -> bool {
        self.cost.lt(&other.cost)
    }

    fn gt(&self, other: &Self) -> bool {
        self.cost.gt(&other.cost)
    }

    fn le(&self, other: &Self) -> bool {
        self.cost.le(&other.cost)
    }

    fn ge(&self, other: &Self) -> bool {
        self.cost.ge(&other.cost)
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for MachinePoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl ClawMachine {
    pub fn find_minimum_price_to_prize(&self) -> Option<isize> {
        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(Reverse(MachinePoint::zero()));

        let mut visited_points = Vec::new();

        let mut distances = HashMap::new();
        distances.insert((0, 0), 0);

        while let Some(point) = priority_queue.pop() {
            let point = point.0;

            let button_a_step = (point.x + self.a_step.0, point.y + self.a_step.1);
            if button_a_step.0 <= self.prize.0 && button_a_step.1 <= self.prize.1 {
                let cost_through_a = point.cost + A_COST;

                distances
                    .entry(button_a_step)
                    .and_modify(|d| {
                        if cost_through_a < *d {
                            *d = cost_through_a;
                        }
                    })
                    .or_insert(cost_through_a);

                if !visited_points.contains(&(point.x, point.y)) && point.a_presses < 100 {
                    priority_queue.push(Reverse(MachinePoint {
                        x: button_a_step.0,
                        y: button_a_step.1,
                        cost: cost_through_a,
                        a_presses: point.a_presses + 1,
                        b_presses: point.b_presses,
                    }));
                }
            }

            let button_b_step = (point.x + self.b_step.0, point.y + self.b_step.1);
            if button_b_step.0 <= self.prize.0 && button_b_step.1 <= self.prize.1 {
                let cost_through_b = point.cost + B_COST;

                distances
                    .entry(button_b_step)
                    .and_modify(|d| {
                        if cost_through_b < *d {
                            *d = cost_through_b;
                        }
                    })
                    .or_insert(cost_through_b);

                if !visited_points.contains(&(point.x, point.y)) && point.b_presses < 100 {
                    priority_queue.push(Reverse(MachinePoint {
                        x: button_b_step.0,
                        y: button_b_step.1,
                        cost: cost_through_b,
                        a_presses: point.a_presses,
                        b_presses: point.b_presses + 1,
                    }));
                }
            }

            visited_points.push((point.x, point.y));
        }

        distances.get(&self.prize).copied()
    }
}
