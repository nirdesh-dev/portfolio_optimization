use thiserror::Error;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Range {
    Day1,
    Month1,
    Month3,
    Month6,
    Year1,
    Year2,
    Year5,
    Year10,
    YearToDate,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    Minute1,
    Minute2,
    Minute3,
    Minute5,
    Minute15,
    Minute30,
    Minute90,
    Hour1,
    Day1,
    Day5,
    Week1,
    Month1,
    Month3,
}

#[derive(Error, Debug, PartialEq)]
pub enum PeriodError {
    #[error("Invalid interval for this range")]
    InvalidInterval,
    #[error("Unknown range string: {0}")]
    UnknownRange(String),
    #[error("Unknown interval string: {0}")]
    UnknownInterval(String),
}

impl Range {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Day1 => "1d",
            Self::Month1 => "1mo",
            Self::Month3 => "3mo",
            Self::Month6 => "6mo",
            Self::Year1 => "1y",
            Self::Year2 => "2y",
            Self::Year5 => "5y",
            Self::Year10 => "10y",
            Self::YearToDate => "ytd",
            Self::Max => "max",
        }
    }

    pub fn valid_intervals(&self) -> &'static [Interval] {
        match self {
            Self::Day1 => &[
                Interval::Minute1,
                Interval::Minute2,
                Interval::Minute5,
                Interval::Minute15,
                Interval::Minute30,
                Interval::Minute90,
                Interval::Hour1,
                Interval::Day1,
                Interval::Day5,
                Interval::Week1,
                Interval::Month1,
                Interval::Month3,
            ],
            Self::Month1 => &[
                Interval::Minute2,
                Interval::Minute3,
                Interval::Minute5,
                Interval::Minute15,
                Interval::Minute30,
                Interval::Minute90,
                Interval::Hour1,
                Interval::Day1,
                Interval::Day5,
                Interval::Week1,
                Interval::Month1,
                Interval::Month3,
            ],
            // ... other ranges with their valid intervals
            _ => &[
                Interval::Hour1,
                Interval::Day1,
                Interval::Week1,
                Interval::Month1,
                Interval::Month3,
            ],
        }
    }

    pub fn from_str(s: &str) -> Result<Self, PeriodError> {
        match s {
            "1d" => Ok(Self::Day1),
            "1mo" => Ok(Self::Month1),
            "3mo" => Ok(Self::Month3),
            "6mo" => Ok(Self::Month6),
            "1y" => Ok(Self::Year1),
            "2y" => Ok(Self::Year2),
            "5y" => Ok(Self::Year5),
            "10y" => Ok(Self::Year10),
            "ytd" => Ok(Self::YearToDate),
            "max" => Ok(Self::Max),
            _ => Err(PeriodError::UnknownRange(s.to_string())),
        }
    }

    pub fn is_valid_interval(&self, interval: Interval) -> bool {
        self.valid_intervals().contains(&interval)
    }
}

impl Interval {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Minute1 => "1m",
            Self::Minute2 => "2m",
            Self::Minute3 => "3m",
            Self::Minute5 => "5m",
            Self::Minute15 => "15m",
            Self::Minute30 => "30m",
            Self::Minute90 => "90m",
            Self::Hour1 => "1h",
            Self::Day1 => "1d",
            Self::Day5 => "5d",
            Self::Week1 => "1wk",
            Self::Month1 => "1mo",
            Self::Month3 => "3mo",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, PeriodError> {
        match s {
            "1m" => Ok(Self::Minute1),
            "2m" => Ok(Self::Minute2),
            "3m" => Ok(Self::Minute3),
            "5m" => Ok(Self::Minute5),
            "15m" => Ok(Self::Minute15),
            "30m" => Ok(Self::Minute30),
            "90m" => Ok(Self::Minute90),
            "1h" => Ok(Self::Hour1),
            "1d" => Ok(Self::Day1),
            "5d" => Ok(Self::Day5),
            "1wk" => Ok(Self::Week1),
            "1mo" => Ok(Self::Month1),
            "3mo" => Ok(Self::Month3),
            _ => Err(PeriodError::UnknownInterval(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_validation() {
        assert!(Range::Year5.is_valid_interval(Interval::Day1));
        assert!(!Range::Year5.is_valid_interval(Interval::Minute1));
    }

    #[test]
    fn test_string_conversion() {
        assert_eq!(Range::Year5.as_str(), "5y");
        assert_eq!(Interval::Minute15.as_str(), "15m");
        assert_eq!(Range::from_str("1mo"), Ok(Range::Month1));
        assert_eq!(Interval::from_str("1h"), Ok(Interval::Hour1));
    }
}
