#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;
use std::fmt;
use std::ops::Range;

use derive_more::From;
use pest::iterators::Pair;
use pest::Parser;

use error::Result;
use parser::*;

mod error;

mod parser;

#[derive(Debug, PartialEq, From)]
pub enum MarinValue {
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    List(Vec<MarinValue>),
    Range(Range<i64>),
}

impl From<&str> for MarinValue {
    fn from(s: &str) -> Self {
        MarinValue::String(s.into())
    }
}


#[derive(Debug, PartialEq)]
pub struct Marin<'a> {
    pub args: Vec<MarinValue>,
    pub kwargs: HashMap<&'a str, MarinValue>,
}

impl Marin<'_> {
    pub fn parse(string: &str) -> Result<Marin> {
        let mut args: Vec<MarinValue> = vec![];
        let mut kwargs: HashMap<&str, MarinValue> = HashMap::new();

        if string.is_empty() {
            return Ok(Marin { args, kwargs });
        };

        let pairs = MarinParser::parse(Rule::Marin, string)?;

        if let Some(main_pair) = pairs.peek() {
            for pair in main_pair.into_inner() {
                match pair.as_rule() {
                    Rule::Keyword => {
                        let inner: Vec<_> = pair.into_inner().collect();
                        let key = inner[0].as_str();
                        let value = Self::serialize(&inner[1])?;
                        kwargs.insert(key, value);
                    }
                    Rule::Flag => {
                        let inner = pair.into_inner().next().unwrap();
                        let key = inner.as_str();
                        kwargs.insert(key, MarinValue::Bool(true));
                    }
                    Rule::Value => args.push(Self::serialize(&pair)?),
                    _ => unreachable!()
                }
            };
        } else {
            return Ok(Marin { args, kwargs });
        }

        Ok(Marin { args, kwargs })
    }

    fn serialize(pair: &Pair<Rule>) -> Result<MarinValue> {
        let val = match pair.as_rule() {
            Rule::Value => {
                let inner: Vec<_> = pair.clone().into_inner().collect();
                Self::serialize(&inner[0])?
            }
            Rule::RangeExpr => {
                let inner = pair.clone().into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::Range => {
                        let values: Vec<_> = inner.into_inner().collect();
                        MarinValue::Range(Range {
                            start: values[0].as_str().parse().unwrap(),
                            end: values[1].as_str().parse().unwrap(),
                        })
                    }
                    Rule::RangeTo => {
                        let to = inner.into_inner().next().unwrap();
                        MarinValue::Range(Range {
                            start: 0,
                            end: to.as_str().parse().unwrap(),
                        })
                    }
                    _ => unreachable!()
                }
            }
            Rule::Float => MarinValue::Float(pair.as_str().parse().unwrap()),
            Rule::Int => MarinValue::Int(pair.as_str().parse().unwrap()),
            Rule::Bool => MarinValue::Bool(pair.as_str().to_lowercase().parse().unwrap()),
            Rule::String => {
                let inner = pair.clone().into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::Inner => MarinValue::String(inner.as_str().replace("\\\"", "\"")),
                    Rule::Bareword => MarinValue::String(inner.as_str().into()),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!()
        };
        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Marin, MarinValue};
    use crate::MarinValue::*;
    use crate::Result;

    impl Marin<'_> {
        pub fn test(string: &str, expected: Marin) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn empty() -> Result<()> {
        let m = Marin::parse("")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn string_with_quoted_escapes() -> Result<()> {
        let m = Marin::parse("\"t e\\\" s t\"")?;
        assert_eq!(m, Marin {
            args: vec!["t e\" s t".into()],
            kwargs: HashMap::new(),
        });
        Ok(())
    }

    #[test]
    fn flags() -> Result<()> {
        let m = Marin::parse("-overwrite -dynamic")?;
                let mut kwargs: HashMap<&str, MarinValue> = HashMap::new();
        kwargs.insert("overwrite", true.into());
        kwargs.insert("dynamic", true.into());
        assert_eq!(m, Marin { args: vec![], kwargs });
        Ok(())
    }

    #[test]
    fn flags_and_keywords() -> Result<()> {
        let m = Marin::parse("-overwrite offset: 30m")?;
                let mut kwargs: HashMap<&str, MarinValue> = HashMap::new();
        kwargs.insert("overwrite", true.into());
        kwargs.insert("offset", "30m".into());
        assert_eq!(m, Marin { args: vec![], kwargs });
        Ok(())
    }

    #[test]
    fn positional_int_and_flags() -> Result<()> {
        let m = Marin::parse("777000 -mention -id")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn positional_username() -> Result<()> {
        let m = Marin::parse("@username")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn invite_link() -> Result<()> {
        let m = Marin::parse("https://t.me/joinchat/CkzknkNYuLsKbTc91GfhGw")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn quoted_key_word_argument() -> Result<()> {
        let m = Marin::parse("reason: \"spam[gban]\"")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn wildcard_keyword_argument() -> Result<()> {
        let m = Marin::parse("reason: \"Kriminalamt *\"")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn keyword_with_link() -> Result<()> {
        let m = Marin::parse("777000 \"ban reason\" link: https://t.me/c/1129887931/26708")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn chat_id_with_flags() -> Result<()> {
        let m = Marin::parse("-1001129887931 -strafanzeige polizei: exclude")?;
        let mut kwargs: HashMap<&str, MarinValue> = HashMap::new();
        kwargs.insert("polizei", "exclude".into());
        kwargs.insert("strafanzeige", true.into());
        assert_eq!(m, Marin {
            args: vec![Int(-1001129887931)],
            kwargs,
        });
        Ok(())
    }

    #[test]
    fn list_of_ids() -> Result<()> {
        let m = Marin::parse("chats: [-1001129887931, -1001367463001]")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn positonal_arguments() -> Result<()> {
        let m = Marin::parse("arg1 arg2 arg3")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn keyword_arguments() -> Result<()> {
        let m = Marin::parse("arg1: val1 arg2: \"val2.1 val2.2\"")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn keyword_with_lists() -> Result<()> {
        let m = Marin::parse("arg: [123, 456] arg2: [\"abc\", \"de f\", \"xyz\"]")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn keyword_with_ranges() -> Result<()> {
        let m = Marin::parse("arg: 1..10 arg2: -5..5 arg2: -10..0")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn scientific_notation() -> Result<()> {
        let m = Marin::parse("1e4 2.5e4 125e-5")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn duration_expression() -> Result<()> {
        let m = Marin::parse("2w3d3h5s")?;
        assert_eq!(m, Marin { args: vec![], kwargs: HashMap::new() });
        Ok(())
    }
}


struct DisplayPair<'i, R: pest::RuleType>(pest::iterators::Pair<'i, R>);

impl<'i, R: pest::RuleType> fmt::Display for DisplayPair<'i, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f, 0)
    }
}

impl<'i, R: pest::RuleType> DisplayPair<'i, R> {
    fn display(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        let span = self.0.clone().as_span();
        let rule = self.0.as_rule();
        let inner = self.0.clone().into_inner();
        let indent = "    ".repeat(depth);
        let children_possible = if let Some(len) = inner.size_hint().1 {
            len > 0
        } else {
            true
        };

        write!(f, "{}{:?}({}", indent, rule, span.as_str())?;
        if children_possible {
            writeln!(f, ", [")?;
            for pair in self.0.clone().into_inner() {
                DisplayPair(pair).display(f, depth + 1)?;
            }
            write!(f, "{}]),", indent)?;
        } else {
            write!(f, ")")?;
        }
        if depth > 0 {
            writeln!(f)?;
        }
        Ok(())
    }
}
