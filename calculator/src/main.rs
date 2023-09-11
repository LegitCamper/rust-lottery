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
    let numbers = Arc::new(data_keymap().unwrap());
    let ticket_length = numbers[0].numbers.len() as u8;
    let draw_date = numbers[numbers.len() - 1]
        .date
        .checked_add_days(Days::new(1))
        .unwrap();

    let mut ticket_numbers: Vec<u8> = Vec::new();
    ticket_numbers.append(&mut optimize(numbers.clone(), ticket_length, friends).await);
    // quiet(numbers.windows(30), ticket_length);
    println!("{ticket_numbers:?}");

    print_as_tickets(ticket_numbers, ticket_length, draw_date);
}

// need to remove duplicate numbers before combinations
fn print_as_tickets(ticket_numbers: Vec<u8>, ticket_size: u8, draw_date: NaiveDate) {
    let tickets = ticket_numbers
        .into_iter()
        .combinations(ticket_size as usize)
        .sorted()
        .collect::<Tickets>(); // next real ticket numbers

    println!("For Draw On: {draw_date:?} \nPredicted Ticket(s):");

    for ticket in tickets {
        let ticket = ticket.iter().sorted().collect::<Vec<&u8>>();
        println!("{ticket:?}");
    }
}
