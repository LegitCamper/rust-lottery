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
use clap::Parser;
use std::boxed::Box;

type LotteryTickets = &'static [LotteryTicket];
type Tickets = Vec<Vec<u8>>;

const MAX_HISTORY: usize = 1000;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// run in testing mode?
    #[arg(short, long)]
    test: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let tickets: LotteryTickets = Box::leak(Box::new(data_keymap().unwrap()));

    if args.test {
        let tickets_trimmed = tickets[tickets.len() - MAX_HISTORY..].to_vec();
        let lottery_tickets: LotteryTickets = Box::leak(Box::new(tickets_trimmed));
        let ticket_len = lottery_tickets[0].numbers.len() as u8;

        let optimal_history = optimize(lottery_tickets, ticket_len, spine_sort).await;
        println!("The optimal history of days is {optimal_history}")
    } else {
        let num_tickets = tickets.len();
        let ticket_len = tickets[0].numbers.len() as u8;
        let draw_date = tickets[tickets.len() - 1]
            .date
            .checked_add_days(Days::new(1))
            .unwrap();

        let mut algo_guesses: Vec<u8> = Vec::new();
        // ensure you set the current most optimal number of days correctly
        algo_guesses.append(&mut friends(&tickets[num_tickets - 13..], ticket_len));
        // algo_guesses.append(&mut quiet(&lotter_tickets[num_tickets - 13..], ticket_len));
        // algo_guesses.append(&mut multiply(&lotter_tickets[num_tickets - 13..], ticket_len));
        // algo_guesses.append(&mut spine_sort(&lotter_tickets[num_tickets - 13..], ticket_len));

        print_as_tickets(algo_guesses, ticket_len, draw_date);
    }
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
