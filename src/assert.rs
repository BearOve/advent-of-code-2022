use crate::*;

pub use self::assert::*;

#[export_module]
mod assert {
    use super::*;

    struct Error(String);

    impl Error {
        pub fn create(e: String) -> Box<EvalAltResult> {
            Box::new(EvalAltResult::ErrorSystem(
                "Assertation failed".to_string(),
                Box::new(Error(e)),
            ))
        }
    }

    impl std::error::Error for Error {}

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Display::fmt(&self.0, f)
        }
    }

    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }

    #[rhai_fn(name = "assert_eq", return_raw)]
    pub fn assert_eq_int(a: INT, b: INT) -> RhaiRes<()> {
        if a == b {
            Ok(())
        } else {
            Err(Error::create(format!("{a} != {b}")))
        }
    }

    #[rhai_fn(return_raw)]
    pub fn assert(a: bool) -> RhaiRes<()> {
        if a {
            Ok(())
        } else {
            Err(Error::create(String::new()))
        }
    }
}
