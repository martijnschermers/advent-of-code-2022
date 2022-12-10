pub fn run() {
    let input = include_str!("./inputs/day04.txt");
    part_one(input);
    part_two(input);
}

pub fn part_one(input: &str) -> u32 {
    let mut pairs = 0;
    let lines = input.lines().map(|f| f.split(","));
    lines.for_each(|mut l| {
        let mut first = l.next().unwrap().split("-");
        let mut last = l.next().unwrap().split("-");

        let first_first = first.next().unwrap().parse::<i32>().unwrap();
        let second_first = first.next().unwrap().parse::<i32>().unwrap();

        let first_last = last.next().unwrap().parse::<i32>().unwrap();
        let second_last = last.next().unwrap().parse::<i32>().unwrap();

        if (first_first >= first_last && second_first <= second_last)
            || (first_last >= first_first && second_last <= second_first)
        {
            pairs = pairs + 1;
        }
    });

    println!("Part 1: {}", pairs);

    pairs
}

pub fn part_two(input: &str) -> i32 {
    let mut overlaps = 0;
    let lines = input.lines().map(|f| f.split(","));
    lines.for_each(|mut l| {
        let mut first = l.next().unwrap().split("-");
        let mut last = l.next().unwrap().split("-");

        let mut first_numbers = Vec::new();
        let first_first = first.next().unwrap().parse::<i32>().unwrap();
        let second_first = first.next().unwrap().parse::<i32>().unwrap();
        for n in first_first..=second_first {
            first_numbers.push(n);
        }

        let mut second_numbers = Vec::new();
        let first_last = last.next().unwrap().parse::<i32>().unwrap();
        let second_last = last.next().unwrap().parse::<i32>().unwrap();
        for n in first_last..=second_last {
            second_numbers.push(n);
        }

        for n in &first_numbers {
            if second_numbers.contains(n) {
                overlaps = overlaps + 1;
                break;
            }
        }
    });

    println!("Part 2: {}", overlaps);

    overlaps
}

#[cfg(test)]
mod tests {
    use crate::day04::*;

    const INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn part_one_test() {
        let result = part_one(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_two_test() {
        let result = part_two(INPUT);
        assert_eq!(result, 4);
    }
}
