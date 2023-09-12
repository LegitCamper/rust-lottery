#[cfg(test)]
mod algo_speed_test {
    use crate::{friends, optimize, quiet, read::data_keymap, LotteryNumbers, LotteryTicket};
    use std::sync::Arc;

    const MAX_DEPTH: usize = 1000;

    fn init() -> (LotteryNumbers, u8) {
        let numbers = data_keymap().expect("Failed to find/read data.xlsx");
        let numbers: Arc<[LotteryTicket]> = numbers[numbers.len() - MAX_DEPTH..numbers.len()]
            .to_vec()
            .into();

        let ticket_length = numbers[0].numbers.len() as u8;
        (numbers, ticket_length)
    }

    #[tokio::test]
    async fn test_friends() {
        let (numbers, ticket_length) = init();
        optimize(numbers.clone(), ticket_length, friends).await;
    }

    // #[tokio::test]
    // async fn test_quiet() {
    // let (numbers, ticket_length) = init();
    // optimize(numbers.clone(), ticket_length, quiet).await;
    // }
}
