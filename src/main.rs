mod algos;
mod filters;
mod optimize;
mod read;
mod tests;

use crate::{
    algos::{friends, multiply, quiet, spine_sort},
    filters::{bell, even_odd},
    optimize::optimize,
    read::{data_keymap, LotteryTicket},
};

use chrono::naive::{Days, NaiveDate};
use clap::Parser;
use itertools::Itertools;
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

    /// Filter to use (even_odd, bell_curve) - limited to 5 filters at a time
    #[arg(short, long, num_args = 0..5, value_delimiter = ' ')]
    filters: Vec<Filters>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Algos {
    Spine,
    Multiply,
    Friends,
    Quite,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Filters {
    BellCurve,
    EvenOdd,
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

    let mut filters: Vec<fn(&mut Vec<Vec<u8>>, u8)> = vec![];
    for filter in args.filters {
        let filter = match filter {
            Filters::BellCurve => bell,
            Filters::EvenOdd => even_odd,
        };
        filters.push(filter);
    }

    if args.test {
        let tickets_trimmed = tickets[tickets.len() - MAX_HISTORY..].to_vec();
        let lottery_tickets: LotteryTickets = Box::leak(Box::new(tickets_trimmed));
        let ticket_len = lottery_tickets[0].numbers.len() as u8;

        optimize(lottery_tickets, ticket_len, algo, filters).await;
    } else {
        let num_tickets = tickets.len();
        let ticket_len = tickets[0].numbers.len() as u8;
        let draw_date = tickets[tickets.len() - 1]
            .date
            .checked_add_days(Days::new(1))
            .unwrap();

        let mut algo_guesses: Vec<u8> = Vec::new();
        algo_guesses.append(&mut algo(&tickets[num_tickets - history..], ticket_len));

        let mut algo_guesses = algo_guesses
            .into_iter()
            .unique()
            .combinations(ticket_len as usize)
            .sorted()
            .collect::<Tickets>(); // next real ticket numbers

        for filter in filters {
            filter(&mut algo_guesses, ticket_len);
        }

        print_as_tickets(algo_guesses, draw_date)
    }
}

fn print_as_tickets(algo_guesses: Vec<Vec<u8>>, draw_date: NaiveDate) {
    println!("For Draw On: {}", draw_date.format("%m-%d-%Y").to_string(),);
    println!("Predicted Ticket(s):");

    for ticket in algo_guesses {
        let ticket = ticket.iter().sorted().collect::<Vec<&u8>>();
        println!("{ticket:?}");
    }
}
