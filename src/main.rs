#[allow(unused)]
use crate::{
    algos::{friends, multiply, optimize, quiet, spine_sort},
    read::{data_keymap, LotteryTicket},
};
use chrono::naive::{Days, NaiveDate};
use itertools::Itertools;
mod algos;
mod read;
mod tests;
use std::boxed::Box;

type LotteryTickets = &'static [LotteryTicket];
type Tickets = Vec<Vec<u8>>;

#[tokio::main]
async fn main() {
    let lottery_tickets: LotteryTickets = Box::leak(Box::new(data_keymap().unwrap()));

    let num_tickets = lottery_tickets.len();
    let ticket_len = lottery_tickets[0].numbers.len() as u8;
    let draw_date = lottery_tickets[lottery_tickets.len() - 1]
        .date
        .checked_add_days(Days::new(1))
        .unwrap();

    let mut algo_guesses: Vec<u8> = Vec::new();
    // ensure you set the current most optimal number of days correctly
    algo_guesses.append(&mut friends(
        &lottery_tickets[num_tickets - 13..],
        ticket_len,
    ));
    // algo_guesses.append(&mut quiet(&lotter_tickets[num_tickets - 13..], ticket_len));
    // algo_guesses.append(&mut multiply(&lotter_tickets[num_tickets - 13..], ticket_len));
    // algo_guesses.append(&mut spine_sort(&lotter_tickets[num_tickets - 13..], ticket_len));

    print_as_tickets(algo_guesses, ticket_len, draw_date);
}

fn print_as_tickets(algo_guesses: Vec<u8>, ticket_len: u8, draw_date: NaiveDate) {
    let tickets = algo_guesses
        .into_iter()
        .unique()
        .combinations(ticket_len as usize)
        .sorted()
        .collect::<Tickets>(); // next real ticket numbers

    println!(
        "For Draw On: {} \nPredicted Ticket(s):",
        draw_date.format("%m-%d-%Y").to_string(),
    );

    for ticket in tickets {
        let ticket = ticket.iter().sorted().collect::<Vec<&u8>>();
        println!("{ticket:?}");
    }
}
