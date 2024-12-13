use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/day05.txt")
        .unwrap()
        .trim()
        .to_owned();

    let mut sections = input.split("\n\n");

    let mut rules = Vec::new();
    for rule in sections.next().unwrap().lines() {
        rules.push(rule.to_owned());
    }

    let mut updates = Vec::new();
    for update in sections.next().unwrap().lines() {
        updates.push(update.to_owned());
    }

    let mut rule_map = HashMap::new();

    for rule in rules {
        let mut parts = rule.split('|');
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();

        rule_map
            .entry(first.to_owned())
            .and_modify(|r: &mut Rule| r.disallow_before.push(second.to_owned()))
            .or_insert(Rule {
                disallow_before: vec![second.to_owned()],
                disallow_after: Vec::new(),
            });

        rule_map
            .entry(second.to_owned())
            .and_modify(|r| r.disallow_after.push(first.to_owned()))
            .or_insert(Rule {
                disallow_before: Vec::new(),
                disallow_after: vec![second.to_owned()],
            });
    }

    let unordered_updates = part1(&rule_map, &updates);
    part2(&rule_map, &unordered_updates);
}

struct Rule {
    disallow_before: Vec<String>,
    disallow_after: Vec<String>,
}

fn part1(rule_map: &HashMap<String, Rule>, updates: &Vec<String>) -> Vec<String> {
    println!("Day 05 - Part 1");

    let mut valid_middle_numbers_sum = 0;
    let mut unordered_updates = Vec::new();
    'update_loop: for update in updates {
        let nums: Vec<&str> = update.split(',').collect();

        for (i, &num) in nums.iter().enumerate() {
            let default_rule = Rule {
                disallow_before: Vec::new(),
                disallow_after: Vec::new(),
            };

            let num_rules = rule_map.get(num).unwrap_or(&default_rule);

            for num_before in &nums[0..i] {
                if num_rules.disallow_before.contains(&num_before.to_string()) {
                    unordered_updates.push(update.clone());
                    continue 'update_loop;
                }
            }
            for num_after in &nums[i + 1..] {
                if num_rules.disallow_after.contains(&num_after.to_string()) {
                    unordered_updates.push(update.clone());
                    continue 'update_loop;
                }
            }
        }

        valid_middle_numbers_sum += nums[nums.len() / 2]
            .parse::<isize>()
            .expect("element in instruction should be a valid number");
    }

    println!("Sum of valid middle numbers: {}", valid_middle_numbers_sum);

    unordered_updates
}

fn part2(rule_map: &HashMap<String, Rule>, updates: &Vec<String>) {
    println!("Day 05 - Part 2");

    let mut corrected_middle_numbers_sum = 0;
    for update in updates {
        let mut nums: Vec<&str> = update.split(',').collect();

        let default_rule = Rule {
            disallow_before: Vec::new(),
            disallow_after: Vec::new(),
        };

        let mut did_swap = true;
        let mut last_elem = nums.len() - 1;
        while did_swap {
            did_swap = false;

            for i in 0..last_elem {
                let num_rules = rule_map.get(nums[i]).unwrap_or(&default_rule);
                let next_num_rules = rule_map.get(nums[i + 1]).unwrap_or(&default_rule);

                if num_rules.disallow_after.contains(&nums[i + 1].to_owned())
                    || next_num_rules.disallow_before.contains(&nums[i].to_owned())
                {
                    nums.swap(i, i + 1);
                    did_swap = true;
                }
            }

            last_elem = last_elem.saturating_sub(1);
        }

        corrected_middle_numbers_sum += nums[nums.len() / 2]
            .parse::<isize>()
            .expect("element in instruction should be a valid number");
    }

    println!(
        "Sum of corrected middle numbers: {}",
        corrected_middle_numbers_sum
    );
}
