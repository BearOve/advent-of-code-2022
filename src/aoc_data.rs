use crate::*;

pub use self::aoc_data::*;

#[export_module]
mod aoc_data {
    use super::*;
    use std::{
        cell::RefCell,
        fs::File,
        io::{BufRead, BufReader},
    };

    type SharedAocData = Shared<Locked<AocData>>;

    pub struct AocData {
        path: PathBuf,
        file: BufReader<File>,
    }

    impl AocData {
        pub fn load(path: PathBuf) -> Result<SharedAocData> {
            let file = File::open(&path)
                .map(BufReader::new)
                .wrap_err_with(|| format!("Failed open data file {path:?}"))?;

            Ok(Shared::new(RefCell::new(Self { path, file })))
        }

        pub fn read_line(&mut self) -> Option<ImmutableString> {
            let mut ret = String::new();
            let actual = self
                .file
                .read_line(&mut ret)
                .wrap_err_with(|| format!("Failed to read the next line from {:?}", self.path))
                .unwrap(); // ToDo: Fallible iterator support?
            while matches!(ret.as_bytes().last(), Some(b'\r' | b'\n')) {
                ret.pop();
            }
            (actual > 0).then_some(ret.into())
        }
    }

    pub fn next_line(data: &mut SharedAocData) -> Dynamic {
        data.borrow_mut()
            .read_line()
            .map(Dynamic::from)
            .unwrap_or(Dynamic::UNIT)
    }

    pub fn lines(data: SharedAocData) -> DynIterator<ImmutableString> {
        DynIterator::new(std::iter::from_fn(move || data.borrow_mut().read_line()))
    }

    pub fn blobs(data: SharedAocData) -> DynIterator<Blob> {
        DynIterator::new(std::iter::from_fn(move || {
            let mut data = data.borrow_mut();
            let mut ret = Vec::new();
            let actual = data
                .file
                .read_until(b'\n', &mut ret)
                .wrap_err_with(|| format!("Failed to read the next line from {:?}", data.path))
                .unwrap(); // ToDo: Fallible iterator support?
            while matches!(ret.last(), Some(b'\r' | b'\n')) {
                ret.pop();
            }
            (actual > 0).then_some(ret)
        }))
    }
}
