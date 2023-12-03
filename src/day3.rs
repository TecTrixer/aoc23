use lib::*;

fn main() {
    let mut io = Io::new();
    let mut res = 0;
    let mut res2 = 0;
    let mut grid = vec![];
    for mut lio in io.line_io() {
        let line: Vec<_> = lio
            .chars()
            .into_iter()
            .map(|c| match c {
                c if c.is_digit(10) => Loc::Digit((c as B - b'0') as I),
                '.' => Loc::Nothing,
                '*' => Loc::Gear,
                _ => Loc::Part,
            })
            .collect();
        grid.push(line);
    }
    let width = grid[0].len();
    let height = grid.len();
    let mut adj = HashMap::<(U, U), Vec<U>>::new();
    for line in 0..height {
        let mut num = 0;
        let mut valid = false;
        let mut gears = HashSet::new();
        for i in 0..width {
            if let Loc::Digit(d) = grid[line][i] {
                num = num * 10 + d;
                for (x, y) in DIAG {
                    let xi = i as I + x;
                    let yi = line as I + y;
                    if 0 <= xi && xi < width as I && 0 <= yi && yi < height as I {
                        if let Loc::Part = grid[yi as U][xi as U] {
                            valid = true;
                        }
                        if let Loc::Gear = grid[yi as U][xi as U] {
                            gears.insert((yi as U, xi as U));
                            valid = true;
                        }
                    }
                }
            } else {
                for gear in gears {
                    adj.entry(gear)
                        .and_modify(|x| x.push(num as U))
                        .or_insert(vec![num as U]);
                }
                gears = HashSet::new();
                if valid {
                    res += num;
                }
                num = 0;
                valid = false;
            }
        }
        for gear in gears {
            adj.entry(gear)
                .and_modify(|x| x.push(num as U))
                .or_insert(vec![num as U]);
        }
        if valid {
            res += num;
        }
    }

    for nums in adj.into_values() {
        if nums.len() == 2 {
            res2 += nums[0] * nums[1];
        }
    }

    wf!(io, "Part 1: {res}");
    wf!(io, "Part 2: {res2}");
}

const DIAG: [(I, I); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

enum Loc {
    Digit(I),
    Gear,
    Part,
    Nothing,
}

mod lib {
    #![allow(dead_code)]
    use regex::Regex;
    use std::{
        fmt::Display,
        io::{
            stdin, stdout, BufRead, BufReader, BufWriter, Cursor, Error, ErrorKind, Read, Stdin,
            Stdout, Write,
        },
        str::{from_utf8_unchecked, FromStr},
    };

    pub use std::collections::{HashMap, HashSet};

    pub type U = usize;
    pub type I = isize;
    pub type F = f64;
    pub type B = u8;

    fn is_skip_char(&b: &u8) -> bool {
        b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' || b == b','
    }

    #[derive(Debug)]
    pub struct Io<R, W>
    where
        R: Read,
        W: Write,
    {
        input: BufReader<R>,
        output: BufWriter<W>,
    }

    impl Io<&[u8], Stdout> {
        #[allow(clippy::should_implement_trait)]
        /// This function creates an io handler from a &str which can be used to make parsing easier.
        pub fn from_str(input: &str) -> Io<&[u8], Stdout> {
            Io {
                input: BufReader::new(input.as_bytes()),
                output: BufWriter::new(stdout()),
            }
        }
        /// This function creates an io handler from a String which can be used to parse lines easier.
        pub fn from_string(input: String) -> Io<Cursor<String>, Stdout> {
            Io {
                input: BufReader::new(Cursor::new(input)),
                output: BufWriter::new(stdout()),
            }
        }
    }

    impl Io<Stdin, Stdout> {
        /// This functions creates the default I/O handler using stdin and stdout as reader and writer.
        pub fn new() -> Io<Stdin, Stdout> {
            Io {
                input: BufReader::new(stdin()),
                output: BufWriter::new(stdout()),
            }
        }
    }

    impl<R: std::io::Read, W: std::io::Write> Io<R, W> {
        pub fn with_reader_and_writer(reader: R, writer: W) -> Io<R, W> {
            Io {
                input: BufReader::new(reader),
                output: BufWriter::new(writer),
            }
        }
        pub fn r<T: FromStr>(&mut self) -> T {
            let buf = self
                .input
                .by_ref()
                .bytes()
                .map(|x| unsafe { x.unwrap_unchecked() })
                .skip_while(is_skip_char)
                .take_while(|c| !is_skip_char(c))
                .collect::<Vec<_>>();
            unsafe { from_utf8_unchecked(&buf) }
                .parse()
                .map_err(|_| Error::new(ErrorKind::Other, "could not parse value"))
                .unwrap()
        }
        pub fn read_line(&mut self) -> String {
            let mut res = String::new();
            unsafe {
                self.input.read_line(&mut res).unwrap_unchecked();
            }
            res.trim_end().to_string()
        }
        pub fn read_all(&mut self) -> String {
            let mut res = String::new();
            unsafe { self.input.read_to_string(&mut res).unwrap_unchecked() };
            res
        }
        pub fn read_char(&mut self) -> char {
            self.input
                .by_ref()
                .bytes()
                .map(|b| b.expect("could not read bytes in io read operation"))
                .find(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t' && b != b',')
                .unwrap() as char
        }
        pub fn chars(&mut self) -> Vec<char> {
            self.r::<String>().chars().collect()
        }
        pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.r::<T>()).collect()
        }
        pub fn line_io(&mut self) -> impl std::iter::Iterator<Item = Io<Cursor<String>, Stdout>> {
            let file = self.read_all();
            file.lines()
                .map(move |line| Io::from_string(line.to_string()))
                .collect::<Vec<Io<Cursor<String>, Stdout>>>()
                .into_iter()
        }
        pub fn blocks(&mut self) -> Vec<Io<Cursor<String>, Stdout>> {
            let file = self.read_all();
            file.split("\n\n")
                .map(move |line| Io::from_string(line.to_string()))
                .collect::<Vec<Io<Cursor<String>, Stdout>>>()
        }
        pub fn split(&mut self, pattern: &str) -> Vec<Io<Cursor<String>, Stdout>> {
            let file = self.read_all();
            file.split(pattern)
                .map(move |line| Io::from_string(line.to_string()))
                .collect::<Vec<Io<Cursor<String>, Stdout>>>()
        }
        pub fn nums<T: std::str::FromStr<Err = impl std::fmt::Debug>>(&mut self) -> Vec<T> {
            let file = self.read_all();
            let re = Regex::new(r"(-?\d+)").unwrap();
            re.captures_iter(&file)
                .map(|x| x.get(1).unwrap().as_str().parse::<T>().unwrap())
                .collect::<Vec<T>>()
        }
        pub fn pnums<T: std::str::FromStr<Err = impl std::fmt::Debug>>(&mut self) -> Vec<T> {
            let file = self.read_all();
            let re = Regex::new(r"(\d+)").unwrap();
            re.captures_iter(&file)
                .map(|x| x.get(1).unwrap().as_str().parse::<T>().unwrap())
                .collect::<Vec<T>>()
        }
        pub fn regex<T: std::str::FromStr<Err = impl std::fmt::Debug>>(
            &mut self,
            re: &str,
        ) -> Vec<T> {
            let file = self.read_all();
            let re = Regex::new(re).unwrap();
            re.captures_iter(&file)
                .map(|x| x.get(1).unwrap().as_str().parse::<T>().unwrap())
                .collect::<Vec<T>>()
        }
        pub fn w<T: Display>(&mut self, t: T) {
            unsafe { write!(&mut self.output, "{t}").unwrap_unchecked() };
        }
        pub fn wl<T: Display>(&mut self, t: T) {
            self.w(t);
            self.nl();
            self.flush();
        }
        pub fn nl(&mut self) {
            self.w('\n');
        }
        pub fn flush(&mut self) {
            unsafe { self.output.flush().unwrap_unchecked() }
        }
    }

    #[macro_export]
    macro_rules! wf {
        ($io:expr, $($arg:tt)*) => {
            $io.w(format!($($arg)*));
            $io.nl();
        };
    }

    #[macro_export]
    macro_rules! w {
        ($io:expr, $v:expr) => {
            $io.w($v);$io.nl()
        };
        ($io:expr, $($v:expr);*, $l:expr) => {
            $(
                $io.w($v);
                $io.w(' ');
            )*
            $io.w($l);
            $io.nl()
        };
        ($io:expr, $($v:expr),*) => {
            $(
                $io.w($v);
                $io.w(' ');
            )*
            $io.nl()
        }
    }

    #[macro_export]
    macro_rules! r {
        ($io:expr, $T:ty) => {
            $io.r::<$T>()
        };
        ($io:expr, $($T:ty),*) => {
            ($(
                $io.r::<$T>()
            ),*)
        }
    }
}
