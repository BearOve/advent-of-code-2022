use crate::*;

pub use self::dyn_iterator::*;

#[export_module]
mod dyn_iterator {
    #[derive(Clone)]
    pub struct DynIterator<T> {
        it: Shared<Locked<Box<dyn Iterator<Item = T>>>>,
    }

    impl<T> DynIterator<T> {
        pub fn new(it: impl Iterator<Item = T> + 'static) -> Self {
            Self {
                it: Shared::new(Locked::new(Box::new(it))),
            }
        }
    }

    #[rhai_fn(name = "split")]
    pub fn str_split_by_char(
        data: DynIterator<ImmutableString>,
        c: char,
    ) -> DynIterator<Vec<ImmutableString>> {
        DynIterator::new(data.map(move |v| v.split(c).map(|s| s.into()).collect()))
    }

    #[rhai_fn(name = "split_once")]
    pub fn str_split_once_by_char(
        data: DynIterator<ImmutableString>,
        c: char,
    ) -> DynIterator<(ImmutableString, ImmutableString)> {
        DynIterator::new(data.map(move |v| {
            v.split_once(c)
                .map(|(a, b)| (a.into(), b.into()))
                .unwrap_or((v, "".into()))
        }))
    }

    #[rhai_fn(name = "split")]
    pub fn dyn_strvec_split_by_char(
        data: DynIterator<Vec<ImmutableString>>,
        c: char,
    ) -> DynIterator<Vec<Dynamic>> {
        DynIterator::new(data.map(move |v| {
            v.iter()
                .map(|s| s.split(c).map(ImmutableString::from).collect())
                .collect()
        }))
    }

    impl<T> Iterator for DynIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.it.borrow_mut().next()
        }
    }
}
