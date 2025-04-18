use core::panic;
use std::io::{Read, Write};

static DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

static GIFTS: [&str; 12] = [
    " partridge in a pear tree",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,",
];

static LEFT: &str = "On the ";
static RIGHT: &str = " day of Christmas,";
static SECOND: &str = "my true love gave to me";
static V1: &str = "A";
static V2: &str = "And a";

#[derive(Debug, Clone, Default, Copy)]
pub struct SongIter {
    day: usize,
    line: usize,
}

impl Iterator for SongIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.day == 12 {
            return None;
        }
        let last = self.day + 2;
        let result = match self.line {
            0 => {
                format!("{}{}{}\n", LEFT, DAYS[self.day], RIGHT)
            }
            1 => {
                format!("{}\n", SECOND)
            }
            l if l < last => {
                format!("{}\n", GIFTS[last - l])
            }
            l if l == last => match self.day {
                0 => {
                    format!("{}{}{}\n", V1, GIFTS[0], ".")
                }
                d if d < 12 => {
                    format!("{}{}{}\n", V2, GIFTS[0], ".")
                }
                12 => {
                    format!("{}{}{}\n", V2, GIFTS[0], "!")
                }
                _ => {
                    panic!("This should never happen");
                }
            },
            l if l == last + 1 => '\n'.to_string(),
            _ => {
                panic!("This should never happen");
            }
        };
        self.line += 1;
        if self.line > last + 1 {
            self.day += 1;
            self.line = 0;
        }
        Some(result)
    }
}

pub fn numbered_lines() -> impl Iterator<Item = String> {
    let song_iter = SongIter::default();
    song_iter
        .enumerate()
        .map(|(i, line)| format!("{}: {}", i + 1, line))
}

// generic iterator wrapper to duplicate values N times
pub struct Repeat<T: Iterator<Item: Clone>, const N: usize> {
    iter: T,
    i: usize,
    current: Option<T::Item>,
}

impl<T: Iterator<Item: Clone>, const N: usize> Repeat<T, N> {
    pub fn new(iter: T) -> Self {
        Self {
            iter,
            i: 0,
            current: None,
        }
    }
}

impl<T: Iterator<Item: Clone>, const N: usize> Iterator for Repeat<T, N> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.current = self.iter.next();
        }
        if self.current.is_some() {
            self.i += 1;
            if self.i == N {
                self.i = 0;
            }
        }
        self.current.clone()
    }
}

pub fn song_to_string(song_iter: impl Iterator<Item = String>) -> String {
    let mut result = String::new();
    for line in song_iter {
        result.push_str(&line);
    }
    result
}

pub fn song_to_file(song_iter: impl Iterator<Item = String>, path: &str) {
    let mut file = std::fs::File::create(path).expect("Unable to create file");
    for line in song_iter {
        std::io::Write::write_all(&mut file, line.as_bytes()).expect("Unable to write data");
    }
}

pub fn song_to_tcp(song_iter: impl Iterator<Item = String>, address: &str) {
    let mut stream = std::net::TcpStream::connect(address).expect("Unable to connect to server");
    for line in song_iter {
        stream
            .write_all(line.as_bytes())
            .expect("Unable to write data");
    }
}

pub fn song_from_tcp(port: u16) {
    let address = format!("0.0.0.0:{}", port);
    let listener = std::net::TcpListener::bind(&address).expect("Unable to bind to address");
    println!("Listening on {}", address);
    // lock stdout
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                loop {
                    let bytes_read = stream.read(&mut buffer).expect("Unable to read data");
                    if bytes_read == 0 {
                        break;
                    }
                    handle
                        .write_all(&buffer[..bytes_read])
                        .expect("Unable to write data");
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
