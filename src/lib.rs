
//! A brute-forcer for finding short mathematical expressions in Rust, for code golf. May also be useful for golfing in other languages, if the language's syntax for math is similar enough to Rust's.
//!
//! This crate provides the following types:
//!
//! - [`Expression`]: A struct representing a mathematical expression, such as `3*(a+5)` or `b>>a|89%c`. Uses an optimized representation internally, but adheres to Rust syntax, Rust operators, Rust precedence levels, and Rust semantics.
//! - [`Searcher`]: A configurable searcher which can be used to systematically search all syntactically valid expressions in increasing order of length, and yield only those which meet a customizeable, user-specified criterion.
//!
//! # Performance note
//!
//! Clubs runs around **10 times faster in release mode** than in debug mode. You can run your code in release mode by executing it with the following command, and you should do this unless you have a great reason not to:
//!
//! ```txt
//! cargo run --release
//! ```
//!
//! Additionally, I recommend enabling [link-time optimizations](https://doc.rust-lang.org/cargo/reference/profiles.html#lto) by putting the following snippet in your `Cargo.toml`. This speeds Clubs up by a further 40% or so:
//!
//! ```toml
//! [profile.release]
//! lto = "fat"
//! ```
//!
//! # Basic example
//!
//! Suppose you want to find an expression `f` in a single variable `a` such that, when you plug in the values 1 through 5 for the variable, the expression yields the first five prime numbers. In other words, you want:
//!
//! - f(1) = 2
//! - f(2) = 3
//! - f(3) = 5
//! - f(4) = 7
//! - f(5) = 11
//!
//! You can ask Clubs to find you such an expression using the following complete program:
//!
//! ```
//! use clubs_diamonds::{Searcher, Expression};
//! 
//! fn main() {
//!     let solutions =
//!         Searcher::<i32, 1>::new(|expr: &Expression::<i32, 1>| {
//!             expr.apply(&[1]) == Some(2) &&
//!             expr.apply(&[2]) == Some(3) &&
//!             expr.apply(&[3]) == Some(5) &&
//!             expr.apply(&[4]) == Some(7) &&
//!             expr.apply(&[5]) == Some(11)
//!         })
//!         .threads(3)
//!         .run_with_ui();
//! 
//!     println!("The first three solutions we found were:");
//!     println!("    — {}", solutions[0]);
//!     println!("    — {}", solutions[1]);
//!     println!("    — {}", solutions[2]);
//! 
//!     // solutions is a Vec<Expression::<i32, 1>>
//! }
//! ```
//!
//! Executing the above code brings up a text-based UI that looks like this:
//!
//! ![A screenshot of the text-based interface of the Clubs expression searcher. On the right, several information boxes tell us what expressions are currently being searched by each of three threads, how long the search has been running for, how many expressions are being searched per second, and other stats. On the left, seven found solutions are listed.][demo]
//!
#![doc = embed_doc_image::embed_image!("demo", "assets/demo_medium.png")]
//!
//! As solutions are found, they will appear in the Solutions column on the left. When you quit the UI, the [`Searcher::run_with_ui()`] method returns, and control flow returns to the main function with a [`Vec`] of all the solutions that were found.
//!
//! # Workflow for performing a search
//!
//! There is a four-step workflow for performing a search using the [`Searcher`] type:
//!
//! 1. Decide the type and number of variables which will appear in the expression.
//!     - Specify these choices as type parameters of the [`Searcher`].
//! 2. Construct the searcher using the method [`Searcher::new()`].
//!     - When you call this method, you supply a closure that accepts an &[`Expression`] and returns a [`bool`]. This is the "judge" that is used to determine which expressions are displayed in the Solutions panel in the UI (and eventually returned in the solutions Vec).
//! 3. Optionally, specify additional parameters for the search by using some of [`Searcher`]'s Builder-Lite methods.
//! 4. Execute the search using either the [`Searcher::run_with_ui()`] method or the [`Searcher::run_silently()`] method.
//!
//! Here is how the steps are followed by the example code above:
//!
//! ```
//! use clubs_diamonds::{Searcher, Expression};
//!
//! // step 1: type and number of variables as type parameters
//! //        ┌──────┐                          ┌──────┐
//! Searcher::<i32, 1>::new(|expr: &Expression::<i32, 1>| {
//!     expr.apply(&[1]) == Some(2) && // ┐
//!     expr.apply(&[2]) == Some(3) && // │
//!     expr.apply(&[3]) == Some(5) && // │ step 2: customized search criterion
//!     expr.apply(&[4]) == Some(7) && // │
//!     expr.apply(&[5]) == Some(11)   // ┘
//! })
//! .threads(3)     // step 3: additional parameters
//! .run_with_ui(); // step 4: execution of the search
//! ```
//!
//! The steps are described in more detail below.
//!
//! ## Step 1a: Number of variables
//!
//! Clubs is capable of finding both single-variable and multi-variable expressions. The number of variables is a type parameter of the [`Searcher`] struct. If you set it to `1`, it will find single-variable expressions. If you set it to `2`, it will find two-variable expressions, etc. Variables are always named with single-letter names, starting with "a" for the first variable, "b" for the second, and so on.
//!
//! For example, a single-variable search will consider expressions like: `3+a`, `a*a*47`, `a%27<<(a&43-a)`, and so on.
//!
//! A two-variable search will consider expressions like: `a+b`, `9*b+73`, `a>>(47%b+a|89)`, and so on.
//!
//! Note that Clubs will consider expressions that do not use all of their input variables, like `9*b+73`, a two-variable expression that doesn't use the variable `a`. In this version of the crate, you cannot control this.
//!
//! ## Step 1b: Type of variables
//!
//! In Rust, every variable has an unchanging type, and Rust is generally a hard-ass about enforcing that types match. If `x` is a `u32` and `y` is an `i128`, then `x*y` is a syntactically invalid expression and will fail to compile with a type error.
//!
//! In Clubs, the [`Searcher`] requires that every variable in an expression has the same type, and this type must be specified as a type parameter of the [`Searcher`] itself. If you set it to `u8`, it will find expressions whose inputs (and output) are `u8`s. If you set it to `isize`, it will find expressions whose inputs (and output) are `isize`s, and so on.
//!
//! If you're a type nerd and want more details, see [Extra details for type
//! nerds](#extra-details-for-type-nerds).
//!
//! ## Step 2: Constructing a Searcher
//!
//! A Searcher is constructed using the [`Searcher::new()`] method, which accepts a closure as its only argument. The closure must accept an &[`Expression`] and return a [`bool`]. This is where you specify your customizeable criterion for expressions to be tested against.
//!
//! Generally speaking, you will judge an expression by calling the [`Expression::apply()`] method and checking things about its return values. This method accepts an array of input variable values and returns the output value the expression evaluates to for those inputs. The return value is wrapped in an [`Option`], and the value `None` is returned when the given inputs would cause the expression to crash with a runtime error (for example, if it ends up dividing by zero).
//!
//! Here is an example of a [`Searcher`] being constructed for a two-variable search using `i16` variables:
//!
//! ```
//! use clubs_diamonds::{Searcher, Expression};
//! 
//! fn main() {
//!     Searcher::<i16, 2>::new(|expr: &Expression::<i16, 2>| {
//!         expr.apply(&[1, 3]) == Some(5) &&
//!         expr.apply(&[5, -2]) == Some(-6) &&
//!         expr.apply(&[-8, 7]) == None
//!     })
//!     .run_with_ui();
//! }
//! ```
//!
//! This [`Searcher`] will consider all expressions containing two `i16` variables, and return only those for which `f(1, 3)` = 5, `f(5, -2)` = -6, and `f(-8, 7)` cannot be evaluated because it would cause Rust to crash.
//!
//! **Golfing tip:** There is room for considerable ingenuity and creativity in specifying the criterion that a [`Searcher`] will apply. It can be any predicate. Using your imagination will take you further than only copying the format of the documented examples.
//!
//! ## Steps 3 and 4: Additional search parameters and execution
//!
//! For a list of [`Searcher`]'s methods, including the ones for specifying additional search parameters and executing the search, see [its documentation page][crate::Searcher].
//!
//! **Note:** If you opt to use the [`Searcher::run_silently()`] method, then there will be no way to quit the search before Clubs decides it's done. So, if you plan to use that method, you probably want to specify a combination of search parameters that make the search task finite.
//!
//! # Inspectors
//!
//! Sometimes, it's useful to know more information about an expression than just that it met the search criterion. For example, you might want to search for expressions that have certain outputs for the input values 1, 2, 3, 4, and 5, but then to actually put an expression to use, you might need to know what its output is for an input of 6.
//!
//! Clubs can be configured to display custom additional information alongside each solution it finds. The way to do this is to use the [`Searcher::inspector`] method. This method accepts a closure whose argument is an &[`Expression`] and whose return value is a [`String`]. Clubs will call this closure once for each solution it finds, and the returned string will be displayed in the "Solution inspector" panel to the right when the solution is highlighted in the UI (use J/K keys to navigate).
//!
//! Here is an example of an inspector being used:
//!
//! ```
//! use clubs_diamonds::{Searcher, Expression};
//! 
//! fn main() {
//!     Searcher::<i32, 1>::new(|expr: &Expression::<i32, 1>| {
//!         expr.apply(&[1]) == Some(2) &&
//!         expr.apply(&[2]) == Some(3) &&
//!         expr.apply(&[3]) == Some(5) &&
//!         expr.apply(&[4]) == Some(7) &&
//!         expr.apply(&[5]) == Some(11)
//!     })
//!     .inspector(|expr: &Expression::<i32, 1>| {
//!         format!("expr(6) = {:?}", expr.apply(&[6]))
//!     })
//!     .threads(3)
//!     .run_with_ui();
//! }
//! ```
//!
//! And here is what it looks like in the UI:
//!
//! ![Another screenshot of Clubs. The fourth solution in the solutions list is highlighted, and the Solution Inspector panel shows us that its output for an input of 6 is Some(11)][inspector]
//!
#![doc = embed_doc_image::embed_image!("inspector", "assets/inspector_medium.png")]
//!
//! # Penalizers
//!
//! By default, Clubs will sort the expressions it discovers in order of length, shortest first, because that's what's usually appropriate for code golf. However, sometimes the length of the expression itself isn't the only thing you care about. The overall program that you're using the expression in may have parts that grow or shrink based on the expression's properties. In that case, the shortest working expression may not be the best one.
//!
//! Clubs can be configured to sort the solutions it finds based on other criteria. The way to do this is using the [`Searcher::penalizer()`] method. This method accepts a closure whose argument is an &[`Expression`] and whose return value is a `usize`. The supplied closure is called a "penalizer". The penalizer is called once for each solution Clubs discovers, and its output is **added to** to the length of each solution to compute the solution's overall score. Solutions are then sorted in order of score, lowest first.
//!
//! As an example, suppose you are searching for a one-variable expression, and your program architecture is such that, if the input variable appears more than once in the expression you choose, another part of the program will grow by 7 bytes. You can ask Clubs to penalize such expressions by 7 bytes like this:
//!
//! ```
//! use clubs_diamonds::{Searcher, Expression};
//! 
//! fn main() {
//!     Searcher::<i32, 1>::new(|expr: &Expression::<i32, 1>| {
//!         expr.apply(&[1]) == Some(2) &&
//!         expr.apply(&[2]) == Some(3) &&
//!         expr.apply(&[3]) == Some(5) &&
//!         expr.apply(&[4]) == Some(7) &&
//!         expr.apply(&[5]) == Some(11)
//!     })
//!     .penalizer(|expr: &Expression::<i32, 1>| {
//!         if format!("{expr}").chars().filter(|&c| c == 'a').count() > 1 {
//!             7
//!         } else {
//!             0
//!         }
//!     })
//!     .threads(3)
//!     .run_with_ui();
//! }
//! ```
//!
//! After running this program for a few minutes, the solutions list will look like this:
//!
//! ![Another screenshot of Clubs. This time, there are many solutions listed, and they are no longer ordered by length. Instead, the top solutions in the list are slightly longer than the ones below, but have lower scores because they use the input variable only once.][penalizer]
//!
#![doc = embed_doc_image::embed_image!("penalizer", "assets/penalizer_medium.png")]
//!
//! The expressions listed in the Solutions panel are no longer ordered only by length; instead, slightly longer expressions have been surfaced to the top because they use the input variable only once and so go unpenalized by the penalizer, while slightly shorter expressions which use the input variable multiple times receive the penalty and have scores that are 7 more than their lengths. Note that in the Threads panel, which displays expressions which are currently being considered, the number to the left of each expression *is* simply its length, because Clubs does not call the penalizer on an expression unless it is accepted by the judge and so does not yet know what these expressions' scores would be.
//!
//! # Formatting, parsing, and Revar
//!
//! [`Expression`]s in Clubs implement [`Display`][std::fmt::Display] and [`FromStr`][std::str::FromStr], meaning they can be rendered to and parsed from strings. When an expression is rendered to a string, it always uses the variable names "a", "b", "c", and so on. Likewise, when an expression is parsed from a string, it always assumes the variable names "a", "b", and "c" are used.
//!
//! However, sometimes it is convenient to use different variable names when displaying expressions to the user, or when parsing strings provided by the user. Clubs provides methods for renaming variables that operate purely on strings; the variables are renamed by transforming one string into another string after rendering or before parsing. There is one method for each scenario.
//!
//!   - When you have a string obtained by `format!()`ing an expression and you want to rename the variables away from their default names, use the `.revar()` method.
//!   - When you have a string you provided that already uses non-default variable names and you want to normalize the names so that you can [`str::parse()`] it into an expression, use the `.unvar()` method.
//!
//! These two methods are implemented directly on the `&str` type via the [`Revar`] trait (which must be in scope to use the methods):
//!
//! ```
//! use clubs_diamonds::{Expression, Revar};
//!
//! let expr = "a+b*3".parse::<Expression<i32, 2>>().unwrap();
//!
//! assert_eq!(
//!     expr.to_string()
//!         .revar(&['i', 'j']),
//!     "i+j*3"
//! );
//!
//! assert_eq!(
//!     "i+j*3".unvar(&['i', 'j'])
//!            .parse::<Expression<i32, 2>>()
//!            .unwrap()
//!            .apply(&[5, 2]),
//!     Some(11)
//! );
//! ```
//!
//! # Verdicts: passing information out of the judge
//!
//! Sometimes, while judging an expression, you generate data that would be useful to have access to after the judge has returned. For example, you might want to search for an expression that has an output of 1,000,000 for some input within a given range, and you might want to use an inspector to display the input that worked.
//!
//! Without Verdicts, the only way to perform a search like this is to loop through all allowable inputs once in the judge (returning true if a working one is found) and then again in the inspector (returning a string representing the first working one when it's found again). If you need this information inside the penalizer, you must run the loop a third time, and if you need it after the search is over you must run it yet a fourth. This is a violation of DRY and a waste of computation power.
//!
//! With Verdicts, relevant information can be computed in the judge and then shared with the inspector and penalizer, and can be made to appear in the solutions `Vec` as well.
//!
//! [`Verdict`] is the trait for things that can be returned from a judge. All of the examples above return a plain `bool` from the judge, and this works because   `bool` implements [`Verdict`]. When you return a `bool`, the inspector and penalizer receive a reference to just a bare [`Expression`], and these bare expressions are collected into the solutions `Vec` that is eventually returned.
//!
//! If you want to pass data out of the judge, you can return an `Option<T>` instead. Return `None` to reject an expression, and return `Some(t)` to accept it; the value `t` is passed along with the accepted expression to the inspector and penalizer, and will be included with it in the solutions `Vec`. Specifically, the expression itself and the `T` yielded by the judge will be packagd into a [`Solution`] struct, which has two `pub` fields to hold these two pieces of data.
//!
//! Here is an example:
//!
//! ```
//! use clubs_diamonds::*;
//! 
//! fn main() {
//!     let solutions =
//!         Searcher::<i32, 1>::new(move |expr: &Expression::<i32, 1>| {
//!             (0..=1000).find(|&i| expr.apply(&[i]) == Some(1_000_000))
//!         })
//!         .inspector(|solution: &Solution<i32, 1, i32>| {
//!             format!("expr({}) works", solution.data)
//!         })
//!         .penalizer(|solution: &Solution<i32, 1, i32>| {
//!             format!("{}", solution.data).len() * 3
//!         })
//!         .threads(4)
//!         .report_every(1<<12)
//!         .run_with_ui();
//! 
//!     println!("The first three solutions we found were:");
//!     println!("    — {} ({})", solutions[0].expr, solutions[0].data);
//!     println!("    — {} ({})", solutions[1].expr, solutions[1].data);
//!     println!("    — {} ({})", solutions[2].expr, solutions[2].data);
//! 
//!     // solutions is a Vec<Solution::<i32, 1, i32>>
//! }
//! ```
//!
//! Note that using `Option` as the Verdict for the search also changes the return type of the run method. It now returns a `Vec<Solution<i32, 1, i32>>`. The first two type parameters of the [`Solution`] struct indicate the variable-type and variable-count of the contained [`Expression`], and the third indicates the type of the extra data which was returned by the judge.
//!
//! `Vec` also implements [`Verdict`]. An empty `Vec` rejects the expression and a non-empty `Vec` accepts it. Just like with `Option`, the extra data is passed along with the accepted expression to any inspector or penalizer, and is included in the final results list, and just like with `Option`, this happens via the `Solution` struct. In the case of `Vec`, the entire non-empty `Vec` itself is what's put in the `data` field of that struct.
//!
//! # Extra details for type nerds
//!
//! For the most part, the requirement of Clubs that an expression only use one numeric type for its inputs and outputs is simply a requirement of Rust, as described above. If `x` is a `u32` and `x*y` is a valid expression, then `y` must be a `u32` as well, and `x*y` will evaluate to one too. This type-matching rule is true of the binary operators `*`, `/`, `%`, `+`, `-`, `&`, `^`, and `|`, and the of unary operators `!` and `-`.
//!
//! However, it is not true of the bitshift operators, `<<` and `>>`. These operators are special in that the left and right operands are NOT required to have the same type. `x>>y` is valid Rust code even if `x` and `y` are different types (in fact, any pair of integer numeric types works). The only constraint enforced for these operators is that the output type must be equal to the type of the left operand.
//!
//! Since this is the case, it is possible in principle to imagine a search for two-variable expressions whose input variables have distinct types — for example, a search for expressions whose input variables are of type `i64` and `u16`. Such a search would consider expressions like `a>>b`, `(b^9)<<33*a`, and `b>>(a<<b)`, since in each of these expressions, the distinct types of the variables `a` and `b` are "protected" from each other by `>>` and `<<` operators which are just so placed as to avoid any type conflicts; but it would not consider `a|b` or `a+b*21`, or `a-(b<<3)`, since in all of these expressions the distinct types eventually "meet" at an operator which requires its operands' types to match and so the expressions are not valid. The search would also provide an interface by which you could apply each candidate expression to pairs of input values of types `i64` and `u16`.
//!
//! **Clubs is not capable of performing this search.** In Clubs, you may only perform searches for expressions whose input variables all have the same type and whose output is the same type as that. The type you choose is a type parameter of the `Searcher` struct and of the `Expression` structs it yields. The `Searcher` is not smart enough to limit itself to expressions which respect type differences between different input variables, and the `Expression` struct does not provide a method by which you can apply the expression it represents to input variables of distinct types. It is a limitation of the current architecture.

mod ui;
mod search;

pub use search::Expression;
pub use search::Searcher;
pub use search::Number;
pub use search::Revar;
pub use search::Verdict;
pub use search::Solution;

