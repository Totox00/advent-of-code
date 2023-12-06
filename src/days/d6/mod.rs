const INPUT: &str = include_str!("input.txt");

pub fn problem1() -> i32 {
    let mut prod = 1;
    let mut times = vec![0];
    let mut distances = vec![0];

    let mut lines = INPUT.lines();

    for chr in lines.next().unwrap().as_bytes() {
        if chr.is_ascii_digit() {
            let last = times.last_mut().unwrap();
            *last *= 10;
            *last += (chr & 0b1111) as i32;
        } else {
            if *times.last().unwrap() > 0 {
                times.push(0);
            }
        }
    }

    for chr in lines.next().unwrap().as_bytes() {
        if chr.is_ascii_digit() {
            let last = distances.last_mut().unwrap();
            *last *= 10;
            *last += (chr & 0b1111) as i32;
        } else {
            if *distances.last().unwrap() > 0 {
                distances.push(0);
            }
        }
    }

    if *times.last().unwrap() == 0 {
        times.pop();
    }

    if *distances.last().unwrap() == 0 {
        distances.pop();
    }

    for (time, distance) in times.iter().zip(distances) {
        let mut options = 0;

        for wait in 1..*time {
            if wait * (time - wait) > distance {
                options += 1;
            }
        }

        prod *= options;
    }

    prod
}

pub fn problem2() -> i64 {
    let mut time = 0;
    let mut distance = 0;

    let mut lines = INPUT.lines();

    for chr in lines.next().unwrap().as_bytes() {
        if chr.is_ascii_digit() {
            time *= 10;
            time += (chr & 0b1111) as i64;
        }
    }

    for chr in lines.next().unwrap().as_bytes() {
        if chr.is_ascii_digit() {
            distance *= 10;
            distance += (chr & 0b1111) as i64;
        }
    }

    let mut options = 0;

    for wait in 1..time {
        if wait * (time - wait) > distance {
            options += 1;
        }
    }

    options
}
