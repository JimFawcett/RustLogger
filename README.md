# RustLogger

https://JimFawcett.github.io/RustLogger.html

Simple logger that writes text to console, file, or both.


Concept:
RustLogger is a facility for inserting time-date stamped string messages into the console and/or a text file concurrently.
Design:
There is one struct, Logger, with methods and several functions in this design:
Methods:
    1. new() -> Self
        Create new Logger which has no attached file and writes to console.
    2. init(f:File, con:bool) -> Self
        Create new Logger attached to f and writes to console only if con is true.
    3. console(&mut self, con:bool)
        sets console writing to true or false.
    file(&mut self, f:File)
    Sets or resets log file f.
    opt(&mut self, f:Option<File>
    sets or resets Logger::fl to the option provided.
    open(&mut self, s:&str) -> bool
    Opens logger, truncating log file if it exists.
    open_append(&mut self, s:&str) -> bool
    Opens logger, appending to log file if it exists.
    ts_write(&mut self, s:&str) -> &mut Self
    Writes date_time stamp then string s to the log target(s).
    write(&mut self, s:&str) -> &mut Self
    Writes string s to the log target(s).
    close(&mut self)
    Closes logger by setting Logger::fl to the option None.

Functions:

    open_file(s:&str, mode:OpenMode) -> Option<File>
    Opens file with OpenMode::Append or OpenMode::Truncate. Returns option that may be used with Logger::opt(f:Option<File>).
    file_exists(s:&str) -> bool
    Does this file exist?
    remove_file(s:&str) -> bool
    Delete file if it exists and has appropriate access.
    file_contains(fl:&str, ts:&str) -> bool
    Does the file named fl contain the string ts?
    file_contents(fl:&str)
    Display text file contents on console.

Operation:
This is intended to be a very simple logger - easy to use and with virtually no setup or configuration.
Build:
Download and, in a command prompt, cargo build or cargo run.
Status:
There may be some changes after I start building bigger Rust applications.
