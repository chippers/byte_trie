pub(crate) type Key = Vec<u8>;
pub(crate) type KeySlice = [u8];

pub(crate) enum KeyMatch {
    /// The keys are **exactly** the same
    Exact,
    /// The original key is fully matched, new key has more bytes
    FullOriginal(usize),
    /// The new key is fully matched, the original has more bytes
    FullNew(usize),
    /// Both keys match partially, and have additional bytes
    Partial(usize),
    /// No parts of the new keys match
    None,
}

impl KeyMatch {
    pub(crate) fn compare(original: &KeySlice, new: &KeySlice) -> Self {
        let prefix = original
            .iter()
            .zip(new.iter())
            .take_while(|(&o, &n)| o == n)
            .count();

        let original = original.len();
        let new = new.len();

        if prefix == original && prefix == new {
            KeyMatch::Exact
        } else if prefix == original && new > original {
            KeyMatch::FullOriginal(prefix)
        } else if prefix == new && original > new {
            KeyMatch::FullNew(prefix)
        } else if prefix > 0 {
            KeyMatch::Partial(prefix)
        } else {
            KeyMatch::None
        }
    }
}
