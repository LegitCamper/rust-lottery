use crate::{LotteryNumbers, LotteryTicket};
use std::collections::HashMap;
use tokio::task::spawn;

pub mod friends;
pub mod quiet;

pub async fn optimize(
    numbers: LotteryNumbers,
    ticket_size: u8,
    algo: fn(&[LotteryTicket], u8) -> Vec<u8>,
) -> Vec<u8> {
    let num_balls = numbers[0].numbers.len();
    let mut results: HashMap<u32, usize> = HashMap::new();
    let mut tasks = Vec::new();

    // finds the best window size that gives the best results
    for w in 1..numbers.len() - 2 {
        let numbers = numbers.clone();

        tasks.push(spawn(async move {
            let windows = numbers.windows(w);

            let mut matching_numbers = 0;
            for window in windows {
                let predicted_numbers = algo(window, ticket_size);

                let mut found_next_ticket = false;
                let mut next_ticket: &LotteryTicket = &LotteryTicket {
                    date: chrono::NaiveDate::MIN,
                    numbers: Vec::new(),
                };
                for ticket in numbers.iter() {
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
    let mut window = numbers.windows(most_numbers.0.try_into().unwrap());
    algo(window.next_back().unwrap(), ticket_size)
}
