//! The time module contains generic scheduling and shift information.

use super::emp;
use rand;
use std::fmt;

// ==============================================

/// A day of the week.
#[derive(Clone, Debug, PartialEq)]
pub enum Day {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl Day {
    pub fn from_index(index: usize) -> Option<Day> {
        match index {
            0 => Some(Day::Saturday),
            1 => Some(Day::Sunday),
            2 => Some(Day::Monday),
            3 => Some(Day::Tuesday),
            4 => Some(Day::Wednesday),
            5 => Some(Day::Thursday),
            6 => Some(Day::Friday),
            _ => None,
        }
    }
    pub fn from_str(s: &str) -> Option<Day> {
        match s {
            "Saturday" => Some(Day::Saturday),
            "Sunday" => Some(Day::Sunday),
            "Monday" => Some(Day::Monday),
            "Tuesday" => Some(Day::Tuesday),
            "Wednesday" => Some(Day::Wednesday),
            "Thursday" => Some(Day::Thursday),
            "Friday" => Some(Day::Friday),
            _ => None,
        }
    }
    pub fn to_index(&self) -> usize {
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

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let day_name = match self {
            Day::Saturday => "Saturday",
            Day::Sunday => "Sunday",
            Day::Monday => "Monday",
            Day::Tuesday => "Tuesday",
            Day::Wednesday => "Wednesday",
            Day::Thursday => "Thursday",
            Day::Friday => "Friday",
        };
        write!(f, "{}", day_name)
    }
}

// ==============================================

/// An event or class run by the store.
#[derive(Clone, Debug)]
pub struct Event {
    pub name: String,
    pub req_emp_ids: Vec<String>,
    pub day: Day,
    pub start: Time,
    pub end: Time,
    pub num_emps: i32,
    pub kind: String,
    pub setup: Time,
    pub breakdown: Time,
}

impl Event {
    // Constructor
    pub fn new(name: String, day: Day, start: Time, end: Time, kind: String) -> Event {
        //! Create a new event with empty employee requirements.
        Event {
            req_emp_ids: Vec::new(),
            name,
            day,
            start,
            end,
            kind,
            num_emps: 1,
            setup: Time::from_qi(2),
            breakdown: Time::from_hour(2),
        }
    }
    // Modification
    pub fn add_employee(&mut self, emp_id: String) {
        //! Add an employee who is required to work this event/class.
        self.req_emp_ids.push(emp_id);
        let reqs: i32 = self.req_emp_ids.len() as i32;
        if reqs > self.num_emps {
            self.num_emps = reqs;
        }
    }
    pub fn add_employees(&mut self, emp_ids: Vec<String>) {
        //! Add a group of employees who are required to work this event/class.
        self.req_emp_ids.extend(emp_ids);
        let reqs: i32 = self.req_emp_ids.len() as i32;
        if reqs > self.num_emps {
            self.num_emps = reqs;
        }
    }
    pub fn staffing_req(&mut self, num_emps: i32) {
        self.num_emps = num_emps;
    }
    pub fn setup_breakdown(&mut self, s: usize, b: usize) {
        self.setup = Time::from_qi(s);
        self.breakdown = Time::from_qi(b);
    }
    // Access
    pub fn req_ids(&self) -> &Vec<String> {
        //! Return IDs of all employees required to work this event.
        &self.req_emp_ids
    }
    pub fn has_reqs(&self) -> bool {
        //! Check if this event has employee requirements.
        !self.req_emp_ids.is_empty()
    }
    pub fn print(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{: >9} {: <6} - {: <6} | {} ({:?})",
            self.day.to_string(),
            self.start.to_string(),
            self.end.to_string(),
            self.name,
            self.kind
        )
    }
}

// ==============================================

/// An employee's shift at the store.
#[derive(Debug)]
struct Shift {
    pub emp_id: String,
    pub start: Time,
    pub end: Time,
}

#[allow(dead_code)]
impl Shift {
    // todo error checking
    pub fn extend(&mut self, forward: bool) {
        if forward {
            self.end = Time::from_qi(self.end.get_qi() + 1);
        } else {
            self.start = Time::from_qi(self.start.get_qi() - 1);
        }
    }
    // todo error checking
    pub fn extend_by(&mut self, forward: bool, amnt: Time) {
        if forward {
            self.end = Time::from_qi(self.end.get_qi() + amnt.get_qi());
        } else {
            self.start = Time::from_qi(self.start.get_qi() - amnt.get_qi());
        }
    }
    pub fn len(&self) -> usize {
        self.end.get_qi() - self.start.get_qi()
    }
}

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} => {} - {}",
            self.emp_id, self.start.string, self.end.string
        )
    }
}

// ==============================================

/// A time of day.
#[derive(Debug, Clone)]
pub struct Time {
    string: String,
    qi: usize,
}

impl Time {
    // Constructors
    pub fn from_str(st: &str) -> Time {
        //! Construct a Time from a &str of the format `"HH:MM"` or `"H:MM"`, using 24-hour notation.
        //!
        //! # Examples
        //! ```
        //! use makeshift::time::Time as Time;
        //! let t = Time::from_str("10:30");
        //! assert_eq!(t.get_qi(), 42);
        //!
        //! let u = Time::from_str("22:45");
        //! assert_eq!(u.get_qi(), 91);
        //! ```
        let qi = Time::string_to_qi(st);
        let string = Time::qi_to_string(qi);
        if qi >= 4 * 24 {
            panic!("Bad time!")
        }
        Time { string, qi }
    }
    pub fn from_qi(qi: usize) -> Time {
        //! Construct a Time from a QuarterIndex (the 0-indexed position of its 15-minute chunk in the day).
        //!
        //! # Examples
        //! ```
        //! use makeshift::time::Time as Time;
        //! let t = Time::from_qi(0);
        //! assert_eq!(t.to_string_24h(), "0:00");
        //!
        //! let u = Time::from_qi(49);
        //! assert_eq!(u.to_string_24h(), "12:15"); // 12:15
        //! ```
        if qi >= 4 * 24 {
            panic!("Bad time!")
        }
        Time {
            string: Time::qi_to_string(qi),
            qi,
        }
    }
    pub fn from_hour(hour: usize) -> Time {
        //! Construct a Time from a simple hour number out of 24. Implemented as from_qi(hour * 4).
        //!
        //! # Examples
        //! ```
        //! use makeshift::time::Time as Time;
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
        //! use makeshift::time::Time as Time;
        //! println!("{}", Time::from_hour(13).to_string_24h()); // "13:00"
        //! println!("{}", Time::from_hour(2).to_string_24h()); // "2:00"
        //! ```
        self.string.clone()
    }
    pub fn to_string(&self) -> String {
        //! Return a 12-hour (US) string of this time.
        //!
        //! # Examples
        //! ```
        //! use makeshift::time::Time as Time;
        //! println!("{}", Time::from_hour(23).to_string()); // "11:00p"
        //! println!("{}", Time::from_hour(9).to_string()); // "9:00a"
        //! ```
        let qi = self.qi;
        if qi < 4 {
            // 12:MMa
            format!("12:{:0>2}a", qi * 15)
        } else if qi >= 4 && qi < 12 * 4 {
            // 1:MMa -> 11:MMa
            format!("{}:{:0>2}a", qi / 4, (qi % 4) * 15)
        } else if qi >= 12 * 4 && qi < 13 * 4 {
            // 12:MMp
            format!("12:{:0>2}p", (qi % 4) * 15)
        } else {
            // 1:MMp -> 11:MMp
            format!("{}:{:0>2}p", (qi / 4) - 12, (qi % 4) * 15)
        }
    }
    pub fn get_qi(&self) -> usize {
        //! Access a time's QuarterIndex (see from_qi for examples of qi).
        self.qi
    }
    // Conversion Utilities
    pub fn duration_string(qi: usize) -> String {
        Time::qi_to_string(qi)
    }
    fn string_to_qi(s: &str) -> usize {
        let v: Vec<&str> = s.split(":").collect();
        let hours: usize = v[0].parse().unwrap();
        let minutes: usize = v[1].parse().unwrap();
        ((hours * 60) + minutes) / 15
    }
    fn qi_to_string(qi: usize) -> String {
        let hours = qi / 4;
        let minutes = qi % 4;
        format!("{}:{:0>2}", hours, minutes * 15)
    }
}

impl fmt::Display for Time {
    //! Display a 12-hour (US) string of this time.
    //!
    //! # Examples
    //! ```
    //! use makeshift::time::Time as Time;
    //! println!("{}", Time::from_hour(23).to_string()); // "11:00p"
    //! println!("{}", Time::from_hour(9).to_string()); // "9:00a"
    //! ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let qi = self.qi;
        if qi < 4 {
            // 12:MMa
            write!(f, "12:{:0>2}a", qi * 15)
        } else if qi >= 4 && qi < 12 * 4 {
            // 1:MMa -> 11:MMa
            write!(f, "{}:{:0>2}a", qi / 4, (qi % 4) * 15)
        } else if qi >= 12 * 4 && qi < 13 * 4 {
            // 12:MMp
            write!(f, "12:{:0>2}p", (qi % 4) * 15)
        } else {
            // 1:MMp -> 11:MMp
            write!(f, "{}:{:0>2}p", (qi / 4) - 12, (qi % 4) * 15)
        }
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string && self.qi == other.qi
    }
}

// ==============================================

/// A full week's schedule, including events and shifts.
pub struct Schedule {
    pub events: Vec<Event>,
    raw_reqs: [[i32; 24 * 4]; 7],
    shifts: [Vec<Shift>; 7],
}

impl fmt::Debug for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rr = String::new();
        for day_arr in self.raw_reqs.iter() {
            rr.push_str("[");
            for (i, cover) in day_arr.iter().enumerate() {
                rr.push_str(&format!("{}", cover));
                if i != day_arr.len() - 1 {
                    rr.push_str(", ")
                }
            }
            rr.push_str("]\n");
        }
        f.debug_struct("Schedule")
            .field("events", &self.events)
            .field("shifts", &self.shifts)
            .field("raw_reqs", &rr)
            .finish()
    }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for (i, day) in self.shifts.iter().enumerate() {
            let day_name = Day::from_index(i).unwrap();
            out.push_str(&format!("\n{}\n=========", day_name));
            for shift in day.iter() {
                out.push_str(&format!("\n{}", shift));
            }
            if i != 6 {
                out.push_str("\n")
            }
        }
        write!(f, "{}", out)
    }
}

impl Schedule {
    // Constructor
    pub fn new() -> Schedule {
        //! Create a new, empty schedule.
        Schedule {
            events: Vec::new(),
            raw_reqs: [[0; 24 * 4]; 7],
            shifts: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
        }
    }
    // Display/Access
    pub fn print_reqs(&self) -> () {
        //! Print the quarter-hourly staffing requirements for all time during which the store is open.
        for (i, day) in self.raw_reqs.iter().enumerate() {
            let day_name = match Day::from_index(i) {
                Some(d) => d.to_string(),
                None => panic!("Bad day above!"),
            };
            println!("\n{}", day_name);
            for (j, quarter_req) in day.iter().enumerate() {
                if *quarter_req > 0 {
                    println!("{} - {}", Time::from_qi(j).to_string(), quarter_req);
                }
            }
        }
    }
    pub fn print_events(&self) -> () {
        //! Print all events listed on this schedule.
        for event in self.events.iter() {
            event.print();
        }
    }
    pub fn get_events(&self) -> &Vec<Event> {
        &self.events
    }
    // Modification
    pub fn add_event(
        &mut self,
        name: &str,
        kind: &str,
        day: Day,
        start: Time,
        end: Time,
    ) -> &mut Event {
        //! Add an event (class, tournament, etc.) to this schedule.
        let ev = Event::new(name.to_string(), day, start, end, kind.to_string());
        self.events.push(ev);
        let li = self.events.len() - 1;
        &mut self.events[li]
    }
    pub fn assign_shift(&mut self, emp_id: String, day: Day, start: Time, end: Time) {
        //! Assign a new shift to the employee with id emp_id.
        let sh = Shift { emp_id, start, end };
        self.shifts[day.to_index()].push(sh);
    }
    pub fn assign_event(&mut self, emp_id: String, event: Event) {
        //! Assign an event to the employee with id emp_id.
        // Move start time back by the setup amount
        let start = event.start.get_qi() - event.setup.get_qi();
        let start = Time::from_qi(start);
        // Move end time forward by the breakdown amount
        let end = event.end.get_qi() + event.breakdown.get_qi();
        let end = Time::from_qi(end);

        let sh = Shift { emp_id, start, end };
        self.shifts[event.day.to_index()].push(sh);
    }
    pub fn set_hours(&mut self, day: Day, start: usize, end: usize) -> () {
        //! Set the store's open and close hours for a given day.
        let start = Time::from_hour(start).qi;
        let end = Time::from_hour(end).qi;
        for qi in start - 1..start {
            self.raw_reqs[day.to_index()][qi] = 3;
        }
        for qi in start..=end {
            self.raw_reqs[day.to_index()][qi] = 4;
        }
        for qi in end + 1..=end + 3 {
            self.raw_reqs[day.to_index()][qi] = 3;
        }
    }
    pub fn assign_required_shifts(&mut self, _ros: &emp::Roster) {
        //! Assign all employees from the Roster to whatever events they must work in this Schedule.
        let mut assignments: Vec<(String, Event)> = Vec::new();
        for event in self.get_events() {
            if event.has_reqs() {
                for emp_id in event.req_ids() {
                    assignments.push((emp_id.clone(), event.clone()));
                }
            }
        }
        for (id, ev) in assignments {
            self.assign_event(id, ev);
        }
    }
    // Validation
    fn hours_assigned_valid(&self, id: &str, ros: &emp::Roster) -> bool {
        let mut total = 0;
        for day in self.shifts.iter() {
            for shift in day.iter() {
                if shift.emp_id == id {
                    total += shift.end.get_qi() - shift.start.get_qi();
                }
            }
        }
        let em = ros.get(id.to_string());
        let min = em.get_hours().min() * 4;
        let max = em.get_hours().max() * 4;
        if min <= total && total <= max {
            true
        } else {
            println!(">>> {} - {}", id, Time::duration_string(total));
            false
        }
    }
    fn all_shifts_okay_length(&self, _ros: &emp::Roster) -> bool {
        true
    }
    fn coverage(&self, day: Day) -> [i32; 96] {
        let mut out = [0; 96];
        for shift in self.shifts[day.to_index()].iter() {
            let s = shift.start.get_qi();
            let e = shift.end.get_qi();
            for index in s..e {
                out[index] += 1;
            }
        }
        for event in self.events.iter() {
            if day == event.day {
                let s = event.start.get_qi() - event.setup.get_qi();
                let e = event.end.get_qi() + event.breakdown.get_qi();
                for index in s..e {
                    out[index] -= event.num_emps;
                }
            }
        }
        out
    }
    fn adequate_coverage(&self) -> bool {
        let mut adequate = true;
        for i in 0..7 {
            println!("Checking coverage for {}", Day::from_index(i).unwrap());
            let len = self.raw_reqs[i].len();
            let coverage = self.coverage(Day::from_index(i).unwrap());
            for j in 0..len {
                if coverage[j] < self.raw_reqs[i][j] {
                    adequate = false;
                    println!(
                        "Low coverage at {} on {}: {}",
                        Time::from_qi(j),
                        Day::from_index(i).unwrap(),
                        coverage[j]
                    );
                }
            }
        }
        adequate
    }
    pub fn is_valid(&self, ros: &emp::Roster) -> bool {
        let mut valid = true;
        for (id, _emp) in ros.iter() {
            if !self.hours_assigned_valid(id, ros) {
                println!("{} has invalid # of hours", id);
                valid = false;
            }
        }
        if !self.all_shifts_okay_length(ros) {
            println!("Shift length issue - schedule invalid");
            valid = false;
        }
        if !self.adequate_coverage() {
            println!("Not enough coverage - schedule invalid");
            valid = false;
        }
        valid
    }
    // Generation
    pub fn expand_shifts(&mut self, emp_id: String) {
        for i in 0..7 {
            for shift in &mut self.shifts[i] {
                if shift.emp_id != emp_id {
                    continue;
                }
                let direc: bool = rand::random();
                while shift.len() < 8 * 4
                    && self.raw_reqs[i][shift.end.get_qi() + 1] != 0
                    && self.raw_reqs[i][shift.start.get_qi() - 1] != 0
                {
                    println!("Extending {}", (if direc { "forward" } else { "backward" }));
                    shift.extend(direc);
                }
                while shift.len() < 8 * 4
                    && self.raw_reqs[i][shift.end.get_qi() + 1] != 0
                    && self.raw_reqs[i][shift.start.get_qi() - 1] != 0
                {
                    println!(
                        "Extending {}",
                        (if !direc { "forward" } else { "backward" })
                    );
                    shift.extend(!direc);
                }
                println!(
                    "New shift: {} {} -> {}",
                    Day::from_index(i).unwrap(),
                    shift.start.to_string(),
                    shift.end.to_string()
                );
            }
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn midnight() {
        let t = Time::from_str("0:15");
        assert_eq!(t.get_qi(), 1);
    }
    #[test]
    fn morning() {
        let t = Time::from_str("10:15");
        assert_eq!(t.get_qi(), 41);
    }
    #[test]
    fn afternoon() {
        let t = Time::from_str("22:30");
        assert_eq!(t.get_qi(), 90);
    }
    #[test]
    #[should_panic]
    fn bad_string1() {
        Time::from_str("-270:555");
    }
    #[test]
    #[should_panic]
    fn bad_string2() {
        Time::from_str("alphabet");
    }
    #[test]
    #[should_panic]
    fn bad_string3() {
        Time::from_str("27:99");
    }
    #[test]
    #[should_panic]
    fn bad_qi() {
        Time::from_qi(100);
    }
    #[test]
    fn time_eq1() {
        let a = Time::from_str("09:00");
        let b = Time::from_str("9:0");
        let c = Time::from_hour(9);
        let d = Time::from_qi(36);
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(c, d);
        assert_eq!(d, a);
    }
    #[test]
    fn string_12h_1() {
        assert_eq!(Time::from_hour(0).to_string(), "12:00a".to_string());
        assert_eq!(Time::from_qi(3).to_string(), "12:45a".to_string());
        assert_eq!(Time::from_str("0:45").get_qi(), 3);
        assert_eq!(Time::from_str("0:45").to_string(), "12:45a".to_string());
    }
    #[test]
    fn string_12h_2() {
        assert_eq!(Time::from_hour(4).to_string(), "4:00a".to_string());
        assert_eq!(Time::from_qi(21).to_string(), "5:15a".to_string());
        assert_eq!(Time::from_str("5:45").to_string(), "5:45a".to_string());
    }
    #[test]
    fn string_12h_3() {
        assert_eq!(Time::from_hour(12).to_string(), "12:00p".to_string());
        assert_eq!(Time::from_qi(49).to_string(), "12:15p".to_string());
        assert_eq!(Time::from_str("12:45").to_string(), "12:45p".to_string());
    }
    #[test]
    fn string_12h_4() {
        assert_eq!(Time::from_hour(17).to_string(), "5:00p".to_string());
        assert_eq!(Time::from_qi(81).to_string(), "8:15p".to_string());
        assert_eq!(Time::from_str("22:45").to_string(), "10:45p".to_string());
    }
    fn ev_setup() -> Event {
        let mut ev = Event::new(
            "Name".to_string(),
            Day::Saturday,
            Time::from_hour(12),
            Time::from_hour(17),
            "Magic".to_string(),
        );
        ev.add_employee("Matt".to_string());
        ev
    }
    #[test]
    fn req_ids() {
        let ev = ev_setup();
        assert_eq!(vec!["Matt".to_string()], *ev.req_ids())
    }
    #[test]
    fn has_reqs() {
        let ev = ev_setup();
        assert!(ev.has_reqs())
    }
}
