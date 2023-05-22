use super::Solution;

#[derive(Debug)]
struct Match {
    time: i32,
    index: usize,
    new_pos: i32,
    new_speed: i32,
}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut stack: Vec<_> = position.into_iter().zip(speed.into_iter()).collect();
        stack.sort_unstable_by(|&(pos1, _), (pos2, _)| pos1.cmp(pos2));
        // stack.push((target, 0));

        println!("Start target = {}, stack = {:?}", target, stack);

        let mut result = 0;
        while let Some(m) = Solution::find_nearest_match(target, &stack) {
            let before = format!("{:?}", stack);

            stack.remove(m.index);
            stack.remove(m.index);
            stack.push((m.new_pos, m.new_speed));
            stack.sort_unstable_by(|&(pos1, _), (pos2, _)| pos1.cmp(pos2));

            println!(
                "  result = {}, m = {:?}, {} -> {:?}",
                result, m, before, stack
            );

            while let Some(&(top_pos, _)) = stack.last() {
                if top_pos < target {
                    break;
                }

                stack.pop();
                result += 1;

                println!("    result = {}, m = {:?}, stack = {:?}", result, m, stack);
            }

            // stack.push((target, 0));
            // if let Some(m2) = Solution::find_nearest_match(target, &stack) {
            //     if m2.new_pos == target && m2.new_speed == 0 {
            //         stack.pop();
            //         result += 1;
            //
            //         println!(
            //             "    result = {}, m2 = {:?}, stack = {:?}",
            //             result,
            //             m2,
            //             stack.iter().rev().skip(1).rev().collect::<Vec<_>>()
            //         );
            //     }
            // }
            // stack.pop();
        }

        result + stack.len() as i32
    }

    fn find_nearest_match(target: i32, stack: &Vec<(i32, i32)>) -> Option<Match> {
        if stack.len() < 2 {
            return None;
        }

        let mut nearest: Option<Match> = None;
        for i in 0..stack.len() - 1 {
            let (cur_pos, cur_speed) = stack[i];
            let (next_pos, next_speed) = stack[i + 1];

            if cur_speed <= next_speed {
                continue;
            }

            let t = ((next_pos as f32 - cur_pos as f32) / (cur_speed as f32 - next_speed as f32))
                .ceil() as i32;
            let pos = next_pos + next_speed * t;
            if pos > target {
                continue;
            }

            let mat = Match {
                time: t,
                index: i,
                new_pos: pos,
                new_speed: next_speed,
            };
            match nearest {
                Some(n) if t < n.time => nearest = Some(mat),
                Some(_) => (),
                None => nearest = Some(mat),
            };
        }

        nearest
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
