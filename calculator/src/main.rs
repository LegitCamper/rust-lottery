use crate::{
    algos::{friends::friends, optimize, quiet::quiet},
    read::{data_keymap, LotteryTicket},
};
use chrono::naive::{Days, NaiveDate};
use itertools::Itertools;
use tokio;
mod algos;
mod read;
use std::sync::Arc;

type LotteryNumbers = Vec<LotteryTicket>;
type Tickets = Vec<Vec<u8>>;

#[tokio::main]
async fn main() {
    let mut numbers = Arc::new(data_keymap().unwrap());
    // only keeps latest 1000 for speed sake
    numbers = numbers[numbers.len() - 1000..numbers.len()].to_vec().into();

    let ticket_length = numbers[0].numbers.len() as u8;
    let draw_date = numbers[numbers.len() - 1]
        .date
        .checked_add_days(Days::new(1))
        .unwrap();

    let mut ticket_numbers: Vec<u8> = Vec::new();
    // ticket_numbers.append(&mut optimize(numbers.clone(), ticket_length, friends).await);
    ticket_numbers.append(&mut optimize(numbers.clone(), ticket_length, quiet).await);

    print_as_tickets(ticket_numbers, ticket_length, draw_date);
}

fn print_as_tickets(ticket_numbers: Vec<u8>, ticket_length: u8, draw_date: NaiveDate) {
    let tickets = ticket_numbers
        .into_iter()
        .unique()
        .combinations(ticket_length as usize)
        .sorted()
        .collect::<Tickets>(); // next real ticket numbers

    println!("For Draw On: {draw_date:?} \nPredicted Ticket(s):");

    for ticket in tickets {
        let ticket = ticket.iter().sorted().collect::<Vec<&u8>>();
        println!("{ticket:?}");
    }
}
