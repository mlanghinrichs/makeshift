#[cfg(test)]
#[allow(unused_imports)]
mod test_time {
    use sched_lib::time::Time;
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
    fn eq1() {
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
        assert_eq!(Time::from_hour(0).to_string_12h(), "12:00a".to_string());
        assert_eq!(Time::from_qi(3).to_string_12h(), "12:45a".to_string());
        assert_eq!(Time::from_str("0:45").get_qi(), 3);
        assert_eq!(Time::from_str("0:45").to_string_12h(), "12:45a".to_string());
    }
    #[test]
    fn string_12h_2() {
        assert_eq!(Time::from_hour(4).to_string_12h(), "4:00a".to_string());
        assert_eq!(Time::from_qi(21).to_string_12h(), "5:15a".to_string());
        assert_eq!(Time::from_str("5:45").to_string_12h(), "5:45a".to_string());
    }
    #[test]
    fn string_12h_3() {
        assert_eq!(Time::from_hour(12).to_string_12h(), "12:00p".to_string());
        assert_eq!(Time::from_qi(49).to_string_12h(), "12:15p".to_string());
        assert_eq!(Time::from_str("12:45").to_string_12h(), "12:45p".to_string());
    }
    #[test]
    fn string_12h_4() {
        assert_eq!(Time::from_hour(17).to_string_12h(), "5:00p".to_string());
        assert_eq!(Time::from_qi(81).to_string_12h(), "8:15p".to_string());
        assert_eq!(Time::from_str("22:45").to_string_12h(), "10:45p".to_string());
    }
}
