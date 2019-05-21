#[allow(dead_code, unused_imports)]
use super::*;

// ========== PUBLIC FUNCTIONS ============

/// Return the full week's schedule for Labyrinth.
pub fn get_schedule() -> Schedule {
    let mut sched = Schedule::new();
    sched.set_hours(Day::Saturday, 9, 21);
    sched.set_hours(Day::Sunday, 10, 18);
    sched.set_hours(Day::Tuesday, 10, 22);
    sched.set_hours(Day::Wednesday, 10, 21);
    sched.set_hours(Day::Thursday, 10, 22);
    sched.set_hours(Day::Friday, 10, 22);

    sched
}

// ========== UTILITY FUNCTIONS ============

fn index_to_time(index: usize) -> String {
    format!("{}:{:0>2}", index/4, (index%4)*15)
}

fn index_to_day(index: usize) -> Option<Day> {
    match index {
        0 => Some(Day::Saturday),
        1 => Some(Day::Sunday),
        2 => Some(Day::Monday),
        3 => Some(Day::Tuesday),
        4 => Some(Day::Wednesday),
        5 => Some(Day::Thursday),
        6 => Some(Day::Friday),
        _ => None
    }
}

// ========== DATA TYPES ============

enum EventType {
    Pkmn,
    Magic,
    Class,
    AdultParty,
    KidsMagic,
}

/// The days of the week. They implement conversion to index and string to enable internal functionality.
#[derive(Debug)]
pub enum Day {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday
}

struct Event {
    required_emp_ids: Vec<String>,
    start: Time,
    end: Time,
    kind: EventType,
}

struct Shift {
    emp_id: String,
    start: Time,
    end: Time,
}

/// A time of day, with internal string and QuarterIndex (`qi`) representation.
#[derive(Debug)]
pub struct Time {
    string: String,
    qi: usize,
}

/// A week's schedule.
///
/// `events` holds a Vec<> of all events and classes within the week.
/// `raw_reqs` holds an array of arrays of quarter-hourly staffing requirements for each weekday.
/// `shifts` is an array of Vec<>s of currently-scheduled shifts.
pub struct Schedule {
    events: Vec<Event>,
    raw_reqs: [[i32; 24 * 4]; 7],
    shifts: [Vec<Shift>; 7]
}

// ========== METHODS & ASSOCIATED FUNCTIONS ============

impl Day {
    fn to_string(&self) -> String {
        let day_name = match self {
            Day::Saturday => "Saturday",
            Day::Sunday => "Sunday",
            Day::Monday => "Monday",
            Day::Tuesday => "Tuesday",
            Day::Wednesday => "Wednesday",
            Day::Thursday => "Thursday",
            Day::Friday => "Friday",
        };
        day_name.to_string()
    }
    fn to_index(&self) -> usize {
        let index = match self {
            Day::Saturday => 0,
            Day::Sunday => 1,
            Day::Monday => 2,
            Day::Tuesday => 3,
            Day::Wednesday => 4,
            Day::Thursday => 5,
            Day::Friday => 6,
        };
        index
    }
}

impl Shift {
    fn to_string(&self) -> String {
        format!("{} => {} - {}", self.emp_id, self.start.string, self.end.string)
    }
}

impl Time {
    // Constructors
    pub fn from_str(st: &str) -> Time {
        //! Construct a Time from a &str of the format `"HH:MM"` or `"H:MM"`, using 24-hour notation.
        //! 
        //! # Examples
        //! ```
        //! use sched_lib::time::Time as Time;
        //! let t = Time::from_str("10:30");
        //! println!("{}", t.get_qi()); // 42
        //! 
        //! let u = Time::from_str("22:45");
        //! println!("{}", t.get_qi()); // 91
        //! ```
        let qi = Time::string_to_qi(st);
        let string = Time::qi_to_string(qi);
        if qi >= 4*24 {
            panic!("Bad time!")
        }
        Time { string, qi }
    }
    pub fn from_qi(qi: usize) -> Time {
        //! Construct a Time from a QuarterIndex (the 0-indexed position of its 15-minute chunk in the day).
        //! 
        //! # Examples
        //! ```
        //! use sched_lib::time::Time as Time;
        //! let t = Time::from_qi(0);
        //! println!("{}", t.to_string_24h()); // 0:00
        //! 
        //! let u = Time::from_qi(49);
        //! println!("{}", t.to_string_24h()); // 12:15
        //! ```
        if qi >= 4*24 {
            panic!("Bad time!")
        }
        Time {
            string: Time::qi_to_string(qi),
            qi
        }
    }
    pub fn from_hour(hour: usize) -> Time {
        //! Construct a Time from a simple hour number out of 24. Implemented as from_qi(hour * 4).
        //! 
        //! # Examples
        //! ```
        //! use sched_lib::time::Time as Time;
        //! let t = Time::from_hour(14);
        //! println!("{}", t.get_qi()); // 52
        //! println!("{}", t.to_string_24h()); // 14:00
        //! ```
        Time::from_qi(hour * 4)
    }
    // Conversion Utilities
    fn string_to_qi(s: &str) -> usize {
        let v: Vec<&str> = s.split(":").collect();
        let hours: usize = v[0].parse().unwrap();
        let minutes: usize = v[0].parse().unwrap();
        ((hours*60) + minutes) / 15
    }
    fn qi_to_string(qi: usize) -> String {
        let hours = qi / 4;
        let minutes = qi % 4;
        format!("{}:{:0>2}", hours, minutes*15)
    }
    // Access
    pub fn to_string_24h(&self) -> String {
        //! Return a 24-hour (military) string of this time.
        //! 
        //! # Examples
        //! ```
        //! use sched_lib::time::Time as Time;
        //! println!("{}", Time::from_hour(13).to_string_24h()); // "13:00"
        //! println!("{}", Time::from_hour(2).to_string_24h()); // "2:00"
        //! ```
        self.string.clone()
    }
    pub fn to_string_12h(&self) -> String {
        //! Return a 12-hour (US) string of this time.
        //! 
        //! # Examples
        //! ```
        //! use sched_lib::time::Time as Time;
        //! println!("{}", Time::from_hour(23).to_string_12h()); // "11:00p"
        //! println!("{}", Time::from_hour(9).to_string_12h()); // "9:00a"
        //! ```
        let qi = self.qi;
        if qi < 4 {
            // 12:MMa
            format!("12:{:0>2}a", qi*15)
        } else if qi >= 4 && qi < 12*4 {
            // 1:MMa -> 11:MMa
            format!("{}:{:0>2}a", qi/4, (qi%4)*15)
        } else if qi >= 12*4 && qi < 13*4 {
            // 12:MMp
            format!("12:{:0>2}p", (qi%4)*15)
        } else {
            // 1:MMp -> 11:MMp
            format!("{}:{:0>2}", (qi/4)-12, (qi%4)*15)
        }
    }
    pub fn get_qi(&self) -> usize {
        //! Access a time's QuarterIndex (see from_qi for examples of qi).
        self.qi
    }
}
impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string && self.qi == other.qi
    }
}

impl Schedule {
    fn new() -> Schedule {
        Schedule {
            events: Vec::new(),
            raw_reqs: [[0; 24*4]; 7],
            shifts: [Vec::new(), Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()],
        }
    }
    /// Print the quarter-hourly staffing requirements for all time during which the store is open.
    pub fn print_reqs(&self) -> () {
        for (i, day) in self.raw_reqs.iter().enumerate() {
            let day_name = match index_to_day(i) {
                Some(d) => d.to_string(),
                None => panic!("Bad day above!")
            };
            println!("\n{}", day_name);
            for (j, quarter_req) in day.iter().enumerate() {
                if *quarter_req > 0 {
                    println!("{} - {}", index_to_time(j), quarter_req);
                }
            }
        }
    }
    /// Print all currently-assigned shifts for the week.
    pub fn print_shifts(&self) -> () {
        for (i, day) in self.shifts.iter().enumerate() {
            let day_name = index_to_day(i).unwrap().to_string();
            println!("\n{}\n=========", day_name);
            for shift in day.iter() {
                println!("{}", shift.to_string());
            }
        }
    }
    pub fn assign_shift(&mut self, emp_id: String,  day: Day, start: Time, end: Time) {
        let sh = Shift { emp_id, start, end };
        self.shifts[day.to_index()].push(sh);
    }
    fn set_hours(&mut self, day: Day, start: usize, end: usize) -> () {
        let start = Time::from_hour(start).qi;
        let end = Time::from_hour(end).qi;
        for qi in start-3..start {
            self.raw_reqs[day.to_index()][qi] = 3;
        }
        for qi in start..=end {
            self.raw_reqs[day.to_index()][qi] = 4;
        }
        for qi in end+1..=end+3 {
            self.raw_reqs[day.to_index()][qi] = 3;
        }
    }
}
