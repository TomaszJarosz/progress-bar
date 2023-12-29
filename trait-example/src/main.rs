//https://www.youtube.com/watch?v=6cp10jVWNl4 Substrate Seminar - Traits and Generic Types

fn main() {
    println!("Hello, world!");
}

mod step1;

#[test]
fn test_step_1() {
    let mut balances = step1::BalanceModule::new();
    balances.set_balances(1, 100);
    balances.set_balances(2, 200);

    assert_eq!(balances.balance(1), 100);
    assert_eq!(balances.balance(2), 200);
}
