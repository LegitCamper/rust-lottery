use crate::{
    algos::friends,
    read::{data_keymap, LotteryTicket},
};
mod algos;
mod read;

type LotteryNumbers = Vec<LotteryTicket>;

fn window<'a>(numbers: &'a LotteryNumbers, size: i32) -> Vec<&'a LotteryTicket> {
    let mut window = Vec::new();
    let mut numbers_iter = numbers.iter();
    for _ in 0..size {
        // add line here like numbers_iter - some to start in the middle of the vec
        window.push(numbers_iter.next().unwrap());
    }
    window
}

fn main() {
    let numbers = data_keymap().unwrap();

    println!("{:?}", friends::friends(numbers));
}
