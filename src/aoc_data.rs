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
    }

    pub fn lines(data: SharedAocData) -> Lines {
        Lines { data }
    }

    pub fn blobs(data: SharedAocData) -> Blobs {
        Blobs { data }
    }

    #[derive(Clone)]
    pub struct Lines {
        data: SharedAocData,
    }

    impl Iterator for Lines {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            let mut data = self.data.borrow_mut();
            let mut ret = String::new();
            let actual = data
                .file
                .read_line(&mut ret)
                .wrap_err_with(|| format!("Failed to read the next line from {:?}", data.path))
                .unwrap(); // ToDo: Fallible iterator support?
            while matches!(ret.as_bytes().last(), Some(b'\r' | b'\n')) {
                ret.pop();
            }
            (actual > 0).then_some(ret)
        }
    }

    #[derive(Clone)]
    pub struct Blobs {
        data: SharedAocData,
    }

    impl Iterator for Blobs {
        type Item = rhai::Blob;

        fn next(&mut self) -> Option<Self::Item> {
            let mut data = self.data.borrow_mut();
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
        }
    }
}
