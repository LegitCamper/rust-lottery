use crate::{
    algos::{friends, get_tickets, optimize},
    read::{data_keymap, LotteryTicket},
};
mod algos;
mod read;
use std::sync::Arc;

type LotteryNumbers = Vec<LotteryTicket>;

fn main() {
    let numbers = Arc::new(data_keymap().unwrap());
    let ticket_length = numbers[0].numbers.len();

    let mut ticket_numbers: Vec<i8> = Vec::new();

    ticket_numbers.append(&mut optimize(
        numbers.clone(),
        ticket_length.try_into().unwrap(),
        friends::friends,
    ));

    let tickets = get_tickets(ticket_numbers, ticket_length);

    println!("{tickets:?}");
}
