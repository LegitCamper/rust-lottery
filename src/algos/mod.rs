use crate::{LotteryTicket, LotteryTickets};
// use rayon::prelude::*;
use std::collections::HashMap;
use tokio::task::spawn;

mod friends;
pub use friends::friends;
mod multiply;
pub use multiply::multiply;
mod quiet;
pub use quiet::quiet;

const MAX_DEPTH: usize = 1000;

pub async fn optimize(
    lottery_tickets: LotteryTickets,
    ticket_size: u8,
    algo: fn(&[LotteryTicket], u8) -> Vec<u8>,
) -> Vec<u8> {
    let num_balls = lottery_tickets[0].numbers.len();
    let mut results: HashMap<u32, usize> = HashMap::new();
    let mut tasks = Vec::new();

    let max_depth: usize;
    if lottery_tickets.len() - 2 < 100 {
        max_depth = lottery_tickets.len() - 2
    } else {
        max_depth = MAX_DEPTH
    }
    // finds the best window size that gives the best results
    for w in 1..max_depth {
        tasks.push(spawn(async move {
            let windows: std::slice::Windows<LotteryTicket> = lottery_tickets.windows(w);

            let mut matching_numbers = 0;
            for window in windows {
                let predicted_numbers = algo(window, ticket_size);

                let mut found_next_ticket = false;
                let mut next_ticket: &LotteryTicket = &LotteryTicket {
                    date: chrono::NaiveDate::MIN,
                    numbers: Vec::new(),
                };
                for ticket in lottery_tickets.iter() {
                    if ticket.date == window[w - 1].date {
                        found_next_ticket = true;
                        continue;
                    }
                    if found_next_ticket {
                        next_ticket = ticket;
                    }
                }

                for num in next_ticket.numbers.iter() {
                    if predicted_numbers.contains(num) {
                        matching_numbers += num_balls; // dont think this is right
                    }
                }
            }
            (w as u32, matching_numbers)
        }));
    }

    for task in tasks {
        let (ws, mn) = task.await.unwrap();
        results.insert(ws, mn);
    }

    let mut most_numbers = (1, 0);
    for item in results.iter() {
        if item.1 > &most_numbers.1 {
            most_numbers = (*item.0, *item.1)
        }
    }

    // makes prediction based on optimizations
    let mut window = lottery_tickets.windows(most_numbers.0.try_into().unwrap());
    algo(window.next_back().unwrap(), ticket_size)
}
