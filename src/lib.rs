/// -------------------------------------------------------
/// logger::lib.rs - log strings to file, console         
/// Jim Fawcett, https://JimFawcett.github,io, 3/16/2020  
/// -------------------------------------------------------
extern crate chrono;
#[allow(unused_imports)]
use chrono::{DateTime, Local};

#[allow(unused_imports)]
use display::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::*;

#[derive(Debug)]
pub struct Logger {
    fl: Option<File>,
    console: bool,
}
#[allow(dead_code)]
impl Logger {
    /// ```
    /// let mut logr = Logger::new();
    ///
    /// sets fl:None, console:true;
    /// ```
    pub fn new() -> Self {
        Self {
            fl: None,
            console: true,
        }
    }
    /// ```
    /// let mut logr = Logger::init(file, true);
    ///
    /// sets fl:Some(file), console:true;
    /// ```
    pub fn init(f: File, con: bool) -> Self {
        Self {
            fl: Some(f),
            console: con,
        }
    }
    /// ```
    /// logr.console(pred);
    ///
    /// sets console:pred;
    /// ```
    pub fn console(&mut self, con: bool) {
        self.console = con
    }
    /// ```
    /// logr.file(file);
    ///
    /// sets | resets fl:Some(file);
    /// ```
    pub fn file(&mut self, f: File) {
        self.fl = Some(f);
    }
    /// ```
    /// logr.opt(opt);
    ///
    /// sets | resets fl:opt;
    /// ```
    pub fn opt(&mut self, f: Option<File>) {
        self.fl = f;
    }
    /// ```
    /// logr.open(file_name);
    ///
    /// attempts to set fl:Some(file)
    /// ```
    pub fn open(&mut self, s: &str) -> bool {
        use std::fs::OpenOptions;
        self.fl = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(s)
            .ok();
        if self.fl.is_some() {
            return true;
        }
        false
    }
    /// ```
    /// logr.open_append(file_name);
    ///
    /// attempts to set fl:Some(file)
    /// ```
    pub fn open_append(&mut self, s: &str) -> bool {
        use std::fs::OpenOptions;
        self.fl = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(s)
            .ok();
        if self.fl.is_some() {
            return true;
        }
        false
    }
    /// ```
    /// logr.ts_write(some_str);
    ///
    /// writes local time stamp and some_str to log
    /// ```
    pub fn ts_write(&mut self, s: &str) -> &mut Self {
        let now: DateTime<Local> = Local::now();
        /* format DateTime string */
        let mut now_str = format!("\n  {}", now.to_rfc2822());
        /* remove trailing -0400 */
        now_str.truncate(now_str.len() - 6);

        let _ = Logger::write(self, &now_str);
        let rslt = Logger::write(self, s);
        rslt
    }
    /// ```
    /// logr.write(some_str);
    ///
    /// writes some_str to log
    /// ```
    pub fn write(&mut self, s: &str) -> &mut Self {
        if self.console {
            print!("{}", s);
        }
        if let Some(ref mut f) = self.fl {
            let rslt = f.write(s.as_bytes());
            match rslt {
                Ok(_) => {}
                Err(_) => print!("\n  file write failed\n"),
            }
        }
        // else {
        //     print!("\n  no attached file");
        // }
        self
    }
    /// ```
    /// logr.close();
    ///
    /// sets fl:None
    /// ```
    pub fn close(&mut self) {
        self.fl = None;
    }
}
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum OpenMode {
    Truncate,
    Append,
}
#[allow(dead_code)]

/// ```
/// let f:Option<File> = open_file(some_string, Append);
///
/// attempts to open file with specified OpenMode: Truncate | Append
/// ```
pub fn open_file(s: &str, mode: OpenMode) -> Option<File> {
    let fl: Option<File>;
    use std::fs::OpenOptions;
    if mode == OpenMode::Truncate {
        fl = OpenOptions::new().write(true).truncate(true).open(s).ok();
    } else {
        fl = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(s)
            .ok();
    }
    if let None = fl {
        print!("\n\n  can't open {:?}\n", s);
    }
    fl
}
/// ```
/// if remove_file(file_name) { ... }
/// ```
pub fn remove_file(s: &str) -> bool {
    let rslt = std::fs::remove_file(s);
    rslt.is_ok()
}
/// ```
/// if file_contains(file_name, test_str) { ... }
/// ```
pub fn file_contains(fl: &str, ts: &str) -> bool {
    let contents = std::fs::read_to_string(fl);
    let mut s = "".to_string();
    if contents.is_ok() {
        s = contents.unwrap();
    }
    s.contains(ts)
}
/// ```
/// file_contents(file_name, test_str) { ... }
///
/// display contents of file
/// ```
pub fn file_contents(fl: &str) {
    let contents = std::fs::read_to_string(fl);
    if contents.is_ok() {
        let s = contents.unwrap();
        print!("{}", s);
    } else {
        print!("\n  no contents");
    }
}
/// ```
/// if file_exists(file_name) { ... }
/// ```
pub fn file_exists(s: &str) -> bool {
    let path = Path::new(s);
    return path.exists();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_open_file() {
        let stest = "test_open.txt";
        let mut l = Logger::new();
        l.open(stest);
        open_file(stest, OpenMode::Truncate);
        assert_eq!(file_exists(stest), true);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
    #[test]
    fn test_file_contains() {
        let stest = "test_contains.txt";
        let mut l = Logger::new();
        l.open(stest);
        assert_eq!(l.fl.is_some(), true);
        l.write("test contents with a short string");
        l.close();
        assert_eq!(file_exists(stest), true);
        assert_eq!(file_contains(stest, "a short"), true);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
    #[test]
    fn test_remove_file() {
        let stest = "test_remove";
        open_file(stest, OpenMode::Truncate);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
    #[test]
    fn test_file_exists() {
        assert_eq!(file_exists("foobar.fee"), false);
    }
    #[test]
    fn test_new() {
        let stest = "test_new";
        let mut l = Logger::new();
        l.open(stest);
        assert_eq!(file_exists(stest), true);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
    #[test]
    fn test_init() {
        let stest = "test_new";
        let opt = open_file(stest, OpenMode::Append);
        let mut l = Logger::init(opt.unwrap(), false);
        l.open(stest);
        assert_eq!(file_exists(stest), true);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
    #[test]
    fn test_console() {
        let mut l = Logger::new();
        assert_eq!(l.console == true, true);
        l.console(false);
        assert_eq!(l.console == false, true);
    }
    #[test]
    fn test_file() {
        let mut l = Logger::new();
        let stest = "test_file";
        l.file(open_file(stest, OpenMode::Append).unwrap());
        assert_eq!(file_exists(stest), true);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
    #[test]
    fn test_opt() {
        let mut l = Logger::new();
        let stest = "test_file";
        let file_opt = open_file(stest, OpenMode::Append);
        l.opt(file_opt);
        assert_eq!(file_exists(stest), true);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
    #[test]
    fn test_open() {
        let mut l = Logger::new();
        let stest = "test_open";
        l.open(stest);
        assert_eq!(file_exists(stest), true);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
    #[test]
    fn test_write() {
        let mut l = Logger::new();
        let stest = "test_write";
        l.open(stest);
        let stxt = "abc 012 xyz 789";
        let _ = l.write(stxt);
        assert_eq!(file_contains(stest, stxt), true);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
    #[test]
    fn test_ts_write() {
        let mut l = Logger::new();
        let stest = "test_ts_write";
        l.open(stest);
        let sdt = "2020"; // change if year != 2020
        let stxt = "abc 012 xyz 789";
        let _ = l.ts_write(stxt);
        assert_eq!(file_contains(stest, sdt), true);
        assert_eq!(file_contains(stest, stxt), true);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
    #[test]
    fn test_close() {
        let mut l = Logger::new();
        let stest = "test_close.txt";
        l.open(stest);
        assert_eq!(l.fl.is_some(), true);
        l.close();
        assert_eq!(l.fl.is_none(), true);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
}
