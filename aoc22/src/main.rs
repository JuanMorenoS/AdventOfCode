use std::cmp;

pub trait Problem {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

fn main() {
    let input = include_str!("../input/day1.txt");
    let day1: Box<dyn Problem>;
    day1 = Box::new(Problem1);
    println!("{}", day1.part1(input));
    println!("{}", day1.part2(input));
}

struct Problem1;

impl Problem for Problem1 {
    fn part1(&self, input: &str) -> String {
        let lines = input.split("\n");
        let mut max = 0;
        let mut current = 0;
        for l in lines {
            if !l.is_empty() {
                let num: i32 = l.parse().unwrap();
                current += num;
            } else {
                max = cmp::max(max, current);
                current = 0;
            }
        }

        return String::from(format!("{}", max));
    }

    fn part2(&self, input: &str) -> String {
        let lines = input.split("\n");
        let mut elfs = vec![];
        let mut current = 0;
        for l in lines {
            if !l.is_empty() {
                let num: i32 = l.parse().unwrap();
                current += num;
            } else {
                elfs.push(current);
                current = 0;
            }
        }
        elfs.sort();
        elfs.reverse();

        let mut result = 0;
        for i in 0..=2 {
            result += elfs[i];
        }

        return String::from(format!("{}", result));
    }
}
