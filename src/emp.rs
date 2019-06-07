//! The emp module contains tools and structures for managing employees and the full store roster thereof.
use super::time;
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
        self.emps.insert(emp.id(), emp);
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
pub struct Hours(usize, usize);

impl Hours {
    pub fn set_min(&mut self, m: usize) -> &mut Hours {
        self.0 = m;
        self
    }
    pub fn set_max(&mut self, m: usize) -> &mut Hours {
        self.1 = m;
        self
    }
    pub fn set(&mut self, min: usize, max: usize) -> &mut Hours {
        self.0 = min;
        self.1 = max;
        self
    }
    pub fn min(&self) -> usize {
        self.0
    }
    pub fn max(&self) -> usize {
        self.1
    }
}

//==============================================

#[derive(Clone, Debug)]
/// An employee of the business, identified by the String `self.id`.
pub struct Employee {
    pub iden: String,
    avail: [bool; 7],
    hrs: Hours,
    abils: HashMap<String, u8>,
    roles: Vec<String>,
}

impl Employee {
    // Construction
    pub fn new(iden: String) -> Employee {
        //! Create a new Employee.
        Employee {
            iden,
            avail: [true; 7],
            hrs: Hours(38, 40),
            abils: HashMap::new(),
            roles: Vec::new(),
        }
    }
    // self.iden
    pub fn id(&self) -> String {
        //! Return an Employee's identifier.
        self.iden.clone()
    }
    // self.avail
    pub fn set_available(&mut self, day: time::Day, b: bool) -> bool {
        //! Set an employee's availability for a day. Returns the previous availability for that day.
        let d_i = day.to_index();
        let out = self.avail[d_i];
        self.avail[d_i] = b;
        out
    }
    pub fn is_available(&self, day: time::Day) -> bool {
        //! Check an if this employee can work a day.
        self.avail[day.to_index()]
    }
    // self.hrs
    pub fn get_hours(&self) -> &Hours {
        &self.hrs
    }
    pub fn hours(&mut self) -> &mut Hours {
        &mut self.hrs
    }
    // self.abils
    pub fn set_abil(&mut self, k: &str, v: u8) {
        let key = k.clone();
        if let Some(val) = self.abils.insert(k.to_owned(), v) {
            println!("Updated {} to {} in {}", key, val, self.id());
        };
    }
    pub fn get_abil(&self, k: &str) -> Option<u8> {
        if let Some(u) = self.abils.get(k) {
            Some(*u)
        } else {
            None
        }
    }
    pub fn is_able(&self, k: &str) -> bool {
        if let Some(u) = self.abils.get(k) {
            true
        } else {
            false
        }
    }
    // self.roles
    pub fn add_role(&mut self, s: &str) {
        self.roles.push(s.to_owned());
    }
    pub fn remove_role(&mut self, s: &str) {
        for (i, role) in self.roles.iter().enumerate() {
            if role == s {
                // self.roles.remove(i);
                println!(
                    "Did not remove role {} from {} because of a quickfix in emp.rs",
                    role,
                    self.id()
                );
            }
        }
    }
    pub fn has_role(&self, s: &str) -> bool {
        for role in self.roles.iter() {
            if role == s {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        out.push_str(&format!("\n=== ID: {} ===", self.id()));
        let weekdays = ["Sat", "Sun", "Mon", "Tue", "Wed", "Thu", "Fri"];
        out.push_str("\nCan work: \n");
        for (n, name) in weekdays.iter().enumerate() {
            if self.avail[n] {
                out.push_str(&format!("{} ", name));
            }
        }
        out.push_str(&format!(
            "\nHours range: {} - {}",
            self.get_hours().min(),
            self.get_hours().max()
        ));
        write!(f, "{}", out)
    }
}
