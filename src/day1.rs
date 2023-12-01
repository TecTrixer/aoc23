use lib::*;

fn main() {
    let mut io = Io::new();
    let mut lines = vec![];
    for _ in 0..1000 {
        lines.push(io.chars());
    }
    let mut res = 0;
    for line in lines {
        let mut first = 123123132;
        let mut last = 0;
        for i in 0..line.len() {
            if line[i].is_digit(10) {
                if first == 123123132 {
                    first = (line[i] as B - b'0') as U;
                }
                last = (line[i] as B - b'0') as U;
            }
        }
        res += first * 10 + last;
    }
    w!(io, res);
}

mod lib {
    #![allow(dead_code)]
    use std::{
        fmt::Display,
        io::{
            stdin, stdout, BufRead, BufReader, BufWriter, Error, ErrorKind, Read, Stdin, Stdout,
            Write,
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

    pub struct Io {
        input: BufReader<Stdin>,
        output: BufWriter<Stdout>,
    }

    impl Io {
        pub fn new() -> Io {
            let input = BufReader::new(stdin());
            let output = BufWriter::new(stdout());
            Io { input, output }
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
        pub fn chars(&mut self) -> Vec<char> {
            self.r::<String>().chars().collect()
        }
        pub fn nums(self) -> Vec<I> {
            self.input
                .lines()
                .map(|x| x.unwrap().parse::<I>().unwrap())
                .collect()
        }

        pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.r::<T>()).collect()
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
