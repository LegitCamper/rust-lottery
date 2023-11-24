use crate::{LotteryTicket, LotteryTickets};

use chrono::NaiveDate;
use combinations::Combinations;
use num::rational;
use std::{cmp::Ordering, collections::HashMap};
use tokio::task;

const WINDOW_SIZE: usize = 150;

pub async fn optimize(
    lottery_tickets: LotteryTickets,
    ticket_size: u8,
    algo: fn(&[LotteryTicket], u8) -> Vec<u8>,
    filters: Vec<fn(algo_guesses: &mut Vec<Vec<u8>>)>,
) {
    let mut tasks = Vec::new();

    let max_depth: usize;
    if lottery_tickets.len() - 2 < WINDOW_SIZE {
        // len - 2 to let last window have ticket to get accuracy
        max_depth = lottery_tickets.len() - 2
    } else {
        max_depth = WINDOW_SIZE;
    }
    for window_size in 1..max_depth {
        let filters = filters.clone();
        tasks.push(task::spawn(async move {
            stats(lottery_tickets, window_size, ticket_size, algo, filters)
        }));
    }

    let mut best_depth = (0, 0);
    for task in tasks {
        let (window_size, weighted_matches, _) = task.await.unwrap();
        if weighted_matches > best_depth.1 {
            best_depth = (window_size, weighted_matches)
        }
    }

    // prints the optimal depth and algo stats
    let (window_size, _, mut ball_counter) = stats(
        lottery_tickets,
        best_depth.0 as usize,
        ticket_size,
        algo,
        filters,
    );

    let mut total_matches = 0;
    for matches in &mut ball_counter {
        *matches.1 = *matches.1 / (lottery_tickets.len() - 2) as u32;
        total_matches += *matches.1;
    }

    let temp_denom = window_size as u32 * ticket_size as u32;
    let ratio = rational::Ratio::new_raw(total_matches, temp_denom).reduced();

    println!("Algo Performance:");
    println!("Tickets: {}", window_size);
    println!("Total Correct Ball Count:");
    for num in ball_counter {
        println!("  correct balls: {}, times: {}", num.0, num.1);
    }
    println!(
        "Ratio of correct balls: {}:{}",
        ratio.numer(),
        ratio.denom()
    );
    println!("The optimal history of days (depth) is {}", best_depth.0);
}

fn stats(
    lottery_tickets: LotteryTickets,
    window_size: usize,
    ticket_size: u8,
    algo: fn(&[LotteryTicket], u8) -> Vec<u8>,
    filters: Vec<fn(algo_guesses: &mut Vec<Vec<u8>>)>,
) -> (usize, u32, HashMap<u8, u32>) {
    let mut weighted_matches = 0;
    let mut ball_counter = HashMap::new();
    for num in 1..ticket_size + 1 {
        ball_counter.insert(num, 0);
    }

    let windows: std::slice::Windows<LotteryTicket> =
        lottery_tickets[..lottery_tickets.len() - 1].windows(window_size);
    let windows_len = windows.len();

    for window in windows {
        let predicted_numbers = algo(window, ticket_size);
        if predicted_numbers.len() < ticket_size.into() {
            continue;
        }

        let mut predicted_tickets: Vec<Vec<u8>> =
            if usize::from(ticket_size) == predicted_numbers.len() {
                vec![predicted_numbers]
            } else {
                Combinations::new(predicted_numbers, ticket_size.into()).collect()
            };

        for filter in filters.iter() {
            filter(&mut predicted_tickets);
        }

        let next_ticket_index =
            find_tommorows_ticket(window.last().unwrap().date, &lottery_tickets[..]).unwrap();

        apply_weights(
            lottery_tickets,
            window,
            predicted_tickets,
            next_ticket_index,
            &mut weighted_matches,
            &mut ball_counter,
        );
    }

    // divide ball total by number of windows for fair eval later
    weighted_matches = weighted_matches / windows_len as u32;
    (window_size, weighted_matches, ball_counter)
}

fn apply_weights(
    lottery_tickets: LotteryTickets,
    window: &[LotteryTicket],
    predicted_tickets: Vec<Vec<u8>>,
    next_ticket_index: usize,
    weighted_matches: &mut u32,
    ball_counter: &mut HashMap<u8, u32>,
) {
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
        let mut correct_balls_count = 0;
        for num in ticket.iter() {
            // apply weight based on whether ticket had 1 - x matching numbers
            if lottery_tickets[next_ticket_index].numbers.contains(num) {
                *weighted_matches += *num as u32 * window_weight;
            }
            for correct_num in lottery_tickets[next_ticket_index]
                .numbers
                .clone()
                .into_iter()
            {
                if correct_num == *num {
                    correct_balls_count += 1;
                }
            }
        }
        ball_counter
            .entry(correct_balls_count)
            .and_modify(|counter| *counter += 1);
    }
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
