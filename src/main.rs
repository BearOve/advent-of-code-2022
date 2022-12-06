use crate::dyn_iterator::DynIterator;
use eyre::{ensure, eyre, Result, WrapErr};
use rhai::plugin::*;
use rhai::{Array, EvalAltResult, Locked, Shared, INT};
use std::{collections::HashSet, path::PathBuf};

mod aoc_data;
mod dyn_iterator;
mod int_array;

fn index_not_found(index: impl Into<Dynamic>) -> Box<EvalAltResult> {
    Box::new(EvalAltResult::ErrorIndexNotFound(
        index.into(),
        Position::NONE,
    ))
}

#[export_module]
mod blob_extras {
    #[rhai_fn(pure, name = "intersection")]
    pub fn blobs_intersection(a: &mut rhai::Blob, b: rhai::Blob) -> rhai::Blob {
        let a: HashSet<u8> = a.iter().copied().collect();
        let b: HashSet<u8> = b.iter().copied().collect();
        a.intersection(&b).copied().collect()
    }

    #[rhai_fn(pure, name = "unique_count")]
    pub fn blobs_unique_count(a: &mut rhai::Blob) -> INT {
        let a: HashSet<u8> = a.iter().copied().collect();
        a.len().try_into().unwrap()
    }
}

#[export_module]
mod string_extras {
    pub fn chunks(a: String, len: INT) -> DynIterator<String> {
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
    pub fn assert_eq_int(a: INT, b: INT) -> Result<(), Box<EvalAltResult>> {
        if a == b {
            Ok(())
        } else {
            Err(Error::create(format!("{a} != {b}")))
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

    engine.register_global_module(exported_module!(aoc_data).into());
    engine.register_global_module(exported_module!(int_array).into());
    engine.register_global_module(exported_module!(blob_extras).into());
    engine.register_global_module(exported_module!(dyn_iterator).into());
    engine.register_global_module(exported_module!(string_extras).into());
    engine.register_global_module(exported_module!(int_extras).into());
    engine.register_global_module(exported_module!(assert).into());

    // ToDo: Is there no magic to register this in the module?
    engine.register_iterator::<aoc_data::Lines>();
    engine.register_iterator::<aoc_data::Blobs>();
    engine.register_iterator::<DynIterator<String>>();
    engine.register_iterator::<DynIterator<Vec<String>>>();
    engine.register_iterator::<DynIterator<Vec<Dynamic>>>();

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
            #[test]
            fn $day() -> Result<()> {
                let day = stringify!($day).split_once('_').unwrap().1.parse()?;

                $({
                    let data = concat!(stringify!($name), ".dat");
                    println!("\n\nRunning script with {data}");
                    let res = run_script(day, data)?;
                    assert_eq!(res[0], $p1, "day {day} part1 failed");
                    assert_eq!(res[1], $p2, "day {day} part2 failed");
                })+
                Ok(())
            }
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
        //day_xx = (test = ("ToDo", "ToDo"), user = ("ToDo", "ToDo"),),
    );
}
