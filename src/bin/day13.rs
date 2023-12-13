#![allow(unused_variables)]
#![allow(dead_code)]

type Pattern = Vec<Vec<char>>;

#[derive(Eq, PartialEq, Debug)]
enum ReflectionAxis {
    Horizontal(usize),
    Vertical(usize),
}

fn is_reflection_axis(pattern: &Pattern, axis: &ReflectionAxis, i: usize, j: usize) -> bool {
    match axis {
        ReflectionAxis::Horizontal(_) => {
            for c in 0..pattern[0].len() {
                if pattern[i][c] != pattern[j][c] {
                    return false;
                }
            }

            if i == 0 || j == pattern.len() - 1 {
                return true;
            }
            return is_reflection_axis(pattern, axis, i - 1, j + 1);
        }
        ReflectionAxis::Vertical(_) => {
            for l in 0..pattern.len() {
                if pattern[l][i] != pattern[l][j] {
                    return false;
                }
            }

            if i == 0 || j == pattern[0].len() - 1 {
                return true;
            }
            return is_reflection_axis(pattern, axis, i - 1, j + 1);
        }
    }
}

fn find_refflection(pattern: &Pattern) -> ReflectionAxis {
    for i in 1..pattern.len() {
        if is_reflection_axis(pattern, &ReflectionAxis::Horizontal(0), i - 1, i) {
            return ReflectionAxis::Horizontal(i);
        }
    }

    for i in 1..pattern[0].len() {
        if is_reflection_axis(pattern, &ReflectionAxis::Vertical(0), i - 1, i) {
            return ReflectionAxis::Vertical(i);
        }
    }

    utils::print_mat(&pattern);
    panic!("No refflection axis found!");
}

fn part1(filename: &str) -> usize {
    let patterns = utils::read_matrices(filename);
    let mut sum = 0;

    for p in patterns {
        match find_refflection(&p) {
            ReflectionAxis::Vertical(x) => sum += x,
            ReflectionAxis::Horizontal(x) => sum += x * 100,
        }
    }
    return sum;
}

fn part2(filename: &str) -> u64 {
    todo!();
}

fn main() {
    println!("Part1: {}", part1("inputs/day13.txt"));
    println!("Part2: {}", part2("inputs/day13.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_reflection_axis() {
        let patterns = utils::read_matrices("examples/day13.txt");
        let r = find_refflection(&patterns[0]);
        assert_eq!(r, ReflectionAxis::Vertical(5));

        let patterns = utils::read_matrices("examples/day13.txt");
        let r = find_refflection(&patterns[1]);
        assert_eq!(r, ReflectionAxis::Horizontal(4));
    }

    #[test]
    fn test_part1() {
        let result = part1("examples/day13.txt");
        assert_eq!(result, 405);
    }
}
