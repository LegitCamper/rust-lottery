use crate::{LotteryTicket, LotteryTickets};
use tokio::task;

mod friends;
pub use friends::friends;
mod multiply;
pub use multiply::multiply;
mod quiet;
pub use quiet::quiet;
mod spine_sort;
pub use spine_sort::spine_sort;

const WINDOW_SIZE: usize = 150;

pub async fn optimize(
    lottery_tickets: LotteryTickets,
    ticket_size: u8,
    algo: fn(&[LotteryTicket], u8) -> Vec<u8>,
) -> u32 {
    let mut tasks = Vec::new();

    let max_depth: usize;
    if lottery_tickets.len() - 2 < WINDOW_SIZE {
        max_depth = lottery_tickets.len() - 2
    } else {
        max_depth = WINDOW_SIZE;
    }
    for w in 1..max_depth {
        tasks.push(task::spawn(async move {
            let mut weighted_matches: f64 = 0.0;

            let windows: std::slice::Windows<LotteryTicket> =
                lottery_tickets[..lottery_tickets.len() - 1].windows(w);

            for window in windows {
                let predicted_numbers = algo(window, ticket_size);

                // finds next ticket to compare for accuracy
                #[allow(unused)]
                let mut next_ticket = &LotteryTicket {
                    date: chrono::NaiveDate::MIN,
                    numbers: Vec::new(),
                };
                let mut lottery_tickets_iter = lottery_tickets.iter();
                loop {
                    let ticket = lottery_tickets_iter.next().unwrap();
                    if ticket.date == window.last().unwrap().date {
                        next_ticket = lottery_tickets_iter.next().unwrap();
                        break;
                    }
                }

                // tally matching numbers and multiply weight - weight of recentness and weight of balls
                let numbers_hit = 0;
                let window_index = windows.position(|win| win == window).unwrap() as f64;
                let window_weight = 1.0 / window_index;
                println!("window_index: {window_index}"); // TODO: remvove once accuracy ensured

                for ticket in predicted_numbers.iter() {
                    for num in ticket.iter() {
                        let ball_weight = num.pow(2);
                        if next_ticket.numbers.contains(num) {
                            numbers_hit += ball_weight * window_weight;
                        }
                    }
                }
            }

            // divide ball total by number of windows for fair eval later
            weighted_matches = weighted_matches / windows.len() as f64;

            (w as u32, weighted_matches)
        }));
    }

    let best_depth = (0, 0.0);
    for task in tasks {
        let (window_size, weighted_matches) = task.await.unwrap();
        if weighted_matches > best_depth.1 {
            best_depth = (window_size, weighted_matches)
        }
    }

    // returnes the optimal depth
    best_depth.0
}
