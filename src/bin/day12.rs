#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct Record {
    springs: Vec<char>,
    sizes: Vec<u8>,
}

impl Record {
    fn new(line: &str) -> Record {
        let mut line = line.split_whitespace();
        let mut springs: Vec<char> = Vec::new();
        let mut sizes: Vec<u8> = Vec::new();

        if let Some(word) = line.next() {
            word.chars().for_each(|c| springs.push(c));
        }
        if let Some(word) = line.next() {
            word.split(',')
                .filter_map(|num_str| num_str.parse::<u8>().ok())
                .for_each(|n| sizes.push(n));
        }

        return Record { springs, sizes };
    }

    fn is_valid(&self, springs: Vec<char>) -> bool {
        let broken: String = springs.into_iter().collect();
        let broken: Vec<String> = broken
            .split('.')
            .map(String::from)
            .filter(|s| s != "")
            .collect();

        if broken.len() != self.sizes.len() {
            return false;
        }

        for (i, size) in self.sizes.iter().enumerate() {
            if *size as usize != broken[i].len() {
                return false;
            }
        }

        return true;
    }

    fn count_arrangements(&self, i: usize, springs: &mut Vec<char>) -> u32 {
        if i == springs.len() {
            if self.is_valid(springs.to_vec()) {
                return 1;
            } else {
                return 0;
            }
        }

        if springs[i] == '?' {
            let mut s2 = springs.clone();
            springs[i] = '#';
            s2[i] = '.';

            return self.count_arrangements(i + 1, springs)
                + self.count_arrangements(i + 1, &mut s2);
        }

        return self.count_arrangements(i + 1, springs);
    }
}

fn parse_input(filename: &str) -> Vec<Record> {
    let input = utils::read_lines(filename);
    let mut record_list: Vec<Record> = Vec::new();

    input
        .iter()
        .for_each(|line| record_list.push(Record::new(line)));

    return record_list;
}

fn part1(filename: &str) -> u32 {
    let record_list = parse_input(filename);

    return record_list
        .iter()
        .map(|r| {
            let mut springs = r.springs.clone();
            r.count_arrangements(0, &mut springs)
        })
        .sum();
}

fn part2(filename: &str) -> u64 {
    todo!();
}

fn main() {
    println!("Part1: {}", part1("inputs/day12.txt"));
    println!("Part2: {}", part2("inputs/day12.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let r = Record::new("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(true, r.is_valid(".#.###.#.######".chars().collect()));
        assert_eq!(false, r.is_valid("###############".chars().collect()));
        assert_eq!(false, r.is_valid("###############".chars().collect()));
        assert_eq!(false, r.is_valid("...............".chars().collect()));

        let r = Record::new("????.######..#####. 1,6,5");
        assert_eq!(true, r.is_valid("#....######..#####.".chars().collect()));
        assert_eq!(true, r.is_valid(".#...######..#####.".chars().collect()));
        assert_eq!(false, r.is_valid(".#.#.######..#####.".chars().collect()));
        assert_eq!(false, r.is_valid(".###.######..#####.".chars().collect()));
    }

    #[test]
    fn test_count_arrangements() {
        let r = Record::new("???.### 1,1,3");
        assert_eq!(1, r.count_arrangements(0, &mut r.springs.clone()));

        let r = Record::new(".??..??...?##. 1,1,3");
        assert_eq!(4, r.count_arrangements(0, &mut r.springs.clone()));

        let r = Record::new("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(1, r.count_arrangements(0, &mut r.springs.clone()));

        let r = Record::new("????.#...#... 4,1,1");
        assert_eq!(1, r.count_arrangements(0, &mut r.springs.clone()));

        let r = Record::new("????.######..#####. 1,6,5");
        assert_eq!(4, r.count_arrangements(0, &mut r.springs.clone()));

        let r = Record::new("?###???????? 3,2,1");
        assert_eq!(10, r.count_arrangements(0, &mut r.springs.clone()));
    }

    #[test]
    fn test_part1() {
        let r = part1("examples/day12.txt");
        assert_eq!(21, r);
    }
}
