use crate::LotteryTicket;
// use indexmap::map::IndexMap;
// use std::collections::HashMap;

pub fn multiply(window: &[LotteryTicket], _length: u8) -> Vec<u8> {
    // let mut quiet_stats: HashMap<u8, u8> = HashMap::new();

    let mut stuff: Vec<usize> = Vec::new();

    for (i, ticket) in window.iter().enumerate() {
        let mut temp_num: usize = 0;
        for num in &ticket.numbers {
            temp_num += *num as usize * i as usize;
        }

        stuff.push(temp_num);
    }

    for num in stuff {
        println!("{num:?}");
    }

    Vec::new() // remove this
}
