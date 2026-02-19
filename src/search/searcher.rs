
use super::run;
use super::UI;
use super::NullUI;
use super::DefaultUI;

use crate::Number;
use crate::Expression;
use crate::Verdict;

/// Used to configure and execute searches for short mathematical expressions.

pub struct Searcher<N: Number, const C: usize, V: Verdict<N, C> = bool> {
    pub(super) judge: Box<dyn Fn(&Expression<N, C>) -> V + Sync + 'static>,
    pub(super) inspector: Option<Box<dyn Fn(&V::Wrapper) -> String>>,
    pub(super) penalizer: Option<Box<dyn Fn(&V::Wrapper) -> usize>>,
    pub(super) description: Option<String>,
    pub(super) threads: usize,
    pub(super) report_every: u128,
    pub(super) min_length: usize,
    pub(super) max_length: usize,
    pub(super) constant_cap: u128,
    pub(super) debug_banner_enabled: bool,
    pub(super) var_names: Option<String>, // if none, default to names 'a', 'b', 'c'...
    pub(super) phantom_data: std::marker::PhantomData<N>,
    pub(super) last_total_count: Option<u128>,
}

impl<N: Number, const C: usize> Searcher<N, C> {

    /// Construct a new `Searcher`. The provided closure is used as a judge to determine which expressions to accept as solutions and which to reject.

    pub fn new<J, V: Verdict<N, C>>(judge: J) -> Searcher::<N, C, V>
        where J: Fn(&Expression<N, C>) -> V + Sync + 'static,
    {
        Searcher::<N, C, V> {
            judge: Box::new(judge),
            inspector: None,
            penalizer: None,
            description: None,
            threads: 1,
            report_every: 1<<20,
            min_length: 1,
            max_length: usize::MAX,
            constant_cap: 156,
            debug_banner_enabled: true,
            var_names: None,
            phantom_data: Default::default(),
            last_total_count: None,
        }
    }
}

impl<N: Number, const C: usize, V: Verdict<N, C>> Searcher<N, C, V> {

    /// Provide an "inspector" for the UI. The inspector is a closure that accepts an &[`Expression`] and returns a [`String`]. If provided, this closure is called on each solution the `Searcher` finds, and the returned String is displayed in the Solution Inspector panel of the UI when the solution is selected. The closure is called only once per solution, when the solution is first discovered.

    pub fn inspector<I>(self, inspector: I) -> Self
        where I: Fn(&V::Wrapper) -> String + 'static,
    {
        Searcher {
            judge: self.judge,
            inspector: Some(Box::new(inspector)),
            penalizer: self.penalizer,
            description: self.description,
            threads: self.threads,
            report_every: self.report_every,
            min_length: self.min_length,
            max_length: self.max_length,
            constant_cap: self.constant_cap,
            debug_banner_enabled: self.debug_banner_enabled,
            var_names: self.var_names,
            phantom_data: self.phantom_data,
            last_total_count: self.last_total_count,
        }
    }

    /// Provide a "penalizer" used to score solutions and order them in the UI.
    ///
    /// By default, the score of a solution is its length in bytes (solutions are sorted with lower scores towards the top). A penalizer is a closure that accepts an &[`Expression`] and returns a `usize`. If provided, this closure is called on each solution the `Searcher` finds, and the returned value is **added to** the length of the solution to calculate the score. (If you don't want this behavior, simply subtract the length of the solution from the value you return. You can obtain the length of the solution by `format!()`ing it.) The closure is called only once per solution, when the solution is first discovered. 

    pub fn penalizer<P>(self, penalizer: P) -> Self
        where P: Fn(&V::Wrapper) -> usize + 'static,
    {
        Searcher {
            judge: self.judge,
            inspector: self.inspector,
            penalizer: Some(Box::new(penalizer)),
            description: self.description,
            threads: self.threads,
            report_every: self.report_every,
            min_length: self.min_length,
            max_length: self.max_length,
            constant_cap: self.constant_cap,
            debug_banner_enabled: self.debug_banner_enabled,
            var_names: self.var_names,
            phantom_data: self.phantom_data,
            last_total_count: self.last_total_count,
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

    /// Set the minimum length of expressions to consider. For example, if you call `.min_len(6)` then expressions of length 1-5 are skipped at the start of the search process.

    pub fn min_len(self, min_length: usize) -> Self {
        Self {
            min_length,
            ..self
        }
    }

    /// Set the maximum length of expression to consider. For example, if you call `.max_len(10)`, then once the searcher has exhausted all expressions of lengths 1-10, it will automatically stop.
    ///
    /// Useful if you want to use the `.run_silently()` method, as setting a maximum expression length is currently the only way to render a search process naturally finite.

    pub fn max_len(self, max_length: usize) -> Self {
        Self {
            max_length,
            ..self
        }
    }

    /// Set the maximum constant value to use in expressions, e.g. if you set this to 20, then Clubs will not consider expressions which contain constants above 20. Clubs always considers all constant values up to the maximum (you can't pick and choose exactly which constants you want it to consider, you can only set a maximum).
    ///
    /// The default value is 155, and due to implementation details, it cannot be set any higher than this. Note that this is just high enough that, when performing a search over `u8` variables, every constant value is accessible in three bytes of text (because 156 is equal to `!99`, 157 is equal to `!98`, and so on).

    pub fn max_constant(self, max_constant: u128) -> Self {
        if max_constant == u128::MAX {
            panic!("set it one lower");
        }

        Self {
            constant_cap: max_constant + 1,
            ..self
        }
    }

    /// Do not consider expressions with constant values in them at all.
    ///
    /// Equivalent to calling `.max_constant()` with an argument of -1 (which is otherwise impossible to do because that method takes a `u8`).

    pub fn no_constants(self) -> Self {
        Self {
            constant_cap: 0,
            ..self
        }
    }

    /// Disable the banner which warns you that you are running Clubs in debug mode. Note that running Clubs in debug mode slows it down by around an order of magnitude. You can run it in release mode instead by executing your code with the command `cargo run --release`. Only use this method if you're completely sure you don't want to do that.

    pub fn no_banner(self) -> Self {
        Self {
            debug_banner_enabled: false,
            ..self
        }
    }

    /// Set the variable names that expressions will be rendered with when they appear in the UI.
    ///
    /// The [`Expression`]s returned in the solutions vector will still have default variable names when rendered with `format!()`; see [`Revar`][crate::Revar] to render expressions with other variable names.

    pub fn revar(self, var_names: &str) -> Self {
        Self {
            var_names: Some(var_names.to_owned()),
            ..self
        }
    }

    /// Execute the configured search process in a text-based UI.

    pub fn run_with_ui(&mut self) -> Vec<V::Wrapper> {
        self.run::<DefaultUI>()
    }

    /// Execute the configured search process silently.
    ///
    /// **Note:** When you use this method, there is no way to quit the search process before Clubs decides it's done. So, if you plan to use it, you probably want to specify a combination of search parameters that make the search task finite.

    pub fn run_silently(&mut self) -> Vec<V::Wrapper> {
        self.run::<NullUI>()
    }

    /// Get the total number of expressions which were considered in the search after it has been executed (will return `None` if the search has not been executed yet). Mainly used for internal testing purposes.

    pub fn last_total_count(&self) -> Option<u128> {
        self.last_total_count
    }
}

impl<N: Number, const C: usize, V: Verdict<N, C>> Searcher<N, C, V> {
    fn run<U: UI>(&mut self) -> Vec<V::Wrapper> {
        let (count, sols) = run::<N, C, V, U>(&self);
        self.last_total_count = Some(count);
        sols
    }
}

