
use super::Judge;
use super::Inspector;
use super::Penalizer;

use super::run;
use super::NullUI;
use super::DefaultUI;

use crate::Number;
use crate::Expression;

/// Used to configure and execute searches for short mathematical expressions.

pub struct Searcher<N: Number, const C: usize> {
    pub(super) judge: Judge<N, C>,
    pub(super) inspector: Option<Inspector<N, C>>,
    pub(super) penalizer: Option<Penalizer<N, C>>,
    pub(super) description: Option<String>,
    pub(super) threads: usize,
    pub(super) report_every: u128,
    pub(super) min_length: usize,
    pub(super) max_length: usize,
}

impl<N: Number, const C: usize> Searcher <N, C> {

    /// Construct a new `Searcher`. The provided closure is used as a judge to determine which expressions to accept as solutions and which to reject.

    pub fn new(judge: Judge<N, C>) -> Self {
        Self {
            judge,
            inspector: None,
            penalizer: None,
            description: None,
            threads: 1,
            report_every: 1<<16,
            min_length: 1,
            max_length: usize::MAX,
        }
    }

    /// Provide an "inspector" for the UI. The inspector is a closure that accepts an `&Expression` and returns a `String`. If provided, this closure is called on each solution the `Searcher` finds, and the returned String is displayed in the Solution Inspector panel of the UI when the solution is selected. The closure is called only once per solution, when the solution is first discovered.

    pub fn inspector(self, inspector: Inspector<N, C>) -> Self {
        Self {
            inspector: Some(inspector),
            ..self
        }
    }

    /// Provide a "penalizer" used to score solutions and order them in the UI.
    ///
    /// By default, the score of a solution is its length in bytes (solutions are sorted with lower scores towards the top). A penalizer is a closure that accepts an `&Expression` and returns a `usize`. If provided, this closure is called on each solution the `Searcher` finds, and the returned value is **added to** the length of the solution to calculate the score. (If you don't want this behavior, simply subtract the length of the solution from the value you return. You can obtain the length of the solution by `format!()`ing it.) The closure is called only once per solution, when the solution is first discovered. 

    pub fn penalizer(self, penalizer: Penalizer<N, C>) -> Self {
        Self {
            penalizer: Some(penalizer),
            ..self
        }
    }

    /// Provide a description of the search. The description is a plain piece of text. If provided, it is displayed in the Description panel of the UI. Useful if you are running multiple searches at the same time in different windows and want to keep track of which is which.

    pub fn description(self, description: &str) -> Self {
        Self {
            description: Some(String::from(description)),
            ..self
        }
    }

    /// Set the number of worker threads used by the search process. Regardless of the number of worker threads, there will always also be one manager thread which assigns tasks to the worker threads, collects results from them, and manages the UI. The minimum number of worker threads is 1, so the minimum number of total threads is 2. The manager thread is computationally light.

    pub fn threads(self, threads: usize) -> Self {
        Self {
            threads,
            ..self
        }
    }

    /// Control the frequency with which worker threads report back to the UI thread. Solutions are always reported immediately when they are found; worker threads also notify the UI thread of their status once for every N candidates they reject. This method is used to configure N. The main user-facing consequence of this parameter is how frequently the threads in the Threads panel get updated visually.

    pub fn report_every(self, report_every: u128) -> Self {
        Self {
            report_every,
            ..self
        }
    }

    /// Set the minimum length of expression to consider. For example, if you call `.min_length(6)` then expressions of length 1-5 are skipped at the start of the search process.

    pub fn min_length(self, min_length: usize) -> Self {
        Self {
            min_length,
            ..self
        }
    }

    /// Set the maximum length of expression to consider. For example, if you call `.max_length(10)`, then once the searcher has exhausted all expressions of length 1-10, it will automatically quit the UI (if there was a UI for this search) and return the results.
    ///
    /// Useful if you want to use the `.run_silently()` method, as setting a maximum expression length is currently the only way to render a search process naturally finite.

    pub fn max_length(self, max_length: usize) -> Self {
        Self {
            max_length,
            ..self
        }
    }

    /// Execute the configured search process in a text-based UI.

    pub fn run_with_ui(&self) -> (u128, Vec<Expression<N, C>>) {
        run::<N, C, DefaultUI>(&self)
    }

    /// Execute the configured search process silently.
    ///
    /// **Note:** When you use this method, there is no way to quit the search process before Clubs decides it's done. So, if you plan to use it, you probably want to specify a combination of search parameters that make the search task finite.

    pub fn run_silently(&self) -> (u128, Vec<Expression<N, C>>) {
        run::<N, C, NullUI>(&self)
    }
}
