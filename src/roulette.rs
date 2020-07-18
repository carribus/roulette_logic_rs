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
            RouletteBetType::Doubleline(v) => write!(f, "Doubleline({}, {}, {}, {}, {}, {})", v[0], v[1], v[2], v[3], v[4], v[5]),
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
        self.wager * match self.bet_type {
            RouletteBetType::Straight(_) => 36,
            RouletteBetType::Split(_) => 18,
            RouletteBetType::Street(_) => 12,
            RouletteBetType::Basket(_) => 12,
            RouletteBetType::Topline(_) => 9,
            RouletteBetType::Corner(_) => 9,
            RouletteBetType::Doubleline(_) => 6,
            RouletteBetType::Dozens(_) => 3,
            RouletteBetType::Columns(_) => 3,
            RouletteBetType::EvenOdd(_) => 2,
            RouletteBetType::Highlow(_) => 2,
            RouletteBetType::Redblack(_) => 2,

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

    pub fn bet(&self) -> &'a RouletteBet {
        self.bet
    }

    pub fn win(&self) -> u64 {
        self.win
    }
}

struct RouletteEvaluator;

impl RouletteEvaluator {
    pub fn calculate_winnings<'a>(winning_number: u8, colour: u8, bets: &'a Vec<RouletteBet>) -> Vec<RouletteBetResult<'a>> {
        let mut results = Vec::new();

        fn calc_win<'a, F>(bet: &'a RouletteBet, f: F) -> RouletteBetResult<'a> where F: FnOnce() -> bool {
            RouletteBetResult::new(bet, if f() {
                bet.win_value()
            } else {
                0
            })
        }

        for bet in bets {
            results.push(
                match bet.bet_type() {
                    RouletteBetType::Straight(v) => calc_win(bet, || v == winning_number),
                    RouletteBetType::Dozens(v) => {
                        calc_win(bet, || {
                            let start = (v-1)*12+1;
                            for n in start..(start+12) {
                                if winning_number == n {
                                    return true;
                                }
                            }
                            false
                        })
                    },
                    RouletteBetType::Columns(v) => {
                        calc_win(bet, || {
                            let mut n = v;
                            while n <= 36 {
                                if winning_number == n {
                                    return true
                                } 
                                n += 3;
                            }
                            false
                        })
                    },
                    RouletteBetType::EvenOdd(v) => calc_win(bet, || (winning_number % 2) == (v % 2)),
                    RouletteBetType::Highlow(v) => calc_win(bet, || {
                        (v == 0 && winning_number >= 1 && winning_number <= 18) ||
                        (v == 1 && winning_number >= 19 && winning_number <= 36)
                    }),
                    RouletteBetType::Redblack(v) => calc_win(bet, || v == colour),
                    RouletteBetType::Split(v) => calc_win(bet, || v.contains(&winning_number)),
                    RouletteBetType::Street(v) => calc_win(bet, || v.contains(&winning_number)),
                    RouletteBetType::Basket(v) => calc_win(bet, || v.contains(&winning_number)),
                    RouletteBetType::Topline(v) => calc_win(bet, || v.contains(&winning_number)),
                    RouletteBetType::Corner(v) => calc_win(bet, || v.contains(&winning_number)),
                    RouletteBetType::Doubleline(v) => calc_win(bet, || v.contains(&winning_number)),
                }
            )
        }

        results
    }
}

#[derive(Debug, Clone)]
pub struct Roulette {
    history: Vec<u8>,
    min_bet_size: u64,
    rng: ThreadRng,
}

impl Roulette {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            min_bet_size: 1,
            rng: thread_rng(),
        }
    }

    pub fn spin<'a>(&mut self, bets: &'a Vec<RouletteBet>) -> Result<(u8, Vec<RouletteBetResult<'a>>), Vec<PlaceBetError>> {
        self.validate_bets(bets)?;

        // spin
        let number = self.rng.gen_range(0, 36);
        // let number = 36;
        self.history.push(number);

        Ok((number, RouletteEvaluator::calculate_winnings(number, Self::get_number_colour(number), &bets)))
    }

    fn validate_bets(&self, bets: &Vec<RouletteBet>) -> Result<(), Vec<PlaceBetError>> {
        let mut errors = Vec::new();

        // check for errors
        for bet in bets {
            if !Self::validate_bet_option(bet.bet_type()) {
                errors.push(PlaceBetError::InvalidBetOption(bet.clone()))
            } else if !self.validate_bet_size(bet) {
                errors.push(PlaceBetError::MinBetNotSatisfied(bet.clone(), self.min_bet_size * Self::min_bet_for_option(bet.bet_type())))
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
            RouletteBetType::Straight(_) => 1,
            RouletteBetType::Split(_) => 1,
            RouletteBetType::Street(_) => 1,
            RouletteBetType::Basket(_) => 1,
            RouletteBetType::Topline(_) => 1,
            RouletteBetType::Corner(_) => 1,
            RouletteBetType::Doubleline(_) => 1,
            RouletteBetType::Dozens(_) => 1,
            RouletteBetType::Columns(_) => 1,
            RouletteBetType::EvenOdd(_) => 1,
            RouletteBetType::Highlow(_) => 1,
            RouletteBetType::Redblack(_) => 1,
        }
    }

    /// Checks that a ```RouletteBetType``` is valid and can be played
    /// *NOTE*: The logic expects the elements in a &[u8] array of values to be sorted in ascending order
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
                (v[0]-1) % 3 == 0 &&
                v[1] - v[0] == 1 &&
                v[2] - v[1] == 1
            }
            RouletteBetType::Basket(v) => {
                v[0] == 0 &&
                ((v[1] == 1 && v[2] == 2) ||
                (v[1] == 2 && v[2] == 3))
            }
            RouletteBetType::Topline(v) => {
                v[0] == 0 && v[1] == 1 && v[2] == 2 && v[3] == 3
            }
            RouletteBetType::Corner(v) => {
                v[0] > 0 &&
                (v[0] % 3 != 0) &&
                v[1] - v[0] == 1 &&
                v[3] - v[2] == 1 &&
                v[3] - v[1] == 3 &&
                v[2] - v[0] == 3
            }
            RouletteBetType::Doubleline(v) => {
                let mut slice1: [u8; 3] = Default::default();
                let mut slice2: [u8; 3] = Default::default();
                slice1.copy_from_slice(&v[0..=2]);
                slice2.copy_from_slice(&v[3..=5]);
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

    fn validate_bet_size(&self, bet: &RouletteBet) -> bool {
        Self::min_bet_for_option(bet.bet_type()) & self.min_bet_size <= bet.wager()
    }

    fn get_number_colour(number: u8) -> u8 {
        match number {
            1 | 3 | 5 | 7 | 9 | 12 | 14 | 16 | 18 | 19 | 21 | 23 | 25 | 27 | 30 | 32 | 34 | 36 => 0,
            _ => 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn roulettebet_win_value() {
        for i in 1..100 {
            let rbs = RouletteBet::new(RouletteBetType::Straight(1), i);
            let rbc = RouletteBet::new(RouletteBetType::Corner([2,3,5,6]), i);

            assert_eq!(rbs.win_value(), i*36);
            assert_eq!(rbc.win_value(), i*9)
        }
    }

    #[test]
    fn rouletteeval_calc_winnings() {
        let wager = 10;
        let bets = vec![
            RouletteBet::new(RouletteBetType::Straight(1), wager),
            RouletteBet::new(RouletteBetType::Split([1, 2]), wager),
            RouletteBet::new(RouletteBetType::Street([1, 2, 3]), wager),
            RouletteBet::new(RouletteBetType::Basket([0, 1, 2]), wager),
            RouletteBet::new(RouletteBetType::Topline([0, 1, 2, 3]), wager),
            RouletteBet::new(RouletteBetType::Corner([1, 2, 4, 5]), wager),
            RouletteBet::new(RouletteBetType::Doubleline([1, 2, 3, 4, 5, 6]), wager),
            RouletteBet::new(RouletteBetType::Dozens(1), wager),
            RouletteBet::new(RouletteBetType::Columns(1), wager),
            RouletteBet::new(RouletteBetType::EvenOdd(0), wager),
            RouletteBet::new(RouletteBetType::Highlow(0), wager),
            RouletteBet::new(RouletteBetType::Redblack(0), wager),
        ];

        let results = RouletteEvaluator::calculate_winnings(2, 0, &bets);
        let mut winnings = 0;

        for res in results {
            match res.bet().bet_type() {
                RouletteBetType::Straight(_) => assert_eq!(res.win(), 0),
                RouletteBetType::Split(_) => assert_eq!(res.win(), 180),
                RouletteBetType::Street(_) => assert_eq!(res.win(), 120),
                RouletteBetType::Basket(_) => assert_eq!(res.win(), 120),
                RouletteBetType::Topline(_) => assert_eq!(res.win(), 90),
                RouletteBetType::Corner(_) => assert_eq!(res.win(), 90),
                RouletteBetType::Doubleline(_) => assert_eq!(res.win(), 60),
                RouletteBetType::Dozens(_) => assert_eq!(res.win(), 30),
                RouletteBetType::Columns(_) => assert_eq!(res.win(), 0),
                RouletteBetType::EvenOdd(_) => assert_eq!(res.win(), 20),
                RouletteBetType::Highlow(_) => assert_eq!(res.win(), 20),
                RouletteBetType::Redblack(_) => assert_eq!(res.win(), 20),                
            }
            winnings += res.win();
        }

        assert_eq!(winnings, 750);
    }

    #[test]
    fn valid_bettype_straights() {
        for i in 0..37 {
            assert_eq!(Roulette::validate_bet_option(RouletteBetType::Straight(i)), true);
        }
    }

    #[test]
    fn invalid_bettype_straights() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Straight(37)), false);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Straight(129)), false);
    }

    #[test]
    fn valid_bettype_split_horizontal() {
        for i in 1..36 {
            if i % 3 != 0 {
                let bt = RouletteBetType::Split([i, i+1]);
                let res = Roulette::validate_bet_option(bt);
                if !res { println!("invalid bettype: {}", bt)}
                assert_eq!(res, true);
            }
        }
    }

    #[test]
    fn valid_bettype_split_vertical() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Split([0, 1])), true);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Split([0, 2])), true);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Split([0, 3])), true);

        for i in 1..36 {
            if i+3 <= 36 {
                let bt = RouletteBetType::Split([i, i+3]);
                let res = Roulette::validate_bet_option(bt);
                if !res { panic!("invalid bettype: {}", bt)}
                assert_eq!(res, true);
            }
        }
    }

    #[test]
    fn invalid_bettype_split() {
        // invalid duplicate split
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Split([1, 1])), false);

        // invalid 0 splits
        for i in 4..37 {
            let bt = RouletteBetType::Split([0, i]);
            let res = Roulette::validate_bet_option(bt);
            if res { panic!("Unexpected valid bettype: {}", bt)}
            assert_eq!(res, false);
        }

        // all other invalid splits
        for i in 1..37 {
            let bt = RouletteBetType::Split([i, i+2]);
            assert_eq!(Roulette::validate_bet_option(bt), false);
            
            for j in 4..37 {
                let bt = RouletteBetType::Split([i, i+j]);
                assert_eq!(Roulette::validate_bet_option(bt), false);
            }
        }
    }

    #[test]
    fn valid_bettype_street() {
        for i in 1..35 {
            if i%3 == 1 {
                assert_eq!(Roulette::validate_bet_option(RouletteBetType::Street([i, i+1, i+2])), true);
            }
        }
    }

    #[test]
    fn invalid_bettype_street() {
        for i in 1..35 {
            if i%3 != 1 {
                assert_eq!(Roulette::validate_bet_option(RouletteBetType::Street([i, i+1, i+2])), false);
            }
        }
    }

    #[test]
    fn valid_bettype_basket() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Basket([0, 1, 2])), true);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Basket([0, 2, 3])), true);
    }

    #[test]
    fn invalid_bettype_basket() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Basket([0, 1, 3])), false);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Basket([0, 1, 4])), false);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Basket([1, 2, 3])), false);

    }

    #[test]
    fn valid_bettype_topline() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Topline([0, 1, 2, 3])), true);
    }

    #[test]
    fn invalid_bettype_topline() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Topline([1, 2, 3, 4])), false);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Topline([0, 2, 3, 4])), false);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Topline([0, 2, 3, 5])), false);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Topline([0, 2, 3, 1])), false);
    }

    #[test]
    fn valid_bettype_corner() {
        for i in 1..33 {
            if i % 3 != 0 {
                assert_eq!(Roulette::validate_bet_option(RouletteBetType::Corner([i, i+1, i+3, i+4])), true);
            }
        }
    }

    #[test]
    fn invalid_bettype_corner() {
        for i in 1..33 {
            if i % 3 == 0 {
                assert_eq!(Roulette::validate_bet_option(RouletteBetType::Corner([i, i+1, i+3, i+4])), false);
            } else {
                assert_eq!(Roulette::validate_bet_option(RouletteBetType::Corner([i, i+1, i+2, i+3])), false);
            }
        }
    }

    #[test]
    fn valid_bettype_doubleline() {
        for i in 1..32 {
            if i % 3 == 1 {
                assert_eq!(Roulette::validate_bet_option(RouletteBetType::Doubleline([i, i+1, i+2, i+3, i+4, i+5])), true);
            }
        }
    }

    #[test]
    fn invalid_bettype_doubleline() {
        for i in 1..37 {
            if i % 3 != 1 {
                assert_eq!(Roulette::validate_bet_option(RouletteBetType::Doubleline([i, i+1, i+2, i+3, i+4, i+5])), false);
            }
        }
    }

    #[test]
    fn valid_bettype_dozens() {
        for i in 1..4 {
            assert_eq!(Roulette::validate_bet_option(RouletteBetType::Dozens(i)), true);
        }
    }

    #[test]
    fn invalid_bettype_dozens() {
        for i in 4..37 {
            assert_eq!(Roulette::validate_bet_option(RouletteBetType::Dozens(i)), false);
        }
    }

    #[test]
    fn valid_bettype_columns() {
        for i in 4..37 {
            assert_eq!(Roulette::validate_bet_option(RouletteBetType::Dozens(i)), false);
        }
    }

    #[test]
    fn invalid_bettype_columns() {
        for i in 4..37 {
            assert_eq!(Roulette::validate_bet_option(RouletteBetType::Dozens(i)), false);
        }
    }

    #[test]
    fn valid_bettype_oddeven() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::EvenOdd(0)), true);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::EvenOdd(1)), true);
    }

    #[test]
    fn invalid_bettype_oddeven() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::EvenOdd(2)), false);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::EvenOdd(3)), false);
    }

    #[test]
    fn valid_bettype_highlow() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Highlow(0)), true);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Highlow(1)), true);
    }

    #[test]
    fn invalid_bettype_highlow() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Highlow(2)), false);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Highlow(3)), false);
    }

    #[test]
    fn valid_bettype_redblack() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Redblack(0)), true);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Redblack(1)), true);
    }

    #[test]
    fn invalid_bettype_redblack() {
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Redblack(2)), false);
        assert_eq!(Roulette::validate_bet_option(RouletteBetType::Redblack(3)), false);
    }

    #[test]
    fn valid_bettype_voisons_du_zero() {
        let vdz = vec![
            RouletteBetType::Basket([0, 2, 3]),
            RouletteBetType::Split([4, 7]),
            RouletteBetType::Split([12, 15]),
            RouletteBetType::Split([18, 21]),
            RouletteBetType::Split([19, 22]),
            RouletteBetType::Split([32, 35]),
            RouletteBetType::Corner([25, 26, 28, 29]),
        ];

        for bet in vdz {
            assert_eq!(Roulette::validate_bet_option(bet), true);
        }
    }
}