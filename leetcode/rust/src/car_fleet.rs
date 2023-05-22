use super::Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let cars = Solution::build_cars(position, speed);

        let mut stack = vec![];
        for (pos, speed) in cars {
            let time = (target - pos) as f32 / speed as f32;

            while let Some(t) = stack.last() {
                if time.partial_cmp(t) == Some(Ordering::Less) {
                    break;
                }

                stack.pop();
            }

            stack.push(time);
        }

        stack.len() as i32
    }

    #[inline]
    fn build_cars(position: Vec<i32>, speed: Vec<i32>) -> Vec<(i32, i32)> {
        let mut cars: Vec<_> = position.into_iter().zip(speed.into_iter()).collect();
        cars.sort_unstable_by(|&(pos1, _), (pos2, _)| pos1.cmp(pos2));

        cars
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn car_fleet() {
        assert_eq!(
            Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
            3
        );
        assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
        assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);

        assert_eq!(Solution::car_fleet(10, vec![6, 8], vec![3, 2]), 2);
        assert_eq!(Solution::car_fleet(10, vec![0, 4, 2], vec![2, 1, 3]), 1);
        assert_eq!(
            Solution::car_fleet(13, vec![10, 2, 5, 7, 4, 6, 11], vec![7, 5, 10, 5, 9, 4, 1]),
            2
        );
    }
}
