//! The emp module contains tools and structures for managing employees and the full store roster thereof.
use std::collections::HashMap;
use std::fmt;

//==============================================

/// The full roster of working employees of the store.
///
/// `self.emps` contains the raw HashMap of ID Strings -> Employees.
#[derive(Debug)]
pub struct Roster {
    emps: HashMap<String, Employee>,
}

impl Roster {
    pub fn new() -> Roster {
        //! Create a new employee roster.
        let emps = HashMap::new();
        Roster { emps }
    }
    pub fn add(&mut self, emp: Employee) {
        //! Add an employee to the roster.
        self.emps.insert(emp.get_id(), emp);
    }
    pub fn get(&self, id: String) -> &Employee {
        //! Get an employee reference from the roster by String ID.
        // Todo fix this nonsense, see issue #16
        self.emps.get(&id).unwrap()
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Employee> {
        //! Return an iterator across the employees in this roster.
        self.emps.iter()
    }
    pub fn print(&self) {
        //! Print all employees on the roster.
        for (_s, emp) in self.iter() {
            println!("{}", emp);
        }
    }
}

impl fmt::Display for Roster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for (_s, emp) in self.iter() {
            out.push_str("\n");
            out.push_str(&emp.to_string());
        }
        write!(f, "{}", out)
    }
}

//==============================================

#[derive(Clone, Debug)]
/// An employee of the business, identified by the String `self.id`.
pub struct Employee {
    pub id: String,
    can_work_days: [bool; 7],
    minimum_hours: usize,
    maximum_hours: usize,
    abils: HashMap<String, u8>,
    roles: Vec<String>,
}

impl Employee {
    pub fn new(id: String) -> Employee {
        //! Create a new Employee.
        Employee {
            id,
            can_work_days: [true; 7],
            minimum_hours: 37,
            maximum_hours: 39,
            abils: HashMap::new(),
            roles: Vec::new(),
        }
    }
    pub fn get_id(&self) -> String {
        //! Return an Employee's identifier.
        self.id.clone()
    }
    pub fn set_abil(&mut self, k: &str, v: u8) {
        let key = k.clone();
        if let Some(val) = self.abils.insert(k.to_owned(), v) {
            println!("Updated {} to {} in {}", key, val, self.id);
        };
    }
    pub fn add_role(&mut self, s: &str) {
        self.roles.push(s.to_owned());
    }
    pub fn is_class_only(&mut self) {
        self.change_hours(0, 10).expect("something went wrong in setting hours somehow");
        self.set_abil("Class", 3);
    }
    pub fn change_hours(&mut self, min: usize, max: usize) -> Result<(), &'static str> {
        //! Change an employee's required hour max/min.
        if max > min && max <= 40 {
            self.minimum_hours = min;
            self.maximum_hours = max;
            Ok(())
        } else {
            // todo expand error messages
            Err("something went wrong!")
        }
    }
    pub fn cant_work(&mut self, day: usize) -> () {
        //! Indicate that this employee can't work a given day.
        if day < 7 {
            self.can_work_days[day] = false;
        } else {
            println!("invalid day # - did not change")
        }
    }
    pub fn min_hours(&self) -> usize {
        self.minimum_hours
    }
    pub fn max_hours(&self) -> usize {
        self.maximum_hours
    }
}

impl fmt::Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        out.push_str(&format!("\n=== ID: {} ===", self.get_id()));
        let weekdays = ["Sat", "Sun", "Mon", "Tue", "Wed", "Thu", "Fri"];
        out.push_str("\nCan work: \n");
        for (n, name) in weekdays.iter().enumerate() {
            if self.can_work_days[n] { out.push_str(&format!("{} ", name)); }
        }
        out.push_str(&format!("\nHours range: {} - {}", self.minimum_hours, self.maximum_hours));
        write!(f, "{}", out)
    }
}
