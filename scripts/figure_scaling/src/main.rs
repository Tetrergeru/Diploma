use std::{fmt::Write, fs::read_to_string};

use once_cell::sync::Lazy;
use regex::{Captures, Regex};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

const FNAME: &str = "../../paper-en.tex";
const NEW_HEIGHT: f64 = 100.0;
const NEW_PADDING: f64 = 5.0;

struct FileParser {
    file: Vec<String>,
    result: Vec<String>,
    pos: usize,
}

impl FileParser {
    pub fn new(file: Vec<String>) -> Self {
        Self {
            file,
            pos: 0,
            result: Vec::new(),
        }
    }

    fn current_line(&self) -> &'_ str {
        &self.file[self.pos]
    }

    fn emit(&mut self, s: &str) {
        self.result.push(s.to_owned())
    }

    fn unchanged_line(&mut self) {
        self.result.push(self.current_line().to_owned())
    }

    fn parse_file(&mut self) {
        while self.pos < self.file.len() {
            if self.current_line().trim_start().starts_with("\\begin{picture}") {
                println!("{}", self.pos);
                self.parse_picture();
            } else {
                self.unchanged_line();
            }
            self.pos += 1;
        }
    }

    fn parse_picture(&mut self) {
        let l = self.current_line().trim_start();

        static BEGIN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\begin\{picture\}\((\d+),(\d+)\)").unwrap());
        static BEZIER_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"(?x)\\bezier
            \{ (\d+) \}
            \( (\d+), ( \d+ ) \)
            \( (\d+), ( \d+ ) \)
            \( (\d+), ( \d+ ) \)",
            )
            .unwrap()
        });
        static PUT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\put\((\d+),(\d+)\)\{(.*)\}").unwrap());

        let m = BEGIN_REGEX.captures_iter(l).next().unwrap();
        let w: usize = m.get(1).unwrap().as_str().parse().unwrap();
        let h: usize = m.get(2).unwrap().as_str().parse().unwrap();
        self.pos += 1;

        let mut items = Vec::new();

        while self.pos < self.file.len() {
            let l = self.current_line().trim_start().to_owned();

            if l.starts_with("\\end{picture}") {
                break;
            }

            if l.starts_with("\\bezier") {
                let m = BEZIER_REGEX.captures(&l).expect(&l);
                let item = Item::Line {
                    t: parse_f64(&m, 1),
                    ps: [
                        (parse_f64(&m, 2), parse_f64(&m, 3)),
                        (parse_f64(&m, 4), parse_f64(&m, 5)),
                        (parse_f64(&m, 6), parse_f64(&m, 7)),
                    ],
                };

                items.push(item);
            } else if l.starts_with("\\put") {
                let m = PUT_REGEX.captures(&l).unwrap();
                let item = Item::Put {
                    x: parse_f64(&m, 1),
                    y: parse_f64(&m, 2),
                    c: m.get(3).unwrap().as_str().to_owned(),
                };

                items.push(item);
            } else {
                items.push(Item::Blank(self.pos, self.current_line().to_owned()))
            }

            self.pos += 1;
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        for i in items.iter() {
            match i {
                Item::Line { t, ps } => {
                    for (x, y) in ps {
                        min_x = x.min(min_x);
                        min_y = y.min(min_y);

                        max_x = x.max(max_x);
                        max_y = y.max(max_y);
                    }
                }
                Item::Put { x, y, c } => {
                    min_x = x.min(min_x);
                    min_y = y.min(min_y);

                    max_x = x.max(max_x);
                    max_y = y.max(max_y);
                }
                Item::Blank(_, _) => (), // println!("{i:?}"),
            }
        }

        let aspect = NEW_HEIGHT / (max_y - min_y);

        let width = ((max_x - min_x) * aspect + NEW_PADDING * 2.0) as usize;
        let height = (NEW_HEIGHT + NEW_PADDING * 2.0) as usize;

        let xx = |x: f64| ((x - min_x) * aspect + NEW_PADDING) as usize;
        let yy = |y: f64| ((y - min_y) * aspect + NEW_PADDING) as usize;

        self.emit(&format!("    \\begin{{picture}}({width},{height})"));
        for i in items.iter() {
            match i {
                Item::Line { t, ps } => {
                    self.emit(&format!(
                        "        \\bezier{{{t}}}({},{})({},{})({},{})",
                        xx(ps[0].0),
                        yy(ps[0].1),
                        xx(ps[1].0),
                        yy(ps[1].1),
                        xx(ps[2].0),
                        yy(ps[2].1),
                    ));
                }
                Item::Put { x, y, c } => {
                    self.emit(&format!("        \\put({},{}){{{c}}}", xx(*x), yy(*y),));
                }
                Item::Blank(_, s) => self.emit(s),
            }
        }
        self.emit("    \\end{picture}")
    }
}

fn parse_f64(m: &Captures, i: usize) -> f64 {
    m.get(i).unwrap().as_str().parse().unwrap()
}

#[derive(Debug)]
enum Item {
    Line { t: f64, ps: [(f64, f64); 3] },
    Put { x: f64, y: f64, c: String },
    Blank(usize, String),
}

fn main() {
    let mut parser = FileParser::new(read_lines(FNAME));
    parser.parse_file();
    let res = parser.result;

    let mut s = String::new(); 
    for line in res {
        s.write_str(&line).unwrap();
        s.write_char('\n').unwrap();
    }

    std::fs::write(FNAME, s).unwrap();
}
