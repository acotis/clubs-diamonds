
//! A brute-forcer for finding short mathematical expressions in Rust, for code golf.
//!
//! This crate provides the following types:
//!
//! - [`Expression`][crate::search::flat::Expression]: A struct representing a mathematical expression that syntactically parses as Rust code, such as `3*(a+5)` or `b>>a|89%c`.
//! - [`ExpressionCore`][crate::search::flat::ExpressionCore]: A more lightweight version of the same thing, which borrows its data. (If you don't know what that means, don't worry too much about it.) This distinction is going away in a future version of the crate.
//! - [`Searcher`][crate::search::flat::Searcher]: A configurable search type which can be used to systematically search all syntactically valid expressions in order of length and yield only those which meet a customizeable, user-specified property.
//!
//! Here is an example of a basic use-case of the library. Suppose you want to find an expression `f` in a single variable `a` such that, when you plug in the values 1 through 5 for the variable, the expression yields the first five prime numbers. In other words, you want:
//!
//! - f(1) = 2
//! - f(2) = 3
//! - f(3) = 5
//! - f(4) = 7
//! - f(5) = 11
//!
//! You can ask Clubs to find you such an expression with the following code:
//!
//! ```
//! use clubs_diamonds::search::flat::{Searcher, ExpressionCore};
//! 
//! fn main() {
//!     let (count, solutions) =
//!         Searcher::<usize, 1>::new(|expr: ExpressionCore::<usize, 1>| {
//!             expr.apply(&[1]) == Some(2) &&
//!             expr.apply(&[2]) == Some(3) &&
//!             expr.apply(&[3]) == Some(5) &&
//!             expr.apply(&[4]) == Some(7) &&
//!             expr.apply(&[5]) == Some(11)
//!         })
//!         .threads(4)
//!         .run_with_ui();
//! 
//!     println!("Searched {} expressions total", count);
//!     println!("The first three solutions we found were:");
//!     println!("    — {}", solutions[0]);
//!     println!("    — {}", solutions[1]);
//!     println!("    — {}", solutions[2]);
//! 
//!     // solutions is a Vec<Expression::<usize, 1>>
//! }
//! ```
//!
//! Executing the above code yields a text-based UI that looks like this:
//!
//! ![A screenshot of the text-based interface of the Clubs expression searcher. On the right, several information boxes tell us what expressions are currently being searched by each of four threads, how long the search has been running for, how many expressions are being searched per second, and other stats. On the left, six found solutions are listed.][demo]
//!
#![doc = embed_doc_image::embed_image!("demo", "assets/demo.png")]
//!
//! The interface is controlled by the keyboard. The controls are:
//!
//! - `D`: show/hide description box
//! - `T`: show/hide thread list
//! - `S`: show/hide runtime stat box
//! - `I`: show/hide solution inspector
//! - `N`: show/hide news feed
//! - `+` / `-`: increase/decrease target thread count*
//! - `J` / `K`: naviate downward/upward in the list of solutions
//! - `Q`: quit
//! 
//! \* Decreases in thread count may take a while to take effect because the thread count cannot decrease until one of the currently-running threads finishes its task, and the tasks can be minutes or hours long depending on how far the search has progressed.

pub mod search;
pub mod utils;

