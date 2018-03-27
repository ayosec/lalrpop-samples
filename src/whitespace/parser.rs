// auto-generated: "lalrpop 0.15.0"
#![cfg_attr(rustfmt, rustfmt_skip)]
use lexer;
use ast;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use lexer;
    use ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Tok),
        Variant1(u8),
        Variant2(::std::vec::Vec<u8>),
        Variant3(ast::Stmt),
        Variant4(String),
        Variant5(ast::Int),
        Variant6(::std::vec::Vec<ast::Stmt>),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        5, 6, 7,
        // State 1
        0, 0, 0,
        // State 2
        -42, -42, -42,
        // State 3
        5, 6, 7,
        // State 4
        10, 11, 0,
        // State 5
        13, 14, 15,
        // State 6
        16, 17, 18,
        // State 7
        -43, -43, -43,
        // State 8
        -35, -35, -35,
        // State 9
        22, 23, 24,
        // State 10
        25, 26, 27,
        // State 11
        -38, -38, -38,
        // State 12
        28, 29, 30,
        // State 13
        0, 31, 0,
        // State 14
        32, 33, 34,
        // State 15
        36, 0, 37,
        // State 16
        39, 0, 40,
        // State 17
        42, 0, 43,
        // State 18
        -5, -5, -5,
        // State 19
        22, 45, 24,
        // State 20
        -31, -31, -31,
        // State 21
        -1, -1, -1,
        // State 22
        -27, -27, -27,
        // State 23
        -2, -2, -2,
        // State 24
        -32, -32, -32,
        // State 25
        -34, -34, -34,
        // State 26
        -33, -33, -33,
        // State 27
        22, 48, 24,
        // State 28
        22, 48, 24,
        // State 29
        22, 48, 24,
        // State 30
        -13, -13, -13,
        // State 31
        22, 48, 24,
        // State 32
        -12, -12, -12,
        // State 33
        22, 48, 24,
        // State 34
        -36, -36, -36,
        // State 35
        53, 54, 55,
        // State 36
        56, 0, 57,
        // State 37
        -39, -39, -39,
        // State 38
        58, 0, 59,
        // State 39
        60, 0, 61,
        // State 40
        -37, -37, -37,
        // State 41
        -14, -14, -14,
        // State 42
        -15, -15, -15,
        // State 43
        -6, -6, -6,
        // State 44
        -28, -28, -28,
        // State 45
        22, 62, 24,
        // State 46
        -7, -7, -7,
        // State 47
        -20, -20, -20,
        // State 48
        -9, -9, -9,
        // State 49
        -8, -8, -8,
        // State 50
        -10, -10, -10,
        // State 51
        -11, -11, -11,
        // State 52
        -22, -22, -22,
        // State 53
        -24, -24, -24,
        // State 54
        -23, -23, -23,
        // State 55
        -25, -25, -25,
        // State 56
        -26, -26, -26,
        // State 57
        -16, -16, -16,
        // State 58
        -17, -17, -17,
        // State 59
        -18, -18, -18,
        // State 60
        -19, -19, -19,
        // State 61
        -21, -21, -21,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -29,
        // State 1
        -44,
        // State 2
        -42,
        // State 3
        -30,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        -43,
        // State 8
        -35,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -38,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
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
        -31,
        // State 21
        0,
        // State 22
        -27,
        // State 23
        0,
        // State 24
        -32,
        // State 25
        -34,
        // State 26
        -33,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        -13,
        // State 31
        0,
        // State 32
        -12,
        // State 33
        0,
        // State 34
        -36,
        // State 35
        0,
        // State 36
        0,
        // State 37
        -39,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -37,
        // State 41
        -14,
        // State 42
        -15,
        // State 43
        0,
        // State 44
        -28,
        // State 45
        0,
        // State 46
        -7,
        // State 47
        -20,
        // State 48
        -9,
        // State 49
        -8,
        // State 50
        -10,
        // State 51
        -11,
        // State 52
        -22,
        // State 53
        -24,
        // State 54
        -23,
        // State 55
        -25,
        // State 56
        -26,
        // State 57
        -16,
        // State 58
        -17,
        // State 59
        -18,
        // State 60
        -19,
        // State 61
        -21,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 4, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        19, 0, 20, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        19, 0, 46, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        19, 0, 46, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        19, 0, 46, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        19, 0, 46, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        19, 0, 46, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"" ""###,
            r###""\\n""###,
            r###""\\t""###,
        ];
        __ACTION[(__state * 3)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ProgramParser {
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            ProgramParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<Error=lexer::LexicalError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<::std::vec::Vec<ast::Stmt>, __lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    lexer::Tok::Space if true => 0,
                    lexer::Tok::Linefeed if true => 1,
                    lexer::Tok::Tab if true => 2,
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
                    let __action = __ACTION[__state * 3 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ lexer::Tok::Space => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ lexer::Tok::Linefeed => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ lexer::Tok::Tab => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<::std::vec::Vec<ast::Stmt>,__lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                // Digit = " " => ActionFn(31);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(__sym0);
                let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                (1, __symbol, 0)
            }
            2 => {
                // Digit = "\\t" => ActionFn(32);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(__sym0);
                let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                (1, __symbol, 0)
            }
            3 => {
                // Digit* =  => ActionFn(33);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action33::<>(&__start, &__end);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (0, __symbol, 1)
            }
            4 => {
                // Digit* = Digit+ => ActionFn(34);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(__sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 1)
            }
            5 => {
                // Digit+ = Digit => ActionFn(39);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(__sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 2)
            }
            6 => {
                // Digit+ = Digit+, Digit => ActionFn(40);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action40::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (2, __symbol, 2)
            }
            7 => {
                // FlowCtrl = " ", " ", Label => ActionFn(18);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (3, __symbol, 3)
            }
            8 => {
                // FlowCtrl = " ", "\\t", Label => ActionFn(19);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (3, __symbol, 3)
            }
            9 => {
                // FlowCtrl = " ", "\\n", Label => ActionFn(20);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (3, __symbol, 3)
            }
            10 => {
                // FlowCtrl = "\\t", " ", Label => ActionFn(21);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (3, __symbol, 3)
            }
            11 => {
                // FlowCtrl = "\\t", "\\t", Label => ActionFn(22);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (3, __symbol, 3)
            }
            12 => {
                // FlowCtrl = "\\t", "\\n" => ActionFn(23);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 3)
            }
            13 => {
                // FlowCtrl = "\\n", "\\n" => ActionFn(24);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 3)
            }
            14 => {
                // HeapOp = " " => ActionFn(16);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (1, __symbol, 4)
            }
            15 => {
                // HeapOp = "\\t" => ActionFn(17);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(__sym0);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (1, __symbol, 4)
            }
            16 => {
                // Io = " ", " " => ActionFn(25);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 5)
            }
            17 => {
                // Io = " ", "\\t" => ActionFn(26);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 5)
            }
            18 => {
                // Io = "\\t", " " => ActionFn(27);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 5)
            }
            19 => {
                // Io = "\\t", "\\t" => ActionFn(28);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 5)
            }
            20 => {
                // Label = "\\n" => ActionFn(41);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(__sym0);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (1, __symbol, 6)
            }
            21 => {
                // Label = Digit+, "\\n" => ActionFn(42);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action42::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (2, __symbol, 6)
            }
            22 => {
                // MathOp = " ", " " => ActionFn(11);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 7)
            }
            23 => {
                // MathOp = " ", "\\t" => ActionFn(12);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 7)
            }
            24 => {
                // MathOp = " ", "\\n" => ActionFn(13);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 7)
            }
            25 => {
                // MathOp = "\\t", " " => ActionFn(14);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 7)
            }
            26 => {
                // MathOp = "\\t", "\\t" => ActionFn(15);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 7)
            }
            27 => {
                // Number = "\\n" => ActionFn(43);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action43::<>(__sym0);
                let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                (1, __symbol, 8)
            }
            28 => {
                // Number = Digit+, "\\n" => ActionFn(44);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action44::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                (2, __symbol, 8)
            }
            29 => {
                // Program =  => ActionFn(45);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action45::<>(&__start, &__end);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (0, __symbol, 9)
            }
            30 => {
                // Program = Statement+ => ActionFn(46);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action46::<>(__sym0);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (1, __symbol, 9)
            }
            31 => {
                // StackOp = " ", Number => ActionFn(7);
                let __sym1 = __pop_Variant5(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 10)
            }
            32 => {
                // StackOp = "\\n", " " => ActionFn(8);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 10)
            }
            33 => {
                // StackOp = "\\n", "\\t" => ActionFn(9);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 10)
            }
            34 => {
                // StackOp = "\\n", "\\n" => ActionFn(10);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 10)
            }
            35 => {
                // Statement = " ", StackOp => ActionFn(2);
                let __sym1 = __pop_Variant3(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action2::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 11)
            }
            36 => {
                // Statement = "\\t", " ", MathOp => ActionFn(3);
                let __sym2 = __pop_Variant3(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3::<>(__sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (3, __symbol, 11)
            }
            37 => {
                // Statement = "\\t", "\\t", HeapOp => ActionFn(4);
                let __sym2 = __pop_Variant3(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4::<>(__sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (3, __symbol, 11)
            }
            38 => {
                // Statement = "\\n", FlowCtrl => ActionFn(5);
                let __sym1 = __pop_Variant3(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 11)
            }
            39 => {
                // Statement = "\\t", "\\n", Io => ActionFn(6);
                let __sym2 = __pop_Variant3(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(__sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (3, __symbol, 11)
            }
            40 => {
                // Statement* =  => ActionFn(35);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action35::<>(&__start, &__end);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (0, __symbol, 12)
            }
            41 => {
                // Statement* = Statement+ => ActionFn(36);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(__sym0);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (1, __symbol, 12)
            }
            42 => {
                // Statement+ = Statement => ActionFn(37);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(__sym0);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (1, __symbol, 13)
            }
            43 => {
                // Statement+ = Statement+, Statement => ActionFn(38);
                let __sym1 = __pop_Variant3(__symbols);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(__sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (2, __symbol, 13)
            }
            44 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
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
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Int, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Stmt, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Tok, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, u8, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Stmt>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<u8>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Program::ProgramParser;

fn __action0<
>(
    (_, __0, _): (usize, ::std::vec::Vec<ast::Stmt>, usize),
) -> ::std::vec::Vec<ast::Stmt>
{
    (__0)
}

fn __action1<
>(
    (_, __0, _): (usize, ::std::vec::Vec<ast::Stmt>, usize),
) -> ::std::vec::Vec<ast::Stmt>
{
    (__0)
}

fn __action2<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, ast::Stmt, usize),
) -> ast::Stmt
{
    (__0)
}

fn __action3<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, ast::Stmt, usize),
) -> ast::Stmt
{
    (__0)
}

fn __action4<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, ast::Stmt, usize),
) -> ast::Stmt
{
    (__0)
}

fn __action5<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, ast::Stmt, usize),
) -> ast::Stmt
{
    (__0)
}

fn __action6<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, ast::Stmt, usize),
) -> ast::Stmt
{
    (__0)
}

fn __action7<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, ast::Int, usize),
) -> ast::Stmt
{
    ast::Stmt::Push(__0)
}

fn __action8<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Dup
}

fn __action9<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Swap
}

fn __action10<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Discard
}

fn __action11<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Add
}

fn __action12<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Sub
}

fn __action13<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Mul
}

fn __action14<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Div
}

fn __action15<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Mod
}

fn __action16<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Store
}

fn __action17<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Load
}

fn __action18<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, String, usize),
) -> ast::Stmt
{
    ast::Stmt::Mark(__0)
}

fn __action19<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, String, usize),
) -> ast::Stmt
{
    ast::Stmt::Call(__0)
}

fn __action20<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, String, usize),
) -> ast::Stmt
{
    ast::Stmt::Jump(__0)
}

fn __action21<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, String, usize),
) -> ast::Stmt
{
    ast::Stmt::Jz(__0)
}

fn __action22<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, String, usize),
) -> ast::Stmt
{
    ast::Stmt::Js(__0)
}

fn __action23<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Return
}

fn __action24<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::Exit
}

fn __action25<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::PrintChar
}

fn __action26<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::PrintNum
}

fn __action27<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::ReadChar
}

fn __action28<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
    (_, __1, _): (usize, lexer::Tok, usize),
) -> ast::Stmt
{
    ast::Stmt::ReadNum
}

fn __action29<
>(
    (_, __0, _): (usize, ::std::vec::Vec<u8>, usize),
    (_, _, _): (usize, lexer::Tok, usize),
) -> ast::Int
{
    ast::number(__0)
}

fn __action30<
>(
    (_, __0, _): (usize, ::std::vec::Vec<u8>, usize),
    (_, _, _): (usize, lexer::Tok, usize),
) -> String
{
    ast::label(__0)
}

fn __action31<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> u8
{
    0
}

fn __action32<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> u8
{
    1
}

fn __action33<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<u8>
{
    vec![]
}

fn __action34<
>(
    (_, v, _): (usize, ::std::vec::Vec<u8>, usize),
) -> ::std::vec::Vec<u8>
{
    v
}

fn __action35<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<ast::Stmt>
{
    vec![]
}

fn __action36<
>(
    (_, v, _): (usize, ::std::vec::Vec<ast::Stmt>, usize),
) -> ::std::vec::Vec<ast::Stmt>
{
    v
}

fn __action37<
>(
    (_, __0, _): (usize, ast::Stmt, usize),
) -> ::std::vec::Vec<ast::Stmt>
{
    vec![__0]
}

fn __action38<
>(
    (_, v, _): (usize, ::std::vec::Vec<ast::Stmt>, usize),
    (_, e, _): (usize, ast::Stmt, usize),
) -> ::std::vec::Vec<ast::Stmt>
{
    { let mut v = v; v.push(e); v }
}

fn __action39<
>(
    (_, __0, _): (usize, u8, usize),
) -> ::std::vec::Vec<u8>
{
    vec![__0]
}

fn __action40<
>(
    (_, v, _): (usize, ::std::vec::Vec<u8>, usize),
    (_, e, _): (usize, u8, usize),
) -> ::std::vec::Vec<u8>
{
    { let mut v = v; v.push(e); v }
}

fn __action41<
>(
    __0: (usize, lexer::Tok, usize),
) -> String
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action33(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        __temp0,
        __0,
    )
}

fn __action42<
>(
    __0: (usize, ::std::vec::Vec<u8>, usize),
    __1: (usize, lexer::Tok, usize),
) -> String
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action34(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        __temp0,
        __1,
    )
}

fn __action43<
>(
    __0: (usize, lexer::Tok, usize),
) -> ast::Int
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action33(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        __temp0,
        __0,
    )
}

fn __action44<
>(
    __0: (usize, ::std::vec::Vec<u8>, usize),
    __1: (usize, lexer::Tok, usize),
) -> ast::Int
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action34(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        __temp0,
        __1,
    )
}

fn __action45<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<ast::Stmt>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action35(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

fn __action46<
>(
    __0: (usize, ::std::vec::Vec<ast::Stmt>, usize),
) -> ::std::vec::Vec<ast::Stmt>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action36(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,lexer::Tok,usize),Self::Error>;
}

impl<> __ToTriple<> for (usize, lexer::Tok, usize) {
    type Error = lexer::LexicalError;
    fn to_triple(value: Self) -> Result<(usize,lexer::Tok,usize),lexer::LexicalError> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, lexer::Tok, usize),lexer::LexicalError> {
    type Error = lexer::LexicalError;
    fn to_triple(value: Self) -> Result<(usize,lexer::Tok,usize),lexer::LexicalError> {
        value
    }
}
