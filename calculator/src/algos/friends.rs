use crate::LotteryTicket;
use std::{collections::HashMap, slice::Windows};

// pub fn friends(numbers: &LotteryNumbers) -> Vec<i8> {}

pub fn friends<'a>(window: Windows<'_, LotteryTicket>, size: i32) -> Vec<i8> {
    let mut friend_counter: HashMap<Vec<i8>, i32> = HashMap::new();

    for ticket in window {
        for number1 in &ticket[0].numbers {
            for number2 in &ticket[0].numbers {
                friend_counter
                    .entry(vec![*number1, *number2])
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    let mut output_vec = Vec::new();
    let mut sort_vec = Vec::new();
    for (num, count) in friend_counter {
        sort_vec.push((num, count))
    }
    sort_vec.sort_by(|a, b| b.1.cmp(&a.1));

    for (num, _) in &sort_vec {
        if output_vec.len() >= size as usize + 1 {
            output_vec.truncate(size as usize + 1);
            break;
        } else {
            if num[0] == num[1] {
                output_vec.push(num[0]);
            } else {
                output_vec.push(num[0]);
                output_vec.push(num[1]);
            }
        }
    }
    output_vec
}
