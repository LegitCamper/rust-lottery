use crate::{LotteryTicket, LotteryTickets};
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
mod spine_sort;
pub use spine_sort::spine_sort;

const MAX_DEPTH: usize = 1000;

pub async fn optimize(
    lottery_tickets: LotteryTickets,
    last_ticket: &mut LotteryTicket,
    ticket_size: u8,
    algo: fn(&[LotteryTicket], u8) -> Vec<u8>,
) -> u32 {
    let results: Arc<Mutex<HashMap<u32, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let last_ticket = Arc::new(last_ticket);
    let num_done = &AtomicUsize::new(0);

    let most_wins = rayon::scope(|s| {
        let max_depth: usize;
        if lottery_tickets.len() - 2 < MAX_DEPTH {
            max_depth = lottery_tickets.len() - 2
        } else {
            max_depth = MAX_DEPTH
        }
        for w in 1..max_depth {
            let results = results.clone();
            let last_ticket = last_ticket.clone();
            s.spawn(move |_| {
                let windows: std::slice::Windows<LotteryTicket> = lottery_tickets.windows(w);

                let mut matching_numbers = 0;
                for window in windows {
                    let predicted_numbers = algo(window, ticket_size);

                    for num in predicted_numbers.iter() {
                        matching_numbers += last_ticket
                            .numbers
                            .iter()
                            .filter(|&n| *n as u8 == *num)
                            .count();
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

        let mut most_wins = (1, 0);
        for (key, item) in results.lock().unwrap().iter() {
            if item > &most_wins.1 {
                most_wins = (*key, *item)
            }
        }
        most_wins
    });

    most_wins.0
}
