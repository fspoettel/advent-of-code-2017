const GRID_SIZE: usize = 256;

pub fn get_int_list() -> [usize; GRID_SIZE] {
    let mut int_list = [0; GRID_SIZE];

    for (i, v) in int_list.iter_mut().enumerate() {
        *v = i
    }

    int_list
}

pub fn hash_round(lens: &[usize], list: &mut [usize], pos: &mut usize, skip: &mut usize) {
    for &len in lens {
        if len > 1 {
            for i in 0..(len / 2) {
                list.swap((*pos + i) % list.len(), (*pos + len - i - 1) % list.len());
            }
        }

        *pos += len + *skip;
        *skip += 1;
    }
}

pub fn knot_hash(input: &str) -> String {
    let mut lens: Vec<usize> = input.as_bytes().iter().map(|&c| c as usize).collect();
    lens.append(&mut vec![17, 31, 73, 47, 23]);

    let mut list = get_int_list();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
