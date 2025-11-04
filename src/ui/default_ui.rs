
use std::time::Duration;
use chrono::{DateTime, Local};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use ratatui::layout::Flex;
use ratatui::style::{Style, Modifier};
use ratatui::widgets::{Widget, List, ListItem};
use ratatui::prelude::{Buffer, Layout, Direction, Rect, Constraint, Line, Span, Color, Stylize};
use crossterm::event::{self, Event::Key, KeyCode::Char, KeyCode::Esc, KeyEvent, KeyEventKind};
use lazy_static::lazy_static;

use super::Thread;
use super::ThreadStatus::*;
use super::UISignal::{self, *};
use super::UI;
use crate::utils;

use DashboardBlock::*;

lazy_static! {
    static ref STYLE_BLANK:                   Style = Style::default();
    static ref STYLE_TITLE:                   Style = Style::default().fg(Color::White);
    static ref STYLE_CONTROLS:                Style = Style::default().add_modifier(Modifier::DIM);
    static ref STYLE_DOCKER_ITEM:             Style = Style::default().add_modifier(Modifier::DIM);
    static ref STYLE_LABEL:                   Style = Style::default().fg(Color::Gray);
    static ref STYLE_VALUE:                   Style = Style::default().fg(Color::Gray);
    static ref STYLE_ALT_VALUE:               Style = Style::default().add_modifier(Modifier::DIM);
    static ref STYLE_DESCRIPTION:             Style = Style::default().fg(Color::Gray);
    static ref STYLE_SOLUTION:                Style = Style::default().fg(Color::Blue);
    static ref STYLE_SOLUTION_HIGHLIGHT:      Style = Style::default().bg(Color::Blue);
    static ref STYLE_SOLUTION_META:           Style = Style::default().fg(Color::LightBlue);
    static ref STYLE_SOLUTION_HIGHLIGHT_META: Style = Style::default().bg(Color::LightBlue);
    static ref STYLE_THREAD:                  Style = Style::default().fg(Color::Yellow);
    static ref STYLE_THREAD_META:             Style = Style::default().fg(Color::LightYellow);
    static ref STYLE_INSPECTION:              Style = Style::default(); //.bg(Color::Magenta);
    static ref STYLE_NEWS_HEADER:             Style = Style::default().fg(Color::Green).underlined();
    static ref STYLE_CONFIRM:                 Style = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DashboardBlock {
    Stats,
    SolutionInspector,
    ThreadViewer,
    NewsFeed,
    Description,
}

struct StatMoment {
    timestamp: DateTime<Local>,
    expr_count: u128,
    thread_seconds: f64,
}

impl StatMoment {
    fn zero() -> Self {
        Self {
            timestamp: Local::now(),
            expr_count: 0,
            thread_seconds: 0.0,
        }
    }

    fn step_to_now(&self, expr_count: u128, thread_count: usize) -> Self {
        let timestamp = Local::now();

        Self {
            expr_count,
            thread_seconds:
                self.thread_seconds + 
                    thread_count as f64 *
                    (timestamp - self.timestamp).as_seconds_f64(),
            timestamp
        }
    }
}

pub struct DefaultUI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    face: DefaultUIFace,
}

struct DefaultUIFace {
    target_thread_count: usize,
    thread_statuses: Vec<Thread>,
    solutions_found: Vec<(String, usize, Option<String>)>,
    stat_moments: Vec<StatMoment>,
    solution_selected: Option<usize>,
    hidden_blocks: Vec<DashboardBlock>,
    shown_blocks: Vec<DashboardBlock>,
    in_quit_dialog: bool,
    news_feed: Vec<(DateTime<Local>, String)>,
    description: Option<String>
}

impl Drop for DefaultUI {
    fn drop(&mut self) {
        ratatui::restore()
    }
}

impl UI for DefaultUI {
    fn new() -> Self {
        Self {
            terminal: ratatui::init(),
            face: DefaultUIFace {
                target_thread_count: 0,
                thread_statuses: vec![],
                solutions_found: vec![],
                stat_moments: vec![StatMoment::zero()],
                solution_selected: None,
                hidden_blocks: vec![],
                shown_blocks: vec![Description, SolutionInspector, Stats, ThreadViewer, NewsFeed],
                in_quit_dialog: false,
                news_feed: vec![],
                description: None,
            }
        }
    }

    fn push_news_item(&mut self, news_item: String) {
        self.face.news_feed.push((Local::now(), news_item));
    }

    fn push_solution(&mut self, face: String, score: usize, inspection: Option<String>) {
        let position = self.face.solutions_found.partition_point(|(_expr, its_score, _inspection)| *its_score <= score);
        self.face.solution_selected = self.face.solution_selected.map(|selected| if position <= selected {selected + 1} else {selected});
        self.face.solutions_found.insert(position, (face, score, inspection))
    }

    fn set_target_thread_count(&mut self, target_thread_count: usize) {
        self.face.target_thread_count = target_thread_count;
    }

    fn set_total_count(&mut self, total_count: u128) {
        self.face.stat_moments.push(
            self.face.last_stat_moment().step_to_now(
                total_count,
                self.face.thread_statuses.len()
            )
        );
    }

    fn set_thread_statuses(&mut self, thread_statuses: Vec<Thread>) {
        self.face.stat_moments.push(
            self.face.last_stat_moment().step_to_now(
                self.face.total_count(),
                self.face.thread_statuses.len()
            )
        );

        self.face.thread_statuses = thread_statuses;
    }

    fn set_description(&mut self, description: String) {
        self.face.description = Some(description)
    }

    fn draw(&mut self) {
        self.terminal.draw(|frame| frame.render_widget(&self.face, frame.area())).unwrap();
    }

    fn handle_inputs(&mut self) -> Vec<UISignal> {
        let mut ret = vec![];

        while event::poll(Duration::from_millis(0)).unwrap() {
            match event::read().unwrap() {
                Key(KeyEvent {kind: KeyEventKind::Press, code, modifiers: _, state: _}) => {
                    if self.face.in_quit_dialog {
                        if code == Char('y') {
                            ret.push(Quit);
                            return ret;
                        } else {
                            self.face.in_quit_dialog = false;
                            continue;
                        }
                    }

                    match code {
                        Char('q') => {
                            self.face.in_quit_dialog = true;
                        }
                        Char('j') => {
                            self.face.solution_selected = 
                                  self.face.solution_selected
                                      .map(|number| (number + 1).clamp(0, self.face.solutions_found.len() - 1))
                                      .or(if self.face.solutions_found.is_empty() {None} else {Some(0)});
                        }
                        Char('k') => {
                            self.face.solution_selected = 
                                 self.face.solution_selected
                                     .map(|number| if number > 0 {number - 1} else {0})
                                     .or(if self.face.solutions_found.is_empty() {None} else {Some(0)});
                        }
                        Char('+') => {
                            ret.push(IncreaseThreadCount);
                        }
                        Char('-') => {
                            ret.push(DecreaseThreadCount);
                        }
                        Char('s') => {
                            if self.face.shown_blocks.contains(&Stats) {
                                self.face.shown_blocks.retain(|x| *x != Stats);
                                self.face.hidden_blocks.push(Stats);
                            } else {
                                self.face.hidden_blocks.retain(|x| *x != Stats);
                                self.face.shown_blocks.push(Stats);
                            }
                        }
                        Char('i') => {
                            if self.face.shown_blocks.contains(&SolutionInspector) {
                                self.face.shown_blocks.retain(|x| *x != SolutionInspector);
                                self.face.hidden_blocks.push(SolutionInspector);
                            } else {
                                self.face.hidden_blocks.retain(|x| *x != SolutionInspector);
                                self.face.shown_blocks.push(SolutionInspector);
                            }
                        }
                        Char('t') => {
                            if self.face.shown_blocks.contains(&ThreadViewer) {
                                self.face.shown_blocks.retain(|x| *x != ThreadViewer);
                                self.face.hidden_blocks.push(ThreadViewer);
                            }  else {
                                self.face.hidden_blocks.retain(|x| *x != ThreadViewer);
                                self.face.shown_blocks.push(ThreadViewer);
                            }
                        }
                        Char('n') => {
                            if self.face.shown_blocks.contains(&NewsFeed) {
                                self.face.shown_blocks.retain(|x| *x != NewsFeed);
                                self.face.hidden_blocks.push(NewsFeed);
                            }  else {
                                self.face.hidden_blocks.retain(|x| *x != NewsFeed);
                                self.face.shown_blocks.push(NewsFeed);
                            }
                        }
                        Char('d') => {
                            if self.face.shown_blocks.contains(&Description) {
                                self.face.shown_blocks.retain(|x| *x != Description);
                                self.face.hidden_blocks.push(Description);
                            }  else {
                                self.face.hidden_blocks.retain(|x| *x != Description);
                                self.face.shown_blocks.push(Description);
                            }
                        }
                        Esc => {
                            self.face.solution_selected = None;
                        }
                        _ => (),
                    }
                },
                _ => ()
            }
        }

        ret
    }
}

impl Widget for &DefaultUIFace {
    fn render(self, area: Rect, buf: &mut Buffer) {

        // Split the main area into a solutions list and a dashboard.

        let [solution_area, dashboard_area] =
            Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(2)
                .vertical_margin(1)
                .spacing(3)
                .constraints([
                    Constraint::Length(22),
                    Constraint::Min(50),
                ])
                .flex(Flex::Start)
                .areas(area);

        // Create the solution list.

        let sl = self.solution_list_ui();

        // Create the dashboard.

        let mut db = vec![]; // Items for the dashboard (it's all implemented as a single list).
        let mut first = true;

        for item in &self.shown_blocks {
            if !first {db.push(ListItem::from(Span::raw("").style(*STYLE_BLANK)));}
            first = false;

            match item {
                Stats             => {db.append(&mut self.stats_ui());}
                SolutionInspector => {db.append(&mut self.solution_inspector_ui());}
                ThreadViewer      => {db.append(&mut self.thread_viewer_ui());}
                NewsFeed          => {db.append(&mut self.news_feed_ui());}
                Description       => {db.append(&mut self.description_ui());}
            }
        }

        // Render the components.

        List::new(sl).render(solution_area, buf);
        List::new(db).render(dashboard_area, buf);
    }
}

impl DefaultUIFace {
    fn format_solution(solution: &str, score: usize, selected: bool) -> ListItem<'_> {
        if selected {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{}", format!("[{score}]"))).style(*STYLE_SOLUTION_HIGHLIGHT_META),
                Span::raw(format!(" ")).style(*STYLE_SOLUTION_HIGHLIGHT_META),
                Span::raw(format!("{}", solution)).style(*STYLE_SOLUTION_HIGHLIGHT),
            ]))
        } else {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{}", format!("[{score}]"))).style(*STYLE_SOLUTION_META),
                Span::raw(format!(" ")).style(*STYLE_SOLUTION_META),
                Span::raw(format!("{}", solution)).style(*STYLE_SOLUTION),
            ]))
        }
    }

    fn last_stat_moment(&self) -> &StatMoment {
        self.stat_moments.last().unwrap()
    }

    fn total_count(&self) -> u128 {
        self.last_stat_moment().expr_count
    }

    fn start_time(&self) -> DateTime<Local> {
        self.stat_moments[0].timestamp
    }

    fn solution_list_ui(&self) -> Vec<ListItem<'_>> {
        let mut ret = vec![];

        // Title.

        let mut solution_title = vec![];
        solution_title.push(Span::raw("Solutions ").style(*STYLE_TITLE));

        // Dock.

        if self.in_quit_dialog && self.hidden_blocks.contains(&Stats) {
            solution_title.push(Span::raw(" ").style(*STYLE_BLANK));
            solution_title.push(Span::raw("quit? [y/N]").style(*STYLE_CONFIRM));
        } else {
            solution_title.push(Span::raw(format!("({})", self.solutions_found.len())).style(*STYLE_TITLE));
            solution_title.push(Span::raw(" ").style(*STYLE_BLANK));

            for block in &self.hidden_blocks {
                solution_title.push(
                    match block {
                        Stats             => Span::raw("S").style(*STYLE_DOCKER_ITEM),
                        SolutionInspector => Span::raw("I").style(*STYLE_DOCKER_ITEM),
                        ThreadViewer      => Span::raw("T").style(*STYLE_DOCKER_ITEM),
                        NewsFeed          => Span::raw("N").style(*STYLE_DOCKER_ITEM),
                        Description       => Span::raw("D").style(*STYLE_DOCKER_ITEM),
                    }
                );
            }
        }

        // Push the title and create the title underline bar.

        ret.push(ListItem::from(Line::from(solution_title)));
        ret.push(ListItem::from(Span::raw("—".repeat(22)).style(*STYLE_TITLE)));

        // Solutions.

        for (idx, (solution, score, _insepction)) in self.solutions_found.iter().enumerate() {
            ret.push(Self::format_solution(solution, *score, self.solution_selected == Some(idx)));
        }

        // Return.

        ret
    }

    fn description_ui(&self) -> Vec<ListItem<'_>> {
        let mut ret = vec![];

        // Title.

        ret.push(ListItem::new(Line::from(vec![
            Span::raw("Description").style(*STYLE_TITLE),
            Span::raw(" ").style(*STYLE_BLANK),
            Span::raw("(D: hide)").style(*STYLE_CONTROLS),
        ])));
        
        ret.push(ListItem::new(Span::raw("—".repeat(50)).style(*STYLE_TITLE)));

        // Description text.

        if let Some(ref description) = self.description {

            for line in description.lines() {
                let mut display_line = String::new();

                for word in line.split(" ") {
                    if display_line != "" && display_line.len() + 1 + word.len() > 50 {
                        ret.push(ListItem::from(Span::raw(format!("{display_line}")).style(*STYLE_DESCRIPTION)));
                        display_line = String::new();
                    }

                    if display_line != "" {display_line += " "}
                    display_line += word;
                }

                ret.push(ListItem::from(Span::raw(format!("{display_line}")).style(*STYLE_DESCRIPTION)));
            }
        } else {
            ret.push(ListItem::new(Line::from("<no description provided>").style(*STYLE_ALT_VALUE)));
        }

        // Return.

        ret
    }

    fn solution_inspector_ui(&self) -> Vec<ListItem<'_>> {
        let mut ret = vec![];

        // Title.

        ret.push(ListItem::new(Line::from(vec![
            Span::raw("Solution inspector").style(*STYLE_TITLE),
            Span::raw(" ").style(*STYLE_BLANK),
            Span::raw("(J/K: navigate, I: hide)").style(*STYLE_CONTROLS),
        ])));
        
        ret.push(ListItem::new(Span::raw("—".repeat(50)).style(*STYLE_TITLE)));

        // Inspection text.

        if self.solutions_found.len() > 0 {
            if let Some(idx) = self.solution_selected {
                // Copy of expression (todo: find better UI to guarantee that all solutions are visible).
                ret.push(Self::format_solution(&self.solutions_found[idx].0, self.solutions_found[idx].1, false));

                if let Some(ref inspection) = self.solutions_found[idx].2 {
                    for line in inspection.lines() {
                        ret.push(ListItem::from(Span::raw(format!("{line:50}")).style(*STYLE_INSPECTION)));
                    }
                } else {
                    ret.push(ListItem::new(Line::from("<no inspection provided>").style(*STYLE_ALT_VALUE)));
                }
            } else {
                ret.push(ListItem::new(Line::from("<no solution selected>").style(*STYLE_ALT_VALUE)));
            }
        } else {
            ret.push(ListItem::new(Line::from("<no solutions found yet>").style(*STYLE_ALT_VALUE)));
        }

        // Return.

        ret
    }

    fn stats_ui(&self) -> Vec<ListItem<'_>> {

        // Title.

        let stats_title = Line::from(vec![
            Span::raw("Stats").style(*STYLE_TITLE),
            Span::raw(" ").style(*STYLE_BLANK),
            Span::raw("(").style(*STYLE_CONTROLS),
            if self.in_quit_dialog {
                Span::raw("really quit? [y/N]").style(*STYLE_CONFIRM)
            } else {
                Span::raw("Q: quit").style(*STYLE_CONTROLS)
            },
            Span::raw(", S: hide)").style(*STYLE_CONTROLS)
        ]);

        // Uptime.

        let label = "Uptime";
        let ts_string = utils::format_timestamp(&self.start_time());
        let du_string = utils::format_duration(&(Local::now() - self.start_time()), false);
        let value_len = ts_string.len() + du_string.len() + 3;
        let padding = 50 - label.len() - value_len;

        let uptime_line = Line::from(vec![
            Span::raw(label).style(*STYLE_LABEL),
            Span::raw(" ".repeat(padding)).style(*STYLE_BLANK),
            Span::raw(format!("({ts_string})")).style(*STYLE_ALT_VALUE),
            Span::raw(" ").style(*STYLE_BLANK),
            Span::raw(du_string).style(*STYLE_VALUE),
        ]);

        // Count.

        let label = "Count";
        let comma_version = utils::with_commas(self.total_count());
        let power_version = utils::as_power_of_two(self.total_count());
        let value_len = comma_version.len() + power_version.len() + 3;
        let padding = 50 - label.len() - value_len;

        let count_line = Line::from(vec![
            Span::raw(label).style(*STYLE_LABEL),
            Span::raw(" ".repeat(padding)).style(*STYLE_BLANK),
            Span::raw(format!("{comma_version} = ")).style(*STYLE_ALT_VALUE),
            Span::raw(format!("{power_version}")).style(*STYLE_VALUE),
        ]);

        // Speed.
        //
        // The speed of the search is defined as the number of expressions
        // searched in the last 5 seconds, divided by 5 seconds. If the search
        // has not yet been running for 5 seconds, then it is defined as the
        // number of expressions searched so far divided by the amount of time
        // the search has been running.

        // Speed (life avg).
        //
        // The life-average speed of the search is defined as the number of
        // expressions searched in all time, divided by the amount of time
        // the search has been running.

        let label = "Expr/s (life avg)";
        let deciseconds_up = 1.max((Local::now() - self.start_time()).num_milliseconds() / 100);
        let per_second = self.total_count() * 10 / deciseconds_up as u128;
        let comma_version = utils::with_commas(per_second);
        let power_version = utils::as_power_of_two(per_second);
        let value_len = comma_version.len() + power_version.len() + 3;
        let padding = 50 - label.len() - value_len;

        let speed_life_avg_line = Line::from(vec![
            Span::raw(label).style(*STYLE_LABEL),
            Span::raw(" ".repeat(padding)).style(*STYLE_BLANK),
            Span::raw(format!("{comma_version} = ")).style(*STYLE_ALT_VALUE),
            Span::raw(format!("{power_version}")).style(*STYLE_VALUE),
        ]);

        // Speed (per-thread).
        //
        // The per-thread speed of the search is defined as the number of
        // expresssions searched in the last 5 seconds, divided by 5 seconds,
        // divided then by the average number of threads which were active at
        // any given moment during the last 5 seconds. If the search has not yet
        // been running for 5 seconds, then it is defined as the result of this
        // algorithm being applied to the whole time the search has been running.
        
        // Speed (per-thread life average).
        //
        // The life-average per-thread speed of the search is defined as the
        // result when the algorithm above is applied to the whole time the
        // search has been running.

        /*
        let label = "Expr/s (per-thread life avg)";
        let deci_thread_seconds_up = 1.max((self.last_stat_moment().thread_seconds * 10.0) as u128);
        let per_second = self.total_count() * 10 / deci_thread_seconds_up as u128;
        let comma_version = utils::with_commas(per_second);
        let power_version = utils::as_power_of_two(per_second);
        let value_len = comma_version.len() + power_version.len() + 3;
        let padding = 50 - label.len() - value_len;

        let speed_thread_avg_line = Line::from(vec![
            Span::raw(label).style(*STYLE_LABEL),
            Span::raw(" ".repeat(padding)).style(*STYLE_BLANK),
            Span::raw(format!("{comma_version} = ")).style(*STYLE_ALT_VALUE),
            Span::raw(format!("{power_version}")).style(*STYLE_VALUE),
        ]);
        */

        // Return.

        vec![
            ListItem::from(format!("[{}]", self.stat_moments.len())),
            ListItem::from(stats_title),
            ListItem::from(Span::raw("—".repeat(50)).style(*STYLE_TITLE)),
            ListItem::from(uptime_line),
            ListItem::from(count_line),
            ListItem::from(speed_life_avg_line),
            //ListItem::from(speed_thread_avg_line),
        ]
    }

    fn thread_viewer_ui(&self) -> Vec<ListItem<'_>> {
        let mut ret = vec![];

        // Title.

        ret.push(ListItem::new(Line::from(vec![
            Span::raw(format!(
                "{}/{} Threads",
                self.thread_statuses.len(),
                self.target_thread_count,
            )).style(*STYLE_TITLE),
            Span::raw(" ").style(*STYLE_BLANK),
            Span::raw("(+/-: change target, T: hide)").style(*STYLE_CONTROLS),
        ])));

        ret.push(ListItem::new(Span::from("—".repeat(50)).style(*STYLE_TITLE)));

        // Threads.

        let number_width = 1 + match self.thread_statuses.len() {0 => 1, x => x.ilog(10) + 1} as usize;

        for (thread, thread_id) in self.thread_statuses.iter().zip(1..) {
            let number_span = Span::raw(format!("{:>number_width$} ", format!("{thread_id}."))).style(*STYLE_LABEL);

            match &thread.status {
                Empty => {
                    ret.push(ListItem::from(Line::from(vec![
                        number_span,
                    ])));
                }
                Initializing => {
                    ret.push(ListItem::from(Line::from(vec![
                        number_span,
                        Span::raw("Initializing...").style(*STYLE_THREAD_META),
                    ])));
                }
                Searching(expr) | Found(expr) => {
                    ret.push(ListItem::from(Line::from(vec![
                        number_span,
                        Span::raw(format!("{}", format!("[{}] ", expr.len()))).style(*STYLE_THREAD_META),
                        Span::raw(format!("{}", expr)).style(*STYLE_THREAD),
                    ])));
                }
            }
        }

        // Return.

        ret
    }

    fn news_feed_ui(&self) -> Vec<ListItem<'_>> {
        let mut ret = vec![];

        // Title.

        ret.push(ListItem::new(Line::from(vec![
            Span::raw(format!("News feed")).style(*STYLE_TITLE),
            Span::raw(" ").style(*STYLE_BLANK),
            Span::raw("(N: hide)").style(*STYLE_CONTROLS),
        ])));

        ret.push(ListItem::new(Span::from("—".repeat(50)).style(*STYLE_TITLE)));

        // News items.

        let mut first = true;

        for news_item in self.news_feed.iter().rev() {
            if first {
                first = false;
            } else {
                //ret.push(ListItem::from(Line::from("")));
            }

            let time_in = news_item.0 - self.start_time();
            let time_ago = Local::now() - news_item.0;

            ret.push(ListItem::from(Line::from(vec![
                Span::raw(utils::format_timestamp(&news_item.0)).style(*STYLE_NEWS_HEADER),
                Span::raw(" ").style(*STYLE_BLANK),
                Span::raw(format!(
                    "({} in | {} ago)",
                    utils::format_duration(&time_in, false),
                    utils::format_duration(&time_ago, true),
                )).style(*STYLE_CONTROLS),
            ])));

            ret.push(ListItem::from(Line::from(vec![
                Span::raw(format!("{}", news_item.1)).style(*STYLE_VALUE)
            ])));
        }

        // Placeholder text if there are no news items.

        if self.news_feed.len() == 0 {
            ret.push(ListItem::new(Line::from("<no news yet>").style(*STYLE_ALT_VALUE)));
        }

        // Return.

        ret
    }
}

