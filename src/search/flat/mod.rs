
mod expression;
mod expression_writer;
mod pivot;
mod ui;

pub use expression::ExpressionCore;
pub use expression::Expression;

use std::thread;
use std::sync::mpsc;

use crate::search::number::Number;
use pivot::Op::{self, *};
use expression_writer::ExpressionWriter;
use ui::SearchUI;
use ui::SearchUISignal::*;
use self::SearchNews::*;
use ThreadStatus::*;
use crate::utils as format_helpers;

// Helper types.

type Judge    <N, const C: usize> = fn(ExpressionCore<N, C>) -> bool;
type Inspector<N, const C: usize> = fn(&Expression<N, C>) -> String;
type Penalizer<N, const C: usize> = fn(&Expression<N, C>) -> usize;

// The Searcher type.

pub struct Searcher<N: Number, const C: usize> {
    judge: Judge<N, C>,
    inspector: Option<Inspector<N, C>>,
    penalizer: Option<Penalizer<N, C>>,
    description: Option<String>,
    threads: usize,
    report_every: u128,
    min_length: usize,
    max_length: usize,
}

impl<N: Number, const C: usize> Searcher <N, C> {
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

    pub fn inspector(self, inspector: Inspector<N, C>) -> Self {
        Self {
            inspector: Some(inspector),
            ..self
        }
    }

    pub fn penalizer(self, penalizer: Penalizer<N, C>) -> Self {
        Self {
            penalizer: Some(penalizer),
            ..self
        }
    }

    pub fn description(self, description: &str) -> Self {
        Self {
            description: Some(String::from(description)),
            ..self
        }
    }

    pub fn threads(self, threads: usize) -> Self {
        Self {
            threads,
            ..self
        }
    }

    pub fn report_every(self, report_every: u128) -> Self {
        Self {
            report_every,
            ..self
        }
    }

    pub fn min_length(self, min_length: usize) -> Self {
        Self {
            min_length,
            ..self
        }
    }

    pub fn max_length(self, max_length: usize) -> Self {
        Self {
            max_length,
            ..self
        }
    }

    pub fn run_with_ui(&self) -> (u128, Vec<Expression<N, C>>) {
        self.run(true)
    }

    pub fn run_silently(&self) -> (u128, Vec<Expression<N, C>>) {
        self.run(false)
    }
}

// Note to future self: because the final array representation of the expression
// is read backwards, the "left" subexpression of a binary operator appears to
// the right of the "left" subexpression in that array; the expression "a/2"
// would be represented as [DIV 2 a].

enum SearchNews<N: Number, const C: usize> {
    ExpressionWorks      {thread_id: usize, expr: Expression<N, C>},
    ExpressionDoesntWork {thread_id: usize, expr: Expression<N, C>, length: usize, count: u128},
    Done                 {thread_id: usize,                         length: usize, count: u128},
}

pub struct Thread {
    id: usize,
    status: ThreadStatus,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ThreadStatus {
    Empty,              // Waiting for a new task.
    Initializing,       // Was just given a new task and hasn't sent an update yet.
    Searching(String),  // Searching an expression.
    Found(String),      // Found an expression that works.
}

impl<N: Number, const C: usize> Searcher<N, C> {
    fn run(&self, display_ui: bool) -> (u128, Vec<Expression<N, C>>) {

        // Set up the TUI.

        let cycle_duration = std::time::Duration::from_millis(1); // Time to sleep between control cycles.
        let frame_clock_max = 16; // Only draw a UI frame every N cycles of the control loop.
        let mut frame_clock = 0;

        let mut ui = if display_ui {Some(SearchUI::new())} else {None};

        if let Some(ref mut ui) = ui {
            ui.enable_solution_inspector();

            if let Some(ref description) = self.description {
                ui.set_description(&*description);
            }
        }

        // Set up the work.

        let mut total_count = 0u128; // total count of expressions
        let mut target_thread_count = self.threads;
        let mut threads: Vec<Thread> = vec![];
        let mut solutions = vec![];

        let op_requirements = [
            None,
            Some(NOT), Some(MUL), Some(DIV), Some(MOD),
            Some(ADD), Some(SUB), Some(LSL), Some(LSR),
            Some(AND), Some(XOR), Some(ORR)
        ];

        let mut task_iterator =
            (self.min_length..=self.max_length)
                .flat_map(|l| op_requirements.clone().into_iter().map(move |or| (l, or)))
                .peekable();
        let (tx, rx) = mpsc::channel();

        let mut counts = [(0, 0); 99];

        'search: loop {

            // Handle messages from the threads.

            while let Ok(msg) = rx.try_recv() {
                match msg {
                    ExpressionWorks {thread_id, expr} => {
                        let thread = threads.iter_mut().find(|thread| thread.id == thread_id).unwrap();
                        let string = format!("{expr}");
                        let inspection = self.inspector.as_ref().map(|insp| insp(&expr));
                        thread.status = Found(string.clone());

                        let score = string.len() + self.penalizer.as_ref().map(|scorer| scorer(&expr)).unwrap_or(0);

                        if let Some(ref mut ui) = ui {ui.push_solution((string, score, inspection));}
                        solutions.push(expr);
                    },

                    ExpressionDoesntWork {thread_id, expr, length, count} => {
                        let thread = threads.iter_mut().find(|thread| thread.id == thread_id).unwrap();
                        let string = format!("{expr}");
                        thread.status = Searching(string);
                        counts[length].0 += count;
                        total_count += count;
                    }

                    Done {thread_id, length, count} => {
                        let thread = threads.iter_mut().find(|thread| thread.id == thread_id).unwrap();
                        thread.status = Empty;
                        counts[length].0 += count;
                        counts[length].1 += 1;
                        total_count += count;

                        if counts[length].1 == op_requirements.len() {
                            if let Some(ref mut ui) = ui {
                                ui.push_news_item(
                                    format!(
                                        "Tried {} expr{} of length {}.",
                                        format_helpers::with_commas(counts[length].0),
                                        if counts[length].0 == 1 {""} else {"s"},
                                        length,
                                    )
                                );
                            }
                        }
                    }
                }
            }

            // If there are empty-handed threads, delete them.

            threads.retain(|thread| thread.status != Empty);
            
            // Spawn threads up to the thread limit (as long as there are
            // tasks to give them).

            while threads.len() < target_thread_count {
                let Some((length, op_requirement)) = task_iterator.next() else {break};

                threads.push(Thread {
                    status: Empty,
                    id: (0..).find(|x| threads.iter().all(|thread| thread.id != *x)).unwrap(),
                });

                let idx = threads.len() - 1;
                let tx_clone = tx.clone();
                let judge_clone = self.judge.clone();
                let thread_id = threads[idx].id;
                let report_every = self.report_every;

                threads[idx].status = Initializing;

                thread::spawn(move || {
                    find_with_length_and_op(
                        thread_id,
                        report_every,
                        judge_clone,
                        length,
                        Some(op_requirement),
                        tx_clone,
                    );
                });
            }

            // If at this point there are no threads and also no tasks left
            // in the task iterator, return.

            if threads.is_empty() && task_iterator.peek() == None {
                break 'search;
            }

            // If this is a UI cycle, handle inputs and draw a frame of the UI.
            
            if let Some(ref mut ui) = ui {
                if frame_clock == 0 {
                    for action in ui.handle_inputs() {
                        match action {
                            Quit => break 'search,
                            IncreaseThreadCount => target_thread_count += 1,
                            DecreaseThreadCount => if target_thread_count > 0 {target_thread_count -= 1},
                        }
                    }

                    ui.set_total_count(total_count);
                    ui.set_target_thread_count(target_thread_count);
                    ui.set_thread_statuses(threads.iter().map(|thread| Thread {
                        id: thread.id,
                        status: thread.status.clone(),
                    }).collect());

                    ui.draw();

                    // Reset the clock.

                    frame_clock = frame_clock_max;
                }

                frame_clock -= 1;
            }

            // Sleep for a bit to stop the control thread from hotlooping.

            std::thread::sleep(cycle_duration);
        }

        // Todo: tear down the threads.

        (total_count, solutions)
    }
}

fn find_with_length_and_op<N: Number, const C: usize>(
    thread_id: usize,
    notification_spacing: u128,
    judge: Judge<N, C>,
    length: usize,
    op_requirement: Option<Option<Op>>,
    mpsc: mpsc::Sender<SearchNews<N, C>>,
) {
    //println!("New thread! — length = {length}, op_requirement = {op_requirement:?}");

    let mut count = 0u128;
    let mut field: [u8; 16] = [255; 16];
    let mut writer = ExpressionWriter::new(C, length, op_requirement);

    while writer.write(&mut field) {
        count += 1;
        let expression_core = ExpressionCore::new(&field);

        if judge(expression_core) {
            mpsc.send(ExpressionWorks {
                thread_id,
                expr: Expression::from_core(expression_core),
            }).unwrap();
        } else if count == notification_spacing {
            mpsc.send(ExpressionDoesntWork {
                thread_id,
                expr: Expression::from_core(expression_core),
                length,
                count,
            }).unwrap();
            count = 0;
        }
    }

    mpsc.send(Done {
        thread_id,
        length,
        count,
    }).unwrap();
}

