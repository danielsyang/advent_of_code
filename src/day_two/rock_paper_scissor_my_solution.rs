use std::str::FromStr;

use crate::utils::read_file;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    ours: Move,
    theirs: Move,
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'Y' | 'B' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissor),
            _ => Err(color_eyre::eyre::eyre!("Not a valid move {:?}", value)),
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Each char should become a move. Should return Round
        let mut chars = s.chars();

        let (Some(theirs), Some(' '), Some(ours)) = (chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <theirs>SP<ours>EOF, got {s:?}"))
        };

        Ok(Round {
            ours: ours.try_into()?,
            theirs: theirs.try_into()?,
        })
    }
}

impl Move {
    fn round_result(self, theirs: Move) -> Outcome {
        match (self, theirs) {
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Scissor, Move::Scissor) => Outcome::Draw,

            (Move::Rock, Move::Paper) => Outcome::Loss,
            (Move::Rock, Move::Scissor) => Outcome::Win,

            (Move::Paper, Move::Scissor) => Outcome::Loss,
            (Move::Paper, Move::Rock) => Outcome::Win,

            (Move::Scissor, Move::Rock) => Outcome::Loss,
            (Move::Scissor, Move::Paper) => Outcome::Win,
        }
    }

    fn move_score(self) -> usize {
        match self {
            Move::Scissor => 3,
            Move::Paper => 2,
            Move::Rock => 1,
        }
    }
}

impl Round {
    // From round, check if I got a draw, win or loss
    fn outcome(self) -> Outcome {
        // get my move, compare to my opponent move and check the outcome
        self.ours.round_result(self.theirs)
    }

    fn score(self) -> usize {
        // get score from outcome + move score
        self.outcome().score() + self.ours.move_score()
    }
}

impl Outcome {
    fn score(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[allow(dead_code)]
fn rock_paper_scissor() -> usize {
    let input = read_file("src/day_two/input.txt");
    // transform each row in rounds
    let my_score = input
        .lines()
        .map(Round::from_str)
        .map(|x| match x {
            Ok(x) => x,
            Err(err) => panic!("Invalid game, got {:?}", err),
        })
        .map(|x| x.score())
        .sum::<usize>();

    return my_score;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rock_paper_scissor_test() {
        assert_eq!(rock_paper_scissor(), 15);
    }
}
