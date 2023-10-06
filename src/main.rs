#[allow(unused)]
use crate::{
    algos::{friends, multiply, optimize, quiet},
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

    let ticket_len = lottery_tickets[0].numbers.len() as u8;
    let draw_date = lottery_tickets[lottery_tickets.len() - 1]
        .date
        .checked_add_days(Days::new(1))
        .unwrap();

    let mut algo_guesses: Vec<u8> = Vec::new();

    // Algos
    algo_guesses.append(&mut optimize(lottery_tickets, ticket_len, friends).await);
    // algo_guesses.append(&mut optimize(lottery_tickets, ticket_len, quiet).await);
    // algo_guesses.append(&mut optimize(lottery_tickets, ticket_len, multiply).await);

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
