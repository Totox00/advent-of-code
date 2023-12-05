use std::str::Lines;

const INPUT: &str = include_str!("input.txt");

pub fn problem1() -> i64 {
    let mut lines = INPUT.lines();
    let mut seeds: Vec<i64> = vec![];

    for num in lines.next().unwrap().as_bytes()[7..].split(|chr| *chr == b' ') {
        seeds.push(
            num.iter()
                .map(|chr| (chr & 0b1111) as i64)
                .reduce(|acc, x| acc * 10 + x)
                .unwrap(),
        );
    }

    let maps = read_maps(lines);

    calc_problem(&seeds, &maps)
}

pub fn problem2() -> i64 {
    let mut min = None;
    let mut lines = INPUT.lines();
    let seed_values: Vec<_> = lines.next().unwrap().as_bytes()[7..]
        .split(|chr| *chr == b' ')
        .collect();
    let maps = read_maps(lines);

    for num in seed_values.chunks_exact(2) {
        let low = num[0]
            .iter()
            .map(|chr| (chr & 0b1111) as i64)
            .reduce(|acc, x| acc * 10 + x)
            .unwrap();
        let len = num[1]
            .iter()
            .map(|chr| (chr & 0b1111) as i64)
            .reduce(|acc, x| acc * 10 + x)
            .unwrap();

        dbg!(low);
        dbg!(len);

        let seeds: Vec<_> = (low..(low + len)).collect();
        let new = calc_problem(&seeds, &maps);

        if min.is_none() || min.unwrap() > new {
            min = Some(new);
        }
    }
    min.unwrap()
}

fn read_maps(lines: Lines<'_>) -> Vec<Vec<(i64, i64, i64)>> {
    let mut maps = vec![];
    for line in lines {
        if line.contains(':') {
            maps.push(vec![]);
        } else if !line.is_empty() {
            let map: Vec<_> = line
                .as_bytes()
                .split(|chr| *chr == b' ')
                .map(|num| {
                    num.iter()
                        .map(|chr| (chr & 0b1111) as i64)
                        .reduce(|acc, x| acc * 10 + x)
                        .unwrap()
                })
                .collect();

            maps.last_mut().unwrap().push((map[0], map[1], map[2]));
        }
    }
    maps
}

fn calc_problem(seeds: &[i64], maps: &[Vec<(i64, i64, i64)>]) -> i64 {
    let mut state: Vec<(i64, bool)> = seeds.iter().map(|seed| (*seed, false)).collect();

    for map in maps {
        for (dest, src, len) in map {
            for (val, has_changed) in &mut state {
                if !*has_changed {
                    if *val >= *src && *val < *src + len {
                        *val += dest - *src;
                        *has_changed = true;
                    }
                }
            }
        }

        clear_state(&mut state);
    }

    *state.iter().map(|(x, _)| x).min().unwrap()
}

fn clear_state(state: &mut Vec<(i64, bool)>) {
    for mut elem in state {
        elem.1 = false;
    }
}
