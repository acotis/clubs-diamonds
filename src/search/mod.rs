
mod searcher;
mod expression;
mod writer;
mod pivot;
mod number;
mod verdict;

pub use searcher::Searcher;
pub use expression::{Expression, Revar};
pub use number::Number;
pub use verdict::Verdict;
pub use verdict::Solution;

use std::thread;
use std::sync::mpsc;
use std::marker::PhantomData;
use std::time::Duration;
use std::thread::sleep;

use pivot::Op;
use crate::ui::UI;
use crate::ui::DefaultUI;
use crate::ui::NullUI;
use crate::ui::UISignal::*;
use crate::ui::ThreadStatus::{self, *};
use writer::*;

use ThreadReport::*;
use ThreadCommand::*;

struct Thread {
    id: usize,
    length: usize, // length of expressions being tested
    reported_status: Option<ThreadStatus>,
    should_be_running: bool,
    tx: mpsc::Sender<ThreadCommand>,
}

// Commands from the manager thread to the worker thread.

enum ThreadCommand {
    Pause,
    Unpause,
}

// Reports from the worker thread back to the manager thread.

enum ThreadReport<N: Number, const C: usize, V: Verdict<N, C>> {
    FoundSolution   {expr: Expression<N, C>, verdict: V},
    TriedN          {thread_id: usize, count: u128},
    Done            {thread_id: usize},
    UpdateStatus    {thread_id: usize, status: ThreadStatus},
}

fn run<
    N: Number,
    const C: usize,
    V: Verdict<N, C>,
    U: UI
>(
    config: &Searcher<N, C, V>
)
    -> Vec<V::Wrapper>
{
    thread::scope(|s| {
        // Set up the TUI.

        let cycle_duration = std::time::Duration::from_millis(1); // Time to sleep between control cycles.
        let frame_clock_max = 16; // Only draw a UI frame every N cycles of the control loop.
        let mut frame_clock = 0;

        let mut ui = U::new();

        ui.set_debug_banner_enabled(config.debug_banner_enabled);
        ui.set_inspector_enabled(config.inspector.is_some());

        if let Some(ref description) = config.description {
            ui.set_description(description.clone());
        }

        // Set up the work.

        let mut total_count = 0u128; // total count of expressions
        let mut target_thread_count = config.threads;
        let mut paused = false;
        let mut threads: Vec<Thread> = vec![];
        let mut solutions = vec![];
        let mut counts = [(0, 0); 99];

        let writer_types = WriterType::all();

        let mut task_iterator =
            (config.min_length..=config.max_length)
                .flat_map(|l| writer_types.clone().into_iter().map(move |or| (l, or)))
                .peekable();

        let (tx, rx) = mpsc::channel::<ThreadReport<N, C, V>>();

        'search: loop {

            // Handle messages from the threads.

            while let Ok(msg) = rx.try_recv() {
                match msg {
                    FoundSolution {expr, verdict} => {
                        let string = optional_revar(&format!("{expr}"), config.var_names.as_deref());
                        let wrapper = verdict.wrap(expr);

                        let inspection = config.inspector.as_ref().map(|insp| insp(&wrapper));
                        let score = string.len() + config.penalizer.as_ref().map(|scorer| scorer(&wrapper)).unwrap_or(0);

                        solutions.push(wrapper);
                        ui.push_solution(string, score, inspection);
                    },

                    TriedN {thread_id, count} => {
                        let thread = threads.iter_mut().find(|thread| thread.id == thread_id).unwrap();
                        counts[thread.length].0 += count;
                        total_count += count;
                    }

                    UpdateStatus {thread_id, status} => {
                        let thread = threads.iter_mut().find(|thread| thread.id == thread_id).unwrap();
                        thread.reported_status = Some(status);
                    }

                    Done {thread_id} => {
                        let thread = threads.iter_mut().find(|thread| thread.id == thread_id).unwrap();

                        thread.reported_status = None;
                        counts[thread.length].1 += 1;

                        if counts[thread.length].1 == writer_types.len() {
                            ui.finished_expression_length(thread.length, counts[thread.length].0);
                        }

                        threads.retain(|thread| thread.id != thread_id);
                    }
                }
            }

            // Compute the effective target thread count (0 if we are paused)
            // and effective current thread count (doesn't include paused
            // threads).

            let effective_target_thread_count = if paused {0} else {target_thread_count};
            let mut active_thread_count =
                threads
                    .iter()
                    .filter(|thread| thread.should_be_running)
                    .count();

            // Unpause threads, and/or spawn new ones, up to the thread limit
            // (as long as there are tasks to give them).

            while active_thread_count < effective_target_thread_count {

                // If we have paused threads, unpause one.

                if active_thread_count < threads.len() {
                    threads[active_thread_count].tx.send(Unpause).unwrap();
                    threads[active_thread_count].should_be_running = true;
                }

                // Otherwise, spawn a new one (as long as there are more tasks to
                // allocate).

                else {
                    let Some((length, writer_type)) = task_iterator.next() else {break};
                    let (thread_tx, thread_rx) = mpsc::channel();

                    threads.push(Thread {
                        should_be_running: true,
                        reported_status: None,
                        length,
                        id: (0..).find(|x| threads.iter().all(|thread| thread.id != *x)).unwrap(),
                        tx: thread_tx,
                    });

                    let idx = threads.len() - 1;
                    let tx_clone = tx.clone();
                    let judge_clone = &config.judge;
                    let thread_id = threads[idx].id;
                    let report_every = config.report_every;
                    let constant_cap = config.constant_cap;
                    let var_names = config.var_names.as_deref();

                    s.spawn(move || {
                        find_with_length_and_op(
                            thread_id,
                            report_every,
                            judge_clone,
                            constant_cap,
                            length,
                            writer_type,
                            var_names,
                            tx_clone,
                            thread_rx,
                        );
                    });
                }

                active_thread_count += 1;
            }

            // Pause threads down to the thread limit if necessary.

            while active_thread_count > effective_target_thread_count {
                threads[active_thread_count-1].tx.send(Pause).unwrap();
                threads[active_thread_count-1].should_be_running = false;
                active_thread_count -= 1;
            }

            // If at this point there are no threads and also no tasks left
            // in the task iterator, and the UI type does not require a manual
            // quit in order to exit the search, return.

            if threads.is_empty() && task_iterator.peek() == None && !U::require_manual_exit() {
                break 'search;
            }

            // If this is a UI cycle, handle inputs and draw a frame of the UI.

            if frame_clock == 0 {
                for action in ui.handle_inputs() {
                    match action {
                        Quit => break 'search,
                        IncreaseThreadCount => {target_thread_count += 1}
                        DecreaseThreadCount => {if target_thread_count > 0 {target_thread_count -= 1}}
                        PauseUnpause => {
                            paused = !paused;
                        }
                    }
                }

                ui.set_total_count(total_count);
                ui.set_target_thread_count(target_thread_count);
                ui.set_thread_statuses(threads.iter().map(|thread| thread.reported_status.clone()).collect());

                ui.draw();

                // Reset the clock.

                frame_clock = frame_clock_max;
            }

            frame_clock -= 1;

            // Sleep for a bit to stop the control thread from hotlooping.

            std::thread::sleep(cycle_duration);
        }

        // Todo: tear down the threads.

        solutions
    })
}

fn optional_revar(string: &str, custom_names: Option<&str>) -> String {
    if let Some(names) = custom_names {
        string.revar(names)
    } else {
        string.to_owned()
    }
}

fn find_with_length_and_op<N: Number, const C: usize, V: Verdict<N, C>>(
    thread_id: usize,
    notification_spacing: u128,
    judge: &dyn Fn(&Expression<N, C>) -> V,
    constant_cap: u8,
    length: usize,
    writer_type: WriterType,
    var_names: Option<&str>,
    tx: mpsc::Sender<ThreadReport<N, C, V>>,
    rx: mpsc::Receiver<ThreadCommand>,
) {
    let mut count = 0u128;
    //let mut writer = TempWriter::<N>::new(C, length, constant_cap, writer_type);
    let mut writer = Writer::<N, C>::new(length, WriterContext {location: Location::TOP, const_allowed: true}, Some(writer_type), constant_cap);
    let mut expr = Expression {
        field: vec![255; length],
        nothing: PhantomData::default(),
    };

    let mut paused = false;

    loop {

        // Process inbound messages from the manager thread.

        while let Ok(msg) = rx.try_recv() {
            match msg {
                Pause => {
                    paused = true;
                    tx.send(UpdateStatus {thread_id, status: Paused(optional_revar(&format!("{expr}"), var_names))}).unwrap();
                }
                Unpause => {
                    paused = false;
                }
            }
        }
        
        // Do an appropriate task (either searching or sleeping) according to
        // whether we are paused.

        if paused {
            sleep(Duration::from_millis(100));
        } else {
            loop {
                if writer.write(&mut expr.field) {
                    count += 1;

                    let verdict = judge(&expr);

                    if verdict.is_accept() {
                        tx.send(FoundSolution {verdict, expr: expr.clone()}).unwrap();
                    }

                    if count == notification_spacing {
                        tx.send(TriedN {thread_id, count}).unwrap();
                        tx.send(UpdateStatus {thread_id, status: Searching(optional_revar(&format!("{expr}"), var_names))}).unwrap();
                        count = 0;
                        break;
                    }
                } else {
                    tx.send(TriedN {thread_id, count}).unwrap();
                    tx.send(Done {thread_id}).unwrap();
                    return;
                }
            }
        }
    }
}

