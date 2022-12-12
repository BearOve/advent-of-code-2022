use crate::*;

pub use self::blob_extras::*;

#[export_module]
mod blob_extras {
    #[rhai_fn(pure)]
    pub fn intersection(a: &mut Blob, b: Blob) -> Blob {
        let a: HashSet<u8> = a.iter().copied().collect();
        let b: HashSet<u8> = b.iter().copied().collect();
        a.intersection(&b).copied().collect()
    }

    #[rhai_fn(pure)]
    pub fn unique_count(a: &mut Blob) -> INT {
        let a: HashSet<u8> = a.iter().copied().collect();
        a.len().try_into().unwrap()
    }

    #[rhai_fn(pure, name = "==")]
    pub fn eq_string(a: &mut Blob, b: ImmutableString) -> bool {
        a == b.as_bytes()
    }

    pub fn strip_prefix(a: &mut Blob, prefix: ImmutableString) -> bool {
        if a.starts_with(prefix.as_bytes()) {
            a.copy_within(prefix.len().., 0);
            a.truncate(a.len() - prefix.len());
            true
        } else {
            false
        }
    }

    pub fn rstrip_off(a: &mut Blob, delim: INT) {
        if let Ok(delim) = u8::try_from(delim) {
            if let Some(suff) = a.rsplit(|&b| b == delim).next() {
                let len = suff.len() + 1;
                a.truncate(a.len() - len);
            }
        }
    }
}
