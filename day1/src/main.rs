fn parse_input(data: &str) -> Vec<usize> {
    data
        .split("\n\n")
        .map(|x| 
            x.split("\n")
                .filter_map(|x| x.parse::<usize>().ok())
                .sum::<usize>()
        )
        .collect()
}

fn main() {
    let data = std::fs::read_to_string("inputs/day1.txt").unwrap();
        
    let mut elves = parse_input(&data);
    elves.sort_by(|a, b| b.cmp(a));

    let top_elf = elves[0];
    eprintln!("Part 1: {:?}", top_elf);

    let top_three_elves:usize = elves.get(0..3).unwrap().iter().sum();
    eprintln!("Part 2: {:?}", top_three_elves);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_parsing() {
        let data = std::fs::read_to_string("example1.txt").unwrap();
        
        let elves = parse_input(&data);

        assert_eq!(elves.len(), 5);
        assert_eq!(elves[0], 6000);
        assert_eq!(elves[1], 4000);
        assert_eq!(elves[2], 11000);
        assert_eq!(elves[3], 24000);
        assert_eq!(elves[4], 10000);
    }
}