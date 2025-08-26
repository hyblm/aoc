fn main() {
    let galaxy_map = include_str!("input.txt");

    // let (coordinates_x, coordinates_y) = coordinates_from_map::<1>(galaxy_map);
    let (coordinates_x, coordinates_y) = coordinates_from_map::<999_999>(galaxy_map);

    let distances_sum =
        distance_from_coordinates(&coordinates_x) + distance_from_coordinates(&coordinates_y);

    println!("{distances_sum}");
}

fn coordinates_from_map<const EXPANSION: usize>(map: &str) -> (Vec<usize>, Vec<usize>) {
    let (expand_x, expand_y) = find_expanding_lines(map);

    map.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let expand = |c: usize, expand_list: &[usize]| {
                c + EXPANSION
                    * expand_list
                        .iter()
                        .position(|&p| p > c)
                        .unwrap_or(expand_list.len())
            };
            let y = expand(y, &expand_y);

            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(|(x, &byte)| (byte == b'#').then_some(expand(x, &expand_x)))
                .map(move |x| (x, y))
        })
        .unzip()
}

fn find_expanding_lines(map: &str) -> (Vec<usize>, Vec<usize>) {
    let width = map.bytes().position(|b| b == b'\n').unwrap();
    let mut columns_empty = vec![true; width];

    let expand_y: Vec<_> = map
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            let mut empty = true;
            for (x, byte) in line.bytes().enumerate() {
                let f = byte != b'#';
                empty &= f;
                columns_empty[x] &= f;
            }
            empty.then_some(y)
        })
        .collect();

    let expand_x: Vec<_> = columns_empty
        .iter()
        .enumerate()
        .filter_map(|(x, empty)| empty.then_some(x))
        .collect();

    (expand_x, expand_y)
}

fn distance_from_coordinates(coordinates: &[usize]) -> usize {
    let mut coordinates_iter = coordinates.iter();
    let mut total = 0;
    while let Some(x) = coordinates_iter.next() {
        for other in coordinates_iter.clone() {
            let distance = (*x as isize - *other as isize).abs();
            total += distance as usize;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const GALAXY_MAP: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn expansion_1() {
        let (coordinates_x, coordinates_y) = coordinates_from_map::<1>(GALAXY_MAP);
        let distances_sum =
            distance_from_coordinates(&coordinates_x) + distance_from_coordinates(&coordinates_y);

        assert_eq!(distances_sum, 374);
    }

    #[test]
    fn expansion_10() {
        let (coordinates_x, coordinates_y) = coordinates_from_map::<9>(GALAXY_MAP);
        let distances_sum =
            distance_from_coordinates(&coordinates_x) + distance_from_coordinates(&coordinates_y);

        assert_eq!(distances_sum, 1030);
    }

    #[test]
    fn expansion_100() {
        let (coordinates_x, coordinates_y) = coordinates_from_map::<99>(GALAXY_MAP);
        let distances_sum =
            distance_from_coordinates(&coordinates_x) + distance_from_coordinates(&coordinates_y);

        assert_eq!(distances_sum, 8410);
    }
}
