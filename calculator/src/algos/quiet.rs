use crate::LotteryTicket;
use indexmap::map::IndexMap;
use std::collections::HashMap;

pub fn quiet(window: &[LotteryTicket], _length: u8) -> Vec<u8> {
    let mut quiet_stats: IndexMap<u8, QuietStats> = IndexMap::new();

    for ticket in window {
        for num in &ticket.numbers {
            let num_stats = quiet_stats.entry(*num).or_insert(QuietStats::new());

            num_stats.resets += 1;
            num_stats.average += num_stats.miss_count; // I this is wrong

            num_stats
                .quiet_count
                .entry(num_stats.quiet_counter.try_into().unwrap())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            num_stats.quiet_counter = 0;
        }

        // increment missed balls by 1
        let quiet_stats_keys = quiet_stats.clone();
        for num in quiet_stats_keys.keys() {
            if !ticket.numbers.contains(num) {
                quiet_stats
                    .entry(*num)
                    .and_modify(|count| count.miss_count += 1)
                    .and_modify(|count| count.quiet_counter += 1);
            }
        }
    }

    // Do some kind of prediction based
    // on the likelyhood of a ball hitting
    // after a specific quiet period

    // Going to sort map and start guessing based
    // highest resets - most common ball

    quiet_stats.sort_by(|_ak, av, _bk, bv| bv.resets.cmp(&av.resets));
    println!("{:?}", quiet_stats);

    Vec::new() // remove this
}

#[derive(Debug, Clone)]
struct QuietStats {
    resets: u32,
    average: u32,
    quiet_count: HashMap<u8, u32>,
    quiet_counter: u32,
    miss_count: u32,
}
impl QuietStats {
    fn new() -> Self {
        QuietStats {
            resets: 0,
            average: 0,
            quiet_count: HashMap::new(),
            quiet_counter: 0,
            miss_count: 0,
        }
    }
}
