use regex::Regex;
use std::cmp::{max, min};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{} ({})", self.start, self.end, self.length())
    }
}

impl Range {
    fn length(&self) -> usize {
        self.end - self.start
    }

    fn subtract(&self, other: Range) -> Vec<Range> {
        let res = if (other.start <= self.start && other.end >= self.end) {
            // Return empty list if fully subtracted
            vec![]
        } else if (self.end <= other.start || self.start >= other.end) {
            // return self if no overlap
            vec![self.clone()]
        } else if other.start > self.start && other.end < self.end {
            // Return two ranges if subtract is in the middle
            vec![
                Range {
                    start: self.start,
                    end: other.start,
                },
                Range {
                    start: other.end,
                    end: self.end,
                },
            ]
        } else if other.end > self.end {
            // Clip results if other is higher
            vec![Range {
                start: self.start,
                end: other.start,
            }]
        } else {
            // or lower
            vec![Range {
                start: other.end,
                end: self.end,
            }]
        };
        println!("Subtract result for {} - {}: {:?}", self, other, res);
        return res;
    }
}

#[derive(Debug)]
struct Mapping {
    src: usize,
    dst: usize,
    length: usize,
}

#[derive(Debug)]
struct Step {
    name: String,
    mappings: Vec<Mapping>,
}

impl Step {
    fn maps_to(&self, from: Range) -> Vec<Range> {
        let mut direct_mappings: Vec<Range> = vec![from.clone()];
        let mut mapping: Vec<Range> = self
            .mappings
            .iter()
            .map(|mapping| {
                direct_mappings = direct_mappings
                    .iter()
                    .flat_map(|it| it.subtract(mapping.src_range()))
                    .collect();
                mapping.maps_to(from)
            })
            .filter(|it| it.is_some())
            .map(|it| it.unwrap())
            .collect();
        println!("{}", self.name);
        println!("direct:");
        direct_mappings.iter().for_each(|it| println!("{}", it));
        println!("normal:");
        mapping.iter().for_each(|it| println!("{}", it));
        direct_mappings.append(&mut mapping);
        println!(
            "length: {}",
            direct_mappings.iter().fold(0, |acc, it| acc + it.length())
        );
        return direct_mappings;
    }
}
impl Mapping {
    fn src_range(&self) -> Range {
        Range {
            start: self.src,
            end: self.src_end(),
        }
    }
    fn src_end(&self) -> usize {
        self.src + self.length
    }
    fn dst_end(&self) -> usize {
        self.dst + self.length
    }
    fn maps_to(&self, from: Range) -> Option<Range> {
        if from.end >= self.src && from.start <= self.src + self.length {
            let offset_start = from.start.checked_sub(self.src).unwrap_or(0);
            let actual_from_start = self.src + offset_start;

            let clipped_end_of_range = min(from.end, self.src + self.length);
            let length = clipped_end_of_range - actual_from_start;

            let res = Some(Range {
                start: self.dst + offset_start,
                end: self.dst + offset_start + length,
            });

            return res;
        } else {
            None
        }
    }
}

pub fn solve(input: &str) -> usize {
    let re = Regex::new(r"([0-9\s]+)").unwrap();
    let lines: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<usize> = re.captures(lines[0]).unwrap()[1]
        .split_whitespace()
        .map(|it| it.parse::<usize>().unwrap())
        .collect();

    let steps: Vec<Step> = lines[1..]
        .iter()
        .map(|it| {
            let name = it.split_whitespace().collect::<Vec<&str>>()[0].to_string();
            let mappings = it.lines().collect::<Vec<&str>>()[1..]
                .iter()
                .map(|line| {
                    let split: Vec<usize> = line
                        .split_whitespace()
                        .map(|it| it.parse::<usize>().unwrap())
                        .collect();
                    return Mapping {
                        src: split[1],
                        dst: split[0],
                        length: split[2],
                    };
                })
                .collect();
            return Step {
                name: name,
                mappings,
            };
        })
        .collect();

    // let style = ProgressStyle::default_bar();
    // let multi_progress = MultiProgress::new();
    // let overall_progress = ProgressBar::new((seeds.len() / 2) as u64).with_style(style);
    // let overall_progress = multi_progress.add(overall_progress);

    let seeds: usize = seeds
        .chunks(2)
        .map(|it| {
            // overall_progress.inc(1);
            let range = Range {
                start: it[0],
                end: it[0] + it[1],
            };
            let res = steps.iter().fold(vec![range], |ranges, step| {
                let folded = ranges
                    .iter()
                    .copied()
                    .flat_map(|range| step.maps_to(range))
                    .collect::<Vec<Range>>();
                return folded;
            });

            return res.iter().map(|it| it.start).min().unwrap_or(0);
        })
        .min()
        .unwrap();

    return seeds;
}
