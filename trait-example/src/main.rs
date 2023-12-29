//https://www.youtube.com/watch?v=6cp10jVWNl4 Substrate Seminar - Traits and Generic Types

use std::hash::Hash;
use num::{CheckedAdd, CheckedSub, Zero};

fn main() {
    println!("Hello, world!");
}

mod step1;
mod step2;
mod step3;
mod step4;
mod step5;

#[test]
fn test_step_1() {
    let mut balances = step1::BalanceModule::new();
    balances.set_balances(1, 100);
    balances.set_balances(2, 200);

    assert_eq!(balances.balance(1), 100);
    assert_eq!(balances.balance(2), 200);
    assert_eq!(balances.balance(3), 0);

    assert!(balances.transfer(1, 2, 50).is_ok());
    assert_eq!(balances.balance(1), 50);
    assert_eq!(balances.balance(2), 250);

    assert!(balances.transfer(1, 2, 100).is_err());
    assert_eq!(balances.balance(1), 50);
    assert_eq!(balances.balance(2), 250);
}

#[test]
fn test_step_2() {
    let mut balances = step2::BalanceModule::new();
    balances.set_balances(1, 100);
    balances.set_balances(2, 200);

    assert_eq!(balances.balance(1), 100);
    assert_eq!(balances.balance(2), 200);
    assert_eq!(balances.balance(3), 0);

    assert!(balances.transfer(1, 2, 50).is_ok());
    assert_eq!(balances.balance(1), 50);
    assert_eq!(balances.balance(2), 250);

    assert!(balances.transfer(1, 2, 100).is_err());
    assert_eq!(balances.balance(1), 50);
    assert_eq!(balances.balance(2), 250);
}

#[test]
fn test_step_4() {
    type AccountId = u16;
    type VoteIndex = u64;
    let user_1 = 1;
    let user_2 = 2;
    let mut balances = step4::BalanceModule::<AccountId, VoteIndex>::new();
    let mut voting = step4::VotingModule::<AccountId, VoteIndex>::new();

    balances.set_balances(user_1, 100);
    balances.set_balances(user_2, 200);

    voting.vote(user_1, 0, true);
    voting.vote(user_2, 0, true);
}

trait Trait {
    type AccountId: Eq + Hash;
    type Balance: Zero + CheckedAdd + CheckedSub + Copy;
    type VoteIndex: Eq + Hash;
}

#[test]
fn test_step_5() {
    struct Runtime;
    impl Trait for Runtime {
        type AccountId = u32;
        type Balance = u32;
        type VoteIndex = u8;
    }
    let user_1 = 1;
    let user_2 = 2;
    let mut balances = step5::BalanceModule::<Runtime>::new();
    let mut voting = step5::VotingModule::<Runtime>::new();

    balances.set_balances(user_1, 100);
    balances.set_balances(user_2, 200);

    voting.vote(user_1, 0, true);
    voting.vote(user_2, 0, true);
}

