use crate::{LotteryTicket, LotteryTickets};
// use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        Arc, Mutex,
    },
    thread,
    time::Duration as StdDuration,
};

mod friends;
pub use friends::friends;
mod multiply;
pub use multiply::multiply;
mod quiet;
pub use quiet::quiet;

const MAX_DEPTH: usize = 1000;

pub async fn optimize(
    lottery_tickets: LotteryTickets,
    ticket_size: u8,
    algo: fn(&[LotteryTicket], u8) -> Vec<u8>,
) -> Vec<u8> {
    let num_balls = lottery_tickets[0].numbers.len();

    // let tasks =
    let results: Arc<Mutex<HashMap<u32, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let num_done = &AtomicUsize::new(0);

    let most_wins = rayon::scope(|s| {
        let max_depth: usize;
        if lottery_tickets.len() - 2 < 100 {
            max_depth = lottery_tickets.len() - 2
        } else {
            max_depth = MAX_DEPTH
        }
        for w in 1..max_depth {
            let results = results.clone();
            s.spawn(move |_| {
                let windows: std::slice::Windows<LotteryTicket> = lottery_tickets.windows(w);

                let mut matching_numbers = 0;
                for window in windows {
                    let predicted_numbers = algo(window, ticket_size);

                    let mut found_next_ticket = false;
                    let mut next_ticket: &LotteryTicket = &LotteryTicket {
                        date: chrono::NaiveDate::MIN,
                        numbers: Vec::new(),
                    };
                    for ticket in lottery_tickets.iter() {
                        if ticket.date == window[w - 1].date {
                            found_next_ticket = true;
                            continue;
                        }
                        if found_next_ticket {
                            next_ticket = ticket;
                        }
                    }

                    for num in next_ticket.numbers.iter() {
                        if predicted_numbers.contains(num) {
                            matching_numbers += num_balls; // dont think this is right
                        }
                    }
                }
                num_done.fetch_add(1, Relaxed);
                results.lock().unwrap().insert(w as u32, matching_numbers);
            });
        }

        loop {
            let n = num_done.load(Relaxed);
            if n == max_depth - 1 {
                break;
            }
            println!("Working.. {n}/{max_depth}");
            thread::sleep(StdDuration::from_secs(1));
        }

        let mut most_numbers = (1, 0);
        for (key, item) in results.lock().unwrap().iter() {
            if item > &most_numbers.1 {
                most_numbers = (*key, *item)
            }
        }
        most_numbers
    });

    println!("Optimized History is {}", most_wins.0);

    // makes prediction based on optimizations
    let mut window = lottery_tickets.windows(most_wins.0.try_into().unwrap());
    algo(window.next_back().unwrap(), ticket_size)
}
