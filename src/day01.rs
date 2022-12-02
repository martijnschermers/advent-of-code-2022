pub fn run() {
    let meals = include_str!("./inputs/day01.txt");
    let mut calories = meals.split("\n");

    let mut top = Vec::new();

    let mut current = calories.next();
    let mut max = 0;
    let mut sum = 0;

    while current.is_some() {
        if current.unwrap().len() > 0 {
            sum += current.unwrap().parse::<i32>().unwrap();

            if sum > max {
                max = sum;
            }

            top.push(sum);
            current = calories.next();
        } else {
            sum = 0;
            current = calories.next();
        }
    }

    top.sort();

    println!("Part 1: {}", top.last().unwrap());
    println!(
        "Part 2: {}",
        top.last().unwrap() + top[top.len() - 2] + top[top.len() - 3]
    );
}
