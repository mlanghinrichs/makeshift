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

fn time_to_index(time: usize) -> usize {
    if time % 15 != 0 {
        panic!("bad time passed to time_to_index! ({})", time);
    }
    usize::from(time / 15)
}

fn index_to_time(index: usize) -> String {
    format!("{}:{:0>2}", index/4, (index%4)*15)
}

fn day_to_index(day: Day) -> usize {
    match day {
        Day::Saturday => 0,
        Day::Sunday => 1,
        Day::Monday => 2,
        Day::Tuesday => 3,
        Day::Wednesday => 4,
        Day::Thursday => 5,
        Day::Friday => 6,
    }
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

#[derive(Debug)]
enum Day {
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
    start: usize,
    end: usize,
    kind: EventType,
}

struct Shift {
    emp_id: String,
    start: usize,
    end: usize,
}

pub struct Time {
    pub string: String,
    pub qi: usize,
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
}

impl Shift {
    fn to_string(&self) -> String {
        "temp".to_string()
    }
}

impl Time {
    // Constructors
    pub fn from_str(st: &str) -> Time {
        let string = String::from(st);
        let qi = Time::string_to_qi(st);
        if qi >= 4*24 {
            panic!("Bad time!")
        }
        Time { string, qi }
    }
    pub fn from_qi(qi: usize) -> Time {
        if qi >= 4*24 {
            panic!("Bad time!")
        }
        Time {
            string: Time::qi_to_string(qi),
            qi
        }
    }
    // Conversion Utilities
    fn string_to_qi(s: &str) -> usize {
        let v: Vec<&str> = s.split(":").collect();
        let hours: usize = v[0].parse().unwrap();
        let minutes: usize = v[0].parse().unwrap();
        ((hours*60) + minutes) / 15
    }
    fn qi_to_string(s: usize) -> String {
        let hours = s / 4;
        let minutes = s % 4;
        format!("{}:{:0>2}", hours*15, minutes*15)
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
    pub fn assign_shift(&mut self, emp_id: String,  day_index: usize, start: usize, end: usize) {
        let sh = Shift { emp_id, start, end };
        self.shifts[day_index].push(sh);
    }
    fn set_hours(&mut self, day: Day, start_hour: usize, end_hour: usize) -> () {
        let day_index = day_to_index(day);
        let start = time_to_index(start_hour*60);
        let end = time_to_index(end_hour*60);
        for qi in start-3..start {
            self.raw_reqs[day_index][qi] = 3;
        }
        for qi in start..=end {
            self.raw_reqs[day_index][qi] = 4;
        }
        for qi in end+1..=end+3 {
            self.raw_reqs[day_index][qi] = 3;
        }
    }
}
