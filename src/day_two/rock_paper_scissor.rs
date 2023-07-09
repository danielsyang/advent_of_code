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
    Draw,
    Win,
    Loss,
}

impl Move {
    fn outcome(self, theirs: Move) -> Outcome {
        match (self, theirs) {
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Scissor, Move::Scissor) => Outcome::Draw,

            (Move::Paper, Move::Scissor) => Outcome::Loss,
            (Move::Paper, Move::Rock) => Outcome::Win,

            (Move::Scissor, Move::Rock) => Outcome::Loss,
            (Move::Scissor, Move::Paper) => Outcome::Win,

            (Move::Rock, Move::Paper) => Outcome::Loss,
            (Move::Rock, Move::Scissor) => Outcome::Win,
        }
    }
}

impl Outcome {
    fn inherent_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }

    fn our_score(self) -> usize {
        self.outcome().inherent_points()
    }

    fn their_score(self) -> usize {
        let outcome = self.theirs.outcome(self.ours);
        outcome.inherent_points()
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissor),
            _ => Err(color_eyre::eyre::eyre!("Not a valid move {:?}", value)),
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (Some(ours), Some(' '), Some(theirs)) = (chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!(
                "expected <theirs>SP<ours>EOF, got {s:?}"
            ))
        };

        Ok(Self {
            ours: ours.try_into()?,
            theirs: theirs.try_into()?,
        })
    }
}

#[allow(dead_code)]
fn rock_paper_scissor_our_score() -> usize {
    let input = read_file("src/day_two/input.txt");
    let rounds = input.lines().map(|x| x.parse::<Round>().unwrap());
    let our_score = rounds.map(|x| x.our_score()).sum::<usize>();

    return our_score;
}

#[allow(dead_code)]
fn rock_paper_scissor_their_score() -> usize {
    let input = read_file("src/day_two/input.txt");
    let rounds = input.lines().map(|x| x.parse::<Round>().unwrap());
    let our_score = rounds.map(|x| x.their_score()).sum::<usize>();

    return our_score;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rock_paper_scissor() {
        assert_eq!(rock_paper_scissor_our_score(), 15);
    }

    #[test]
    fn test_rock_paper_scissor_their_score() {
        assert_eq!(rock_paper_scissor_their_score(), 9);
    }
}
