use aoc_2021::*;

fn main() {
    let mut positions = input(7, &ex::CommaNumbers);
    positions.sort_unstable();

    println!("part 1: {:#?}", {
        // median
        let target_pos = positions[positions.len() / 2];
        positions
            .iter()
            .map(|&p| (target_pos - p).abs())
            .sum::<i64>()
    });

    println!("part 2: {:#?}", {
        let cost_function = |target_pos: i64| -> i64 {
            positions
                .iter()
                .map(|&pos| {
                    let steps = (target_pos - pos).abs();
                    // automatically uses (n * (n + 1)) / 2
                    (1..=steps).sum::<i64>()
                })
                .sum()
        };

        let mut left = 0;
        let mut right = *positions.last().unwrap();
        let mut min_cost = None;
        while left <= right {
            let third = (right - left) / 3;
            let left_third = left + third;
            let right_third = right - third;

            let left_third_val = cost_function(left_third);
            if left_third_val >= cost_function(right_third) {
                min_cost = Some(left_third_val);
                left = left_third + 1;
            } else {
                right = right_third - 1;
            }
        }

        min_cost.unwrap()
    });
}
