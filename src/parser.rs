#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MarinParser;

#[cfg(test)]
mod tests {
    use std::fmt;

    use pest::Parser;
    use pest::parses_to;

    use crate::parser::MarinParser;
    use crate::parser::Rule;

    struct ParsesToDisplay<'i, R: pest::RuleType>(pest::iterators::Pair<'i, R>);

    impl<'i, R: pest::RuleType> fmt::Display for ParsesToDisplay<'i, R> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.display(f, 0)
        }
    }

    impl<'i, R: pest::RuleType> ParsesToDisplay<'i, R> {
        fn display(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
            let span = self.0.clone().as_span();
            let rule = self.0.as_rule();
            let inner = self.0.clone().into_inner();
            let indent = "    ".repeat(depth + 4);
            let children_possible = if let Some(len) = inner.size_hint().1 {
                len > 0
            } else {
                true
            };

            write!(f, "{}{:?}({}, {}", indent, rule, span.start_pos().pos(), span.end_pos().pos())?;
            if children_possible {
                writeln!(f, ", [")?;
                for pair in self.0.clone().into_inner() {
                    ParsesToDisplay(pair).display(f, depth + 1)?;
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

    #[test]
    fn positional_ints() {
        parses_to! {
            parser: MarinParser,
            input:  "1 2 -3 -3.5",
            rule:   Rule::Marin,
            tokens: [
                Marin(0, 11, [
                    Value(0, 1, [
                        Int(0, 1, []),
                    ]),
                    Value(2, 3, [
                        Int(2, 3, []),
                    ]),
                    Value(4, 6, [
                        Int(4, 6, []),
                    ]),
                    Value(7, 11, [
                        Float(7, 11, []),
                    ]),
                    EOI(11, 11, []),
                ]),
            ]
        };
    }

    #[test]
    fn positional_strings() {
        parses_to! {
            parser: MarinParser,
            input:  "arg1 arg2 arg3 4arg",
            rule:   Rule::Marin,
            tokens: [
                Marin(0, 19, [
                    Value(0, 4, [
                        String(0, 4, [
                            Bareword(0, 4, []),
                        ]),
                    ]),
                    Value(5, 9, [
                        String(5, 9, [
                            Bareword(5, 9, []),
                        ]),
                    ]),
                    Value(10, 14, [
                        String(10, 14, [
                            Bareword(10, 14, []),
                        ]),
                    ]),
                    Value(15, 19, [
                        String(15, 19, [
                            Bareword(15, 19, []),
                        ]),
                    ]),
                    EOI(19, 19, []),
                ]),
            ]
        };
    }

    #[test]
    fn keyword_arguments() {
        parses_to! {
            parser: MarinParser,
            input:  "kw: 1 kwargs: True",
            rule:   Rule::Marin,
            tokens: [
                Marin(0, 18, [
                    Keyword(0, 5, [
                        String(0, 2, [
                            Bareword(0, 2, []),
                        ]),
                        Value(4, 5, [
                            Int(4, 5, []),
                        ]),
                    ]),
                    Keyword(6, 18, [
                        String(6, 12, [
                            Bareword(6, 12, []),
                        ]),
                        Value(14, 18, [
                            Bool(14, 18, []),
                        ]),
                    ]),
                    EOI(18, 18, []),
                ]),
            ]
        };
    }

    #[test]
    fn flags() {
        parses_to! {
            parser: MarinParser,
            input:  "-flag1 -flag2",
            rule:   Rule::Marin,
            tokens: [
                Marin(0, 13, [
                    Flag(0, 6, [
                        FlagInner(1, 6, []),
                    ]),
                    Flag(7, 13, [
                        FlagInner(8, 13, []),
                    ]),
                    EOI(13, 13, []),
                ]),
            ]
        };
    }

    #[test]
    fn keyword_arguments_strings() {
        parses_to! {
            parser: MarinParser,
            input:  "kw: \"String with spaces\"",
            rule:   Rule::Marin,
            tokens: [
                Marin(0, 24, [
                    Keyword(0, 24, [
                        String(0, 2, [
                            Bareword(0, 2, []),
                        ]),
                        Value(4, 24, [
                            String(4, 24, [
                                Inner(5, 23, []),
                            ]),
                        ]),
                    ]),
                    EOI(24, 24, []),
                ]),
            ]
        };
    }

    #[test]
    fn lists() {
        parses_to! {
            parser: MarinParser,
            input:  "vals: [1, 2, 3] [1,2,3]",
            rule:   Rule::Marin,
            tokens: [
               Marin(0, 23, [
                    Keyword(0, 15, [
                        String(0, 4, [
                            Bareword(0, 4, []),
                        ]),
                        Value(6, 15, [
                            List(6, 15, [
                                Value(7, 8, [
                                    Int(7, 8, []),
                                ]),
                                Value(10, 11, [
                                    Int(10, 11, []),
                                ]),
                                Value(13, 14, [
                                    Int(13, 14, []),
                                ]),
                            ]),
                        ]),
                    ]),
                    Value(16, 23, [
                        List(16, 23, [
                            Value(17, 18, [
                                Int(17, 18, []),
                            ]),
                            Value(19, 20, [
                                Int(19, 20, []),
                            ]),
                            Value(21, 22, [
                                Int(21, 22, []),
                            ]),
                        ]),
                    ]),
                    EOI(23, 23, []),
                ]),

            ]
        };
    }

    #[test]
    fn ranges() {
        parses_to! {
            parser: MarinParser,
            input:  "range: 1..10 -5..15 ..10",
            rule:   Rule::Marin,
            tokens: [
                Marin(0, 24, [
                    Keyword(0, 12, [
                        String(0, 5, [
                            Bareword(0, 5, [
                            ]),
                        ]),
                        Value(7, 12, [
                            RangeExpr(7, 12, [
                                Range(7, 12, [
                                    Number(7, 8, []),
                                    Number(10, 12, []),
                                ]),
                            ]),
                        ]),
                    ]),
                    Value(13, 19, [
                        RangeExpr(13, 19, [
                            Range(13, 19, [
                                Number(13, 15, []),
                                Number(17, 19, []),
                            ]),
                        ]),
                    ]),
                    Value(20, 24, [
                        RangeExpr(20, 24, [
                            RangeTo(20, 24, [
                                Number(22, 24, []),
                            ]),
                        ]),
                    ]),
                    EOI(24, 24, []),
                ]),
            ]
        };
    }
}
