pub fn run() {
    let input = include_str!("./inputs/day03.txt");
    part_one(input);
    part_two(input);
}

static CHARS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn part_one(input: &str) -> usize {
    let mut priorites = 0;

    input.lines().for_each(|line| {
        let middle = line.len() / 2;

        let first = &line[0..middle];
        let second = &line[middle..line.len()];

        let mut shared = '-';

        for char in first.chars() {
            if second.contains(char) {
                shared = char;
            }
        }

        priorites += CHARS.iter().position(|c| c == &shared).unwrap() + 1;
    });

    println!("Part 1: {}", priorites);

    priorites
}

pub fn part_two(input: &str) -> usize {
    let mut priorites = 0;

    let mut vec = Vec::new();

    input.lines().for_each(|line| {
        vec.push(line);
    });

    let groups = vec.chunks(3);
    for group in groups {
        let first = group[0];
        let second = group[1];
        let third = group[2];

        let mut shared = '-';
        for char in first.chars() {
            if second.contains(char) && third.contains(char) {
                shared = char;
            }
        }

        priorites += CHARS.iter().position(|c| c == &shared).unwrap() + 1;
    }

    println!("Part 2: {}", priorites);

    priorites
}

#[cfg(test)]
mod tests {
    use crate::day03::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_one_test() {
        let result = part_one(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn part_two_test() {
        let result = part_two(INPUT);
        assert_eq!(result, 70);
    }
}
