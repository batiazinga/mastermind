const SIZE: usize = 4;

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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ScorePeg {
    Black,
    White,
}

#[derive(PartialEq, Debug)]
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

    pub fn score(self, guess: Code) -> Score {
        let mut score_accumulator: Vec<ScorePeg> = Vec::with_capacity(SIZE);

        let mut score_peg_not_matched: Vec<CodePeg> = Vec::with_capacity(SIZE);
        let mut guess_peg_not_matched: Vec<CodePeg> = Vec::with_capacity(SIZE);

        for i in 0..SIZE {
            if self.code.pegs[i] == guess.pegs[i] {
                score_accumulator.push(ScorePeg::Black);
            } else {
                score_peg_not_matched.push(self.code.pegs[i]);
                guess_peg_not_matched.push(guess.pegs[i]);
            }
        }

        for peg in guess_peg_not_matched {
            let index = score_peg_not_matched.iter().position(|&item| item == peg);
            if let Some(i) = index {
                score_accumulator.push(ScorePeg::White);
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

#[cfg(test)]
mod tests {
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
                    Some(ScorePeg::Black),
                    Some(ScorePeg::Black),
                    Some(ScorePeg::Black),
                    Some(ScorePeg::Black),
                ]),
            },
            TestCase {
                name: "match_all_colors_with_wrong_positions",
                code: Code::new([CodePeg::A, CodePeg::B, CodePeg::C, CodePeg::D]),
                guess: Code::new([CodePeg::D, CodePeg::C, CodePeg::B, CodePeg::A]),
                score: Score::new([
                    Some(ScorePeg::White),
                    Some(ScorePeg::White),
                    Some(ScorePeg::White),
                    Some(ScorePeg::White),
                ]),
            },
            TestCase {
                name: "two_blacks",
                code: Code::new([CodePeg::C, CodePeg::C, CodePeg::A, CodePeg::F]),
                guess: Code::new([CodePeg::C, CodePeg::D, CodePeg::D, CodePeg::F]),
                score: Score::new([Some(ScorePeg::Black), Some(ScorePeg::Black), None, None]),
            },
            TestCase {
                name: "black_and_white",
                code: Code::new([CodePeg::A, CodePeg::C, CodePeg::E, CodePeg::F]),
                guess: Code::new([CodePeg::C, CodePeg::D, CodePeg::D, CodePeg::F]),
                score: Score::new([Some(ScorePeg::Black), Some(ScorePeg::White), None, None]),
            },
            TestCase {
                name: "count_match_only_once",
                code: Code::new([CodePeg::A, CodePeg::B, CodePeg::E, CodePeg::F]),
                guess: Code::new([CodePeg::A, CodePeg::A, CodePeg::D, CodePeg::D]),
                score: Score::new([Some(ScorePeg::Black), None, None, None]),
            },
        ];

        for test_case in test_cases {
            let scorer = Scorer::new(test_case.code);
            let score = scorer.score(test_case.guess);
            assert_eq!(score, test_case.score, "test case{}", test_case.name,);
        }
    }
}
