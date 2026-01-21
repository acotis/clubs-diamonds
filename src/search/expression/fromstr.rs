
use std::str::FromStr;

use crate::Expression;
use crate::Number;

impl <N: Number, const C: usize> FromStr for Expression<N, C> {
    type Err = ();

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
    /// "a*a-32".parse::<Expression<i32, 2>>().unwrap(); // works (the second input variable is unused)
    /// "a<<b-1".parse::<Expression<i32, 2>>().unwrap(); // works
    /// "a<<b-1".parse::<Expression<i32, 1>>().unwrap(); // bad — the returned expression crashes whenever `.apply()` is called
    ///
    /// ```
    ///
    /// In this version of the crate, the error type returned if an expression
    /// fails to parse is just `()`, the unit type. The method doesn't give any
    /// details about what failed because in 99% of cases it's completely
    /// obvious if you just look at the expression that didn't parse.

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Err(())
    }
}

/*

use PartialParse::*;

enum PartialParse {
    Token(String),
    PartialExpression(Vec<Pivot>),
}

impl <N: Number, const C: usize> Expression<N, C> {

    pub fn from_str_and_var_names(s: &str, var_names: [char; C]) -> Result<Self, ()> {
        static SYMBOLS: &[&str] = &[
            "(", ")", "!", "*", "/", "%", "+", "-", "<<", ">>", "&", "^", "|",
        ];

        let mut tokens = vec![];
        let mut remaining_to_tokenize = s;

        'tokenize: while remaining_to_tokenize != "" {

            // If there is a parenthesis or operator at the start, add that
            // token into the token list.

            for symbol in SYMBOLS {
                if remaining_to_tokenize.starts_with(symbol) {
                    tokens.push(Token(format!("{symbol}")));
                    remaining_to_tokenize = &remaining_to_tokenize[symbol.len()..];
                    continue 'tokenize;
                }
            }

            // If there is a variable at the start, add that token into the
            // token list.

            for var in var_names {
                if remaining_to_tokenize.starts_with(*var) {
                    tokens.push(Token(format!("{var}")));
                    remaining_to_tokenize = &remaining_to_tokenize[1..];
                    continue 'tokenize;
                }
            }

            // If there is a number at the start, add an expression that builds
            // that number into the token list.

            let number_digits = remaining_to_tokenize
                .chars()
                .take_while(char::is_ascii_digit)
                .map(|d| ConstPivot(d.to_digit(10).unwrap() as u8))
                .collect::<Vec<_>>();
            
            if !number_digits.is_empty() {
                remaining_to_tokenize = &remaining_to_tokenize[number_digits.len()..];
                tokens.push(PartialExpression(number_digits));
                continue 'tokenize;
            }

            // If none of these things were found at the start, tokenization
            // has failed and we return an error.

            return Err(format!("can't parse a valid token from the start of this string: {remaining_to_tokenize}"));
        }

        Err(format!("not implemented"))
        //Self::from_tokens(
    }

}
*/

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




/* TODO: implement this later
 *
 *
 *
    /// Parse an expression from a string, inferring and returning the names
    /// of the variables which appear in it. Note that Clubs still needs to be
    /// told the arity and variable type of the expression it is parsing. 
    ///
    /// This method returns both the parsed expression and a Vec listing the
    /// variable names that Clubs found in the expression. The number of
    /// variable names may be less, but can't be more, than the declared arity
    /// of the expression. (If it finds more variables than the declared arity,
    /// this method errors.)
    ///
    /// The variable names will appear in the order in which inputs are accepted
    /// by the resulting expression, i.e., if the returned Vec is
    /// `['a', 'x', 'y']`, then the returned Expression's first input will be
    /// used as the variable 'a', its second input will be used as the variable
    /// 'x', and its third input will be used as the variable 'y'. The order
    /// that Clubs chooses here will always be alphabetical order.

    pub fn from_str(s: &str) -> Result<(Self, Vec<char>), String> {

        // This method simply extracts the actual variable names appearing
        // in the string and passes them on to .parse_with_var_names().

        let mut var_names = s.chars().filter(char::is_ascii_lowercase).collect::<Vec<_>>();

        var_names.sort();
        var_names.dedup();
        
        Self::parse_with_var_names(s, &var_names).map(|expr| (expr, var_names))
    }
*/
