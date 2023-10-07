#[cfg(test)]
mod algo_speed_test {
    use crate::{
        optimize,
        read::{data_keymap, LotteryTicket},
        LotteryTickets,
    };
    use std::boxed::Box;

    async fn test(algo: fn(&[LotteryTicket], u8) -> Vec<u8>) {
        let mut tickets = data_keymap().unwrap();
        let lottery_tickets: LotteryTickets = Box::leak(Box::new(tickets));
        let ticket_len = lottery_tickets[0].numbers.len() as u8;

        let optimal_history = optimize(lottery_tickets, ticket_len, algo).await;

        panic!("The optimal history of days is {optimal_history}")
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn friends() {
        test(crate::friends).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn quiet() {
        test(crate::quiet).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn multiply() {
        test(crate::multiply).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn spine_sort() {
        test(crate::spine_sort).await;
    }
}
