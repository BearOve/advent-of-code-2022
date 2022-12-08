use crate::*;

pub type RhaiRes<T> = Result<T, Box<EvalAltResult>>;

pub fn index_not_found(ctx: &NativeCallContext, index: impl Into<Dynamic>) -> Box<EvalAltResult> {
    Box::new(EvalAltResult::ErrorIndexNotFound(
        index.into(),
        ctx.position(),
    ))
}

pub fn mismatching_data_type(
    ctx: &NativeCallContext,
    req: impl Into<String>,
    act: impl Into<String>,
) -> Box<EvalAltResult> {
    Box::new(EvalAltResult::ErrorMismatchDataType(
        req.into(),
        act.into(),
        ctx.position(),
    ))
}

pub fn try_from<T, R, E>(ctx: &NativeCallContext, v: T) -> RhaiRes<R>
where
    T: Copy + std::fmt::Display + TryInto<R, Error = E>,
    E: std::fmt::Display,
{
    v.try_into().map_err(|e| {
        Box::new(EvalAltResult::ErrorArithmetic(
            format!("Failed to convert {v}: {e}"),
            ctx.position(),
        ))
    })
}
