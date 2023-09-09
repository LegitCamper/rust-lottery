use crate::{
    algos::{friends::friends, optimize},
    read::{data_keymap, LotteryTicket},
};
use chrono::naive::{Days, NaiveDate};
use itertools::Itertools;
use tokio;
mod algos;
mod read;
use std::sync::Arc;

type LotteryNumbers = Vec<LotteryTicket>;
type Tickets = Vec<Vec<i8>>;

#[tokio::main]
async fn main() {
    let numbers = Arc::new(data_keymap().unwrap());
    let ticket_length = numbers[0].numbers.len() as i32;
    let draw_date = numbers[numbers.len() - 1]
        .date
        .checked_add_days(Days::new(1))
        .unwrap();

    let mut ticket_numbers: Vec<i8> = Vec::new();
    // ticket_numbers.append(&mut optimize(numbers.clone(), ticket_length, friends).await); // THIS SEEMS WRONG
    // outputs way more than expected
    ticket_numbers.append(&mut optimize(numbers.clone(), ticket_length, friends).await);

    print_as_tickets(ticket_numbers, ticket_length, draw_date);
}

fn print_as_tickets(ticket_numbers: Vec<i8>, ticket_size: i32, draw_date: NaiveDate) {
    let tickets = ticket_numbers
        .into_iter()
        .combinations(ticket_size as usize)
        .sorted()
        .collect::<Tickets>(); // next real ticket numbers

    println!("Ticket Date: {draw_date:?} \nPredicted Tickets:",);

    for ticket in tickets {
        let ticket = ticket.iter().sorted().collect::<Vec<&i8>>();
        println!("{ticket:?}",);
    }
}
