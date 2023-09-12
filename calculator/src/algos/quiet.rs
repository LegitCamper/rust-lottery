use crate::LotteryTicket;
use std::collections::HashMap;

pub fn quiet(window: &[LotteryTicket], _length: u8) -> Vec<u8> {
    let mut quiet_stats: HashMap<u8, QuietStats> = HashMap::new();

    for ticket in window {
        for num in &ticket.numbers {
            let num_stats = quiet_stats.entry(*num).or_insert(QuietStats::new());

            num_stats.resets += 1;
            num_stats.average = num_stats.average + num_stats.miss_count; // this is wrong

            // need to find better way to count misses and increment them
            num_stats
                .quiet_count
                .entry(num_stats.miss_count.try_into().unwrap())
                .and_modify(|count| *count += 1)
                .or_insert(0);
        }

        // increment missed balls by 1
        let quiet_stats_keys = quiet_stats.clone();
        for num in quiet_stats_keys.keys() {
            if !ticket.numbers.contains(num) {
                quiet_stats
                    .entry(*num)
                    .and_modify(|count| count.miss_count += 1);
            }
        }
        // println!("\n{:?}\n", ticket);
    }

    println!("{quiet_stats:?}");

    Vec::new() // remove this
}

#[derive(Debug, Clone)]
struct QuietStats {
    resets: u32,
    average: u32,
    quiet_count: HashMap<u8, u32>,
    miss_count: u32,
}
impl QuietStats {
    fn new() -> Self {
        QuietStats {
            resets: 0,
            average: 0,
            quiet_count: HashMap::new(),
            miss_count: 0,
        }
    }
}
