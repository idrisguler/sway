/// Represents the high level language parser generated by [pest].
/// [Rule] is also generated here by [pest].
#[derive(Parser)]
#[grammar = "hll.pest"]
pub struct HllParser;

#[cfg(test)]
mod test {
    use super::{HllParser, Rule};
    use pest::Parser;
    // basic sway-core tests
    #[test]
    fn test_var_decl() {
        let parsed = HllParser::parse(Rule::var_decl, r#"let x = 2;"#.into());
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }

    #[test]
    fn test_comment() {
        let parsed = HllParser::parse(
            Rule::var_decl,
            r#"let x = 2; // and a comment

        /* and a multiline comment
         * second line */"#
                .into(),
        );
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }

    #[test]
    fn test_fn_decl() {
        let parsed = HllParser::parse(
            Rule::fn_decl,
            r#"fn myfunc(x: i32, y: i32) -> i32 {
            // a function body
            let x = 5;
            let y = 10;
            return 10;
        }"#
            .into(),
        );
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }

    #[test]
    fn test_if_else_expr() {
        let parsed = HllParser::parse(
            Rule::fn_decl,
            r#"fn myfunc(x: i32, y: i32) -> i32 {
            // a function body
            let x = if
                true
                { 5 }
                else { 6 };
            let y = 10;
            return 10;
        }"#
            .into(),
        );
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }

    #[test]
    fn test_if_expr() {
        let parsed = HllParser::parse(
            Rule::fn_decl,
            r#"fn myfunc(x: i32, y: i32) -> i32 {
            // a function body
            if true { /* comment */ 5 /*comment test*/ };
            /* some comments */
        }"#
            .into(),
        );
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }

    #[test]
    fn test_if_else_expr_2() {
        let parsed = HllParser::parse(
            Rule::fn_decl,
            r#"fn myfunc(x: i32, y: i32) -> i32 {
            // a function body
            if ((true)) { /* comment */ (((5))) /*comment test*/ };
            /* some comments */
        }"#
            .into(),
        );
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }
    #[test]
    fn mismatched_parens() {
        let parsed = HllParser::parse(
            Rule::fn_decl,
            r#"fn myfunc(x: i32, y: i32) -> i32 {
            // a function body
            if ((true)) { /* comment */ (((5)) };
        }"#
            .into(),
        );
        // this parse should fail since parens are wrong
        match parsed {
            Err(_) => (),
            Ok(o) => {
                panic!("{:?}", o)
            }
        }
    }

    #[test]
    fn parse_infix_op() {
        let parsed = HllParser::parse(
            Rule::fn_decl,
            r#"fn myfunc(x: i32, y: i32) -> i32 {
        let x = 5 + 10;
        }"#
            .into(),
        );
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }

    #[test]
    fn var_exp() {
        let parsed = HllParser::parse(
            Rule::fn_decl,
            r#"fn myfunc(x: i32, y: i32) -> i32 {
        let x = 5 + 10;
        let foo = 20;
        let y = (x + foo) - x ;
        return y;
        }"#
            .into(),
        );
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }
    #[test]
    fn var_exp_triple_op() {
        let parsed = HllParser::parse(
            Rule::fn_decl,
            r#"fn myfunc(x: i32, y: i32) -> i32 {
        let x = 5 + 10;
        let foo = 20;
        let y = (x + foo + 3) - x ;
        return y;
        }"#
            .into(),
        );
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }
    #[test]
    fn trait_decl_unimplemented_method() {
        let parsed = HllParser::parse(
            Rule::program,
            r#"script;
                trait MyTrait {
                    fn some_method_you_need_to_implement(x: i32) -> i32;
            }"#
            .into(),
        );
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }
    #[test]
    fn trait_decl_unimplemented_and_implemented() {
        let parsed = HllParser::parse(
            Rule::program,
            r#"
           predicate;
            trait MyTrait {
                fn some_method_you_need_to_implement(x: i32) -> i32;
            } {
                fn some_method_that_the_trait_implements(x: i32) -> i32 {
                    let x = 5;
                    return x;
                }
                }

            "#
            .into(),
        );
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }
    #[test]
    fn import_statement() {
        let parsed = HllParser::parse(
            Rule::use_statement,
            r#"use otherlibrary::packagename;
            "#
            .into(),
        );
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }
    #[test]
    fn import_statement_2() {
        let parsed = HllParser::parse(
            Rule::program,
            r#"
            contract;
                use otherlibrary::packagename;
                fn main(){
                let x = 5;
                return x;
                
            }
            "#
            .into(),
        );
        // this parse should fail since parens are wrong
        if let Err(e) = parsed {
            panic!("{:#?}", e);
        }
    }
    #[test]
    fn byte_literals() {
        let parsed = HllParser::parse(
            Rule::program,
            r#"
            script;
                fn main(){
                    let x = 0b01011010;
                    let y = 0xAF;
                    return 0;
            }"#
            .into(),
        );
        // this parse should fail since parens are wrong
        parsed.unwrap();
    }
    #[test]
    fn bytes_literals() {
        let parsed = HllParser::parse(
            Rule::program,
            r#"
            predicate;
                fn main(){
                    let x = 0b01011010;
                    // 32 bytes in a bytes32
                    let y = 0xAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAF;
                    return x;
                }
            "#
            .into(),
        );
        // this parse should fail since parens are wrong
        parsed.unwrap();
    }

    #[test]
    #[should_panic]
    fn multiple_programs() {
        let parsed = HllParser::parse(
            Rule::program,
            r#"
            predicate;
                fn main(){
                    let x = 0b01011010;
                    // 32 bytes in a bytes32
                    let y = 0xAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAF;
                    return x;
                }
            predicate;
                fn main(){
                    let x = 0b01011010;
                    // 32 bytes in a bytes32
                    let y = 0xAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAFAF;
                    return x;
                }
            "#
            .into(),
        );
        parsed.unwrap();
    }

    #[test]
    fn test_filename() {
        let parsed = HllParser::parse(
            Rule::fn_decl,
            r#"fn myfunc(x: i32, y: i32) -> i32 {
            // a function body
            let x = 5;
            let y = 10;
            return 10;
        }"#
            .into(),
        );
        parsed.unwrap();
    }
}
