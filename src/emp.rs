//! The emp module contains tools and structures for managing employees and the full store roster thereof.
use std::collections::HashMap;

//==============================================

/// The full roster of working employees of the store.
///
/// `self.emps` contains the raw HashMap of ID Strings -> Employees.
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
        self.emps.get(&id).unwrap()
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Employee> {
        //! Return an iterator across the employees in this roster.
        self.emps.iter()
    }
    pub fn print(&self) {
        //! Print all employees on the roster.
        for (_s, emp) in self.iter() {
            emp.print();
        }
    }
}

//==============================================

#[derive(Clone)]
/// An employee of the business, identified by the String `self.id`.
pub struct Employee {
    pub id: String,
    reqs: Requirements,
    abils: Abilities,
}

impl Employee {
    pub fn new(id: String) -> Employee {
        //! Create a new Employee.
        let reqs = Requirements::new();
        let abils = Abilities::new();
        Employee { id, reqs, abils }
    }
    pub fn get_id(&self) -> String {
        //! Return an Employee's identifier.
        self.id.clone()
    }
    pub fn can_do_pkmn(&mut self) {
        self.abils.pkmn = true;
    }
    pub fn can_do_magic(&mut self) {
        self.abils.magic = true;
    }
    pub fn cant_do_class(&mut self) {
        self.abils.class = false;
    }
    pub fn cant_do_adult_parties(&mut self) {
        self.abils.adult_parties = false;
    }
    pub fn can_do_kids_magic(&mut self) {
        self.abils.kids_magic = true;
    }
    pub fn print(&self) {
        //! `print!()` info about an employee's abilities and availability.
        println!("\n=== ID: {} ===", self.get_id());
        self.reqs.print();
        self.abils.print();
    }
    pub fn can_only_close(&mut self) {
        self.reqs.closer_only = true;
    }
    pub fn is_class_only(&mut self) {
        self.reqs.class_only = true;
        self.change_hours(0, 10).expect("something went wrong in setting hours somehow");
        self.cant_do_adult_parties();
    }
    pub fn change_hours(&mut self, min: i32, max: i32) -> Result<(), &'static str> {
        //! Change an employee's required hour max/min.
        if max > min && min >= 0 && max <= 40 {
            self.reqs.minimum_hours = min;
            self.reqs.maximum_hours = max;
            Ok(())
        } else {
            // todo expand error messages
            Err("something went wrong!")
        }
    }
    pub fn cant_work(&mut self, day: usize) -> () {
        //! Indicate that this employee can't work a given day.
        if day < 7 {
            self.reqs.can_work_days[day] = false;
        } else {
            println!("invalid day # - did not change")
        }
    }
}

//==============================================

#[derive(Clone)]
struct Requirements {
    can_work_days: [bool; 7],
    minimum_hours: i32,
    maximum_hours: i32,
    closer_only: bool,
    class_only: bool,
}

impl Requirements {
    fn new() -> Requirements {
        Requirements {
            can_work_days: [true; 7],
            minimum_hours: 37,
            maximum_hours: 39,
            closer_only: false,
            class_only: false,
        }
    }
    fn print(&self) -> () {
        let weekdays = ["Sat", "Sun", "Mon", "Tue", "Wed", "Thu", "Fri"];
        print!("Can work: ");
        for (n, name) in weekdays.iter().enumerate() {
            if self.can_work_days[n] { print!("{} ", name); }
        }
        println!("\nHours range: {} - {}", self.minimum_hours, self.maximum_hours)
    }
}

//==============================================

#[derive(Clone)]
struct Abilities {
    pkmn: bool,
    magic: bool,
    class: bool,
    adult_parties: bool,
    kids_magic: bool,
}

impl Abilities {
    fn new() -> Abilities {
        Abilities {
            pkmn: false,
            magic: false,
            class: true,
            adult_parties: true,
            kids_magic: false,
        }
    }
    fn print(&self) {
        println!("Can run:");
        if self.pkmn || self.magic || self.class || self.adult_parties || self.kids_magic {
            if self.pkmn { println!("- Pokemon") }
            if self.magic { println!("- Magic") }
            if self.class { println!("- Kids' classes") }
            if self.adult_parties { println!("- Adult parties") }
            if self.kids_magic { println!("- Kids' magic") }
        } else {
            println!("Jack diddly.");
        }
    }
}
