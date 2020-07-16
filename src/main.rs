use std::fmt;
use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum PlaceBetError {
    InvalidBetOption(RouletteBet),
    MaxBetOnOption(RouletteBet, u64),
    MinBetNotSatisfied(RouletteBet, u64),
}

impl fmt::Display for PlaceBetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlaceBetError::InvalidBetOption(option) => write!(f, "Invalid Bet Option: {}", option),
            PlaceBetError::MaxBetOnOption(option, max) => write!(f, "Max bet of {} reached on option {}", max, option),
            PlaceBetError::MinBetNotSatisfied(option, min) => write!(f, "Minimum ({}) not met for option {}", min, option),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RouletteBetType {
    /// Single number for the bet
    Straight(u8),

    /// Numbers from 2 adjacent spots
    Split([u8; 2]),

    /// Numbers for the row chosen
    Street([u8; 3]),

    /// Numbers covering either 0,1,2 or 0,2,3
    Basket([u8; 3]),

    /// Numbers covering 0, 1, 2, 3
    Topline([u8; 4]),

    /// Number of 4 adjacent spots
    Corner([u8; 4]),

    /// Numbers covering 2 adjacent lines 
    Doubleline([u8; 6]),

    /// 1 for 1-12, 2 for 13-24, 3 for 25-36
    Dozens(u8),

    /// Indicate the column based on the lowest number in that column (1, 2 or 3)
    Columns(u8), 

    /// 0 for even, 1 for odd
    EvenOdd(u8),

    /// 0 for 1-18, 1 for 19-36
    Highlow(u8),

    /// 0 for red, 1 for black
    Redblack(u8),
}

impl fmt::Display for RouletteBetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RouletteBetType::Straight(v) => write!(f, "Straight({})", v),
            RouletteBetType::Split(v) => write!(f, "Split({}, {})", v[0], v[1]),
            RouletteBetType::Street(v) => write!(f, "Street({}, {}, {})", v[0], v[1], v[2]),
            RouletteBetType::Basket(v) => write!(f, "Basket({}, {}, {})", v[0], v[1], v[2]),
            RouletteBetType::Topline(v) => write!(f, "Topline({}, {}, {}, {})", v[0], v[1], v[2], v[3]),
            RouletteBetType::Corner(v) => write!(f, "Corner({}, {}, {}, {})", v[0], v[1], v[2], v[3]),
            RouletteBetType::Doubleline(v) => write!(f, "Doubleline({}, {}, {}, {}, {}, {}", v[0], v[1], v[2], v[3], v[4], v[5]),
            RouletteBetType::Dozens(v) => write!(f, "Dozens({})", v),
            RouletteBetType::Columns(v) => write!(f, "Columns({})", v),
            RouletteBetType::EvenOdd(v) => write!(f, "EvenOdd({})", match v {
                0 => "even",
                1 => "odd",
                _ => "INVALID",
            }),
            RouletteBetType::Highlow(v) => write!(f, "Highlow({})", match v {
                0 => "1-18",
                1 => "19-36",
                _ => "INVALID",
            }),
            RouletteBetType::Redblack(v) => write!(f, "Redblack({})", match v {
                0 => "red",
                1 => "black",
                _ => "INVALID",
            })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RouletteBet {
    bet_type: RouletteBetType,
    wager: u64,
}

impl fmt::Display for RouletteBet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type: {}, wager: {}", self.bet_type, self.wager)
    }
}

impl RouletteBet {
    pub fn new(bet_type: RouletteBetType, wager: u64) -> Self {
        Self {
            bet_type,
            wager,
        }
    }

    pub fn win_value(&self) -> u64 {
        match self.bet_type {
            RouletteBetType::Straight(_) => self.wager * 36,
            RouletteBetType::Split(_) => self.wager * 18,
            RouletteBetType::Street(_) => self.wager * 12,
            RouletteBetType::Basket(_) => self.wager * 12,
            RouletteBetType::Topline(_) => self.wager * 9,
            RouletteBetType::Corner(_) => self.wager * 9,
            RouletteBetType::Doubleline(_) => self.wager * 6,
            RouletteBetType::Dozens(_) => self.wager * 3,
            RouletteBetType::Columns(_) => self.wager * 3,
            RouletteBetType::EvenOdd(_) => self.wager * 2,
            RouletteBetType::Highlow(_) => self.wager * 2,
            RouletteBetType::Redblack(_) => self.wager * 2,
        }
    }

    pub fn bet_type(&self) -> RouletteBetType {
        self.bet_type
    }

    pub fn wager(&self) -> u64 {
        self.wager
    }
}

pub struct RouletteBetResult<'a> {
    bet: &'a RouletteBet,
    win: u64,
}

impl<'a> RouletteBetResult<'a> {
    pub fn new(bet: &'a RouletteBet, win: u64) -> Self {
        Self {
            bet,
            win
        }
    }
}

struct RouletteEvaluator;

impl RouletteEvaluator {
    pub fn calculate_winnings<'a>(winning_number: u8, colour: u8, bets: &'a Vec<RouletteBet>) -> Vec<RouletteBetResult<'a>> {
        let mut results = Vec::new();
        let calc_win = |bet, v: &[u8]| {
            RouletteBetResult::new(bet, if v.contains(&winning_number) {
                bet.win_value()
            } else {
                0
            })
        };

        for bet in bets {
            results.push(
                match bet.bet_type() {
                    RouletteBetType::Straight(v) => {
                        RouletteBetResult::new(bet, if v == winning_number {
                            bet.win_value()
                        } else {
                            0
                        })
                    },
                    RouletteBetType::Dozens(v) => {
                        let mut result = RouletteBetResult::new(bet, 0);
                        let start = (v-1)*12+1;
                        for n in start..(start+12) {
                            if v == n {
                                result = RouletteBetResult::new(bet, bet.win_value());
                                break;
                            }
                        }
                        result
                    },
                    RouletteBetType::Columns(v) => {
                        let mut result = RouletteBetResult::new(bet, 0);
                        let mut n = v;
                        while n <= 36 {
                            if winning_number == n {
                                result = RouletteBetResult::new(bet, bet.win_value());
                                break;
                            } 
                            n += 3;
                        }
                        result
                    },
                    RouletteBetType::EvenOdd(v) => {
                        RouletteBetResult::new(bet, if (winning_number % 2) == (v % 2) {
                            bet.win_value()
                        } else {
                            0
                        })
                    },
                    RouletteBetType::Highlow(v) => {
                        RouletteBetResult::new(bet, if v == 0 && winning_number >= 1 && winning_number <= 18 {
                            bet.win_value()
                        } else if v == 1 && winning_number >= 19 && winning_number <= 36 {
                            bet.win_value()
                        } else {
                            0
                        })
                    },
                    RouletteBetType::Redblack(v) => {
                        RouletteBetResult::new(bet, if v == colour {
                            bet.win_value()
                        } else {
                            0
                        })
                    },
                    RouletteBetType::Split(v) => calc_win(bet, &v),
                    RouletteBetType::Street(v) => calc_win(bet, &v),
                    RouletteBetType::Basket(v) => calc_win(bet, &v),
                    RouletteBetType::Topline(v) => calc_win(bet, &v),
                    RouletteBetType::Corner(v) => calc_win(bet, &v),
                    RouletteBetType::Doubleline(v) => calc_win(bet, &v),
                }
            )
        }

        results
    }
}

#[derive(Debug, Clone)]
pub struct Roulette {
    history: Vec<u8>,
    rng: ThreadRng,
}

impl Roulette {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            rng: thread_rng(),
        }
    }

    pub fn spin<'a>(&mut self, bets: &'a Vec<RouletteBet>) -> Result<(u8, Vec<RouletteBetResult<'a>>), Vec<PlaceBetError>> {
        self.validate_bets(bets)?;

        // spin
        // let number = random::<u8>() % 37;
        let number = self.rng.gen_range(0, 36);
        self.history.push(number);

        Ok((number, RouletteEvaluator::calculate_winnings(number, Self::get_number_colour(number), &bets)))
    }

    fn validate_bets(&self, bets: &Vec<RouletteBet>) -> Result<(), Vec<PlaceBetError>> {
        let mut errors = Vec::new();

        // check for errors
        for bet in bets {
            if !Self::validate_bet_option(bet.bet_type()) {
                errors.push(PlaceBetError::InvalidBetOption(bet.clone()))
            } else if !Self::validate_bet_size(bet) {
                errors.push(PlaceBetError::MinBetNotSatisfied(bet.clone(), Self::min_bet_for_option(bet.bet_type())))
            }
        }

        if errors.len() == 0 {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn min_bet_for_option(bet_type: RouletteBetType) -> u64 {
        match bet_type {
            RouletteBetType::Straight(_v) => 1,
            RouletteBetType::Split(_v) => 1,
            RouletteBetType::Street(_v) => 1,
            RouletteBetType::Basket(_v) => 1,
            RouletteBetType::Topline(_v) => 1,
            RouletteBetType::Corner(_v) => 1,
            RouletteBetType::Doubleline(_v) => 1,
            RouletteBetType::Dozens(_v) => 1,
            RouletteBetType::Columns(_v) => 1,
            RouletteBetType::EvenOdd(_v) => 1,
            RouletteBetType::Highlow(_v) => 1,
            RouletteBetType::Redblack(_v) => 1,
        }
    }

    fn validate_bet_option(bet_type: RouletteBetType) -> bool {
        match bet_type {
            RouletteBetType::Straight(v) => v <= 36,
            RouletteBetType::Split(v) => {
                // range and duplicate check
                (v[0] != v[1] && (v[0] <= 35 && v[1] <= 36) && v[1] > v[0]) 
                &&
                // splits with zero
                (
                    v[0] == 0 && (v[1] == 1 || v[1] == 2 || v[1] == 3)
                ) 
                ||
                // numbers 1 to 33
                ((v[0] > 0 && v[0] <= 33) && 
                    (
                        // right edge
                        (v[1] % 3 == 0 && v[1] - v[0] == 1 || v[1] - v[0] == 3) ||
                        // left edge
                        (v[0] % 3 == 1 && v[1] - v[0] == 1 || v[1] - v[0] == 3)
                    )
                ) 
                ||
                // bottom edge (34, 35, 36)
                (v[0] >= 34 && v[1] - v[0] == 1)
            }
            RouletteBetType::Street(v) => {
                v[0] > 0 && 
                v[0] <= 34 && 
                v[0] % 3 == 0 &&
                v[1] - v[0] == 1 &&
                v[2] - v[1] == 1
            }
            RouletteBetType::Basket(v) => {
                v[0] == 0 &&
                (v[1] == 1 && v[2] == 2) ||
                (v[1] == 2 && v[2] == 3)
            }
            RouletteBetType::Topline(v) => {
                v[0] == 0 && v[1] == 1 && v[2] == 2 && v[3] == 3
            }
            RouletteBetType::Corner(v) => {
                v[0] > 0 &&
                v[1] - v[0] == 1 &&
                v[3] - v[2] == 1 &&
                v[3] - v[1] == 3 &&
                v[2] - v[0] == 3
            }
            RouletteBetType::Doubleline(v) => {
                let mut slice1: [u8; 3] = Default::default();
                let mut slice2: [u8; 3] = Default::default();
                slice1.copy_from_slice(&v[0..2]);
                slice2.copy_from_slice(&v[3..5]);
                Self::validate_bet_option(RouletteBetType::Street(slice1)) &&
                Self::validate_bet_option(RouletteBetType::Street(slice2))
            },
            RouletteBetType::Dozens(v) => v >= 1 && v <= 3,
            RouletteBetType::Columns(v) => v >= 1 && v <= 3,
            RouletteBetType::EvenOdd(v) => v <= 1,
            RouletteBetType::Highlow(v) => v <= 1,
            RouletteBetType::Redblack(v) => v <= 1,
        }
    }

    fn validate_bet_size(bet: &RouletteBet) -> bool {
        true
    }

    fn get_number_colour(number: u8) -> u8 {
        match number {
            1 | 3 | 5 | 7 | 9 | 12 | 14 | 16 | 18 | 19 | 21 | 23 | 25 | 27 | 30 | 32 | 34 | 36 => 0,
            _ => 1,
        }
    }
}

fn calculate_total_bet(bets: &Vec<RouletteBet>) -> u64 {
    bets.iter().fold(0, |acc, bet| acc + bet.wager())
}

fn main() {
    let mut balance = 10000;
    let mut r = Roulette::new();
    let bets = vec![
        RouletteBet::new(RouletteBetType::Straight(11), 100),
        RouletteBet::new(RouletteBetType::Straight(10), 100),
        RouletteBet::new(RouletteBetType::Corner([7, 8, 10, 11]), 100),
        RouletteBet::new(RouletteBetType::Corner([8, 9, 11, 12]), 100),
        RouletteBet::new(RouletteBetType::Corner([10, 11, 13, 14]), 100),
        RouletteBet::new(RouletteBetType::Corner([11, 12, 14, 15]), 100),
        RouletteBet::new(RouletteBetType::Columns(2), 300),
        RouletteBet::new(RouletteBetType::Basket([0, 1, 2]), 100),
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

        match r.spin(&bets) {
            Ok(results) => {
                println!("Ball dropped on {}", results.0);
                for (ndx, result) in results.1.iter().enumerate() {
                    println!("Bet {}: {} wins {}", ndx, result.bet, result.win);
                    balance += result.win;
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
