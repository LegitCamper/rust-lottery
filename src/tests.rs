#[cfg(test)]
mod algo_speed_test {
    use crate::{
        optimize,
        read::{data_keymap, LotteryTicket},
        LotteryTickets,
    };
    use std::boxed::Box;

    const MAX_HISTORY: usize = 1000;

    async fn test(algo: fn(&[LotteryTicket], u8) -> Vec<u8>) {
        let tickets = data_keymap().unwrap();
        let tickets_trimmed = tickets[tickets.len() - MAX_HISTORY..].to_vec();
        let lottery_tickets: LotteryTickets = Box::leak(Box::new(tickets_trimmed));
        let ticket_len = lottery_tickets[0].numbers.len() as u8;

        // not using any filters
        optimize(lottery_tickets, ticket_len, algo, vec![]).await
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
