
use std::str::FromStr;
use std::cmp::Ordering;
use std::marker::PhantomData;

use crate::Expression;
use crate::Number;
use crate::search::pivot::ALL_OPS;
use crate::search::pivot::Pivot::*;
use crate::search::Op;

use PartialParse::*;

#[derive(Clone)]
enum PartialParse {
    Op(Op),
    PartialExpression(Vec<u8>),
}

impl <N: Number, const C: usize> FromStr for Expression<N, C> {
    type Err = String;

    /// Parse an expression from a string.
    ///
    /// In the string, the variable `a` will taken to mean the resulting
    /// Expression's first input variable, `b` the second, and so on. If
    /// the arity you specify isn't enough to cover all input varaibles that
    /// actually appear (like if you do `s.parse::<Expression<i32, 2>>()`
    /// on a string that contains a `c`) then the resulting expression will
    /// be unusable. The reverse is fine; calling
    /// `s.parse::<Expression<i32, 2>>()` just yields an Expression that
    /// doesn't use its second input variable.
    ///
    /// ```
    /// use clubs_diamonds::Expression;
    ///
    /// "a*a-32".parse::<Expression<i32, 1>>().unwrap(); // works
    /// "a*a-32".parse::<Expression<i32, 2>>().unwrap(); // works (the Expression's second input variable is unused)
    /// "a<<b-1".parse::<Expression<i32, 2>>().unwrap(); // works
    /// "a<<b-1".parse::<Expression<i32, 1>>().unwrap(); // bad — the returned Expression crashes whenever `.apply()` is called
    ///
    /// ```
    ///
    /// The error type for a failed parse is a String explaining in words
    /// what went wrong.

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = vec![];
        let mut remaining_to_tokenize = s;

        'tokenize: while remaining_to_tokenize != "" {

            // If there is a variable at the start, create a PartialExpression
            // for it directly.

            for v in 0..26 {
                if remaining_to_tokenize.starts_with(&"abcdefghijklmnopqrstuvwxyz"[v..=v]) {
                    tokens.push(PartialExpression(vec![VarPivot(v as u8).encode()]));
                    remaining_to_tokenize = &remaining_to_tokenize[1..];
                    continue 'tokenize;
                }
            }

            // If there is a number at the start, create a PartialExpression
            // for it directly.

            let number_len = remaining_to_tokenize
                .chars()
                .take_while(char::is_ascii_digit)
                .count();
            
            if number_len > 0 {
                let number_value = remaining_to_tokenize
                    .chars()
                    .take_while(char::is_ascii_digit)
                    .map(|d| d.to_digit(10).unwrap() as u128)
                    .fold(0, |n, d| n*10 + d);
                
                if number_value > 155 {
                    return Err(format!("constants above 155 are not supported in Expressions"));
                }

                remaining_to_tokenize = &remaining_to_tokenize[number_len..];
                tokens.push(PartialExpression(vec![ConstPivot(number_value as u8).encode()]));
                continue 'tokenize;
            }

            // If there is a symbol at the start, create an Op for it to
            // be dealt with later.

            for op in ALL_OPS {

                // Unary ops can't come directly after variables, numbers, or
                // parenthesized sub-expressions, and we need to acknowledge
                // that now because otherwise we could confuse unary "-" for
                // binary "-".

                if matches!(tokens.last(), Some(PartialExpression(_))) && op.arity() == 1 {
                    continue;
                }

                if remaining_to_tokenize.starts_with(op.render_face()) {
                    tokens.push(Op(*op));
                    remaining_to_tokenize = &remaining_to_tokenize[op.render_face().len()..];
                    continue 'tokenize;
                }
            }
            
            // If there is an open parenthesis at the start, read forward
            // tothe matching parenthesis, recurse, and create a
            // PartialExpression containing the result.

            if remaining_to_tokenize.starts_with("(") {
                let mut depth = 1;
                let mut finger = 1;
                
                while finger < remaining_to_tokenize.len() {
                    match &remaining_to_tokenize[finger..=finger] {
                        "(" => {depth += 1;}
                        ")" => {depth -= 1;}
                        _ => {/* do nothing */}
                    }

                    if depth == 0 {
                        break;
                    }

                    finger += 1;
                }

                if depth != 0 {
                    return Err(format!("mismatched parentheses"));
                }

                match remaining_to_tokenize[1..finger].parse::<Self>() {
                    Err(err) => {
                        return Err(err);
                    }
                    Ok(expr) => {
                        tokens.push(PartialExpression(expr.field));
                        remaining_to_tokenize = &remaining_to_tokenize[finger+1..];
                    }
                }
            }

            // If none of these things were found at the start, tokenization
            // has failed and we return an error.

            return Err(format!("can't parse a valid token from the start of this string: {remaining_to_tokenize}"));
        }

        // Now we must take the operators and partial expressions we have
        // just parsed and assemble them into a full expression. In each
        // iteration, we single out either the final unary operator, or the
        // first binary operator of highest precedence, to be next up for
        // processing.

        while let Some((index, Op(op))) =
            tokens.clone().into_iter()
                .enumerate()
                .filter(|(_index, token)| matches!(token, Op(_)))
                .max_by(|(_index_1, token_1), (_index_2, token_2)| {
                    let Op(op_1) = token_1 else {panic!()};
                    let Op(op_2) = token_2 else {panic!()};

                    if op_1.prec() > op_2.prec() {return Ordering::Greater;}
                    if op_1.prec() < op_2.prec() {return Ordering::Less;}
                    if op_1.arity() == 1 {return Ordering::Greater;}
                    Ordering::Less
                })
        {
            if op.arity() == 1 {
                if index == tokens.len() - 1 {return Err(format!("invalid syntax"));}

                let _ = tokens.remove(index);
                let PartialExpression(mut field) = tokens.remove(index) else {return Err(format!("invalid syntax"))};

                field.push(OpPivot(op).encode());
                tokens.insert(index, PartialExpression(field));
            } else {
                if index == tokens.len() - 1 {return Err(format!("invalid syntax"));}
                if index == 0                {return Err(format!("invalid syntax"));}

                let _ = tokens.remove(index);
                let PartialExpression(mut field_left)  = tokens.remove(index-1) else {return Err(format!("invalid syntax"))};
                let PartialExpression(mut field_right) = tokens.remove(index-1) else {return Err(format!("invalid syntax"))};

                field_left.extend(field_right);
                field_left.push(OpPivot(op).encode());
                tokens.insert(index-1, PartialExpression(field_left));
            }
        }
        
        // If all those operations brought us down to a single PartialExpression
        // unit, we're done.

        if let [PartialExpression(field)] = &tokens[..] {
            return Ok(Self {
                field: field.clone(),
                nothing: PhantomData,
            })
        }

        // Otherwise, there was invalid syntax somewhere.

        return Err(format!("invalid syntax"));
    }
}

// Expression-parsing tests: basic operators.

#[test] fn parse_001() {roundtrip_parse_test::<i32, 1>("a");}
#[test] fn parse_002() {roundtrip_parse_test::<i32, 1>("17");}
#[test] fn parse_003() {roundtrip_parse_test::<i32, 1>("!a");}
#[test] fn parse_004() {roundtrip_parse_test::<i32, 1>("!17");}
#[test] fn parse_005() {roundtrip_parse_test::<i32, 1>("-a");}
#[test] fn parse_006() {roundtrip_parse_test::<i32, 1>("-17");}
#[test] fn parse_007() {roundtrip_parse_test::<i32, 1>("a*17");}
#[test] fn parse_008() {roundtrip_parse_test::<i32, 1>("17*a");}
#[test] fn parse_009() {roundtrip_parse_test::<i32, 1>("a/17");}
#[test] fn parse_010() {roundtrip_parse_test::<i32, 1>("17/a");}
#[test] fn parse_011() {roundtrip_parse_test::<i32, 1>("a%17");}
#[test] fn parse_012() {roundtrip_parse_test::<i32, 1>("17%a");}
#[test] fn parse_013() {roundtrip_parse_test::<i32, 1>("a+17");}
#[test] fn parse_014() {roundtrip_parse_test::<i32, 1>("17+a");}
#[test] fn parse_015() {roundtrip_parse_test::<i32, 1>("a-17");}
#[test] fn parse_016() {roundtrip_parse_test::<i32, 1>("17-a");}
#[test] fn parse_017() {roundtrip_parse_test::<i32, 1>("a<<17");}
#[test] fn parse_018() {roundtrip_parse_test::<i32, 1>("17<<a");}
#[test] fn parse_019() {roundtrip_parse_test::<i32, 1>("a>>17");}
#[test] fn parse_020() {roundtrip_parse_test::<i32, 1>("17>>a");}
#[test] fn parse_021() {roundtrip_parse_test::<i32, 1>("a&17");}
#[test] fn parse_022() {roundtrip_parse_test::<i32, 1>("17&a");}
#[test] fn parse_023() {roundtrip_parse_test::<i32, 1>("a^17");}
#[test] fn parse_024() {roundtrip_parse_test::<i32, 1>("17^a");}
#[test] fn parse_025() {roundtrip_parse_test::<i32, 1>("a|17");}
#[test] fn parse_026() {roundtrip_parse_test::<i32, 1>("17|a");}

// Expression-parsing tests: multiple operators: neighboring precedences.

#[test] fn parse_027() {roundtrip_parse_test::<i32, 1>("17|a+3");}
#[test] fn parse_028() {roundtrip_parse_test::<i32, 1>("4<<9+a");}
#[test] fn parse_029() {roundtrip_parse_test::<i32, 1>("!a*5");}
#[test] fn parse_030() {roundtrip_parse_test::<i32, 1>("a*-5");}
#[test] fn parse_031() {roundtrip_parse_test::<i32, 1>("-5/a");}
#[test] fn parse_032() {roundtrip_parse_test::<i32, 1>("a%-a");}
#[test] fn parse_033() {roundtrip_parse_test::<i32, 1>("a*3/5");}
#[test] fn parse_034() {roundtrip_parse_test::<i32, 1>("a%3/5");}
#[test] fn parse_035() {roundtrip_parse_test::<i32, 1>("8/8%1");}
#[test] fn parse_036() {roundtrip_parse_test::<i32, 1>("a/4+1");}
#[test] fn parse_037() {roundtrip_parse_test::<i32, 1>("2-9%2");}
#[test] fn parse_038() {roundtrip_parse_test::<i32, 1>("a*a+a");}
#[test] fn parse_039() {roundtrip_parse_test::<i32, 1>("3<<a+9");}
#[test] fn parse_040() {roundtrip_parse_test::<i32, 1>("4+7>>a");}
#[test] fn parse_041() {roundtrip_parse_test::<i32, 1>("a&1<<a");}
#[test] fn parse_042() {roundtrip_parse_test::<i32, 1>("a>>4&2");}
#[test] fn parse_043() {roundtrip_parse_test::<i32, 1>("a^a&3");}
#[test] fn parse_044() {roundtrip_parse_test::<i32, 1>("3&a^5");}
#[test] fn parse_045() {roundtrip_parse_test::<i32, 1>("a|a^5");}
#[test] fn parse_046() {roundtrip_parse_test::<i32, 1>("a^5|3");}

// Expression-parsing tests: multiple operators: random stuff.

#[test] fn parse_047() {roundtrip_parse_test::<i32, 1>("1<<a+a*a");}
#[test] fn parse_048() {roundtrip_parse_test::<i32, 1>("!0/a+!5/a+!18/a");}
#[test] fn parse_049() {roundtrip_parse_test::<i32, 1>("155&a&4&1&2&9");}
#[test] fn parse_050() {roundtrip_parse_test::<i32, 1>("0|0^0&0>>0<<0+0-0*0%0/0");}
#[test] fn parse_051() {roundtrip_parse_test::<i32, 1>("!-!-!-4");}
#[test] fn parse_052() {roundtrip_parse_test::<i32, 1>("!!!4");}
#[test] fn parse_053() {roundtrip_parse_test::<i32, 1>("---4");}

// Expression-parsing tests: parentheses.

#[test] fn parse_054() {roundtrip_parse_test::<i32, 1>("a*(a+5)");}
#[test] fn parse_055() {roundtrip_parse_test::<i32, 1>("a+(3|9)");}
#[test] fn parse_056() {roundtrip_parse_test::<i32, 1>("!(a<<9>>4)");}
#[test] fn parse_057() {roundtrip_parse_test::<i32, 1>("-(a<<9>>4)");}
#[test] fn parse_058() {roundtrip_parse_test::<i32, 1>("1*(1/(1%(1-(1+(1<<(1>>(1&(1^(1|1)))))))))");}
#[test] fn parse_059() {roundtrip_parse_test::<i32, 1>("4>>(a<<3)");}

// Expression-parsing tests: multiple variables.

#[test] fn parse_060() {roundtrip_parse_test::<i32, 2>("a+b");}
#[test] fn parse_061() {roundtrip_parse_test::<i32, 2>("a+b*b");}
#[test] fn parse_062() {roundtrip_parse_test::<i32, 2>("a*a+b");}
#[test] fn parse_063() {roundtrip_parse_test::<i32, 2>("a*(a+b)");}
#[test] fn parse_064() {roundtrip_parse_test::<i32, 3>("c*(c<<c)");}

// Test that when we parse <input> into an Expression and then
// render that Expression, we get <expected_output>.

#[cfg(test)]
fn parse_test<N: Number, const C: usize>(input: &str, expected_output: &str) {
    let expr = input.parse::<Expression<N, C>>().unwrap();
    let output = format!("{expr}");
    assert_eq!(expected_output, &output);
}

#[cfg(test)]
fn roundtrip_parse_test<N: Number, const C: usize>(input: &str) {
    parse_test::<N, C>(input, input);
}

