use crate::*;

pub use self::int_array::*;

#[export_module]
mod int_array {
    use super::*;

    type SharedIntArray = Shared<Locked<IntArray>>;

    #[derive(Default)]
    pub struct IntArray {
        values: Vec<INT>,
    }

    impl IntArray {
        fn new_shared(values: Vec<INT>) -> SharedIntArray {
            Shared::new(Locked::new(IntArray { values }))
        }
    }

    impl IntArray {
        fn resolve_index(&self, index: INT) -> Result<usize, Box<EvalAltResult>> {
            let len = self.values.len();
            if index < 0 {
                let abs_index = index.checked_abs().unwrap_or(0) as usize;
                if abs_index <= len {
                    return Ok(len - abs_index);
                } else {
                    None
                }
            } else {
                let abs_index = index as usize;
                if abs_index < len {
                    return Ok(abs_index);
                } else {
                    None
                }
            }
            .ok_or_else(|| index_not_found(index))
        }
    }

    #[rhai_fn(name = "int_array")]
    pub fn int_array() -> SharedIntArray {
        SharedIntArray::default()
    }

    #[rhai_fn(name = "int_array")]
    pub fn int_array_with_init(input: &mut Array) -> SharedIntArray {
        let mut values = Vec::with_capacity(input.len());
        for val in input.iter() {
            values.push(val.as_int().unwrap());
        }
        IntArray::new_shared(values)
    }

    pub fn push(array: &mut SharedIntArray, value: INT) {
        array.borrow_mut().values.push(value)
    }

    #[rhai_fn(pure)]
    pub fn max(array: &mut SharedIntArray) -> INT {
        array.borrow().values.iter().copied().max().unwrap_or(0)
    }

    #[rhai_fn(pure)]
    pub fn sum(array: &mut SharedIntArray) -> INT {
        array.borrow().values.iter().copied().sum()
    }

    pub fn rsort(array: &mut SharedIntArray) -> SharedIntArray {
        array.borrow_mut().values.sort_unstable_by(|a, b| b.cmp(a));
        array.clone()
    }

    pub fn sort(array: &mut SharedIntArray) -> SharedIntArray {
        array.borrow_mut().values.sort_unstable();
        array.clone()
    }

    #[rhai_fn(pure, name = "extract", return_raw)]
    pub fn extract_from(
        array: &mut SharedIntArray,
        start: INT,
    ) -> Result<SharedIntArray, Box<EvalAltResult>> {
        let array = array.borrow();
        let start = array.resolve_index(start)?;
        Ok(IntArray::new_shared(array.values[start..].to_vec()))
    }

    #[rhai_fn(pure)]
    pub fn to_debug(array: &mut SharedIntArray) -> String {
        format!("{:?}", array.borrow().values)
    }

    #[rhai_fn(pure, index_get, return_raw)]
    pub fn index_get(array: &mut SharedIntArray, index: INT) -> Result<INT, Box<EvalAltResult>> {
        let array = array.borrow();
        let i = array.resolve_index(index)?;
        Ok(array.values[i])
    }

    #[rhai_fn(index_set, return_raw)]
    pub fn index_set(
        array: &mut SharedIntArray,
        index: INT,
        value: INT,
    ) -> Result<(), Box<EvalAltResult>> {
        let mut array = array.borrow_mut();
        let i = array.resolve_index(index)?;
        array.values[i] = value;
        Ok(())
    }
}
