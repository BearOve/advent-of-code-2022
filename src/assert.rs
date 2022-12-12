use crate::*;

pub use self::assert::*;

#[export_module]
mod assert {
    use super::*;

    #[derive(Clone)]
    struct Error(ImmutableString);

    impl Error {
        fn new(s: impl Into<ImmutableString>) -> Self {
            Self(s.into())
        }

        fn into_err(self, ctx: &NativeCallContext) -> Box<EvalAltResult> {
            Box::new(EvalAltResult::ErrorRuntime(
                Dynamic::from(self),
                ctx.position(),
            ))
        }
    }

    macro_rules! fail_if {
        ($ctx:expr, $a:ident $op:tt $b:ident) => {
            if $a $op $b {
                Err(Error::new(
                    format!(
                        concat!(
                            "Assertation failed: {}",
                            stringify!($op),
                            " {}",
                        ),
                        $a, $b,
                    )
                ).into_err(&$ctx))
            } else {
                Ok(())
            }
        }
    }

    #[rhai_fn(name = "assert_eq", return_raw)]
    pub fn assert_eq_int(ctx: NativeCallContext, a: INT, b: INT) -> RhaiRes<()> {
        fail_if!(ctx, a != b)
    }

    #[rhai_fn(return_raw)]
    pub fn assert(ctx: NativeCallContext, a: bool) -> RhaiRes<()> {
        fail_if!(ctx, a == false)
    }

    #[rhai_fn(return_raw)]
    pub fn todo(ctx: NativeCallContext) -> RhaiRes<()> {
        Err(Box::new(EvalAltResult::ErrorRuntime(
            Dynamic::from("Not implemented"),
            ctx.position(),
        )))
    }
}
