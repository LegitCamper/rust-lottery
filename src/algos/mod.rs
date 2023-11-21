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
            let mut weighted_matches = 0;

            let windows: std::slice::Windows<LotteryTicket> =
                lottery_tickets[..lottery_tickets.len() - 1].windows(window_size);
            let windows_len = windows.len();

            for window in windows {
                let predicted_numbers = algo(window, ticket_size);
                if predicted_numbers.len() < ticket_size.into() {
                    continue;
                }

                let predicted_tickets: Vec<Vec<u8>> =
                    if usize::from(ticket_size) == predicted_numbers.len() {
                        vec![predicted_numbers]
                    } else {
                        Combinations::new(predicted_numbers, ticket_size.into()).collect()
                    };

                let next_ticket_index =
                    find_tommorows_ticket(window.last().unwrap().date, &lottery_tickets[..])
                        .unwrap();

                // find index of first and last ticket in lottery_ticket and average them
                // then apply weight to results based on recentness
                let first_window_weight = lottery_tickets
                    .iter()
                    .position(|t| t == &window[0])
                    .unwrap() as i32;
                let last_window_weight = lottery_tickets
                    .iter()
                    .position(|t| t == &window[window.len() - 1])
                    .unwrap() as i32;
                let window_weight = first_window_weight - last_window_weight;
                let window_weight = window_weight.abs() as u32;

                // tallies correct balls
                for ticket in predicted_tickets.iter() {
                    for num in ticket.iter() {
                        // apply weight based on whether ticket had 1 - x matching numbers
                        if lottery_tickets[next_ticket_index].numbers.contains(num) {
                            weighted_matches += *num as u32 * window_weight
                        }
                    }
                }
            }

            // divide ball total by number of windows for fair eval later
            weighted_matches = weighted_matches / windows_len as u32;

            (window_size as u32, weighted_matches)
        }));
    }

    let mut best_depth = (0, 0);
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
