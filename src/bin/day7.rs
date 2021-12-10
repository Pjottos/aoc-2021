use aoc_2021::*;

fn main() {
    Harness::begin()
        .day(7)
        .extract(|text| {
            // Parse the input as a list of numbers.
            let mut result: Vec<i32> = text.split(',').map(|p| p.trim().parse().unwrap()).collect();

            // Both parts require the input to be sorted, besides, we don't have mutable
            // access in the part closures so we must sort it here.
            result.sort_unstable();

            result
        })
        .run_part(1, |positions| {
            // The value in an array with the minimum sum of absolute differences to
            // the other values of the array is the median.
            // Since the input is sorted we can simply take the middle element to get
            // the median position.
            let target_pos = positions[positions.len() / 2];

            // Calculate the total fuel cost.
            positions
                .iter()
                .map(|&p| (target_pos - p).abs())
                .sum::<i32>()
        })
        .run_part(2, |positions| {
            // Calculates the total amount of fuel spent to move all crabs to
            // the specified position
            let cost_function = |target_pos: i32| -> i32 {
                positions
                    .iter()
                    .map(|&pos| {
                        let steps = (target_pos - pos).abs();
                        // Every step takes 1 additional fuel over the cost
                        // of the last step.
                        (1..=steps).sum::<i32>()
                    })
                    .sum()
            };

            // Perform a ternary search to find the position with
            // the lowest total fuel cost.
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
