use crate::LotteryTicket;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
struct BallCount {
    ball: u8,
    count: usize,
}
impl Ord for BallCount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ball.cmp(&other.ball)
    }
}
impl PartialOrd for BallCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for BallCount {
    fn eq(&self, other: &Self) -> bool {
        self.ball == other.ball
    }
}

#[allow(unused)]
pub fn spine_sort(window: &[LotteryTicket], length: u8) -> Vec<u8> {
    let mut spine_counter: Vec<BallCount> = vec![];

    for ticket in window {
        for ball in ticket.numbers.clone().into_iter() {
            for (index, mut e_num) in spine_counter.iter_mut().enumerate() {
                if e_num.ball == ball {
                    e_num.count += 1;
                    continue;
                }
            }
            spine_counter.push(BallCount { ball, count: 1 })
        }
    }

    spine_counter.sort_by(|a, b| a.count.cmp(&a.count));
    let mut predicted_nums: Vec<u8> = spine_counter
        .into_iter()
        .map(|ball_count| ball_count.ball)
        .collect();

    predicted_nums.truncate(8);
    predicted_nums
}
