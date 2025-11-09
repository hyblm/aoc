use std::str::FromStr;

use Symbol::*;

fn main() {
    let input = include_str!("input.txt");

    let mut map: Map = input.parse().unwrap();
    let enclosed_area = map.enclosed_area(input);

    println!("{enclosed_area}");
}

struct Map {
    stride: usize,
    surface: Vec<Symbol>,
}

impl Map {
    fn enclosed_area(&mut self, input: &str) -> usize {
        let start_index = self
            .surface
            .iter()
            .position(|&x| x == Symbol::Start)
            .expect("start exists");

        let mut cursors = [Cursor::default(); 2];
        self.init_cursors(start_index, &mut cursors);
        self.surface[start_index] = match (cursors[0].direction, cursors[1].direction) {
            (Direction::South, Direction::North) | (Direction::North, Direction::South) => Vertical,
            (Direction::West, Direction::East) | (Direction::East, Direction::West) => Horizontal,
            (Direction::South, Direction::West)
            | (Direction::South, Direction::East)
            | (Direction::West, Direction::South)
            | (Direction::East, Direction::South) => CornerDown,
            (Direction::North, Direction::East)
            | (Direction::East, Direction::North)
            | (Direction::West, Direction::North)
            | (Direction::North, Direction::West) => CornerUp,
            _ => unreachable!(),
        };

        while cursors[0].index != cursors[1].index {
            let old0 = cursors[0].index;
            let old1 = cursors[1].index;

            cursors[0].next(self);
            cursors[1].next(self);

            self.surface[old0].visit();
            self.surface[old1].visit();
        }

        self.surface[cursors[0].index].visit();

        let mut enclosed_area = 0;
        let mut inside = false;
        let mut last_corner: Symbol = Ground;

        for (i, s) in self.surface.iter().enumerate() {
            match s {
                Vertical => {
                    inside = !inside;
                }
                Horizontal => (),
                CornerDown | CornerUp => {
                    if last_corner == Ground {
                        last_corner = *s;
                    } else {
                        if last_corner != *s {
                            inside = !inside;
                        }
                        last_corner = Ground;
                    }
                }
                _ => {
                    enclosed_area += inside as usize;
                }
            }

            #[cfg(debug_assertions)]
            {
                for (j, s) in self.surface[..=i].iter().enumerate() {
                    eprint!("{s}");
                    if j > 0 && (j + 1) % self.stride == 0 {
                        eprintln!();
                    }
                }
                eprintln!();

                eprintln!("{}", input.split_at(i + 1 + (i / self.stride)).0);

                eprintln!("{:10}\t{inside}\t{enclosed_area}", format!("{s:?}"));
            }
        }
        enclosed_area
    }

    #[allow(unused)]
    fn find_farthest(&self) -> usize {
        let start_index = self
            .surface
            .iter()
            .position(|&x| x == Symbol::Start)
            .expect("start exists");

        let mut cursors = [Cursor::default(); 2];
        self.init_cursors(start_index, &mut cursors);

        let mut distance = 1;
        while cursors[0].index != cursors[1].index {
            cursors[0].next(self);
            cursors[1].next(self);
            distance += 1;
        }
        distance
    }

    fn init_cursors(&self, start_index: usize, cursors: &mut [Cursor]) {
        let mut cursor_index = 0;

        use Direction::*;

        for direction in [North, South, West, East] {
            let index = (start_index as isize + direction.to_offset(self.stride)) as usize;
            if direction.connects(self.surface[index]) {
                cursors[cursor_index] = Cursor { index, direction };
                cursor_index += 1;
            }
        }
        assert_eq!(cursor_index, 2);
    }
}
impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stride = s.as_bytes().iter().position(|&c| c == b'\n').unwrap();
        let surface = s
            .lines()
            .flat_map(|line| line.as_bytes().iter().copied().map(Symbol::from))
            .collect();
        Ok(Self { stride, surface })
    }
}

#[derive(Clone, Copy, Default)]
struct Cursor {
    direction: Direction,
    index: usize,
}
impl Cursor {
    fn next(&mut self, map: &Map) {
        use Direction::*;
        self.direction = match (self.direction, map.surface[self.index]) {
            (North, PipeNorthSouth) | (West, BendNorthEast) | (East, BendNorthWest) => North,
            (South, PipeNorthSouth) | (West, BendSouthEast) | (East, BendSouthWest) => South,
            (North, BendSouthWest) | (South, BendNorthWest) | (West, PipeEastWest) => West,
            (North, BendSouthEast) | (South, BendNorthEast) | (East, PipeEastWest) => East,
            _ => unreachable!(
                "entered {:?} from {:?} at i: {}",
                map.surface[self.index], self.direction, self.index
            ),
        };
        self.index = (self.index as isize + self.direction.to_offset(map.stride)) as usize;
        assert!(
            self.direction.connects(map.surface[self.index]),
            "{:?} {:?}",
            self.direction,
            map.surface[self.index]
        );
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum Direction {
    #[default]
    North,
    South,
    West,
    East,
}

impl Direction {
    /// if we're entering a symbol from direction does it connect
    fn connects(&self, symbol: Symbol) -> bool {
        match self {
            Direction::North => matches!(symbol, PipeNorthSouth | BendSouthEast | BendSouthWest),
            Direction::South => matches!(symbol, PipeNorthSouth | BendNorthWest | BendNorthEast),
            Direction::West => matches!(symbol, PipeEastWest | BendSouthEast | BendNorthEast),
            Direction::East => matches!(symbol, PipeEastWest | BendSouthWest | BendNorthWest),
        }
    }

    fn to_offset(self, stride: usize) -> isize {
        use Direction::*;
        match self {
            North => -(stride as isize),
            South => stride as isize,
            West => -1,
            East => 1,
        }
    }
}

#[repr(u8)]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Symbol {
    Ground,
    PipeNorthSouth,
    PipeEastWest,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Start,
    CornerUp,
    CornerDown,
    Horizontal,
    Vertical,
}

impl Symbol {
    fn visit(&mut self) {
        *self = match self {
            PipeNorthSouth => Vertical,
            PipeEastWest => Horizontal,
            BendNorthEast | BendNorthWest => CornerUp,
            BendSouthWest | BendSouthEast => CornerDown,
            _ => unreachable!(),
        }
    }
}

impl core::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Vertical => "|",
                Horizontal => "-",
                CornerDown => "v",
                CornerUp => "^",
                _ => ".",
            },
        )
    }
}

impl From<u8> for Symbol {
    fn from(value: u8) -> Self {
        use Symbol::*;
        match value {
            b'.' => Ground,
            b'|' => PipeNorthSouth,
            b'-' => PipeEastWest,
            b'L' => BendNorthEast,
            b'J' => BendNorthWest,
            b'7' => BendSouthWest,
            b'F' => BendSouthEast,
            b'S' => Start,
            _ => unreachable!("invalid symbol in input {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        #[rustfmt::skip]
        let input = 
".....
.S-7.
.|.|.
.L-J.
.....";

        let map: Map = input.parse().unwrap();
        let distance = map.find_farthest();

        assert_eq!(distance, 4);
    }

    #[test]
    fn area() {
        #[rustfmt::skip]
        let input = 
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

        let mut map: Map = input.parse().unwrap();
        let enclosed_area = map.enclosed_area(input);

        assert_eq!(enclosed_area, 4);
    }
}
