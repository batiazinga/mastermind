pub const SIZE: usize = 4;

#[derive(Clone, Copy, PartialEq)]
pub enum CodePeg {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Clone, Copy)]
pub struct Code {
    pegs: [CodePeg; SIZE],
}

impl Code {
    pub fn new(pegs: [CodePeg; SIZE]) -> Self {
        Code { pegs }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScorePeg {
    Match,
    Present,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Score {
    pegs: [Option<ScorePeg>; SIZE],
}

impl Score {
    fn new(pegs: [Option<ScorePeg>; SIZE]) -> Self {
        Score { pegs }
    }
}

pub struct Scorer {
    code: Code,
}

impl Scorer {
    pub fn new(code: Code) -> Self {
        Scorer { code }
    }

    pub fn score(&self, guess: Code) -> Score {
        let mut score_accumulator: Vec<ScorePeg> = Vec::with_capacity(SIZE);

        let mut score_peg_not_matched: Vec<CodePeg> = Vec::with_capacity(SIZE);
        let mut guess_peg_not_matched: Vec<CodePeg> = Vec::with_capacity(SIZE);

        for i in 0..SIZE {
            if self.code.pegs[i] == guess.pegs[i] {
                score_accumulator.push(ScorePeg::Match);
            } else {
                score_peg_not_matched.push(self.code.pegs[i]);
                guess_peg_not_matched.push(guess.pegs[i]);
            }
        }

        for peg in guess_peg_not_matched {
            let index = score_peg_not_matched.iter().position(|&item| item == peg);
            if let Some(i) = index {
                score_accumulator.push(ScorePeg::Present);
                score_peg_not_matched.remove(i);
            }
        }

        let mut score: [Option<ScorePeg>; SIZE] = [None; SIZE];
        for i in 0..score_accumulator.len() {
            score[i] = Some(score_accumulator[i])
        }
        Score::new(score)
    }
}

pub trait CodeMaker {
    fn make_code(&self) -> Code;
}

pub trait CodeBreaker {
    fn guess_code(&self) -> Code;
    fn set_score(&mut self, score: Score);
    fn loses(&mut self);
}

pub struct Game<'a, T: CodeMaker, U: CodeBreaker> {
    max_round: usize,
    code_maker: &'a T,
    code_breaker: &'a mut U,
}

impl<'a, T: CodeMaker, U: CodeBreaker> Game<'a, T, U> {
    pub fn new(max_round: usize, code_maker: &'a T, code_breaker: &'a mut U) -> Self {
        Game {
            max_round,
            code_maker,
            code_breaker,
        }
    }

    pub fn play(self) {
        let scorer = Scorer::new(self.code_maker.make_code());
        for _round in 0..self.max_round {
            let score = scorer.score(self.code_breaker.guess_code());
            self.code_breaker.set_score(score);
            if score == Score::new([Some(ScorePeg::Match); SIZE]) {
                return;
            }
        }
        self.code_breaker.loses();
    }
}

#[cfg(test)]
mod test_scorer {
    use super::*;

    struct TestCase<'a> {
        name: &'a str,
        code: Code,
        guess: Code,
        score: Score,
    }

    #[test]
    fn score() {
        let test_cases = vec![
            TestCase {
                name: "fully_wrong",
                code: Code::new([CodePeg::A, CodePeg::B, CodePeg::C, CodePeg::D]),
                guess: Code::new([CodePeg::E, CodePeg::E, CodePeg::F, CodePeg::F]),
                score: Score::new([None, None, None, None]),
            },
            TestCase {
                name: "success",
                code: Code::new([CodePeg::A, CodePeg::B, CodePeg::C, CodePeg::D]),
                guess: Code::new([CodePeg::A, CodePeg::B, CodePeg::C, CodePeg::D]),
                score: Score::new([
                    Some(ScorePeg::Match),
                    Some(ScorePeg::Match),
                    Some(ScorePeg::Match),
                    Some(ScorePeg::Match),
                ]),
            },
            TestCase {
                name: "all_present_with_wrong_positions",
                code: Code::new([CodePeg::A, CodePeg::B, CodePeg::C, CodePeg::D]),
                guess: Code::new([CodePeg::D, CodePeg::C, CodePeg::B, CodePeg::A]),
                score: Score::new([
                    Some(ScorePeg::Present),
                    Some(ScorePeg::Present),
                    Some(ScorePeg::Present),
                    Some(ScorePeg::Present),
                ]),
            },
            TestCase {
                name: "two_matches",
                code: Code::new([CodePeg::C, CodePeg::C, CodePeg::A, CodePeg::F]),
                guess: Code::new([CodePeg::C, CodePeg::D, CodePeg::D, CodePeg::F]),
                score: Score::new([Some(ScorePeg::Match), Some(ScorePeg::Match), None, None]),
            },
            TestCase {
                name: "match_and_present",
                code: Code::new([CodePeg::A, CodePeg::C, CodePeg::E, CodePeg::F]),
                guess: Code::new([CodePeg::C, CodePeg::D, CodePeg::D, CodePeg::F]),
                score: Score::new([Some(ScorePeg::Match), Some(ScorePeg::Present), None, None]),
            },
            TestCase {
                name: "count_match_only_once",
                code: Code::new([CodePeg::A, CodePeg::B, CodePeg::E, CodePeg::F]),
                guess: Code::new([CodePeg::A, CodePeg::A, CodePeg::D, CodePeg::D]),
                score: Score::new([Some(ScorePeg::Match), None, None, None]),
            },
        ];

        for test_case in test_cases {
            let scorer = Scorer::new(test_case.code);
            let score = scorer.score(test_case.guess);
            assert_eq!(score, test_case.score, "test case{}", test_case.name,);
        }
    }
}

#[cfg(test)]
mod test_game {
    use super::*;

    struct DeterministicCodeMaker {
        code: Code,
    }

    impl DeterministicCodeMaker {
        fn new(code: Code) -> Self {
            DeterministicCodeMaker { code }
        }
    }

    impl CodeMaker for DeterministicCodeMaker {
        fn make_code(&self) -> Code {
            self.code
        }
    }

    struct DummyCodeBreaker {
        code: Code,
        has_won: bool,
        has_lost: bool,
        num_rounds: usize,
    }

    impl DummyCodeBreaker {
        fn new(code: Code) -> Self {
            DummyCodeBreaker {
                code,
                has_won: false,
                has_lost: false,
                num_rounds: 0,
            }
        }
    }

    impl CodeBreaker for DummyCodeBreaker {
        fn guess_code(&self) -> Code {
            self.code
        }

        fn set_score(&mut self, score: Score) {
            self.num_rounds += 1;
            if score != Score::new([Some(ScorePeg::Match); SIZE]) {
                return;
            }
            self.has_won = true;
        }

        fn loses(&mut self) {
            self.has_lost = true;
        }
    }

    #[test]
    fn wins_at_first_guess() {
        let code = Code::new([CodePeg::B, CodePeg::B, CodePeg::A, CodePeg::E]);
        let code_maker = DeterministicCodeMaker::new(code);
        let mut code_breaker = DummyCodeBreaker::new(code);
        let game = Game::new(3, &code_maker, &mut code_breaker);
        game.play();
        assert!(code_breaker.has_won);
        assert!(!code_breaker.has_lost);
        assert_eq!(code_breaker.num_rounds, 1);
    }

    #[test]
    fn loses() {
        let num_round = 8;
        let code_maker = DeterministicCodeMaker::new(Code::new([
            CodePeg::A,
            CodePeg::E,
            CodePeg::F,
            CodePeg::C,
        ]));
        let mut code_breaker =
            DummyCodeBreaker::new(Code::new([CodePeg::B, CodePeg::B, CodePeg::F, CodePeg::D]));
        let game = Game::new(num_round, &code_maker, &mut code_breaker);
        game.play();
        assert!(code_breaker.has_lost);
        assert!(!code_breaker.has_won);
        assert_eq!(code_breaker.num_rounds, num_round);
    }
}
