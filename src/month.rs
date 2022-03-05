use crate::error::Error;
use crate::error::Error::*;

#[derive(Debug, PartialEq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn from(s: &str) -> Result<Month, Error> {
        match s.trim().to_lowercase().as_str() {
            "jan" => Ok(Month::January),
            "feb" => Ok(Month::February),
            "mar" => Ok(Month::March),
            "apr" => Ok(Month::April),
            "may" => Ok(Month::May),
            "jun" => Ok(Month::June),
            "jul" => Ok(Month::July),
            "aug" => Ok(Month::August),
            "sep" => Ok(Month::September),
            "oct" => Ok(Month::October),
            "nov" => Ok(Month::November),
            "dec" => Ok(Month::December),
            _ => Err(MonthParseError(format!("Unable to parse month: `{}`", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Month;
    use super::Month::*;

    #[test]
    fn from_valid_months() {
        let months = 
            [ (January  , "JAN" )
            , (February , "FEB" )
            , (March    , "MAR" )
            , (April    , "APR" )
            , (May      , "MAY" )
            , (June     , "JUN" )
            , (July     , "JUL" )
            , (August   , "AUG" )
            , (September, "sEp" )
            , (October  , "ocT" )
            , (November , " Nov")
            , (December , " dec")
            ];

        for (expected, s) in months {
            assert_eq!(expected, Month::from(s).unwrap());
        }
    }

    #[test]
    fn from_invalid_months() {
        let strs = [ "j", "" ];

        for s in strs {
            assert!(Month::from(s).is_err());
        }
    }
}