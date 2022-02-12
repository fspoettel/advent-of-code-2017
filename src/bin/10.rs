fn get_int_list(size: usize) -> Vec<usize> {
    let mut int_list = vec![0; size];

    for (i, v) in int_list.iter_mut().enumerate() {
        *v = i
    }

    int_list
}

fn hash_round(lens: &[usize], list: &mut [usize], pos: &mut usize, skip: &mut usize) {
    for &len in lens {
        if len > 1 {
            let mut reversed = vec![0; len];

            for i in 0..len {
                reversed[i] = list[(*pos + i) % list.len()];
            }

            for i in 0..len {
                list[(*pos + i) % list.len()] = reversed[len - i - 1];
            }
        }

        *pos += len + *skip;
        *skip += 1;
    }
}

fn hash(input: &str) -> String {
    let mut lens: Vec<usize> = input.as_bytes().iter().map(|&c| c as usize).collect();
    lens.append(&mut vec![17, 31, 73, 47, 23]);

    let mut list = get_int_list(256);
    let mut pos = 0;
    let mut skip = 0;

    for _ in 0..64 {
        hash_round(&lens, &mut list, &mut pos, &mut skip);
    }

    list.chunks(16)
        .map(|c| {
            let n = c.iter().cloned().reduce(|acc, curr| acc ^ curr).unwrap();
            format!("{:02x}", n)
        })
        .fold(String::new(), |mut acc, curr| {
            acc.push_str(&curr);
            acc
        })
}

pub fn part_one(input: &str) -> usize {
    let lens: Vec<usize> = input
        .lines()
        .next()
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .unwrap();

    let mut list = get_int_list(256);
    hash_round(&lens, &mut list, &mut 0, &mut 0);

    list.iter().take(2).product()
}

pub fn part_two(input: &str) -> String {
    hash(input.lines().next().unwrap())
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 10), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
