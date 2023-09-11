use crate::{LotteryNumbers, LotteryTicket};
use std::{
    sync::Mutex,
    {collections::HashMap, sync::Arc},
};
use tokio::task::spawn;

pub mod friends;
pub mod quiet;

pub async fn optimize(
    numbers: Arc<LotteryNumbers>,
    ticket_size: u8,
    algo: fn(&[LotteryTicket], u8) -> Vec<u8>,
) -> Vec<u8> {
    let num_balls = numbers[0].numbers.len();
    let results: Arc<Mutex<HashMap<u32, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    for w in 1..numbers.len() - 2 {
        let results = results.clone();
        let numbers = numbers.clone();

        let _ = spawn(async move {
            // finds the best window size that gives the best results
            let windows = numbers.windows(w);

            let mut matching_numbers = 0;
            for window in windows {
                let predicted_numbers = algo(window, ticket_size.into());

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

                for num in next_ticket.numbers.clone().into_iter() {
                    if predicted_numbers.contains(&num) {
                        matching_numbers += num_balls; // dont think this is right
                    }
                }
            }
            results.lock().unwrap().insert(w as u32, matching_numbers);
        });
    }

    let mut most_numbers = (1, 0);
    for item in results.lock().unwrap().iter() {
        if item.1 > &most_numbers.1 {
            most_numbers = (*item.0, *item.1)
        }
    }

    // makes prediction based on optimizations
    let mut window = numbers.windows(most_numbers.0.try_into().unwrap());
    algo(window.next_back().unwrap(), ticket_size.into())
}
