
mod searcher;
mod expression;
mod expression_writer;
mod pivot;
mod number;

pub use searcher::Searcher;
pub use expression::Expression;
pub use number::Number;

use std::thread;
use std::sync::mpsc;
use std::marker::PhantomData;

use pivot::Op::{self, *};
use expression_writer::ExpressionWriter;
use crate::ui::UI;
use crate::ui::DefaultUI;
use crate::ui::NullUI;
use crate::ui::UISignal::*;

use self::ThreadReport::*;
use crate::ui::ThreadStatus::{self, *};

// Helper types.

type Judge    <N, const C: usize> = fn(&Expression<N, C>) -> bool;
type Inspector<N, const C: usize> = fn(&Expression<N, C>) -> String;
type Penalizer<N, const C: usize> = fn(&Expression<N, C>) -> usize;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Thread {
    pub id: usize,
    pub status: Option<ThreadStatus>,
}

// Reports from the worker thread back to the manager thread.

enum ThreadReport<N: Number, const C: usize> {
    FoundSolution   {expr: Expression<N, C>},
    FinishedBatch   {length: usize, count: u128},
    UpdateStatus    {thread_id: usize, status: ThreadStatus},
    Done            {thread_id: usize, length: usize},
}

fn run<N: Number, const C: usize, U: UI>(config: &Searcher<N, C>) -> (u128, Vec<Expression<N, C>>) {

    // Set up the TUI.

    let cycle_duration = std::time::Duration::from_millis(1); // Time to sleep between control cycles.
    let frame_clock_max = 16; // Only draw a UI frame every N cycles of the control loop.
    let mut frame_clock = 0;

    let mut ui = U::new();

    ui.set_inspector_enabled(config.inspector.is_some());

    if let Some(ref description) = config.description {
        ui.set_description(description.clone());
    }

    // Set up the work.

    let mut total_count = 0u128; // total count of expressions
    let mut target_thread_count = config.threads;
    let mut threads: Vec<Thread> = vec![];
    let mut solutions = vec![];

    let op_requirements = [
        None,
        Some(NOT), Some(MUL), Some(DIV), Some(MOD),
        Some(ADD), Some(SUB), Some(LSL), Some(LSR),
        Some(AND), Some(XOR), Some(ORR)
    ];

    let mut task_iterator =
        (config.min_length..=config.max_length)
            .flat_map(|l| op_requirements.clone().into_iter().map(move |or| (l, or)))
            .peekable();
    let (tx, rx) = mpsc::channel();

    let mut counts = [(0, 0); 99];

    'search: loop {

        // Handle messages from the threads.

        while let Ok(msg) = rx.try_recv() {
            match msg {
                FoundSolution {expr} => {
                    let string = format!("{expr}");
                    let inspection = config.inspector.as_ref().map(|insp| insp(&expr));
                    let score = string.len() + config.penalizer.as_ref().map(|scorer| scorer(&expr)).unwrap_or(0);

                    solutions.push(expr);
                    ui.push_solution(string, score, inspection);
                },

                FinishedBatch {length, count} => {
                    counts[length].0 += count;
                    total_count += count;
                }

                UpdateStatus {thread_id, status} => {
                    let thread = threads.iter_mut().find(|thread| thread.id == thread_id).unwrap();
                    thread.status = Some(status);
                }

                Done {thread_id, length} => {
                    let thread = threads.iter_mut().find(|thread| thread.id == thread_id).unwrap();

                    thread.status = None;
                    counts[length].1 += 1;

                    if counts[length].1 == op_requirements.len() {
                        ui.finished_expression_length(length, counts[length].0);
                    }

                    threads.retain(|thread| thread.id != thread_id);
                }
            }
        }

        // Spawn threads up to the thread limit (as long as there are
        // tasks to give them).

        while threads.len() < target_thread_count {
            let Some((length, op_requirement)) = task_iterator.next() else {break};

            threads.push(Thread {
                status: None,
                id: (0..).find(|x| threads.iter().all(|thread| thread.id != *x)).unwrap(),
            });

            let idx = threads.len() - 1;
            let tx_clone = tx.clone();
            let judge_clone = config.judge.clone();
            let thread_id = threads[idx].id;
            let report_every = config.report_every;

            threads[idx].status = None;

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
            ui.set_thread_statuses(threads.iter().map(|thread| thread.status.clone()).collect());

            ui.draw();

            // Reset the clock.

            frame_clock = frame_clock_max;
        }

        frame_clock -= 1;

        // Sleep for a bit to stop the control thread from hotlooping.

        std::thread::sleep(cycle_duration);
    }

    // Todo: tear down the threads.

    (total_count, solutions)
}

fn find_with_length_and_op<N: Number, const C: usize>(
    thread_id: usize,
    notification_spacing: u128,
    judge: Judge<N, C>,
    length: usize,
    op_requirement: Option<Option<Op>>,
    mpsc: mpsc::Sender<ThreadReport<N, C>>,
) {
    //println!("New thread! — length = {length}, op_requirement = {op_requirement:?}");

    let mut count = 0u128;
    let mut expr = Expression {
        field: vec![255; 16],
        nothing: PhantomData::default(),
    };
    let mut writer = ExpressionWriter::new(C, length, op_requirement);

    while writer.write(&mut expr.field) {
        count += 1;

        if judge(&expr) {
            mpsc.send(FoundSolution {
                expr: expr.clone(),
            }).unwrap();
        } else if count == notification_spacing {
            mpsc.send(UpdateStatus {
                thread_id,
                expr: expr.clone(),
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

