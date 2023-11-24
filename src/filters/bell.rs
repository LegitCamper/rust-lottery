const HIGHEST_NUM: u32 = 32;

pub fn bell(algo_guesses: &mut Vec<Vec<u8>>, ticket_len: u8) {
    let num_grouping = HIGHEST_NUM / ticket_len as u32;

    algo_guesses.retain(|ticket| {
        let mut correct = 0;
        for (count, num) in ticket.iter().enumerate() {
            let cast_num = *num as u32;
            let upper_bound = num_grouping * (count as u32 + 1);
            let lower_bound = num_grouping * (count as u32);

            if count == ticket_len.into() {
                if cast_num > lower_bound && cast_num < HIGHEST_NUM {
                    correct += 1;
                }
            } else {
                if cast_num > lower_bound && cast_num < upper_bound {
                    correct += 1;
                }
            }
        }

        if correct == ticket_len - 1 {
            true
        } else {
            false
        }
    })
}
