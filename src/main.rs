use crate::{dyn_iterator::DynIterator, error::*, tuple_extras::index_tup2};
use eyre::{ensure, eyre, Result, WrapErr};
use rhai::{plugin::*, Array, Blob, EvalAltResult, Locked, Shared, FLOAT, INT};
use std::{
    collections::HashSet,
    fmt::{Debug, Formatter, Result as FmtResult},
    path::PathBuf,
};

mod aoc_data;
mod assert;
mod blob_extras;
mod dyn_iterator;
mod dynamic_image;
mod error;
mod int_array;

type SharedSet<T> = Shared<Locked<HashSet<T>>>;

#[export_module]
mod string_extras {
    pub fn chunks(a: ImmutableString, len: INT) -> DynIterator<ImmutableString> {
        let len = usize::try_from(len).unwrap();
        let mut i = 0;
        DynIterator::new(std::iter::from_fn(move || {
            if let Some((end, _)) = a[i..].char_indices().nth(len) {
                let end = end + i;
                let ret = a[i..end].into();
                i = end;
                Some(ret)
            } else if i < a.len() {
                let ret = a[i..].into();
                i = a.len();
                Some(ret)
            } else {
                None
            }
        }))
    }
}

#[export_module]
mod int_extras {
    #[rhai_fn(name = "tuple")]
    pub fn tuple_int_int(a: INT, b: INT) -> (INT, INT) {
        (a, b)
    }

    #[rhai_fn(name = "max")]
    pub fn max_int_int(a: INT, b: INT) -> INT {
        a.max(b)
    }

    #[rhai_fn(name = "min")]
    pub fn min_int_int(a: INT, b: INT) -> INT {
        a.min(b)
    }

    #[rhai_fn(name = "!")]
    pub fn bitnot_int(a: INT) -> INT {
        !a
    }
}

#[export_module]
mod array_extras {
    #[rhai_fn(pure, return_raw)]
    pub fn sum(ctx: NativeCallContext, a: &mut rhai::Array) -> RhaiRes<INT> {
        let mut ret = 0;
        for val in a.iter() {
            ret += val
                .as_int()
                .map_err(|e| mismatching_data_type(&ctx, "integer", e))?;
        }
        Ok(ret)
    }
}

#[export_module]
mod tuple_extras {
    #[rhai_fn(return_raw, index_get)]
    pub fn index_int_int(ctx: NativeCallContext, tup: (INT, INT), index: INT) -> RhaiRes<INT> {
        index_tup2(ctx, tup, index)
    }

    #[rhai_fn(name = "to_debug")]
    pub fn to_debug_int_int(tup: (INT, INT)) -> String {
        format!("{:?}", tup)
    }

    #[rhai_fn(return_raw, index_get)]
    pub fn index_str_str(
        ctx: NativeCallContext,
        tup: (ImmutableString, ImmutableString),
        index: INT,
    ) -> RhaiRes<ImmutableString> {
        index_tup2(ctx, tup, index)
    }

    #[rhai_fn(name = "to_debug")]
    pub fn to_debug_str_str(tup: (ImmutableString, ImmutableString)) -> String {
        format!("{:?}", tup)
    }

    #[rhai_fn(skip)]
    pub fn index_tup2<T>(ctx: NativeCallContext, tup: (T, T), index: INT) -> RhaiRes<T> {
        match index {
            0 => Ok(tup.0),
            1 => Ok(tup.1),
            _ => Err(Box::new(EvalAltResult::ErrorIndexNotFound(
                index.into(),
                ctx.position(),
            ))),
        }
    }

    #[rhai_fn(name = "min")]
    pub fn min_int_int(a: (INT, INT), b: (INT, INT)) -> (INT, INT) {
        (a.0.min(b.0), a.1.min(b.1))
    }

    #[rhai_fn(name = "max")]
    pub fn max_int_int(a: (INT, INT), b: (INT, INT)) -> (INT, INT) {
        (a.0.max(b.0), a.1.max(b.1))
    }

    #[rhai_fn(name = "cmp")]
    pub fn cmp_int_int(lhs: (INT, INT), rhs: (INT, INT)) -> INT {
        lhs.cmp(&rhs) as INT
    }

    #[rhai_fn(name = "==")]
    pub fn eq_int_int(lhs: (INT, INT), rhs: (INT, INT)) -> bool {
        lhs == rhs
    }

    #[rhai_fn(name = "!=")]
    pub fn neq_int_int(lhs: (INT, INT), rhs: (INT, INT)) -> bool {
        lhs != rhs
    }

    #[rhai_fn(name = "<")]
    pub fn lt_int_int(lhs: (INT, INT), rhs: (INT, INT)) -> bool {
        lhs < rhs
    }

    #[rhai_fn(name = "<=")]
    pub fn le_int_int(lhs: (INT, INT), rhs: (INT, INT)) -> bool {
        lhs <= rhs
    }

    #[rhai_fn(name = ">")]
    pub fn gt_int_int(lhs: (INT, INT), rhs: (INT, INT)) -> bool {
        lhs > rhs
    }

    #[rhai_fn(name = ">=")]
    pub fn ge_int_int(lhs: (INT, INT), rhs: (INT, INT)) -> bool {
        lhs >= rhs
    }

    #[rhai_fn(name = "+")]
    pub fn add_int_int(lhs: (INT, INT), rhs: (INT, INT)) -> (INT, INT) {
        (lhs.0 + rhs.0, lhs.1 + rhs.1)
    }

    #[rhai_fn(name = "-")]
    pub fn sub_int_int(lhs: (INT, INT), rhs: (INT, INT)) -> (INT, INT) {
        (lhs.0 - rhs.0, lhs.1 - rhs.1)
    }

    #[rhai_fn(name = "sign")]
    pub fn sign_int_int(v: (INT, INT)) -> (INT, INT) {
        (v.0.signum(), v.1.signum())
    }

    #[rhai_fn(name = "fixed_set")]
    pub fn fixed_set_int_int(v: (INT, INT)) -> SharedSet<(INT, INT)> {
        let mut ret = HashSet::new();
        ret.insert(v);
        Shared::new(Locked::new(ret))
    }

    #[rhai_fn(name = "insert")]
    pub fn fixed_set_int_int_insert(set: &mut SharedSet<(INT, INT)>, v: (INT, INT)) -> bool {
        set.borrow_mut().insert(v)
    }

    #[rhai_fn(name = "len", pure)]
    pub fn fixed_set_int_int_len(set: &mut SharedSet<(INT, INT)>) -> INT {
        set.borrow_mut().len().try_into().unwrap()
    }
}

fn run_script(day: u8, data_name: &str) -> Result<[String; 2]> {
    let dir = PathBuf::from(format!("solutions/day-{day:02}"));
    let script_path = dir.join("script.rhai");
    let data_path = dir.join(data_name);

    let mut engine = rhai::Engine::new();
    let mut scope = rhai::Scope::new();

    scope.push("data", aoc_data::AocData::load(data_path)?);

    engine.set_fast_operators(false);

    engine.register_global_module(exported_module!(aoc_data).into());
    engine.register_global_module(exported_module!(int_array).into());
    engine.register_global_module(exported_module!(blob_extras).into());
    engine.register_global_module(exported_module!(dyn_iterator).into());
    engine.register_global_module(exported_module!(string_extras).into());
    engine.register_global_module(exported_module!(int_extras).into());
    engine.register_global_module(exported_module!(assert).into());
    engine.register_global_module(exported_module!(dynamic_image).into());
    engine.register_global_module(exported_module!(array_extras).into());
    engine.register_global_module(exported_module!(tuple_extras).into());

    // ToDo: Is there no magic to register this in the module?
    engine.register_iterator::<DynIterator<ImmutableString>>();
    engine.register_iterator::<DynIterator<Blob>>();
    engine.register_iterator::<DynIterator<(ImmutableString, ImmutableString)>>();
    engine.register_iterator::<DynIterator<Vec<ImmutableString>>>();
    engine.register_iterator::<DynIterator<Vec<Dynamic>>>();
    engine.register_iterator::<DynIterator<dynamic_image::Row>>();
    engine.register_iterator::<DynIterator<dynamic_image::Col>>();
    engine.register_iterator::<DynIterator<dynamic_image::Pixel>>();
    engine.register_iterator::<dynamic_image::Row>();
    engine.register_iterator::<dynamic_image::Col>();

    engine
        .eval_file_with_scope(&mut scope, script_path.clone())
        .map_err(|e| eyre!("{e}"))
        .wrap_err_with(|| format!("Failed to run {script_path:?}"))
        .and_then(|ret: Array| {
            ensure!(
                ret.len() == 2,
                "Invalid return type, expected array with two strings"
            );
            let mut it = ret.into_iter().map(|v| v.into_string().unwrap());
            Ok([it.next().unwrap(), it.next().unwrap()])
        })
}

#[derive(clap::Parser)]
struct Args {
    day: u8,
    #[clap(value_parser = ["test.dat", "user.dat"])]
    data: String,
}

fn main() -> Result<()> {
    let Args { day, data } = clap::Parser::parse();
    println!("Result: {:?}", run_script(day, &data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! impl_tests {
        ($(
            $day:ident = (
                $($name:ident=($p1:literal, $p2:literal),)+
            )
        ,)+) => {$(
            mod $day {$(
                #[test]
                fn $name() -> super::Result<()> {
                    let day = stringify!($day).split_once('_').unwrap().1.parse()?;
                    let data = concat!(stringify!($name), ".dat");
                    println!("\n\nRunning script with {data}");
                    let res = super::run_script(day, data)?;
                    assert_eq!(res[0], $p1, "day {day} part1 failed");
                    assert_eq!(res[1], $p2, "day {day} part2 failed");
                    Ok(())
                }
            )+}
        )+}
    }

    impl_tests!(
        day_01 = (test = ("24000", "45000"), user = ("75622", "213159"),),
        day_02 = (test = ("15", "12"), user = ("8392", "10116"),),
        day_03 = (test = ("157", "70"), user = ("8176", "2689"),),
        day_04 = (test = ("2", "4"), user = ("562", "924"),),
        day_05 = (test = ("CMZ", "MCD"), user = ("QNNTGTPFN", "GGNPJBTTR"),),
        day_06 = (
            test = ("[7, 5, 6, 10, 11]", "[19, 23, 23, 29, 26]"),
            user = ("[1855]", "[3256]"),
        ),
        day_07 = (test = ("95437", "24933642"), user = ("1427048", "2940614"),),
        day_08 = (test = ("21", "8"), user = ("1708", "504000"),),
        day_09 = (
            test = ("13", "1"),
            test2 = ("88", "36"),
            user = ("6470", "2658"),
        ),
        //day_xx = (test = ("ToDo: test.dat", "ToDo: test.dat"), user = ("ToDo: user.dat", "ToDo: user.dat"),),
    );
}
