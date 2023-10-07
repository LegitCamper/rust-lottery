use crate::{LotteryTicket, LotteryTickets};
use std::collections::HashMap;
use tokio::task;

mod friends;
pub use friends::friends;
mod multiply;
pub use multiply::multiply;
mod quiet;
pub use quiet::quiet;
mod spine_sort;
pub use spine_sort::spine_sort;

const MAX_DEPTH: usize = 150;

pub async fn optimize(
    lottery_tickets: LotteryTickets,
    ticket_size: u8,
    algo: fn(&[LotteryTicket], u8) -> Vec<u8>,
) -> u32 {
    let mut results: HashMap<u32, usize> = HashMap::new();
    let mut tasks = Vec::new();
    let max_depth: usize;
    if lottery_tickets.len() - 2 < MAX_DEPTH {
        max_depth = lottery_tickets.len() - 2
    } else {
        max_depth = MAX_DEPTH;
    }
    for w in 1..max_depth {
        tasks.push(task::spawn(async move {
            let windows: std::slice::Windows<LotteryTicket> =
                lottery_tickets[..lottery_tickets.len() - 1].windows(w);
            let mut matching_numbers = 0;
            for window in windows {
                let predicted_numbers = algo(window, ticket_size);

                #[allow(unused)]
                let mut next_ticket = &LotteryTicket {
                    date: chrono::NaiveDate::MIN,
                    numbers: Vec::new(),
                };
                let mut lottery_tickets_iter = lottery_tickets.iter();
                loop {
                    let ticket = lottery_tickets_iter.next().unwrap();
                    if ticket.date == window.last().unwrap().date {
                        next_ticket = lottery_tickets_iter.next().unwrap();
                        break;
                    }
                }
                for num in predicted_numbers.iter() {
                    matching_numbers += next_ticket
                        .numbers
                        .iter()
                        .filter(|&n| *n as u8 == *num)
                        .count();
                }
            }
            (w as u32, matching_numbers)
        }));
    }

    for task in tasks {
        let (ws, mn) = task.await.unwrap();
        results.insert(ws, mn);
    }

    let mut most_wins = (1, 0);
    for res in results.iter() {
        if res.1 > &most_wins.1 {
            most_wins = (*res.0, *res.1);
        }
    }
    most_wins.0
}
