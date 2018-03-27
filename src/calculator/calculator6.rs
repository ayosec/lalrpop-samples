// auto-generated: "lalrpop 0.15.0"
#![cfg_attr(rustfmt, rustfmt_skip)]
use std::str::FromStr;
use ast::{Expr, Opcode};
use lalrpop_util::ErrorRecovery;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Exprs {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use ast::{Expr, Opcode};
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(__lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>),
        Variant2(Box<Expr>),
        Variant3(::std::vec::Vec<Box<Expr>>),
        Variant4(Vec<Box<Expr>>),
        Variant5(::std::option::Option<Box<Expr>>),
        Variant6(Opcode),
        Variant7(i32),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        11, 0, 0, 0, 0, 0, 0, 12, 13,
        // State 1
        11, 0, 0, 0, 0, 0, 0, 12, 13,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 15, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, -24, 0, -24, -24, -24, 0, 0, 0,
        // State 6
        0, -20, -20, -20, -20, -20, -20, 0, 0,
        // State 7
        0, -26, -26, -26, -26, -26, -26, 0, 0,
        // State 8
        0, -10, 0, 17, -10, 18, 0, 0, 0,
        // State 9
        0, -16, 20, -16, -16, -16, 21, 0, 0,
        // State 10
        11, 0, 0, 0, 0, 0, 0, 12, 13,
        // State 11
        0, -19, -19, -19, -19, -19, -19, 0, 0,
        // State 12
        0, -22, -22, -22, -22, -22, -22, 0, 0,
        // State 13
        0, 0, 0, 0, 23, 0, 0, 0, 0,
        // State 14
        -4, 0, 0, 0, 0, 0, 0, -4, -4,
        // State 15
        11, 0, 0, 0, 0, 0, 0, 12, 13,
        // State 16
        -13, 0, 0, 0, 0, 0, 0, -13, -13,
        // State 17
        -14, 0, 0, 0, 0, 0, 0, -14, -14,
        // State 18
        11, 0, 0, 0, 0, 0, 0, 12, 13,
        // State 19
        -17, 0, 0, 0, 0, 0, 0, -17, -17,
        // State 20
        -18, 0, 0, 0, 0, 0, 0, -18, -18,
        // State 21
        0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        -5, 0, 0, 0, 0, 0, 0, -5, -5,
        // State 23
        0, -23, 0, -23, -23, -23, 0, 0, 0,
        // State 24
        0, -25, -25, -25, -25, -25, -25, 0, 0,
        // State 25
        0, -21, -21, -21, -21, -21, -21, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -7,
        // State 1
        -9,
        // State 2
        -15,
        // State 3
        -6,
        // State 4
        -27,
        // State 5
        -24,
        // State 6
        -20,
        // State 7
        -26,
        // State 8
        -10,
        // State 9
        -16,
        // State 10
        0,
        // State 11
        -19,
        // State 12
        -22,
        // State 13
        -8,
        // State 14
        -4,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -5,
        // State 23
        -23,
        // State 24
        -25,
        // State 25
        -21,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 2, 3, 4, 0, 0, 5, 6, 0, 7, 8, 9, 10, 0,
        // State 1
        0, 0, 0, 0, 14, 0, 0, 0, 6, 0, 7, 8, 9, 10, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 22, 0, 0, 0, 6, 0, 7, 8, 9, 10, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 7, 8, 0, 10, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 25, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"r#"[0-9]+"#"###,
        ];
        __ACTION[(__state * 9)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ExprsParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl ExprsParser {
        pub fn new() -> ExprsParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            ExprsParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'err,
        >(
            &self,
            errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
            input: &'input str,
        ) -> Result<Vec<Box<Expr>>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(e),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token(1, _) if true => 0,
                    Token(2, _) if true => 1,
                    Token(3, _) if true => 2,
                    Token(4, _) if true => 3,
                    Token(5, _) if true => 4,
                    Token(6, _) if true => 5,
                    Token(7, _) if true => 6,
                    Token(0, _) if true => 7,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 9 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        match __error_recovery(errors, input,  &mut __tokens, &mut __states, &mut __symbols, __last_location, &mut __err_lookahead, &mut __err_integer, ::std::marker::PhantomData::<()>) {
                            Err(__e) => return Err(__e),
                            Ok(Some(__v)) => return Ok(__v),
                            Ok(None) => (),
                        }
                        match (__err_lookahead, __err_integer) {
                            (Some(__l), Some(__i)) => {
                                __lookahead = __l;
                                __integer = __i;
                                continue '__inner;
                            }
                            _ => break '__shift,
                        }
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    match __error_recovery(errors, input,  &mut __tokens, &mut __states, &mut __symbols, __last_location, &mut __err_lookahead, &mut __err_integer, ::std::marker::PhantomData::<()>) {
                        Err(__e) => return Err(__e),
                        Ok(Some(__v)) => return Ok(__v),
                        Ok(None) => (),
                    }
                }
            }
        }
    }
    fn __error_recovery<
        'input,
        'err,
        __I,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __tokens: &mut __I,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        __last_location: &mut usize,
        __opt_lookahead: &mut Option<(usize, Token<'input>, usize)>,
        __opt_integer: &mut Option<usize>,
        _: ::std::marker::PhantomData<()>,
    ) -> Result<Option<Vec<Box<Expr>>>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> where
      __I: Iterator<Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>,
    {
        let __state = *__states.last().unwrap() as usize;
        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
            token: __opt_lookahead.clone(),
            expected: __expected_tokens(__state),
        };
        let mut __dropped_tokens = vec![];
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __ACTION[__state * 9 + 8];
            if __action >= 0 {
                break;
            }
            let __lookahead_start = __opt_lookahead.as_ref().map(|l| &l.0);
            if let Some(r) = __reduce( errors, input,  __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()> ) {
                return Ok(Some(r?));
            }
        }
        let __states_len = __states.len();
        let __top0;
        '__find_state: loop {
            for __top in (0..__states_len).rev() {
                let __state = __states[__top] as usize;
                let __action = __ACTION[__state * 9 + 8];
                if __action <= 0 { continue; }
                let __error_state = __action - 1;
                if __accepts(errors, input,  __error_state, &__states[..__top + 1], *__opt_integer, ::std::marker::PhantomData::<()>,) {
                    __top0 = __top;
                    break '__find_state;
                }
            }
            '__eof: loop {
                match __opt_lookahead.take() {
                    None => {
                        return Err(__error)
                    }
                    Some(mut __lookahead) => {
                        __dropped_tokens.push(__lookahead);
                        __lookahead = match __tokens.next() {
                            Some(Ok(v)) => v,
                            None => break '__eof,
                            Some(Err(e)) => return Err(e),
                        };
                        *__last_location = __lookahead.2.clone();
                        let __integer;
                        __integer = match __lookahead.1 {
                            Token(1, _) if true => 0,
                            Token(2, _) if true => 1,
                            Token(3, _) if true => 2,
                            Token(4, _) if true => 3,
                            Token(5, _) if true => 4,
                            Token(6, _) if true => 5,
                            Token(7, _) if true => 6,
                            Token(0, _) if true => 7,
                            _ => {
                                let __state = *__states.last().unwrap() as usize;
                                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                                    token: Some(__lookahead),
                                    expected: __expected_tokens(__state),
                                };
                                return Err(__error);
                            }
                        };
                        *__opt_lookahead = Some(__lookahead);
                        *__opt_integer = Some(__integer);
                        continue '__find_state;
                    }
                }
            }
            *__opt_lookahead = None;
            *__opt_integer = None;
        };
        let __top = __top0;
        let __start = if let Some(__popped_sym) = __symbols.get(__top) {
            __popped_sym.0.clone()
        } else if let Some(__dropped_token) = __dropped_tokens.first() {
            __dropped_token.0.clone()
        } else if __top > 0 {
            __symbols[__top - 1].2.clone()
        } else {
            Default::default()
        };
        let __end = if let Some(__dropped_token) = __dropped_tokens.last() {
            __dropped_token.2.clone()
        } else if __states_len - 1 > __top {
            __symbols.last().unwrap().2.clone()
        } else if let Some(__lookahead) = __opt_lookahead.as_ref() {
            __lookahead.0.clone()
        } else {
            __start.clone()
        };
        __states.truncate(__top + 1);
        __symbols.truncate(__top);
        let __recover_state = __states[__top] as usize;
        let __error_action = __ACTION[__recover_state * 9 + 8];
        let __error_state = __error_action - 1;
        __states.push(__error_state);
        let __recovery = __lalrpop_util::ErrorRecovery {
            error: __error,
            dropped_tokens: __dropped_tokens,
        };
        __symbols.push((__start, __Symbol::Variant1(__recovery), __end));
        Ok(None)
    }
    fn __accepts<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __error_state: i8,
        __states: & [i8],
        __opt_integer: Option<usize>,
        _: ::std::marker::PhantomData<()>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.push(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1] as usize;
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __ACTION[__top * 9 + __integer],
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match -__action {
                1 => {
                    (2, 0)
                }
                2 => {
                    (0, 1)
                }
                3 => {
                    (1, 1)
                }
                4 => {
                    (2, 2)
                }
                5 => {
                    (3, 2)
                }
                6 => {
                    (1, 3)
                }
                7 => {
                    (0, 3)
                }
                8 => {
                    (2, 3)
                }
                9 => {
                    (1, 3)
                }
                10 => {
                    (1, 4)
                }
                11 => {
                    (1, 5)
                }
                12 => {
                    (0, 5)
                }
                13 => {
                    (1, 6)
                }
                14 => {
                    (1, 6)
                }
                15 => {
                    (1, 7)
                }
                16 => {
                    (1, 8)
                }
                17 => {
                    (1, 9)
                }
                18 => {
                    (1, 9)
                }
                19 => {
                    (1, 10)
                }
                20 => {
                    (1, 11)
                }
                21 => {
                    (3, 11)
                }
                22 => {
                    (1, 11)
                }
                23 => {
                    (3, 12)
                }
                24 => {
                    (1, 12)
                }
                25 => {
                    (3, 13)
                }
                26 => {
                    (1, 13)
                }
                27 => return true,
                _ => panic!("invalid action code {}", __action)
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1] as usize;
            let __next_state = __GOTO[__top * 15 + __nt] - 1;
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Box<Expr>>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(21);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (2, __symbol, 0)
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(19);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action19::<>(errors, input, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (0, __symbol, 1)
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(20);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (1, __symbol, 1)
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(24);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 2)
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(25);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant2(__symbols);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (3, __symbol, 2)
            }
            6 => {
                // Comma<Expr> = Expr => ActionFn(28);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (1, __symbol, 3)
            }
            7 => {
                // Comma<Expr> =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(errors, input, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (0, __symbol, 3)
            }
            8 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(30);
                let __sym1 = __pop_Variant2(__symbols);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (2, __symbol, 3)
            }
            9 => {
                // Comma<Expr> = (<Expr> ",")+ => ActionFn(31);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (1, __symbol, 3)
            }
            10 => {
                // Expr = Tier<ExprOp, Factor> => ActionFn(2);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 4)
            }
            11 => {
                // Expr? = Expr => ActionFn(17);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                (1, __symbol, 5)
            }
            12 => {
                // Expr? =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18::<>(errors, input, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                (0, __symbol, 5)
            }
            13 => {
                // ExprOp = "+" => ActionFn(4);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (1, __symbol, 6)
            }
            14 => {
                // ExprOp = "-" => ActionFn(5);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (1, __symbol, 6)
            }
            15 => {
                // Exprs = Comma<Expr> => ActionFn(1);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (1, __symbol, 7)
            }
            16 => {
                // Factor = Tier<FactorOp, Term> => ActionFn(3);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 8)
            }
            17 => {
                // FactorOp = "*" => ActionFn(6);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (1, __symbol, 9)
            }
            18 => {
                // FactorOp = "/" => ActionFn(7);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (1, __symbol, 9)
            }
            19 => {
                // Num = r#"[0-9]+"# => ActionFn(11);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 10)
            }
            20 => {
                // Term = Num => ActionFn(8);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 11)
            }
            21 => {
                // Term = "(", Expr, ")" => ActionFn(9);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant2(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (3, __symbol, 11)
            }
            22 => {
                // Term = error => ActionFn(10);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 11)
            }
            23 => {
                // Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => ActionFn(14);
                let __sym2 = __pop_Variant2(__symbols);
                let __sym1 = __pop_Variant6(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (3, __symbol, 12)
            }
            24 => {
                // Tier<ExprOp, Factor> = Factor => ActionFn(15);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 12)
            }
            25 => {
                // Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => ActionFn(12);
                let __sym2 = __pop_Variant2(__symbols);
                let __sym1 = __pop_Variant6(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (3, __symbol, 13)
            }
            26 => {
                // Tier<FactorOp, Term> = Term => ActionFn(13);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 13)
            }
            27 => {
                // __Exprs = Exprs => ActionFn(0);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 15 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Opcode, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Box<Expr>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Exprs::ExprsParser;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use ast::{Expr, Opcode};
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt(&self, formatter: &mut __fmt::Formatter) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __MatcherBuilder {
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl __MatcherBuilder {
        pub fn new() -> __MatcherBuilder {
            let __strs: &[&str] = &[
                "^(?u:[0-9])+",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:\\*)",
                "^(?u:\\+)",
                "^(?u:,)",
                "^(?u:\\-)",
                "^(?u:/)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:\\*)").unwrap(),
                __regex::Regex::new("^(?u:\\+)").unwrap(),
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u:\\-)").unwrap(),
                __regex::Regex::new("^(?u:/)").unwrap(),
            ];
            __MatcherBuilder { regex_set: __regex_set, regex_vec: __regex_vec }
        }
        pub fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> __Matcher<'input, 'builder> {
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: &self.regex_set,
                regex_vec: &self.regex_vec,
            }
        }
    }

    pub struct __Matcher<'input, 'builder> {
        text: &'input str,
        consumed: usize,
        regex_set: &'builder __regex::RegexSet,
        regex_vec: &'builder Vec<__regex::Regex>,
    }

    impl<'input, 'builder> Iterator for __Matcher<'input, 'builder> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 8 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Add
}

#[allow(unused_variables)]
fn __action5<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Sub
}

#[allow(unused_variables)]
fn __action6<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Mul
}

#[allow(unused_variables)]
fn __action7<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Div
}

#[allow(unused_variables)]
fn __action8<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> Box<Expr>
{
    Box::new(Expr::Number(__0))
}

#[allow(unused_variables)]
fn __action9<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize),
) -> Box<Expr>
{
    { errors.push(__0); Box::new(Expr::Error) }
}

#[allow(unused_variables)]
fn __action11<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action12<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, __1, _): (usize, Opcode, usize),
    (_, __2, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Op(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action13<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, __1, _): (usize, Opcode, usize),
    (_, __2, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Op(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action15<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action16<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
    (_, e, _): (usize, ::std::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    match e { // (1)
        None=> v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action17<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> ::std::option::Option<Box<Expr>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action18<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Box<Expr>>
{
    None
}

#[allow(unused_variables)]
fn __action19<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Box<Expr>>
{
    vec![]
}

#[allow(unused_variables)]
fn __action20<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    v
}

#[allow(unused_variables)]
fn __action21<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action22<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action23<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action24<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, Box<Expr>, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action21(
        errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action25<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __1: (usize, Box<Expr>, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action21(
        errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action26<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, ::std::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action19(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        errors,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action27<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __1: (usize, ::std::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action20(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        errors,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action28<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, Box<Expr>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action17(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action29<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Box<Expr>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action18(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action30<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __1: (usize, Box<Expr>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action17(
        errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action31<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        errors,
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, 'err, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, 'err, > __ToTriple<'input, 'err, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, 'err, > __ToTriple<'input, 'err, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}
