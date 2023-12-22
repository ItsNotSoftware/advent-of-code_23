#![allow(unused_variables)]
#![allow(dead_code)]

type Map = Vec<Vec<char>>;

#[derive(Debug)]
struct Beam {
    l: usize,
    c: usize,
    vl: i32,
    vc: i32,
}

impl Beam {
    fn new(l: usize, c: usize, vl: i32, vc: i32) -> Beam {
        Beam { l, c, vl, vc }
    }
}

fn print_map(map: &Map, b: &Beam, energised: &mut Vec<Vec<bool>>) {
    let x = (b.vl, b.vc);
    let marker;
    match x {
        (0, 1) => marker = '>',
        (0, -1) => marker = '<',
        (1, 0) => marker = 'v',
        (-1, 0) => marker = '^',
        _ => marker = 'X',
    }

    println!();
    for line in 0..map.len() {
        for c in 0..map[0].len() {
            if line == b.l && c == b.c {
                print!("{}", marker);
            } else if energised[line][c] && map[line][c] == '.' {
                print!("#");
            } else {
                print!("{}", map[line][c]);
            }
        }
        println!();
    }
    println!();
}

fn move_beam(beams: &mut Vec<Beam>, energised: &mut Vec<Vec<bool>>, tile: &mut char) {
    let vl_prev = beams[0].vl;
    let vc_prev = beams[0].vc;
    energised[beams[0].l][beams[0].c] = true;

    match tile {
        '/' => {
            beams[0].vl = -vc_prev;
            beams[0].vc = -vl_prev;
        }
        '\\' => {
            beams[0].vl = vc_prev;
            beams[0].vc = vl_prev;
        }
        '|' => {
            if beams[0].vc.abs() == 1 {
                beams.push(Beam::new(beams[0].l, beams[0].c, -1, 0));
                beams[0].vl = 1;
                beams[0].vc = 0;

                *tile = '!'
            }
        }
        '-' => {
            if beams[0].vl.abs() == 1 {
                beams.push(Beam::new(beams[0].l, beams[0].c, 0, -1));
                beams[0].vl = 0;
                beams[0].vc = 1;

                *tile = '_'
            }
        }
        '!' => {
            if beams[0].vc.abs() == 1 {
                beams.remove(0);
                return;
            }
        }
        '_' => {
            if beams[0].vl.abs() == 1 {
                beams.remove(0);
                return;
            }
        }
        '.' => {}
        _ => panic!("Invalid tile"),
    }
    let new_l = (beams[0].l as i32) + beams[0].vl;
    let new_c = (beams[0].c as i32) + beams[0].vc;

    if new_l < 0
        || new_c < 0
        || (new_l as usize) >= energised.len()
        || (new_c as usize) >= energised[0].len()
    {
        beams.remove(0);
    } else {
        beams[0].l = new_l as usize;
        beams[0].c = new_c as usize;
    }
}

fn part1(filename: &str) -> usize {
    let mut map: Map = utils::read_chars(filename);
    let mut beams = vec![Beam::new(0, 0, 0, 1)];
    let mut energised = vec![vec![false; map[0].len()]; map.len()];

    while beams.len() > 0 {
        let l = beams[0].l;
        let c = beams[0].c;
        move_beam(&mut beams, &mut energised, &mut map[l][c]);
    }

    energised
        .iter()
        .flat_map(|line| line.iter())
        .filter(|&&val| val)
        .count()
}

fn part2(filename: &str) -> usize {
    todo!();
}

fn main() {
    println!("Part1: {}", part1("inputs/day16.txt"));
    println!("Part2: {}", part2("inputs/day16.txt"));
}

// *====================== Tests ======================* //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let r = part1("examples/day16.txt");
        assert_eq!(r, 46);
    }

    fn test_part2() {
        todo!();
    }
}
