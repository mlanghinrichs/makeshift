use std::collections::HashMap;

#[allow(unused_mut)]
/// Return the full employee roster for Labyrinth Games & Puzzles.
pub fn get_roster() -> Roster {
    let mut ros = Roster::new();
    // Alex
    let mut alex = Employee::new("Alex".to_string());
    alex.can_do_magic();
    alex.can_do_kids_magic();
    ros.add(alex);
    // Arsenio
    let mut arsenio = Employee::new("Arsenio".to_string());
    arsenio.can_do_pkmn();
    arsenio.cant_work(0);
    ros.add(arsenio);
    // Ben
    let mut ben = Employee::new("Ben".to_string());
    ben.can_do_magic();
    ben.can_do_kids_magic();
    ros.add(ben);
    // Camilla
    let mut camilla = Employee::new("Camilla".to_string());
    ros.add(camilla);
    // Darryl
    let mut darryl = Employee::new("Darryl".to_string());
    ros.add(darryl);
    // Dzhoy
    let mut dzhoy = Employee::new("Dzhoy".to_string());
    ros.add(dzhoy);
    // Evan
    let mut evan = Employee::new("Evan".to_string());
    evan.cant_do_class();
    evan.can_only_close();
    ros.add(evan);
    // Hannah
    let mut hannah = Employee::new("Hannah".to_string());
    hannah.cant_work(0);
    hannah.cant_work(1);
    hannah.cant_work(2);
    hannah.cant_work(4);
    hannah.cant_work(6);
    ros.add(hannah);
    // Heather
    let mut heather = Employee::new("Heather".to_string());
    heather.is_class_only();
    ros.add(heather);
    // Joe
    let mut joe = Employee::new("Joe".to_string());
    joe.can_do_kids_magic();
    ros.add(joe);
    // Justin
    let mut justin = Employee::new("Justin".to_string());
    justin.is_class_only();
    ros.add(justin);
    // Kathleen
    //
    // Doesn't really need to be scheduled for shifts
    //
    // Mariah
    let mut mariah = Employee::new("Mariah".to_string());
    mariah.is_class_only();
    ros.add(mariah);
    // Matt
    let mut matt = Employee::new("Matt".to_string());
    matt.can_do_magic();
    matt.can_do_kids_magic();
    ros.add(matt);
    // Nick
    let mut nick = Employee::new("Nick".to_string());
    ros.add(nick);
    // Rich
    let mut rich = Employee::new("Rich".to_string());
    ros.add(rich);
    // William
    // Yoni
    let mut yoni = Employee::new("Yoni".to_string());
    ros.add(yoni);
    // Zach
    let mut zach = Employee::new("Zach".to_string());
    ros.add(zach);

    return ros;
}

/// The full roster of working employees of the store.
///
/// `self.emps` contains the raw HashMap of ID Strings -> Employees.
pub struct Roster {
    emps: HashMap<String, Employee>,
}

#[derive(Clone)]
/// An employee of the business, identified by the String `self.id`.
pub struct Employee {
    pub id: String,
    reqs: Requirements,
    abils: Abilities,
}

#[derive(Clone)]
struct Requirements {
    can_work_days: [bool; 7],
    minimum_hours: i32,
    maximum_hours: i32,
    closer_only: bool,
    class_only: bool,
}

#[derive(Clone)]
struct Abilities {
    pkmn: bool,
    magic: bool,
    class: bool,
    adult_parties: bool,
    kids_magic: bool,
}

impl Roster {
    fn new() -> Roster {
        let emps = HashMap::new();
        Roster { emps }
    }
    fn add(&mut self, emp: Employee) {
        self.emps.insert(emp.get_id(), emp);
    }
    /// Get an employee reference from the roster by String ID.
    pub fn get(&self, id: String) -> &Employee {
        self.emps.get(&id).unwrap()
    }
    /// Return an iterator across the employees in this roster.
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Employee> {
        self.emps.iter()
    }
}

#[allow(dead_code)]
impl Employee {
    fn new(id: String) -> Employee {
        let reqs = Requirements::new();
        let abils = Abilities::new();
        Employee { id, reqs, abils }
    }
    /// Return Employee's identifier.
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    fn can_do_pkmn(&mut self) {
        self.abils.pkmn = true;
    }
    fn can_do_magic(&mut self) {
        self.abils.magic = true;
    }
    fn cant_do_class(&mut self) {
        self.abils.class = false;
    }
    fn cant_do_adult_parties(&mut self) {
        self.abils.adult_parties = false;
    }
    fn can_do_kids_magic(&mut self) {
        self.abils.kids_magic = true;
    }
    /// `print!()` info about an employee's abilities and availability.
    pub fn print(&self) {
        println!("\n=== ID: {} ===", self.get_id());
        self.reqs.print();
        self.abils.print();
    }
    fn can_only_close(&mut self) {
        self.reqs.closer_only = true;
    }
    fn change_hours(&mut self, min: i32, max: i32) -> Result<(), &'static str> {
        if max > min && min >= 0 && max <= 40 {
            self.reqs.minimum_hours = min;
            self.reqs.maximum_hours = max;
            Ok(())
        } else {
            // todo expand error messages
            Err("something went wrong!")
        }
    }
    fn is_class_only(&mut self) {
        self.reqs.class_only = true;
        self.change_hours(0, 10).expect("something went wrong in setting hours somehow");
        self.cant_do_adult_parties();
    }
    fn cant_work(&mut self, day: usize) -> () {
        if day < 7 {
            self.reqs.can_work_days[day] = false;
        } else {
            println!("invalid day # - did not change")
        }
    }
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
