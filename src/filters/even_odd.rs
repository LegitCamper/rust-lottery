pub fn even_odd(algo_guesses: &mut Vec<Vec<u8>>, _ticket_len: u8) {
    algo_guesses.retain(|ticket| {
        let mut even = 0;
        let mut odd = 0;

        for num in ticket.iter() {
            if num % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }

        if even > 3 || odd > 3 {
            false
        } else {
            true
        }
    });
}
