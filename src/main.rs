use crate::{dyn_iterator::DynIterator, error::*, tuple_extras::index_tup2};
use eyre::{ensure, eyre, Result, WrapErr};
use rhai::{plugin::*, Array, EvalAltResult, Locked, Shared, INT};
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

#[export_module]
mod string_extras {
    pub fn chunks(a: ImmutableString, len: INT) -> DynIterator<String> {
        let len = usize::try_from(len).unwrap();
        let mut i = 0;
        DynIterator::new(std::iter::from_fn(move || {
            if let Some((end, _)) = a[i..].char_indices().nth(len) {
                let end = end + i;
                let ret = String::from(&a[i..end]);
                i = end;
                Some(ret)
            } else if i < a.len() {
                let ret = String::from(&a[i..]);
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
    #[rhai_fn(name = "max")]
    pub fn max_int_int(a: INT, b: INT) -> INT {
        a.max(b)
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
    engine.register_iterator::<aoc_data::Lines>();
    engine.register_iterator::<aoc_data::Blobs>();
    engine.register_iterator::<DynIterator<String>>();
    engine.register_iterator::<DynIterator<Vec<String>>>();
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
        //day_xx = (test = ("ToDo: test.dat", "ToDo: test.dat"), user = ("ToDo: user.dat", "ToDo: user.dat"),),
    );
}
