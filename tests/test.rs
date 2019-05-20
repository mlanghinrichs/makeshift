use sched_lib;

#[cfg(test)]
#[allow(unused_imports)]
mod test_time {
    use sched_lib::time::Time;
    #[test]
    fn morning() {
        let t = Time::from_str("10:15");
        assert_eq!(t.qi, 40);
    }
    #[test]
    fn afternoon() {
        let t = Time::from_str("22:30");
        assert_eq!(t.qi, 89);
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
}
