const INPUT: &str = include_str!("input.txt");

pub fn problem1() -> i32 {
    let lines: Vec<_> = INPUT.lines().map(str::as_bytes).collect();
    let mut sum = 0;
    let nums = get_nums(&lines);

    for (num, row, col, len) in nums {
        if (col..=col + len)
            .flat_map(|col| (row..=row + 2).map(move |row| (row, col)))
            .any(|(row, col)| {
                if col > 0 && row > 0 {
                    let chr = lines
                        .get(row - 1)
                        .copied()
                        .unwrap_or(&[b'.'])
                        .get(col - 1)
                        .copied()
                        .unwrap_or(b'.');
                    chr != b'.' && !chr.is_ascii_digit()
                } else {
                    false
                }
            })
        {
            sum += num;
        }
    }

    sum
}

pub fn problem2() -> i32 {
    let lines: Vec<_> = INPUT.lines().map(str::as_bytes).collect();
    let mut sum = 0;
    let nums = get_nums(&lines);

    for (line, row) in lines.iter().zip(0..) {
        for (chr, col) in line.iter().zip(0..) {
            if *chr == b'*' {
                let mut found_nums = vec![];

                for (num, nrow, ncol, len) in &nums {
                    let (num, nrow, ncol, len) = (*num, *nrow, *ncol, *len);
                    if (ncol..=ncol + len)
                        .flat_map(|ncol| (nrow..=nrow + 2).map(move |nrow| (nrow, ncol)))
                        .any(|(nrow, ncol)| {
                            if ncol > 0 && nrow > 0 {
                                row == nrow - 1 && col == ncol - 1
                            } else {
                                false
                            }
                        })
                    {
                        found_nums.push(num);
                    }
                }

                if found_nums.len() == 2 {
                    sum += found_nums[0] * found_nums[1];
                }
            }
        }
    }

    sum
}

fn get_nums(lines: &[&[u8]]) -> Vec<(i32, usize, usize, usize)> {
    let mut nums: Vec<(i32, usize, usize, usize)> = vec![];

    for (line, row) in lines.iter().zip(0..) {
        let mut prev = b'.';

        for col in 0..line.len() {
            if line[col].is_ascii_digit() {
                if prev.is_ascii_digit() {
                    let last = nums.last_mut().unwrap();
                    last.0 *= 10;
                    last.0 += (line[col] & 0b1111) as i32;
                    last.3 += 1;
                } else {
                    nums.push(((line[col] & 0b1111) as i32, row, col, 2));
                }
            }
            prev = line[col];
        }
    }

    nums
}
