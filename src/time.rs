//! The time module contains generic scheduling and shift information.

// ========== INTERNAL UTILITY FUNCTIONS ============

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

#[derive(Clone)]
pub enum EventType {
    Pkmn,
    Magic,
    Class,
    AdultParty,
    KidsMagic,
}

#[derive(Clone, Debug)]
pub enum Day {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday
}

#[derive(Clone)]
pub struct Event {
    required_emp_ids: Vec<String>,
    day: Day,
    start: Time,
    end: Time,
    kind: EventType,
}

struct Shift {
    emp_id: String,
    start: Time,
    end: Time,
}

#[derive(Debug, Clone)]
pub struct Time {
    string: String,
    qi: usize,
}

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

impl Event {
    // Constructor
    pub fn new(day: Day, start: Time, end: Time, kind: EventType) -> Event {
        //! Creates a new event with empty employee requirements.
        Event { required_emp_ids: Vec::new(), day, start, end, kind }
    }
    // Modification
    pub fn add_employee(&mut self, emp_id: String) {
        //! Add an employee who is required to work this event/class.
        self.required_emp_ids.push(emp_id);
    }
    pub fn add_employees(&mut self, emp_ids: Vec<String>) {
        //! Add a group of employees who are required to work this event/class.
        self.required_emp_ids.extend(emp_ids);
    }
    // Access
    pub fn req_ids(&self) -> &Vec<String> {
        //! Return IDs of all employees required to work this event.
        &self.required_emp_ids
    }
    pub fn has_reqs(&self) -> bool {
        //! Check if this event has employee requirements.
        !self.required_emp_ids.is_empty()
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
        //! assert_eq!(t.get_qi(), 42);
        //! 
        //! let u = Time::from_str("22:45");
        //! assert_eq!(u.get_qi(), 91);
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
        //! assert_eq!(t.to_string_24h(), "0:00");
        //! 
        //! let u = Time::from_qi(49);
        //! assert_eq!(u.to_string_24h(), "12:15"); // 12:15
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
        //! assert_eq!(t.get_qi(), 56);
        //! assert_eq!(t.to_string_24h(), "14:00");
        //! ```
        Time::from_qi(hour * 4)
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
            format!("{}:{:0>2}p", (qi/4)-12, (qi%4)*15)
        }
    }
    pub fn get_qi(&self) -> usize {
        //! Access a time's QuarterIndex (see from_qi for examples of qi).
        self.qi
    }
    // Conversion Utilities
    fn string_to_qi(s: &str) -> usize {
        let v: Vec<&str> = s.split(":").collect();
        let hours: usize = v[0].parse().unwrap();
        let minutes: usize = v[1].parse().unwrap();
        ((hours*60) + minutes) / 15
    }
    fn qi_to_string(qi: usize) -> String {
        let hours = qi / 4;
        let minutes = qi % 4;
        format!("{}:{:0>2}", hours, minutes*15)
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string && self.qi == other.qi
    }
}

impl Schedule {
    // Constructor
    pub fn new() -> Schedule {
        Schedule {
            events: Vec::new(),
            raw_reqs: [[0; 24*4]; 7],
            shifts: [Vec::new(), Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()],
        }
    }
    // Display/Access
    pub fn print_reqs(&self) -> () {
        //! Print the quarter-hourly staffing requirements for all time during which the store is open.
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
    pub fn print(&self) -> () {
        //! Print all currently-assigned shifts for the week.
        for (i, day) in self.shifts.iter().enumerate() {
            let day_name = index_to_day(i).unwrap().to_string();
            println!("\n{}\n=========", day_name);
            for shift in day.iter() {
                println!("{}", shift.to_string());
            }
        }
    }
    pub fn get_events(&self) -> &Vec<Event> {
        &self.events
    }
    // Modification
    pub fn add_event(&mut self, kind: EventType, day: Day, start: Time, end: Time) -> &mut Event {
        let ev = Event::new(day, start, end, kind);
        self.events.push(ev);
        let li = self.events.len() - 1;
        &mut self.events[li]
    }
    pub fn assign_shift(&mut self, emp_id: String,  day: Day, start: Time, end: Time) {
        //! Assign a new shift to the employee with id emp_id.
        let sh = Shift { emp_id, start, end };
        self.shifts[day.to_index()].push(sh);
    }
    pub fn assign_event(&mut self, emp_id: String,  event: Event) {
        //! Assign an event to the employee with id emp_id.
        let sh = Shift { emp_id, start: event.start.clone(), end: event.end.clone() };
        self.shifts[event.day.to_index()].push(sh);
    }
    pub fn set_hours(&mut self, day: Day, start: usize, end: usize) -> () {
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
