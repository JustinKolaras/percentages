use eval::eval;
use regex::Captures;
use regex::Regex;
use std::str::FromStr;

const VERIFY: &str = r"\(((\d+\.?\d*|\+|\-)+)\)/(\-?\d+)";
const WHITESPACE_ONLY: &str = r"\A\s*\z";

enum TypeConversionError {
    Space,
    Digit,
    Form,
}

enum DataParseError {
    NotMatch(TypeConversionError),
}

struct CalculationData {
    elements: u64,
    divider: u64,
}

pub struct SuccessData {
    pub elements: u64,
    pub percentage: f64,
}

pub struct ErrorData<'a> {
    pub error: String,
    pub emphasis: Option<&'a str>,
}

impl FromStr for CalculationData {
    type Err = DataParseError;

    fn from_str(previous: &str) -> Result<Self, Self::Err> {
        let verify_parser: Regex = Regex::new(VERIFY).unwrap();
        let whitespace_parser: Regex = Regex::new(WHITESPACE_ONLY).unwrap();

        if whitespace_parser.is_match(previous) {
            return Err(DataParseError::NotMatch(TypeConversionError::Form));
        }

        if !verify_parser.is_match(previous) {
            if previous.contains(' ') {
                return Err(DataParseError::NotMatch(TypeConversionError::Space));
            }
            return Err(DataParseError::NotMatch(TypeConversionError::Form));
        }

        let captures: Captures = verify_parser.captures(previous.trim()).unwrap();

        // Has to be done separately from the "catch all" if statement on line 42.
        // There is freedom to unwrap here as we've already checked if the text matches the RegEx pattern.
        if captures
            .get(3)
            .unwrap()
            .as_str()
            .trim()
            .parse::<i32>()
            .unwrap()
            < 1
        {
            return Err(DataParseError::NotMatch(TypeConversionError::Digit));
        }

        let raw: String = captures.get(1).unwrap().as_str().to_owned();
        let elements: u64 = raw.split(['+', '-']).count() as u64;
        let divider: u64 = previous.split('/').collect::<Vec<&str>>()[1]
            .trim()
            .parse::<u64>()
            .unwrap();

        Ok(CalculationData { elements, divider })
    }
}

pub fn run(input: String) -> Result<SuccessData, ErrorData<'static>> {
    let error_message: String = String::from("Invalid equation, try again.\nNote: no spaces permitted; must be in the form of (...)/x\nwhere `...` is an addition sequence and `x` is a positive integer.");

    let parsed: CalculationData = match CalculationData::from_str(&input) {
        Ok(result) => result,
        Err(error) => {
            return Err(ErrorData {
                error: error_message,
                emphasis: match error {
                    DataParseError::NotMatch(TypeConversionError::Space) => {
                        Some("no spaces permitted")
                    }
                    DataParseError::NotMatch(TypeConversionError::Digit) => {
                        Some("is a positive integer")
                    }
                    DataParseError::NotMatch(TypeConversionError::Form) => {
                        Some("must be in the form of (...)/x")
                    }
                },
            })
        }
    };

    if parsed.elements != parsed.divider {
        return Err(ErrorData {
            error: format!(
                "Number of elements does not equal divider. Number elements: {} Divider: {}",
                parsed.elements, parsed.divider
            ),
            emphasis: None,
        });
    }

    let expression: String = format!("({}) * 100", &input);
    let evaluation: f64 = eval(expression.as_str()).unwrap().as_f64().unwrap().round();

    Ok(SuccessData {
        elements: parsed.elements,
        percentage: evaluation,
    })
}
