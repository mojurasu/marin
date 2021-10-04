#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Range;

use pest::iterators::Pair;
use pest::Parser;

pub use error::Error;
use error::Result;
use marin_value::MarinValue;
use parser::*;

mod error;

mod parser;

mod marin_value;

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
                    Rule::Value | Rule::Number => args.push(Self::serialize(&pair)?),
                    Rule::EOI => (),
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
            Rule::Int | Rule::Number => MarinValue::Int(pair.as_str().parse().unwrap()),
            Rule::Bool => MarinValue::Bool(pair.as_str().to_lowercase().parse().unwrap()),
            Rule::String => {
                let inner = pair.clone().into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::Inner => MarinValue::String(inner.as_str().replace("\\\"", "\"")),
                    Rule::Bareword => MarinValue::String(inner.as_str().into()),
                    _ => unreachable!(),
                }
            }
            Rule::List => {
                let list: Vec<MarinValue> = pair.clone().into_inner()
                                                .map(|v| Self::serialize(&v))
                                                .collect::<Result<Vec<MarinValue>>>()?;
                MarinValue::List(list)
            }
            _ => unreachable!()
        };
        Ok(val)
    }
}

impl Display for Marin<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Arguments:")?;
        for arg in &self.args {
            writeln!(f, "    {:?}", arg)?;
        }
        writeln!(f, "Keyword Arguments:")?;

        for (key, value) in self.kwargs.iter() {
            writeln!(f, "    {}: {:?}", key, value)?;
        }
        Ok(())
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
        let kwargs: HashMap<&str, MarinValue> = vec![
            ("mention", true.into()),
            ("id", true.into()),
        ].into_iter().collect();
        assert_eq!(m, Marin { args: vec![777000.into()], kwargs });
        Ok(())
    }

    #[test]
    fn positional_username() -> Result<()> {
        let m = Marin::parse("@username")?;
        assert_eq!(m, Marin { args: vec!["@username".into()], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn invite_link() -> Result<()> {
        let m = Marin::parse("https://t.me/joinchat/CkzknkNYuLsKbTc91GfhGw")?;
        assert_eq!(m, Marin { args: vec!["https://t.me/joinchat/CkzknkNYuLsKbTc91GfhGw".into()], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn quoted_key_word_argument() -> Result<()> {
        let m = Marin::parse("reason: \"spam[gban]\"")?;
        let kwargs: HashMap<&str, MarinValue> = vec![
            ("reason", "spam[gban]".into()),
        ].into_iter().collect();
        assert_eq!(m, Marin { args: vec![], kwargs });
        Ok(())
    }

    #[test]
    fn wildcard_keyword_argument() -> Result<()> {
        let m = Marin::parse("reason: \"Kriminalamt *\"")?;
        let kwargs: HashMap<&str, MarinValue> = vec![
            ("reason", "Kriminalamt *".into()),
        ].into_iter().collect();
        assert_eq!(m, Marin { args: vec![], kwargs });
        Ok(())
    }

    #[test]
    fn keyword_with_link() -> Result<()> {
        let m = Marin::parse("777000 \"ban reason\" link: https://t.me/c/1129887931/26708")?;
        let kwargs: HashMap<&str, MarinValue> = vec![
            ("link", "https://t.me/c/1129887931/26708".into()),
        ].into_iter().collect();
        assert_eq!(m, Marin {
            args: vec![777000.into(), "ban reason".into()],
            kwargs,
        });
        Ok(())
    }

    #[test]
    fn chat_id_with_flags() -> Result<()> {
        let m = Marin::parse("-1001129887931 -strafanzeige polizei: exclude")?;
        let kwargs: HashMap<&str, MarinValue> = vec![
            ("strafanzeige", true.into()),
            ("polizei", "exclude".into()),
        ].into_iter().collect();
        assert_eq!(m, Marin {
            args: vec![Int(-1001129887931)],
            kwargs,
        });
        Ok(())
    }

    #[test]
    fn list_of_ids() -> Result<()> {
        let m = Marin::parse("chats: [-1001129887931, -1001367463001]")?;
        let kwargs: HashMap<&str, MarinValue> = vec![
            ("chats", vec![Int(-1001129887931), Int(-1001367463001)].into()),
        ].into_iter().collect();
        assert_eq!(m, Marin { args: vec![], kwargs });
        Ok(())
    }

    #[test]
    fn positonal_arguments() -> Result<()> {
        let m = Marin::parse("arg1 arg2 arg3 4arg")?;
        assert_eq!(m, Marin {
            args: vec![
                "arg1".into(),
                "arg2".into(),
                "arg3".into(),
                "4arg".into(),
            ].into(),
            kwargs: HashMap::new(),
        });
        Ok(())
    }

    #[test]
    fn keyword_arguments() -> Result<()> {
        let m = Marin::parse("arg1: val1 arg2: \"val2.1 val2.2\"")?;
        let kwargs: HashMap<&str, MarinValue> = vec![
            ("arg1", "val1".into()),
            ("arg2", "val2.1 val2.2".into()),
        ].into_iter().collect();
        assert_eq!(m, Marin { args: vec![], kwargs });
        Ok(())
    }

    #[test]
    fn keyword_with_lists() -> Result<()> {
        let m = Marin::parse("arg: [123, 456] arg2: [\"abc\", \"de f\", \"xyz\"]")?;
        let kwargs: HashMap<&str, MarinValue> = vec![
            ("arg", vec![123.into(), 456.into()].into()),
            ("arg2", vec!["abc".into(), "de f".into(), "xyz".into()].into()),
        ].into_iter().collect();
        assert_eq!(m, Marin { args: vec![], kwargs });
        Ok(())
    }

    #[test]
    fn keyword_with_ranges() -> Result<()> {
        let m = Marin::parse("arg: 1..10 arg2: -5..5 arg3: -10..0")?;
        let kwargs: HashMap<&str, MarinValue> = vec![
            ("arg", Range(1..10)),
            ("arg2", Range(-5..5)),
            ("arg3", Range(-10..0)),
        ].into_iter().collect();
        assert_eq!(m, Marin { args: vec![], kwargs });
        Ok(())
    }

    #[test]
    fn scientific_notation() -> Result<()> {
        let m = Marin::parse("1e4 2.5e4 125e-5")?;
        assert_eq!(m, Marin { args: vec![Float(10000.0), Float(25000.0), Float(0.00125)], kwargs: HashMap::new() });
        Ok(())
    }

    #[test]
    fn duration_expression() -> Result<()> {
        let m = Marin::parse("2w3d3h5s")?;
        assert_eq!(m, Marin {
            args: vec!["2w3d3h5s".into()],
            kwargs: HashMap::new(),
        });
        Ok(())
    }

    #[test]
    #[should_panic]
    fn flag_keyword() {
        let m = Marin::parse("-flag: 123").unwrap();
        ()
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
