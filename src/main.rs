mod roulette;

use crate::roulette::*;

fn calculate_total_bet(bets: &Vec<RouletteBet>) -> u64 {
    bets.iter().fold(0, |acc, bet| acc + bet.wager())
}

fn main() {
    let mut balance = 10000;
    let mut r = Roulette::new();
    let bets = vec![
        RouletteBet::new(RouletteBetType::Straight(11), 100),
        RouletteBet::new(RouletteBetType::Split([10, 11]), 100),
        RouletteBet::new(RouletteBetType::Corner([7, 8, 10, 11]), 100),
        RouletteBet::new(RouletteBetType::Corner([8, 9, 11, 12]), 100),
        RouletteBet::new(RouletteBetType::Corner([10, 11, 13, 14]), 100),
        RouletteBet::new(RouletteBetType::Corner([11, 12, 14, 15]), 100),
        RouletteBet::new(RouletteBetType::Columns(2), 300),
        RouletteBet::new(RouletteBetType::Basket([0, 1, 2]), 100),
        RouletteBet::new(RouletteBetType::Dozens(1), 100),
        RouletteBet::new(RouletteBetType::EvenOdd(0), 100),
        RouletteBet::new(RouletteBetType::Highlow(1), 100),
        RouletteBet::new(RouletteBetType::Redblack(1), 100),
        RouletteBet::new(RouletteBetType::Doubleline([25, 26, 27, 28, 29, 30]), 100),
    ];

    let mut counter = 1;
    let mut highest_balance = balance;
    loop {
        println!("\nGame {}", counter);
        let total_bet = calculate_total_bet(&bets);
        if total_bet > balance {
            println!("Not enough balance to place the bet(s)! (balance: {}, bets: {})", balance, total_bet);
            break;
        } else {
            balance -= total_bet;
            println!("Bets placed. Balance = {}", balance);
        }

        let now = std::time::SystemTime::now();
        let results = r.spin(&bets);
        println!("Game took {}ns to run", now.elapsed().unwrap().as_nanos());

        match results {
            Ok(results) => {
                println!("Ball dropped on {}", results.0);
                for (ndx, result) in results.1.iter().enumerate() {
                    println!("Bet {}: {} wins {}", ndx, result.bet(), result.win());
                    balance += result.win();
                }
            },
            Err(errors) => {
                println!("Errors found:");
                for error in errors {
                    println!("- {}", error);
                }
            }
        }

        if balance > highest_balance {
            highest_balance = balance;
        }
        counter += 1;
    }

    println!("Highest balance achieved = {}", highest_balance);
}
