use crate::LotteryTicket;
use std::collections::HashMap;

#[allow(unused)]
pub fn spine_sort(window: &[LotteryTicket], length: u8) -> Vec<u8> {
    let mut friend_counter: HashMap<[u8; 2], u32> = HashMap::new();

    for ticket in window {
        for number1 in &ticket.numbers {
            for number2 in &ticket.numbers {
                friend_counter
                    .entry([*number1, *number2])
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    let mut output_vec = Vec::new();
    let mut output: Vec<([u8; 2], u32)> = friend_counter
        .into_iter()
        .map(|(friends, count)| (friends, count))
        .collect::<Vec<([u8; 2], u32)>>();
    output.sort_by(|a, b| b.1.cmp(&a.1));

    for (nums, _) in output {
        if nums[0] != nums[1] {
            if output_vec.len() > length as usize {
                output_vec.truncate(length as usize + 1);
                break;
            } else if nums[0] != nums[1] {
                if !output_vec.contains(&nums[0]) {
                    output_vec.push(nums[0]);
                } else if !output_vec.contains(&nums[1]) {
                    output_vec.push(nums[1]);
                }
            }
        }
    }
    output_vec
}
