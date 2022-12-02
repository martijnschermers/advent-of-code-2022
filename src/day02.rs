pub fn run() {
    #[derive(Default)]
    struct Play {
        char: char,
        points: i32,
        lose: char,
        win: char,
    }

    impl Play {
        pub fn new(char: char, points: i32, lose: char, win: char) -> Play {
            Play {
                char,
                points,
                lose,
                win,
            }
        }

        pub fn battle(&self, other: &Play) -> i32 {
            if self.char == other.win {
                return other.points + 6;
            }

            if self.char == other.lose {
                return other.points;
            }

            if self.char == other.char {
                return other.points + 3;
            }

            return 0;
        }
    }

    let mut total_score = 0;

    include_str!("./inputs/day02.txt")
        .split("\n")
        .for_each(|f| {
            let mut play = f.split(" ");
            let first = play.next().unwrap().parse::<char>().unwrap();
            let counter = play.next().unwrap().parse::<char>().unwrap();

            let rock = Play::new('A', 1, 'B', 'C');
            let paper = Play::new('B', 2, 'C', 'A');
            let scissors = Play::new('C', 3, 'A', 'B');

            let mut first_play = &Play::default();
            let mut counter_play = &Play::default();

            match first {
                'A' => first_play = &rock,
                'B' => first_play = &paper,
                'C' => first_play = &scissors,
                _ => first_play = first_play,
            }

            // Part 1
            // match counter {
            //     'X' => counter_play = &rock,
            //     'Y' => counter_play = &paper,
            //     'Z' => counter_play = &scissors,
            //     _ => counter_play = counter_play
            // }

            // Part 2
            match counter {
                'X' => {
                    if first_play.char == rock.char {
                        counter_play = &scissors;
                    }

                    if first_play.char == paper.char {
                        counter_play = &rock;
                    }

                    if first_play.char == scissors.char {
                        counter_play = &paper;
                    }
                }
                'Y' => counter_play = &first_play,
                'Z' => {
                    if first_play.char == rock.char {
                        counter_play = &paper;
                    }

                    if first_play.char == paper.char {
                        counter_play = &scissors;
                    }

                    if first_play.char == scissors.char {
                        counter_play = &rock;
                    }
                }
                _ => counter_play = counter_play,
            }

            total_score += first_play.battle(counter_play);
        });

    println!("Part 1: {}", total_score);
    println!("Part 2: {}", total_score);
}
