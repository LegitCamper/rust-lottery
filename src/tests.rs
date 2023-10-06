#[cfg(test)]
mod algo_speed_test {
    use crate::{
        friends as friends_algo, optimize, quiet as quiet_algo, read::data_keymap, LotteryTickets,
    };
    use std::boxed::Box;

    fn init() -> (LotteryTickets, u8) {
        let lottery_tickets: LotteryTickets = Box::leak(Box::new(data_keymap().unwrap()));

        let ticket_len = lottery_tickets[0].numbers.len() as u8;
        (lottery_tickets, ticket_len)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn friends() {
        let (lottery_tickets, ticket_len) = init();
        optimize(lottery_tickets.clone(), ticket_len, friends_algo).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn quiet() {
        let (lottery_tickets, ticket_len) = init();
        optimize(lottery_tickets.clone(), ticket_len, quiet_algo).await;
    }
}
