use chrono::{DateTime, Local, NaiveDate};
use terminal_size::{Width, terminal_size};

const VALUES_AXIS_QUARTERS: usize = 4;

pub(crate) struct DateTimePoint {
    pub datetime: DateTime<Local>,
    pub value: f32,
}

pub(crate) trait ChartPlotter {
    fn plot(&self, points: Vec<DateTimePoint>) -> anyhow::Result<()>;
}

pub(crate) struct ConsoleChartPlotter {
    pub fallback_width: usize,
    pub min_bar_width: usize,
    pub max_bar_width: usize,
    pub date_to_bar_gap: usize,
    pub bar_to_value_gap: usize,
}

impl Default for ConsoleChartPlotter {
    fn default() -> Self {
        Self {
            fallback_width: 80,
            min_bar_width: 10,
            max_bar_width: 100,
            date_to_bar_gap: 2,
            bar_to_value_gap: 2,
        }
    }
}

impl ChartPlotter for ConsoleChartPlotter {
    fn plot(&self, points: Vec<DateTimePoint>) -> anyhow::Result<()> {
        if points.is_empty() {
            println!("No points to display");
            return Ok(());
        }

        let rows = make_rows(&points);
        let layout = self.layout_for(&rows);
        let range = ValueRange::of(&rows);

        print_rows(&rows, &layout, range);
        println!("{}", axis_line(&layout));
        println!("{}", axis_labels(&layout, range));
        Ok(())
    }
}

impl ConsoleChartPlotter {
    fn layout_for(&self, rows: &[Row]) -> Layout {
        let label_w = rows.iter().map(|r| r.label.len()).max().unwrap_or(0);
        let value_w = rows.iter().map(|r| r.value.len()).max().unwrap_or(0);
        let axis_start = label_w + self.date_to_bar_gap;

        let terminal_w = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(self.fallback_width);
        let bar_w = terminal_w
            .saturating_sub(axis_start + 1 + self.bar_to_value_gap + value_w)
            .max(self.min_bar_width)
            .min(self.max_bar_width);

        Layout {
            label_w,
            value_w,
            axis_start,
            bar_w,
            value_gap: self.bar_to_value_gap,
        }
    }
}

/// Column widths of the chart. `axis_start` is the column of the `├`/`┤` symbol,
/// the bar cells follow it.
struct Layout {
    label_w: usize,
    value_w: usize,
    axis_start: usize,
    bar_w: usize,
    value_gap: usize,
}

struct Row<'a> {
    point: &'a DateTimePoint,
    label: String,
    value: String,
}

impl<'a> Row<'a> {
    fn new(point: &'a DateTimePoint) -> Self {
        Self {
            point,
            label: point.datetime.format("%d.%m %H:%M").to_string(),
            value: format_value(point.value),
        }
    }

    fn date(&self) -> NaiveDate {
        self.point.datetime.date_naive()
    }
}

#[derive(Clone, Copy)]
struct ValueRange {
    min: f32,
    max: f32,
}

impl ValueRange {
    fn of(rows: &[Row]) -> Self {
        rows.iter().fold(
            Self {
                min: f32::MAX,
                max: f32::MIN,
            },
            |r, row| Self {
                min: r.min.min(row.point.value),
                max: r.max.max(row.point.value),
            },
        )
    }

    fn middle(&self) -> f32 {
        (self.min + self.max) / 2.0
    }

    /// Index of the bar cell (0..cells) that corresponds to the value.
    fn cell_of(&self, value: f32, cells: usize) -> usize {
        if self.max <= self.min {
            return quarter_offset(2, cells);
        }
        let ratio = (value - self.min) / (self.max - self.min);
        (ratio * (cells - 1) as f32).round() as usize
    }
}

fn make_rows(points: &[DateTimePoint]) -> Vec<Row<'_>> {
    let mut rows: Vec<Row> = points.iter().map(Row::new).collect();
    rows.sort_by_key(|row| row.point.datetime);
    rows
}

fn print_rows(rows: &[Row], layout: &Layout, range: ValueRange) {
    let mut previous_date: Option<NaiveDate> = None;
    for row in rows {
        if let Some(older) = previous_date {
            print_day_gap(row.date(), older);
        }
        println!("{}", row_line(row, layout, range));
        previous_date = Some(row.date());
    }
}

fn print_day_gap(newer: NaiveDate, older: NaiveDate) {
    if newer == older {
        return;
    }
    println!();

    let empty_days = (newer - older).num_days() - 1;
    if empty_days > 0 {
        println!("... {} ...", empty_days_text(empty_days));
        println!();
    }
}

fn row_line(row: &Row, layout: &Layout, range: ValueRange) -> String {
    let marker = range.cell_of(row.point.value, layout.bar_w);
    format!(
        "{label:<label_w$}{label_gap}┤{bar}●{padding}{value_gap}{value:>value_w$}",
        label = row.label,
        label_w = layout.label_w,
        label_gap = spaces(layout.axis_start - layout.label_w),
        bar = "━".repeat(marker),
        padding = spaces(layout.bar_w - 1 - marker),
        value_gap = spaces(layout.value_gap),
        value = row.value,
        value_w = layout.value_w,
    )
}

fn axis_labels(layout: &Layout, range: ValueRange) -> String {
    let first_cell = layout.axis_start + 1;
    let last_cell = layout.axis_start + layout.bar_w;
    let (low, middle, high) = (
        format_value(range.min),
        format_value(range.middle()),
        format_value(range.max),
    );

    let mut line = vec![' '; last_cell + 1];
    write_at(&mut line, layout.axis_start, &low);
    write_at(
        &mut line,
        centered_start(first_cell + quarter_offset(2, layout.bar_w), &middle),
        &middle,
    );
    write_at(&mut line, last_cell + 1 - high.len(), &high);

    line.into_iter().collect::<String>().trim_end().to_string()
}

fn axis_line(layout: &Layout) -> String {
    let last = layout.bar_w - 1;
    let is_quarter_tick = |cell: usize| {
        (1..VALUES_AXIS_QUARTERS).any(|quarter| quarter_offset(quarter, layout.bar_w) == cell)
    };

    let cells: String = (0..layout.bar_w)
        .map(|cell| match cell {
            c if c == last => '┤',
            c if is_quarter_tick(c) => '┼',
            _ => '─',
        })
        .collect();

    format!("{}├{}", spaces(layout.axis_start), cells)
}

/// Cell index where the given quarter of the axis falls (0 = start, 4 = end),
/// rounded to the nearest cell.
fn quarter_offset(quarter: usize, bar_w: usize) -> usize {
    (quarter * (bar_w - 1) + VALUES_AXIS_QUARTERS / 2) / VALUES_AXIS_QUARTERS
}

fn centered_start(center: usize, text: &str) -> usize {
    center.saturating_sub(text.len() / 2)
}

fn write_at(line: &mut [char], start: usize, text: &str) {
    for (offset, ch) in text.chars().enumerate() {
        if let Some(slot) = line.get_mut(start + offset) {
            *slot = ch;
        }
    }
}

fn empty_days_text(count: i64) -> String {
    match count {
        1 => "1 empty day".to_string(),
        n => format!("{n} empty days"),
    }
}

fn format_value(value: f32) -> String {
    format!("{value:.1}")
}

fn spaces(count: usize) -> String {
    " ".repeat(count)
}

/// не смотрел, не ревьюил
#[cfg(test)]
mod manual_tests {
    use super::*;
    use chrono::{Duration, TimeZone};
    use rand::{Rng, SeedableRng, rngs::StdRng, RngExt};
    use std::io::{self, Write};

    struct Scenario {
        name: &'static str,
        count: usize,
        span_days: i64,
        values: (f32, f32),
    }

    const SCENARIOS: &[Scenario] = &[
        Scenario {
            name: "single point",
            count: 1,
            span_days: 1,
            values: (10.0, 100.0),
        },
        Scenario {
            name: "all values equal",
            count: 5,
            span_days: 2,
            values: (42.0, 42.0),
        },
        Scenario {
            name: "one dense day",
            count: 15,
            span_days: 1,
            values: (0.0, 100.0),
        },
        Scenario {
            name: "sparse, with gaps",
            count: 8,
            span_days: 30,
            values: (0.0, 100.0),
        },
        Scenario {
            name: "many points",
            count: 40,
            span_days: 5,
            values: (0.0, 100.0),
        },
        Scenario {
            name: "narrow range, big values",
            count: 10,
            span_days: 3,
            values: (1000.0, 1005.0),
        },
        Scenario {
            name: "negative values",
            count: 10,
            span_days: 4,
            values: (-50.0, 20.0),
        },
    ];

    /// cargo test manual_chart -- --ignored --nocapture
    /// Reproduce a run: CHART_SEED=123 cargo test ...
    #[test]
    #[ignore = "interactive: needs a human and --nocapture"]
    fn manual_chart() {
        let seed = seed_from_env_or_clock();
        println!("seed = {seed}\n");
        let mut rng = StdRng::seed_from_u64(seed);

        for (i, scenario) in SCENARIOS.iter().enumerate() {
            println!(
                "=== [{}/{}] {} ===\n",
                i + 1,
                SCENARIOS.len(),
                scenario.name
            );
            ConsoleChartPlotter::default()
                .plot(random_points(&mut rng, scenario))
                .unwrap();

            if !ask_ok() {
                panic!(
                    "rejected by human: scenario '{}', seed {seed}",
                    scenario.name
                );
            }
            println!();
        }
    }

    fn random_points(rng: &mut StdRng, scenario: &Scenario) -> Vec<DateTimePoint> {
        // Fixed base time keeps a seed fully reproducible.
        let base = Local.with_ymd_and_hms(2026, 10, 8, 14, 30, 0).unwrap();
        let (low, high) = scenario.values;
        let span_minutes = scenario.span_days * 24 * 60;

        (0..scenario.count)
            .map(|_| DateTimePoint {
                datetime: base - Duration::minutes(rng.random_range(0..span_minutes)),
                value: if low < high {
                    rng.random_range(low..high)
                } else {
                    low
                },
            })
            .collect()
    }

    fn ask_ok() -> bool {
        loop {
            print!("\nOK? [y/n]: ");
            io::stdout().flush().unwrap();

            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();
            match answer.trim().to_lowercase().as_str() {
                "y" | "" => return true,
                "n" => return false,
                _ => println!("Type y or n"),
            }
        }
    }

    fn seed_from_env_or_clock() -> u64 {
        std::env::var("CHART_SEED")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64
            })
    }
}
