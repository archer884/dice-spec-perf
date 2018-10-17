#![feature(dbg_macro)]

#[macro_use]
extern crate pest_derive;

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "../specification.pest"]
struct SpecificationParser;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PestSpecification {
    pub count: u8,
    pub size: u8,
    pub modifier: u8,
}

#[derive(Debug)]
pub enum ParseSpecificationError {
    Invalid,
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParseSpecificationError {
    fn from(e: ParseIntError) -> Self {
        ParseSpecificationError::ParseInt(e)
    }
}

impl FromStr for PestSpecification {
    type Err = ParseSpecificationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use pest::Parser;

        // Our grammar declares two potential elements of a full specification, both of which are
        // represented here.
        let mut elements = SpecificationParser::parse(Rule::FullSpec, s)
            .map_err(|_| ParseSpecificationError::Invalid)?
            .next()
            .unwrap()
            .into_inner();

        // The Dice element must be included.
        let (left, right) = {
            // Unfortunately, the actual Digits are wrapped several layers deep...
            let mut dice = elements
                .next()
                .unwrap()
                .into_inner()
                .next()
                .unwrap()
                .into_inner()
                .filter(|x| x.as_rule() == Rule::Digit);

            let left = dice.next().unwrap().as_str().parse()?;
            match dice.next().map(|x| x.as_str()) {
                None => (left, None),
                Some(right) => (left, Some(right.parse()?)),
            }
        };

        // The Modifier element is optional.
        let modifier = match elements.next() {
            None => 0,
            Some(element) => element
                .into_inner()
                .filter(|x| x.as_rule() == Rule::Digit)
                .next()
                .unwrap()
                .as_str()
                .parse()?,
        };

        match right {
            None => Ok(PestSpecification {
                count: 1,
                size: left,
                modifier,
            }),

            Some(right) => Ok(PestSpecification {
                count: left,
                size: right,
                modifier,
            }),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RegexSpecification {
    count: u8,
    size: u8,
}

impl RegexSpecification {
    fn single(size: u8) -> RegexSpecification {
        RegexSpecification { size, count: 1 }
    }

    fn multiple(count: u8, size: u8) -> RegexSpecification {
        RegexSpecification { count, size }
    }
}

impl FromStr for RegexSpecification {
    type Err = ParseSpecificationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use regex::Regex;

        let pattern = Regex::new(r#"(\d)+d(\d+)|(\d+)"#).unwrap();
        let captures = pattern
            .captures(s)
            .ok_or(ParseSpecificationError::Invalid)?;

        if let Some(single) = captures.get(3) {
            Ok(RegexSpecification::single(single.as_str().parse()?))
        } else {
            Ok(RegexSpecification::multiple(
                // Unwrap is safe if we reach this branch.
                captures.get(1).unwrap().as_str().parse()?,
                captures.get(2).unwrap().as_str().parse()?,
            ))
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SplitSpecification {
    pub count: u8,
    pub size: u8,
    pub modifier: u8,
}

impl FromStr for SplitSpecification {
    type Err = ParseSpecificationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The two potential elements of a full specification are the dice spec and the modifier.
        // Of these, the modifier is optional.
        let mut elements = s.trim().split('+');

        let mut dice = elements
            .next()
            .ok_or(ParseSpecificationError::Invalid)?
            .split('d');

        let left = dice
            .next()
            .ok_or(ParseSpecificationError::Invalid)?
            .parse()?;

        let modifier = match elements.next() {
            None => 0,
            Some(x) => x.parse()?,
        };

        // Additional expression elements are invalid.
        if elements.next().is_some() {
            return Err(ParseSpecificationError::Invalid);
        }

        match dice.next() {
            None => Ok(SplitSpecification {
                count: 1,
                size: left,
                modifier,
            }),

            Some(right) => {

                // Further dice expression elements are invalid.
                if dice.next().is_some() {
                    return Err(ParseSpecificationError::Invalid);
                }

                Ok(SplitSpecification {
                    count: left,
                    size: right.parse()?,
                    modifier,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pest() {
        use super::PestSpecification;

        let actual: PestSpecification = "2d6+3".parse().unwrap();
        let expected = PestSpecification {
            count: 2,
            size: 6,
            modifier: 3,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn split() {
        use super::SplitSpecification;

        let actual: SplitSpecification = "2d6+3".parse().unwrap();
        let expected = SplitSpecification {
            count: 2,
            size: 6,
            modifier: 3,
        };

        assert_eq!(actual, expected);
    }
}
