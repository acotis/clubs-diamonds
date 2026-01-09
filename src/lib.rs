
//! A brute-forcer for finding short mathematical expressions in Rust, for code golf. May also be useful for golfing in other languages, if the language's syntax for math is similar enough to Rust's.
//!
//! This crate provides the following types:
//!
//! - [`Expression`]: A struct representing a mathematical expression, such as `3*(a+5)` or `b>>a|89%c`. Uses an optimized representation internally, but adheres to Rust syntax, Rust operators, Rust precedence levels, and Rust semantics.
//! - [`Searcher`]: A configurable search type which can be used to systematically search all syntactically valid expressions in increasing order of length, and yield only those which meet a customizeable, user-specified criterion.
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
//!     let (count, solutions) =
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
//!     println!("Searched {} expressions total", count);
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
//! As solutions are found, they will appear in the Solutions column on the left. When you quit the UI, the `.run_with_ui()` method returns, and control flow returns to the main function. In this demo program, we then print out the information which is returned to us by that method. The information is:
//!
//! - `count`: a `u128` representing the total number of candidate expressions which were considered during the search (including those which were rejected because they didn't meet the specified criterion).
//! - `solutions`: a `Vec` containing the expressions that did meet the criterion.
//! 
//! **Limitation note:** Clubs does not currently consider expressions containing the unary `-` operator (arithmetic negation). For unsigned types, this doesn't matter because the operator is inapplicable anyway. For signed types, this means Clubs will sometimes miss valid expressions that could have been solutions. In their place, it will find longer versions of these expressions that contain terms like `0-a` instead of `-a`. This is planned to be fixed in a later version of the crate.
//!
//! # Workflow for performing a search
//!
//! There is a four-step workflow for performing a search using the `Searcher` type:
//!
//! 1. **Decide the type and number of variables** which will appear in the expression.
//!     - Specify these choices as type parameters of the `Searcher`.
//! 2. **Construct** the searcher using the method `Searcher::new()`.
//!     - When you call this method, you supply a closure that accepts an `&Expression` and returns a `bool`. This is the "judge" that is used to determine which expressions are displayed in the Solutions panel in the UI (and eventually returned in the solutions Vec).
//! 3. Optionally, **specify additional parameters** for the search by using some of `Searcher`'s Builder-Lite methods.
//! 4. **Execute** the search using either the `.run_with_ui()` method or the `.run_silently()` method.
//!
//! Here is how the steps are followed by the example code above:
//!
//! ```
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
//! Clubs is capable of finding both single-variable and multi-variable expressions. The number of variables is a type parameter of the `Searcher` struct. If you set it to `1`, it will find single-variable expressions. If you set it to `2`, it will find two-variable expressions, etc. Variables are always named with single-letter names, starting with "a" for the first variable, "b" for the second, and so on.
//!
//! For example, a single-variable search will consider expressions like:
//! - `3+a`
//! - `a*a*47`
//! - `a%27<<(a&43-a)`
//! - ...and so on.
//!
//! A two-variable search will consider expressions like:
//! - `a+b`
//! - `a*b+73`
//! - `a>>(47%b+a|89)`
//! - ...and so on.
//!
//! In this version of the crate, a `Searcher` will only consider expressions which contain every variable at least once. For example, a two-variable `Searcher` will not consider the expression `b+9` because the variable `a` does not appear in it.
//!
//! ## Step 1b: Type of variables
//!
//! In Rust, every variable has an unchanging type, and Rust is generally a hard-ass about enforcing that types match. If `x` is a `u32` and `y` is an `i128`, then `x*y` is a syntactically invalid expression and will fail to compile with a type error.
//!
//! In Clubs, the `Searcher` requires that every variable in an expression has the same type, and this type must be specified as a type parameter of the `Searcher` itself. If you set it to `u8`, it will find expressions whose inputs (and output) are `u8`s. If you set it to `isize`, it will find expressions whose inputs (and output) are `isize`s, and so on.
//!
//! #### Extra details for type nerds
//!
//! For the most part, this requirement of Clubs is simply a requirement of Rust, as described above. If `x` is a `u32` and `x*y` is a valid expression, then `y` must be a `u32` as well, and `x*y` will evaluate to one too. This type-matching rule is true of the binary operators `*`, `/`, `%`, `+`, `-`, `&`, `^`, and `|`, and the of unary operators `!` and `-`.
//!
//! However, it is not true of the bitshift operators, `<<` and `>>`. These operators are special in that the left and right operands are NOT required to have the same type. `x>>y` is valid Rust code even if `x` and `y` are different types (in fact, any pair of integer numeric types works). The only constraint enforced for these operators is that the output type must be equal to the type of the left operand.
//!
//! Since this is the case, it is possible in principle to imagine a search for two-variable expressions whose input variables have distinct types — for example, a search for expressions whose input variables are of type `i64` and `u16`. Such a search would consider expressions like `a>>b`, `(b^9)<<33*a`, and `b>>(a<<b)`, since in each of these expressions, the distinct types of the variables `a` and `b` are "protected" from each other by `>>` and `<<` operators which are just so placed as to avoid any type conflicts; but it would not consider `a|b` or `a+b*21`, or `a-(b<<3)`, since in all of these expressions the distinct types eventually "meet" at an operator which requires its operands' types to match and so the expressions are not valid. The search would also provide an interface by which you could apply each candidate expression to pairs of input values of types `i64` and `u16`.
//!
//! **Clubs is not capable of performing this search.** In Clubs, you may only perform searches for expressions whose input variables all have the same type and whose output is the same type as that. The type you choose is a type parameter of the `Searcher` struct and of the `Expression` structs it yields. The `Searcher` is not smart enough to limit itself to expressions which respect type differences between different input variables, and the `Expression` struct does not provide a method by which you can apply the expression it represents to input variables of distinct types. It is a limitation of the current architecture.
//!
//! ## Step 2: Constructing a Searcher
//!
//! A Searcher is constructed using the method `Searcher::new()`, which accepts a closure as its only argument. The closure must accept an `&Expression` and return a `bool`. This is where you specify your customizeable criterion for expressions to be tested against.
//!
//! Generally speaking, you will judge an expression by calling the `.apply()` method provided by the `Expression` type and checking things about its return values. This method accepts an array of input variable values and returns the output value the expression evaluates to for those inputs. The return value is wrapped in an `Option`, and the value `None` is returned when the given inputs would cause the expression to crash with a runtime error (for example, if it ends up dividing by zero).
//!
//! Here is an example of a `Searcher` being constructed for a two-variable search using `i16` variables:
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
//! This `Searcher` will consider all expressions containing two `i16` variables. For example:
//! - At some point, the `Searcher` will consider the expression `a+b`:
//!     1. To evaluate this expression, it will pass an `&Expression` representing it to the provided closure.
//!     2. The closure will call `expr.apply(&[1, 3])` and get `Some(4)` as the answer.
//!     3. Since that doesn't match the expected value, the boolean logic will short-circuit, the closure will return `false`, and the `Searcher` will reject the expression.
//! - At some point, the `Searcher` will consider the expression `a^b+1`.
//!     1. To evaluate this expression, it will pass an `&Expression` representing it to the provided closure.
//!     2. The provided closure will call `expr.apply(&[1, 3])` and get `Some(5)` as the output (note that due to the operator precedence of `+` and `^`, the expression is equivalent to `a^(b+1)`).
//!     3. Since this matches, the closure will continue to the next condition, calling `expr.apply(&[5, -2])`, and getting `Some(-6)` as the answer.
//!     4. Since this matches too, the closure will continue to the last condition, calling `expr.apply(&[-8, 7])` and getting `Some(-16)` as the answer.
//!     5. Since this doesn't match, the closure will return `false` and the `Searcher` will reject this expression as well.
//! - At some point, the `Searcher` will consider the expression `4^a/(4/b)`.
//!     1. To evaluate this expression, it will pass an `&Expression` representing it to the provided closure.
//!     2. The provided closure will call `expr.apply(&[1, 3])` and get `Some(5)` as the output.
//!     3. Since this matches, the closure will continue to the next condition, calling `expr.apply(&[5, -2])`, and getting `Some(-6)` as the answer.
//!     4. Since this matches too, the closure will continue to the last condition, calling `expr.apply(&[-8, 7])` and getting `None` as the answer because evaluating the expression at the those inputs would cause Rust to divide by zero and crash.
//!     5. Since this is the expected output as well, the closure will return `true` and the `Searcher` will accept this expression, displaying it in the Solutions panel of the UI (if this search included a UI) and returning it in the final `results` Vec.
//!
//! **Golfing tip:** There is room for considerable ingenuity and creativity in specifying the criterion that a `Searcher` will apply. It can be any predicate. Using your imagination will take you further than only copying the format of the documented examples.
//!
//! ## Steps 3 and 4: Additional search parameters and execution
//!
//! For a list of `Searcher`'s methods, including the ones for specifying additional search parameters and executing the search, see [its documentation page][crate::Searcher].
//!
//! **Note:** If you opt to use the `.run_silently()` method, then there will be no way to quit the search before Clubs decides it's done. So, if you plan to use that method, you probably want to specify a combination of search parameters that make the search task finite.
//!
//! # The `.penalizer()` method
//!
//! By default, Clubs will sort the expressions it discovers in order of length, shortest first, because that's what's usually appropriate for code golf. However, sometimes the length of the expression itself isn't the only thing you care about. The overall program that you're using the expression in may have parts that grow or shrink based on the expression's properties. In that case, the shortest working expression may not be the best one.
//!
//! Clubs can be configured to sort the solutions it finds based on other criteria. The way to do this is using the `.penalizer()` method of the `Searcher` type. The `.penalizer()` method accepts a closure whose argument is an `&Expression` and whose return value is a `usize`. The supplied closure is called a "penalizer". The penalizer is called once for each solution Clubs discovers, and its output is **added to** to the length of each solution to compute the solution's overall score. Solutions are then sorted in order of score, lowest first.
//!
//! As an example, suppose you are searching for a one-variable expression, and your program architecture is such that, if the input variable appears multiple times in the expression you choose, another part of the program will grow by 7 bytes. You can ask Clubs to penalize such expressions by 7 bytes like this:
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
//! ![Another screenshot of Clubs. This time, there are many solutions listed, and they are no longer ordered by length. Instead, the top solutions in the list are slightly longer than the ones below, but have lower scores because they use the input variable only once.][demo_penalizer]
//!
#![doc = embed_doc_image::embed_image!("demo_penalizer", "assets/demo_penalizer_medium.png")]
//!
//! The expressions listed in the Solutions panel are no longer ordered only by length; instead, slightly longer expressions have been surfaced to the top because they use the input variable only once and so go unpenalized by the penalizer, while slightly shorter expressions which use the input variable multiple times receive the penalty and have scores that are 7 more than their lengths. Note that in the Threads panel, which displays expressions which are currently being considered, the number to the left of each expression *is* simply its length, because Clubs does not call the penalizer on an expression unless it is accepted by the judge and so does not yet know what these expressions' scores would be.
//!

mod ui;
mod search;

pub use search::Expression;
pub use search::Searcher;
pub use search::Number;

pub use search::Writer; // temporary, for testing only; todo: remove this
pub use search::AddSubtractWriter; // temporary, for testing only; todo: remove this
