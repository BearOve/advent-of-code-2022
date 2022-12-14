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
        fn resolve_index(&self, ctx: &NativeCallContext, index: INT) -> RhaiRes<usize> {
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
            .ok_or_else(|| index_not_found(ctx, index))
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
    pub fn len(array: &mut SharedIntArray) -> INT {
        array.borrow().values.len().try_into().unwrap()
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
        ctx: NativeCallContext,
        array: &mut SharedIntArray,
        start: INT,
    ) -> RhaiRes<SharedIntArray> {
        let array = array.borrow();
        let start = array.resolve_index(&ctx, start)?;
        Ok(IntArray::new_shared(array.values[start..].to_vec()))
    }

    pub fn drain(array: &mut SharedIntArray) -> Vec<INT> {
        let mut array = array.borrow_mut();
        let ret = array.values.clone();
        array.values.clear();
        ret
    }

    #[rhai_fn(pure)]
    pub fn to_debug(array: &mut SharedIntArray) -> String {
        format!("{:?}", array.borrow().values)
    }

    #[rhai_fn(pure, index_get, return_raw)]
    pub fn index_get(
        ctx: NativeCallContext,
        array: &mut SharedIntArray,
        index: INT,
    ) -> RhaiRes<INT> {
        let array = array.borrow();
        let i = array.resolve_index(&ctx, index)?;
        Ok(array.values[i])
    }

    #[rhai_fn(index_set, return_raw)]
    pub fn index_set(
        ctx: NativeCallContext,
        array: &mut SharedIntArray,
        index: INT,
        value: INT,
    ) -> Result<(), Box<EvalAltResult>> {
        let mut array = array.borrow_mut();
        let i = array.resolve_index(&ctx, index)?;
        array.values[i] = value;
        Ok(())
    }
}
