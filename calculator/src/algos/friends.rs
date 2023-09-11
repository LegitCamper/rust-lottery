use crate::LotteryTicket;
use std::collections::HashMap;

pub fn friends<'a>(window: &[LotteryTicket], size: u8) -> Vec<u8> {
    let mut friend_counter: HashMap<Vec<u8>, u32> = HashMap::new();

    for ticket in window {
        for number1 in &ticket.numbers {
            for number2 in &ticket.numbers {
                friend_counter
                    .entry(vec![*number1, *number2])
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    let mut output_vec = Vec::new();
    let mut sort_vec = Vec::new();
    for (friends, count) in friend_counter {
        sort_vec.push((friends, count))
    }
    sort_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let mut used_vecs: Vec<[u8; 2]> = Vec::new();
    for (num, _) in &sort_vec {
        if num[0] != num[1] {
            if output_vec.len() >= size as usize + 1 {
                output_vec.truncate(size as usize + 1);
                break;
            } else {
                if num[0] != num[1] {
                    if !output_vec.contains(&num[0]) {
                        output_vec.push(num[0]);
                    }
                    if !output_vec.contains(&num[1]) {
                        output_vec.push(num[1]);
                    }
                }
            }
        }
    }
    output_vec
}
