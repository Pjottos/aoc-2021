use aoc_2021::*;

fn main() {
    Harness::begin()
        .day(7)
        .extract(|text| {
            let mut result: Vec<i32> = text.split(',').map(|p| p.trim().parse().unwrap()).collect();
            result.sort_unstable();
            result
        })
        .run_part(1, |positions| {
            // median
            let target_pos = positions[positions.len() / 2];
            positions
                .iter()
                .map(|&p| (target_pos - p).abs())
                .sum::<i32>()
        })
        .run_part(2, |positions| {
            let cost_function = |target_pos: i32| -> i32 {
                positions
                    .iter()
                    .map(|&pos| {
                        let steps = (target_pos - pos).abs();
                        // automatically uses (n * (n + 1)) / 2
                        (1..=steps).sum::<i32>()
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
