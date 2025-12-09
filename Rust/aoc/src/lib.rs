use std::ops::Range;

pub fn ranges_merge_overlapping<T: Ord + Copy>(ranges: &mut [Range<T>]) -> Vec<Range<T>> {
    let mut ranges_nonoverlapping = Vec::new();
    ranges.sort_by_key(|r| r.start);
    ranges_merge_consecutive_into(ranges, &mut ranges_nonoverlapping);
    ranges_nonoverlapping
}

pub fn ranges_merge_consecutive_into<T: Ord + Copy>(ranges: &[Range<T>], out: &mut Vec<Range<T>>) {
    let Range { mut start, mut end } = ranges[0];
    for range in &ranges[1..] {
        if range.start <= end {
            if range.end > end {
                end = range.end;
            }
        } else {
            out.push(start..end);
            start = range.start;
            end = range.end;
        }
    }
    out.push(start..end);
}

pub fn all_pairs<T: Copy>(red_tile_positions: &[T]) -> impl Iterator<Item = (T, T)> {
    red_tile_positions
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| red_tile_positions[i + 1..].iter().map(move |&p2| (p1, p2)))
}
