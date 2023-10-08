use crate::{LotteryTicket, LotteryTickets};

use chrono::NaiveDate;
use combinations::Combinations;
use std::cmp::Ordering;
use tokio::task;

mod friends;
pub use friends::friends;
mod multiply;
pub use multiply::multiply;
mod quiet;
pub use quiet::quiet;
mod spine_sort;
pub use spine_sort::spine_sort;

const WINDOW_SIZE: usize = 150;

pub async fn optimize(
    lottery_tickets: LotteryTickets,
    ticket_size: u8,
    algo: fn(&[LotteryTicket], u8) -> Vec<u8>,
) -> u32 {
    let mut tasks = Vec::new();

    let max_depth: usize;
    if lottery_tickets.len() - 2 < WINDOW_SIZE {
        // len - 2 to let last window have ticket to get accuracy
        max_depth = lottery_tickets.len() - 2
    } else {
        max_depth = WINDOW_SIZE;
    }
    for window_size in 1..max_depth {
        tasks.push(task::spawn(async move {
            let mut weighted_matches: f64 = 0.0;

            let windows: std::slice::Windows<LotteryTicket> =
                lottery_tickets[..lottery_tickets.len() - 1].windows(window_size);

            for (window_index, window) in windows.clone().enumerate() {
                let predicted_numbers = algo(window, ticket_size);
                let predicted_tickets: Vec<Vec<u8>> =
                    Combinations::new(predicted_numbers, ticket_size.into()).collect();

                let next_ticket_index =
                    find_tommorows_ticket(window.last().unwrap().date, &lottery_tickets[..])
                        .unwrap();

                // tally matching numbers and multiply weight - weight of recentness and weight of balls
                let window_weight = 1.0 / window_index as f64;

                for ticket in predicted_tickets.iter() {
                    for num in ticket.iter() {
                        let ball_weight = num.pow(2) as f64;
                        if lottery_tickets[next_ticket_index].numbers.contains(num) {
                            weighted_matches += ball_weight * window_weight;
                        }
                    }
                }
            }

            // divide ball total by number of windows for fair eval later
            weighted_matches = weighted_matches / windows.len() as f64;

            (window_size as u32, weighted_matches)
        }));
    }

    let mut best_depth = (0, 0.0);
    for task in tasks {
        let (window_size, weighted_matches) = task.await.unwrap();
        if weighted_matches > best_depth.1 {
            best_depth = (window_size, weighted_matches)
        }
    }

    // returnes the optimal depth
    best_depth.0
}

fn find_tommorows_ticket(k: NaiveDate, items: &[LotteryTicket]) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = items.len();

    while low < high {
        let middle = (high + low) / 2;
        match items[middle].date.cmp(&k) {
            Ordering::Equal => return Some(middle),
            Ordering::Greater => high = middle,
            Ordering::Less => low = middle + 1,
        }
    }
    None
}
