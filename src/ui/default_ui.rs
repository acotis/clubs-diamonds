
use std::time::Duration;
use chrono::{DateTime, Local, TimeDelta};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use ratatui::layout::Flex;
use ratatui::style::{Style, Modifier};
use ratatui::widgets::{Widget, List, ListItem};
use ratatui::prelude::{Buffer, Layout, Direction, Rect, Constraint, Line, Span, Color, Stylize};
use crossterm::event::{self, Event::Key, KeyCode::Char, KeyCode::Esc, KeyEvent, KeyEventKind};
use lazy_static::lazy_static;

use super::ThreadStatus::{self, *};
use super::UISignal::{self, *};
use super::UI;
use super::utils;

lazy_static! {
    static ref STYLE_BLANK:                   Style = Style::default();
    static ref STYLE_TITLE:                   Style = Style::default().fg(Color::White);
    static ref STYLE_CONTROLS:                Style = Style::default().add_modifier(Modifier::DIM);
    static ref STYLE_DOCKER_ITEM:             Style = Style::default().add_modifier(Modifier::DIM);
    static ref STYLE_LABEL:                   Style = Style::default().fg(Color::Gray);
    static ref STYLE_VALUE:                   Style = Style::default().fg(Color::Gray);
    static ref STYLE_ALT_VALUE:               Style = Style::default().add_modifier(Modifier::DIM);
    static ref STYLE_MISSING_VALUE:           Style = Style::default().fg(Color::Indexed(241)).add_modifier(Modifier::ITALIC);
    static ref STYLE_DESCRIPTION:             Style = Style::default().fg(Color::Indexed(246));
    static ref STYLE_SOLUTION:                Style = Style::default().fg(Color::Blue);
    static ref STYLE_SOLUTION_HIGHLIGHT:      Style = Style::default().bg(Color::Blue);
    static ref STYLE_SOLUTION_META:           Style = Style::default().fg(Color::Indexed(245)); // gray
    static ref STYLE_SOLUTION_HIGHLIGHT_META: Style = Style::default().bg(Color::Indexed(245)); // gray
    static ref STYLE_THREAD:                  Style = Style::default().fg(Color::Indexed(172)); // orange
    static ref STYLE_THREAD_META:             Style = Style::default().fg(Color::Indexed(241)); // darker gray
    static ref STYLE_INSPECTION:              Style = Style::default(); //.bg(Color::Magenta);
    static ref STYLE_NEWS_HEADER:             Style = Style::default().fg(Color::Indexed(106)).underlined(); // green
    static ref STYLE_CONFIRM:                 Style = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    static ref STYLE_DEBUG_BANNER:            Style = Style::default().fg(Color::Indexed(52)).bg(Color::Indexed(167)); // dark red on pale red
}

struct StatMoment {
    timestamp: DateTime<Local>,
    expr_count: u128,
    raw_seconds: f64,
    unpaused_seconds: f64,
    thread_seconds: f64,
}

impl StatMoment {
    fn zero() -> Self {
        Self {
            timestamp: Local::now(),
            expr_count: 0,
            raw_seconds: 0.0,
            unpaused_seconds: 0.0,
            thread_seconds: 0.0,
        }
    }

    fn step_to_now(&self, expr_count: u128, thread_count: usize, paused: bool) -> Self {
        let timestamp = Local::now();
        let interval = (timestamp - self.timestamp).as_seconds_f64();

        Self {
            timestamp,
            expr_count,
            raw_seconds: self.raw_seconds + interval,
            thread_seconds: self.thread_seconds + thread_count as f64 * interval,
            unpaused_seconds: self.unpaused_seconds + (if paused {0.0} else {1.0}) * interval,
        }
    }
}

pub struct DefaultUI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    face: DefaultUIFace,
}

struct DefaultUIFace {
    solutions_found: Vec<(String, usize, Option<String>)>,
    solution_selected: Option<usize>,
    description: Option<String>,
    inspector_enabled: bool,
    debug_banner_enabled: bool,
    stat_moments: Vec<StatMoment>,
    in_quit_dialog: bool,
    paused: bool,
    target_thread_count: usize,
    thread_statuses: Vec<Option<ThreadStatus>>,
    news_feed: Vec<(DateTime<Local>, f64, String)>,

    debug_banner_shown: bool,
    description_shown: bool,
    inspector_shown: bool,
    stats_shown: bool,
    threads_shown: bool,
    news_feed_shown: bool,
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
                solutions_found: vec![],
                solution_selected: None,
                description: None,
                inspector_enabled: false,
                debug_banner_enabled: true,
                stat_moments: vec![StatMoment::zero()],
                in_quit_dialog: false,
                paused: false,
                target_thread_count: 0,
                thread_statuses: vec![],
                news_feed: vec![],

                #[cfg(debug_assertions)] debug_banner_shown: true,
                #[cfg(not(debug_assertions))] debug_banner_shown: false,
                description_shown: true,
                inspector_shown: true,
                stats_shown: true,
                threads_shown: true,
                news_feed_shown: true,
            }
        }
    }

    fn require_manual_exit() -> bool {
        true
    }

    fn set_inspector_enabled(&mut self, enabled: bool) {
        self.face.inspector_enabled = enabled;
    }

    fn set_debug_banner_enabled(&mut self, enabled: bool) {
        self.face.debug_banner_enabled = enabled;
    }

    fn finished_expression_length(&mut self, length: usize, count: u128) {
        self.face.news_feed.push((
            Local::now(),
            self.face.last_stat_moment().unpaused_seconds,
            format!(
                "Tried {} expr{} of length {}.",
                utils::with_commas(count),
                if count == 1 {""} else {"s"},
                length,
            )
        ));
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
        self.face.push_stat_moment(
            self.face.last_stat_moment().step_to_now(
                total_count,
                self.face.unpaused_thread_count(),
                self.face.paused,
            )
        );
    }

    fn set_thread_statuses(&mut self, thread_statuses: Vec<Option<ThreadStatus>>) {
        self.face.push_stat_moment(
            self.face.last_stat_moment().step_to_now(
                self.face.total_count(),
                self.face.unpaused_thread_count(),
                self.face.paused,
            )
        );

        self.face.thread_statuses = thread_statuses;
    }

    fn set_description(&mut self, description: String) {
        self.face.description = Some(description)
    }

    fn draw(&mut self) {

        // Clean up data points from before five seconds ago (but keep the very
        // first data point).

        let five_second_mark = self.face.five_second_mark();
        if five_second_mark > 1000 {
            self.face.stat_moments.drain(1..five_second_mark);
        }

        // Draw self.

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

                        Char('+') => {ret.push(IncreaseThreadCount);}
                        Char('-') => {ret.push(DecreaseThreadCount);}
                        Char('p') => {ret.push(PauseUnpause); self.face.paused = !self.face.paused;}

                        Char('d') => {self.face.description_shown = !self.face.description_shown;}
                        Char('i') => {self.face.inspector_shown = !self.face.inspector_shown;}
                        Char('s') => {self.face.stats_shown = !self.face.stats_shown;}
                        Char('t') => {self.face.threads_shown = !self.face.threads_shown;}
                        Char('n') => {self.face.news_feed_shown = !self.face.news_feed_shown;}

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

        let mut db_items = vec![]; // Items for the dashboard (it's all implemented as a single list).

        if self.debug_banner_enabled && self.debug_banner_shown {db_items.push(self.debug_banner_ui());}
        if self.description_shown  {db_items.push(self.description_ui());}
        if self.inspector_shown    {db_items.push(self.solution_inspector_ui());}
        if self.stats_shown        {db_items.push(self.stats_ui());}
        if self.threads_shown      {db_items.push(self.thread_viewer_ui());}
        if self.news_feed_shown    {db_items.push(self.news_feed_ui());}

        // Intersperse blank lines between the panels.

        let mut i = 1;

        while i < db_items.len() {
            db_items.insert(i, vec![ListItem::from(Span::raw("").style(*STYLE_BLANK))]);
            i += 2;
        }

        let db = db_items.concat();

        // Render the components.

        List::new(sl).render(solution_area, buf);
        List::new(db).render(dashboard_area, buf);
    }
}

impl DefaultUIFace {
    fn format_solution(solution: &str, score: usize, selected: bool) -> Vec<Span<'_>> {
        if selected {
            vec![
                Span::raw(format!("{}", format!("[{score}]"))).style(*STYLE_SOLUTION_HIGHLIGHT_META),
                Span::raw(format!(" ")).style(*STYLE_SOLUTION_HIGHLIGHT_META),
                Span::raw(format!("{}", solution)).style(*STYLE_SOLUTION_HIGHLIGHT),
            ]
        } else {
            vec![
                Span::raw(format!("{}", format!("[{score}]"))).style(*STYLE_SOLUTION_META),
                Span::raw(format!(" ")).style(*STYLE_SOLUTION_META),
                Span::raw(format!("{}", solution)).style(*STYLE_SOLUTION),
            ]
        }
    }

    fn unpaused_thread_count(&self) -> usize {
        self.thread_statuses.iter().filter(|status| !matches!(status, Some(Paused(_)))).count()
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

        if self.in_quit_dialog && !self.stats_shown {
            solution_title.push(Span::raw(" ").style(*STYLE_BLANK));
            solution_title.push(Span::raw("quit? [y/N]").style(*STYLE_CONFIRM));
        } else {
            solution_title.push(Span::raw(format!("({})", self.solutions_found.len())).style(*STYLE_TITLE));
            solution_title.push(Span::raw(" ").style(*STYLE_BLANK));

            if !self.description_shown {solution_title.push(Span::raw("D").style(*STYLE_DOCKER_ITEM))}
            if !self.inspector_shown   {solution_title.push(Span::raw("I").style(*STYLE_DOCKER_ITEM))}
            if !self.stats_shown       {solution_title.push(Span::raw("S").style(*STYLE_DOCKER_ITEM))}
            if !self.threads_shown     {solution_title.push(Span::raw("T").style(*STYLE_DOCKER_ITEM))}
            if !self.news_feed_shown   {solution_title.push(Span::raw("N").style(*STYLE_DOCKER_ITEM))}
        }

        // Push the title and create the title underline bar.

        ret.push(ListItem::from(Line::from(solution_title)));
        ret.push(ListItem::from(Span::raw("—".repeat(22)).style(*STYLE_TITLE)));

        // Solutions.

        for (idx, (solution, score, _insepction)) in self.solutions_found.iter().enumerate() {
            ret.push(ListItem::new(Line::from(Self::format_solution(solution, *score, self.solution_selected == Some(idx)))));
        }

        // Return.

        ret
    }

    fn debug_banner_ui(&self) -> Vec<ListItem<'_>> {
        let lines = [
        //   12345678901234567890123456789012345678901234567890
            "                                                  ",
            "  Warning: you are running Clubs in debug mode,   ",
            "  which slows it down by ~10x. Consider running   ",
            "  it in release mode with `cargo run --release`   ",
            "  Hide this banner with Searcher::no_banner().    ",
            "                                                  ",
        ];

        lines
            .into_iter()
            .map(|line| ListItem::new(Line::from(Span::raw(line).style(*STYLE_DEBUG_BANNER),)))
            .collect()
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
            ret.push(ListItem::new(Line::from("add a description with Searcher::description()").style(*STYLE_MISSING_VALUE)));
        }

        // Return.

        ret
    }

    fn solution_inspector_ui(&self) -> Vec<ListItem<'_>> {
        let mut ret = vec![];

        // Title.

        let title_span = Span::raw("Solution inspector").style(*STYLE_TITLE);
        let space_span = Span::raw(" ").style(*STYLE_BLANK);

        let spotlight_spans = if let Some(idx) = self.solution_selected {
            Self::format_solution(&self.solutions_found[idx].0, self.solutions_found[idx].1, false)
        } else {
            vec![]
        };
        
        // (Compute the total width that the controls blurb is allowed to
        // take up.)
        
        let mut controls_width = 50
            - title_span.width()
            - space_span.width()
            - spotlight_spans.iter().map(|span| span.width()).sum::<usize>()
            - space_span.width(); // a second time for after the controls

        if controls_width < 3 || controls_width > 50 {
            controls_width = 3;
        }

        let mut controls_text = format!("(J/K: navigate, I: hide)");

        if controls_text.len() > controls_width {
            controls_text.truncate(controls_width - 3);
            controls_text += "...";
        }

        if controls_text.len() < controls_width {
            controls_text += &" ".repeat(controls_width - controls_text.len());
        }

        let controls_span = Span::raw(controls_text).style(*STYLE_CONTROLS);

        // Create the title and title bar.

        ret.push(ListItem::from(Line::from(vec![
            vec![title_span, space_span.clone(), controls_span, space_span],
            spotlight_spans
        ].into_iter().flatten().collect::<Vec<_>>())));

        ret.push(ListItem::new(Span::raw("—".repeat(50)).style(*STYLE_TITLE)));

        // Inspection text.

        if self.inspector_enabled {
            if self.solutions_found.len() > 0 {
                if let Some(idx) = self.solution_selected {
                    if let Some(ref inspection) = self.solutions_found[idx].2 {
                        for line in inspection.lines() {
                            ret.push(ListItem::from(Span::raw(format!("{line:50}")).style(*STYLE_INSPECTION)));
                        }
                    } else {
                        ret.push(ListItem::new(Line::from("error: missing inspection (???)").style(*STYLE_MISSING_VALUE)));
                    }
                } else {
                    ret.push(ListItem::new(Line::from("no solution selected").style(*STYLE_MISSING_VALUE)));
                }
            } else {
                ret.push(ListItem::new(Line::from("no solutions found yet").style(*STYLE_MISSING_VALUE)));
            }
        } else {
            ret.push(ListItem::new(Line::from("add an inspector with Searcher::inspector()").style(*STYLE_MISSING_VALUE)));
        }

        // Return.

        ret
    }

    fn stat_line(label: &str, alt_value: &str, main_value: &str) -> Line<'static> {
        let padding = 50 - label.chars().count() - alt_value.chars().count() - main_value.chars().count();

        Line::from(vec![
            Span::raw(format!("{label}")).style(*STYLE_LABEL),
            Span::raw(" ".repeat(padding)).style(*STYLE_BLANK),
            Span::raw(format!("{alt_value}")).style(*STYLE_ALT_VALUE),
            Span::raw(format!("{main_value}")).style(*STYLE_VALUE),
        ])
    }

    fn numeric_stat_line(label: &str, value: u128) -> Line<'static> {
        Self::stat_line(
            label,
            &format!("{} = ", utils::with_commas(value)),
            &format!("{}", utils::as_power_of_two(value))
        )
    }

    fn five_second_mark(&self) -> usize {
        let last_unpaused_seconds = self.last_stat_moment().unpaused_seconds;

        self.stat_moments.partition_point(|moment| {
            (last_unpaused_seconds - moment.unpaused_seconds) > 5.0
        }).min(self.stat_moments.len() - 1)
    }

    fn push_stat_moment(&mut self, moment: StatMoment) {
        let last = self.last_stat_moment();

        if moment.expr_count == last.expr_count
        && moment.unpaused_seconds == last.unpaused_seconds
        && moment.thread_seconds == last.thread_seconds {
            *self.stat_moments.last_mut().unwrap() = moment;
        } else {
            self.stat_moments.push(moment);
        }
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
            Span::raw(", S: hide)").style(*STYLE_CONTROLS),
        ]);
        
        // Intermediate calculations.

        let five_second_mark = self.five_second_mark();
        let count = self.total_count();
        let deci_seconds = 1.max((self.last_stat_moment().unpaused_seconds * 10.0) as u128);
        let deci_thread_seconds = 1.max((self.last_stat_moment().thread_seconds * 10.0) as u128);

        let count_recent = self.total_count() - self.stat_moments[five_second_mark].expr_count;
        let deci_seconds_recent = 1.max(((self.last_stat_moment().unpaused_seconds - self.stat_moments[five_second_mark].unpaused_seconds) * 10.0) as u128);
        let deci_thread_seconds_recent = 1.max(((self.last_stat_moment().thread_seconds - self.stat_moments[five_second_mark].thread_seconds) * 10.0) as u128);

        // Return.

        vec![
            ListItem::from(stats_title),
            ListItem::from(Span::raw("—".repeat(50)).style(*STYLE_TITLE)),

            ListItem::from(Self::stat_line(
                "Uptime",
                &format!("{} — ", utils::format_timestamp(&self.start_time())),
                &format!("{}", utils::format_duration(&TimeDelta::seconds((deci_seconds / 10) as _), false)),
            )),
            ListItem::from(Self::numeric_stat_line("Count", count)),
            ListItem::from(Self::numeric_stat_line("Expr/s", count_recent * 10 / deci_seconds_recent)),
            ListItem::from(Self::numeric_stat_line("Expr/s/thread", count_recent * 10 / deci_thread_seconds_recent)),
            ListItem::from(Self::numeric_stat_line("Life avg. expr/s", count * 10 / deci_seconds)),
            ListItem::from(Self::numeric_stat_line("Life avg. expr/s/thread", count * 10 / deci_thread_seconds)),
            ListItem::from(Self::numeric_stat_line("Moment count", self.stat_moments.len() as _)),
        ]
    }

    fn thread_viewer_ui(&self) -> Vec<ListItem<'_>> {
        let mut ret = vec![];

        // Title.

        ret.push(ListItem::new(Line::from(vec![
            Span::raw(format!(
                "{}/{} threads",
                self.thread_statuses.len(),
                self.target_thread_count,
            )).style(*STYLE_TITLE),
            Span::raw(" ").style(*STYLE_BLANK),
            Span::raw("(+/-: change, ").style(*STYLE_CONTROLS),
            if self.paused {
                Span::raw("P: resume, ").style(*STYLE_CONTROLS)
            } else {
                Span::raw("P: pause, ").style(*STYLE_CONTROLS)
            },
            Span::raw("T: hide)").style(*STYLE_CONTROLS),
        ])));

        ret.push(ListItem::new(Span::from("—".repeat(50)).style(*STYLE_TITLE)));

        // Threads.

        let number_width = 1 + match self.thread_statuses.len() {0 => 1, x => x.ilog(10) + 1} as usize;

        for (thread, thread_id) in self.thread_statuses.iter().zip(1..) {
            let number_span = Span::raw(format!("{:>number_width$} ", format!("{thread_id}."))).style(*STYLE_LABEL);

            match thread {
                None => {
                    ret.push(ListItem::from(Line::from(vec![
                        number_span,
                        Span::raw("<New thread...>").style(*STYLE_THREAD_META),
                    ])));
                }
                Some(Paused(expr)) => {
                    ret.push(ListItem::from(Line::from(vec![
                        number_span,
                        Span::raw(format!("[{}] ", expr.len())).style(*STYLE_THREAD_META),
                        Span::raw(format!("{}", expr)).style(*STYLE_ALT_VALUE),
                        Span::raw(" <Paused>").style(*STYLE_THREAD_META),
                    ])));
                }
                Some(Searching(expr)) => {
                    ret.push(ListItem::from(Line::from(vec![
                        number_span,
                        Span::raw(format!("[{}] ", expr.len())).style(*STYLE_THREAD_META),
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

            let time_in = news_item.1;
            let time_ago = self.last_stat_moment().unpaused_seconds - news_item.1;

            ret.push(ListItem::from(Line::from(vec![
                Span::raw(utils::format_timestamp(&news_item.0)).style(*STYLE_NEWS_HEADER),
                Span::raw(" ").style(*STYLE_BLANK),
                Span::raw(format!(
                    "({} in | {} ago)",
                    utils::format_duration(&TimeDelta::seconds(time_in as _), false),
                    utils::format_duration(&TimeDelta::seconds(time_ago as _), true),
                )).style(*STYLE_CONTROLS),
            ])));

            ret.push(ListItem::from(Line::from(vec![
                Span::raw(format!("{}", news_item.2)).style(*STYLE_VALUE)
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

