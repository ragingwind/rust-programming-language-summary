#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 2, 5);
    }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another() {
        panic!("Make this tesdt fail")
    }

    #[test]
    fn boolean_test() {
        assert!(true)
    }

    fn panicer() {
        panic!("panic!")
    }

    #[test]
    #[should_panic()]
    fn should_panic() {
        panicer()
    }

    #[test]
    #[should_panic(expected ="panic!")]
    fn should_panic_expect() {
        panicer()
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
