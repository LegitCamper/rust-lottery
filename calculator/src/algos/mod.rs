pub mod friends;
use crate::{LotteryNumbers, LotteryTicket};
use itertools::Itertools;
use std::slice::Windows;
use std::{collections::HashMap, sync::Arc};

type Tickets = Vec<Vec<i8>>;

pub fn optimize(
    numbers: Arc<LotteryNumbers>,
    ticket_size: i32,
    algo: fn(Windows<'_, LotteryTicket>, i32) -> Vec<i8>,
) -> Vec<i8> {
    let mut results: HashMap<i32, i32> = HashMap::new();
    for w in 1..1000 {
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
        results.insert(w.try_into().unwrap(), matching_numbers.try_into().unwrap());
    }

    let mut most_numbers = (1, 0);
    for item in results {
        if item.1 > most_numbers.1 {
            most_numbers = item
        }
    }

    // runs with now optimized window size
    let window = numbers.windows(most_numbers.0.try_into().unwrap());
    algo(window, ticket_size)
}

pub fn get_tickets(ticket_numbers: Vec<i8>, ticket_size: usize) -> Tickets {
    ticket_numbers
        .into_iter()
        .combinations(ticket_size)
        .collect::<Tickets>() // next real ticket numbers
}
