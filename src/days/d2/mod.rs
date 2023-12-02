const INPUT: &str = include_str!("input.txt");

const CONTENTS: [&[u8]; 3] = ["red".as_bytes(), "green".as_bytes(), "blue".as_bytes()];

pub fn problem1() -> i32 {
    let mut sum = 0;

    for (line, id) in INPUT.lines().zip(1..) {
        let chrs = line.as_bytes();
        let cubes = chrs.split(|chr| *chr == b':').nth(1).unwrap();

        if cubes
            .split(|chr| *chr == b',' || *chr == b';')
            .all(|reveal| {
                let num =
                    &reveal[1..reveal.iter().skip(1).position(|chr| *chr == b' ').unwrap() + 1];
                let mut val = 0;

                for chr in num {
                    val *= 10;
                    val += chr & 0b1111;
                }

                CONTENTS.iter().zip(12..).all(|(colour, max)| {
                    if reveal.ends_with(colour) {
                        val <= max
                    } else {
                        true
                    }
                })
            })
        {
            sum += id;
        }
    }

    sum
}

pub fn problem2() -> i32 {
    let mut sum = 0;

    for line in INPUT.lines() {
        let chrs = line.as_bytes();
        let cubes = chrs.split(|chr| *chr == b':').nth(1).unwrap();
        let mut mins = [0; 3];

        for reveal in cubes.split(|chr| *chr == b',' || *chr == b';') {
            let num = &reveal[1..reveal.iter().skip(1).position(|chr| *chr == b' ').unwrap() + 1];
            let mut val = 0;

            for chr in num {
                val *= 10;
                val += chr & 0b1111;
            }

            for (colour, i) in CONTENTS.iter().zip(0..) {
                if mins[i] < val && reveal.ends_with(colour) {
                    mins[i] = val
                }
            }
        }
        sum += mins[0] as i32 * mins[1] as i32 * mins[2] as i32;
    }

    sum
}
