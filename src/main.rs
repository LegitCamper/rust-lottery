#[allow(unused)]
use crate::{
    algos::{friends, multiply, optimize, quiet, spine_sort},
    filters::even_odd,
    read::{data_keymap, LotteryTicket},
};
use chrono::naive::{Days, NaiveDate};
use itertools::Itertools;
mod algos;
mod filters;
mod read;
mod tests;
use clap::Parser;
use std::boxed::Box;

type LotteryTickets = &'static [LotteryTicket];
type Tickets = Vec<Vec<u8>>;

const MAX_HISTORY: usize = 1000;
const TICKET_COST: usize = 1;

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

    /// Filter to use (even_odd, bell_curve)
    #[arg(short, long)]
    filters: Vec<Filters>,

    /// If you know the next days numbers, you can run this to test profitablity
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    next_day_nums: Option<Vec<u8>>,
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
    // BellCurve,
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

    let mut filters = vec![];
    for filter in args.filters {
        filters.push(match filter {
            // Filters::BellCurve => bell
            Filters::EvenOdd => even_odd,
        });
    }

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

        let mut algo_guesses = algo_guesses
            .into_iter()
            .unique()
            .combinations(ticket_len as usize)
            .sorted()
            .collect::<Tickets>(); // next real ticket numbers

        for filter in filters {
            filter(&mut algo_guesses, ticket_len);
        }

        match args.next_day_nums {
            Some(nums) => print_algo_performance(algo_guesses, nums, draw_date),
            None => print_as_tickets(algo_guesses, draw_date),
        }
    }
}

fn print_algo_performance(
    algo_guesses: Vec<Vec<u8>>,
    next_day_nums: Vec<u8>,
    draw_date: NaiveDate,
) {
    println!(
        "For ticket drew on: {}",
        draw_date.format("%m-%d-%Y").to_string(),
    );
    println!("Algo Performance:");

    let mut most_balls = 0;
    let mut matching_nums = 0;
    for ticket in &algo_guesses {
        let mut temp_most_balls = 0;
        for t_num in ticket {
            for n_d_num in &next_day_nums {
                if t_num == n_d_num {
                    matching_nums += 1;
                    temp_most_balls += 1;
                }
            }
        }
        if temp_most_balls > most_balls {
            most_balls = temp_most_balls
        }
    }
    let matching_nums_avg = matching_nums as f32 / algo_guesses.len() as f32;
    println!(
        "cost: {}\naverage correct ballz per ticket: {}\nmost correct balls on ticket: {}\nratio of correct balls: {}:{}",
        algo_guesses.len() * TICKET_COST,
        matching_nums_avg,
        most_balls,
        matching_nums,
        algo_guesses.len() * algo_guesses[0].len(),
    );
}

fn print_as_tickets(algo_guesses: Vec<Vec<u8>>, draw_date: NaiveDate) {
    println!("For Draw On: {}", draw_date.format("%m-%d-%Y").to_string(),);
    println!("Predicted Ticket(s):");

    for ticket in algo_guesses {
        let ticket = ticket.iter().sorted().collect::<Vec<&u8>>();
        println!("{ticket:?}");
    }
}
