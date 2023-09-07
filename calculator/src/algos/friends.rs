use super::Tickets;
use crate::{window, LotteryNumbers, LotteryTicket};
use itertools::Itertools;
use std::collections::HashMap;

pub fn friends(numbers: LotteryNumbers) -> Tickets {
    let mut results: HashMap<i32, i32> = HashMap::new();
    for w in 1..1000 {
        // finds the best window size that gives the best results
        let window = window(&numbers, w);
        // let window = numbers.windows(w); // TODO: this is way better
        let tickets = number_friends(window);
        let next_ticket = &numbers[w as usize].numbers;

        let mut matching_numbers = 0;
        for ticket in tickets.iter().combinations(numbers[0].numbers.len()) {
            matching_numbers += ticket
                .iter()
                .zip(next_ticket)
                .filter(|&(a, b)| a == &b)
                .count();
        }
        results.insert(w, matching_numbers.try_into().unwrap());
    }

    let mut most_numbers = 0;
    for item in results {
        if item.1 > most_numbers {
            most_numbers = item.1
        }
    }

    let window = window(&numbers, most_numbers.try_into().unwrap());
    number_friends(window)
        .into_iter()
        .combinations(numbers[0].numbers.len())
        .collect::<Tickets>() // next real ticket numbers
}

pub fn number_friends<'a>(tickets: Vec<&'a LotteryTicket>) -> Vec<i8> {
    let mut friend_counter: HashMap<Vec<i8>, i32> = HashMap::new();

    for ticket in &tickets {
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
    for (num, count) in friend_counter {
        sort_vec.push((num, count))
    }
    sort_vec.sort_by(|a, b| b.1.cmp(&a.1));

    for (num, _) in &sort_vec {
        if output_vec.len() >= tickets[0].numbers.len() + 1 {
            output_vec.truncate(tickets[0].numbers.len() + 1);
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
