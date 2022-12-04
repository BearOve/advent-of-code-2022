use eyre::{ensure, eyre, Result, WrapErr};
use rhai::plugin::*;
use rhai::{Array, EvalAltResult, Locked, Shared, INT};
use std::path::PathBuf;

mod aoc_data;
mod int_array;

fn index_not_found(index: impl Into<Dynamic>) -> Box<EvalAltResult> {
    Box::new(EvalAltResult::ErrorIndexNotFound(
        index.into(),
        Position::NONE,
    ))
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

    // ToDo: Is there no magic to register this in the module?
    engine.register_iterator::<aoc_data::Lines>();

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
                    assert_eq!(res, [$p1, $p2]);
                })+
                Ok(())
            }
        )+}
    }

    impl_tests!(
        day_01 = (test = ("24000", "45000"), user = ("75622", "213159"),),
        day_02 = (test = ("15", "12"), user = ("8392", "10116"),),
        //day_xx = (test = ("ToDo", "ToDo"), user = ("ToDo", "ToDo"),),
    );
}
