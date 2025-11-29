static INPUT: &str = include_str!("input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let (rules, updates) = INPUT.split_once("\n\n").unwrap();
    let rules: Vec<_> = rules
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .collect();

    let mut result = 0;
    'update_loop: for update in updates.lines() {
        for rule in &rules {
            if let Some((_, after)) = update.split_once(rule.1)
                && after.contains(rule.0)
            {
                continue 'update_loop;
            }
        }
        let update: Vec<_> = update.split(',').collect();
        let middle_page = update[update.len() / 2];
        result += middle_page.parse::<u32>().unwrap();
    }
    println!("{result}");
}

fn part2() {
    let (rules, mut updates) = parse_input();

    let mut result = 0;
    for update in updates.iter_mut() {
        let mut corrections_made = 0;
        'correction: loop {
            for rule in &rules {
                if let Some(i) = update.iter().position(|&x| x == rule.1)
                    && let Some(j) = update.iter().position(|&x| x == rule.0)
                    && i < j
                {
                    update.swap(i, j);
                    corrections_made += 1;
                    continue 'correction;
                }
            }
            break;
        }
        if corrections_made > 0 {
            result += update[update.len() / 2];
        }
    }

    println!("{result}");
}

fn parse_input() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let (rules, updates) = INPUT.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            line.split_once('|')
                .map(|(lhs, rhs)| (lhs.parse().unwrap(), rhs.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|l| l.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}
