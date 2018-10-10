#[macro_use]
extern crate pest_derive;

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "../specification.pest"]
struct SpecificationParser;

#[derive(Copy, Clone, Debug)]
pub struct PestSpecification {
    count: u8,
    size: u8,
}

impl PestSpecification {
    fn single(size: u8) -> PestSpecification {
        PestSpecification { size, count: 1 }
    }

    fn multiple(count: u8, size: u8) -> PestSpecification {
        PestSpecification { count, size }
    }
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

        let mut elements = SpecificationParser::parse(Rule::spec, s)
            .map_err(|_| ParseSpecificationError::Invalid)?
            .next()
            .ok_or(ParseSpecificationError::Invalid)?
            .into_inner()
            .filter(|x| x.as_rule() == Rule::digit);

        // Our grammar makes this safe to unwrap.
        let left = elements.next().unwrap().as_str();

        match elements.next().map(|x| x.as_str()) {
            None => Ok(PestSpecification::single(left.parse()?)),
            Some(right) => Ok(PestSpecification::multiple(left.parse()?, right.parse()?)),
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

#[derive(Copy, Clone, Debug)]
pub struct SplitSpecification {
    count: u8,
    size: u8,
}

impl SplitSpecification {
    fn single(size: u8) -> SplitSpecification {
        SplitSpecification { size, count: 1 }
    }

    fn multiple(count: u8, size: u8) -> SplitSpecification {
        SplitSpecification { count, size }
    }
}

impl FromStr for SplitSpecification {
    type Err = ParseSpecificationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split('d');

        let left: u8 = parts
            .next()
            .ok_or(ParseSpecificationError::Invalid)?
            .parse()?;

        match parts.next() {
            None => Ok(SplitSpecification::single(left)),
            Some(right) => {
                if parts.next().is_some() {
                    return Err(ParseSpecificationError::Invalid);
                }

                Ok(SplitSpecification::multiple(left, right.parse()?))
            }
        }
    }
}
