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

/// Simple program to predict the lottery and written in Rust!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of days to run algo on
    #[arg(long, default_value_t = 10)]
    history: u8,

    /// Run in testing mode?
    #[arg(short, long)]
    test: bool,

    /// Algo to run (spine, friends, multiply, quiet)
    #[arg(short, long)]
    algo: Algos,
    // todo
    // /// Filter to use (not impl)
    // #[arg(short, long, default_value_t = String::from("not impl"))]
    // filter: String,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Algos {
    Spine,
    Multiply,
    Friends,
    Quite,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let tickets: LotteryTickets = Box::leak(Box::new(data_keymap().unwrap()));

    let history = args.history as usize;

    let algo = match args.algo {
        Algos::Spine => spine_sort,
        Algos::Multiply => multiply,
        Algos::Quite => quiet,
        Algos::Friends => friends,
    };

    // let filter = match args.filter {

    if args.test {
        let tickets_trimmed = tickets[tickets.len() - MAX_HISTORY..].to_vec();
        let lottery_tickets: LotteryTickets = Box::leak(Box::new(tickets_trimmed));
        let ticket_len = lottery_tickets[0].numbers.len() as u8;

        let optimal_history = optimize(lottery_tickets, ticket_len, algo).await;
        println!("The optimal history of days is {optimal_history}")
    } else {
        let num_tickets = tickets.len();
        let ticket_len = tickets[0].numbers.len() as u8;
        let draw_date = tickets[tickets.len() - 1]
            .date
            .checked_add_days(Days::new(1))
            .unwrap();

        let mut algo_guesses: Vec<u8> = Vec::new();
        algo_guesses.append(&mut algo(&tickets[num_tickets - history..], ticket_len));

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
