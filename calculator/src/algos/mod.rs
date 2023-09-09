pub mod friends;
use crate::{LotteryNumbers, LotteryTicket};
use itertools::Itertools;
use std::{
    slice::Windows,
    sync::Mutex,
    {collections::HashMap, sync::Arc},
};
use tokio::task::spawn;

pub async fn optimize(
    numbers: Arc<LotteryNumbers>,
    ticket_size: i32,
    algo: fn(Windows<'_, LotteryTicket>, i32) -> Vec<i8>,
) -> Vec<i8> {
    let results: Arc<Mutex<HashMap<i32, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    // let mut task_handles: Vec<JoinHandle<()>> = Vec::new();
    for w in 1..numbers.len() - 2 {
        let numbers = numbers.clone();
        let results = results.clone();
        spawn(async move {
            // finds the best window size that gives the best results
            let window = numbers.windows(w);

            let tickets = algo(window, ticket_size);
            let next_ticket = &numbers[w as usize].numbers;

            let mut matching_numbers = 0;
            for ticket in tickets.iter().combinations(numbers[0].numbers.len()) {
                matching_numbers += ticket
                    .iter()
                    .zip(next_ticket.clone())
                    .filter(|&(a, b)| a == &&b)
                    .count();
            }
            results
                .lock()
                .unwrap()
                .insert(w as i32, matching_numbers.try_into().unwrap());
        });
    }

    let mut most_numbers = (1, 0);
    for item in results.lock().unwrap().iter() {
        if item.1 > &most_numbers.1 {
            most_numbers = (*item.0, *item.1)
        }
    }

    // runs with now optimized window size
    let window = numbers.windows(most_numbers.0.try_into().unwrap());
    algo(window, ticket_size)
}
