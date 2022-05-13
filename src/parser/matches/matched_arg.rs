// Std
use std::{
    ffi::{OsStr, OsString},
    iter::{Cloned, Flatten},
    slice::Iter,
};

use crate::builder::ArgPredicate;
use crate::parser::AnyValue;
use crate::parser::ValueSource;
use crate::util::eq_ignore_case;
use crate::INTERNAL_ERROR_MSG;

#[derive(Debug, Clone)]
pub(crate) struct MatchedArg {
    occurs: u64,
    ty: Option<ValueSource>,
    indices: Vec<usize>,
    vals: Vec<Vec<AnyValue>>,
    raw_vals: Vec<Vec<OsString>>,
    ignore_case: bool,
}

impl MatchedArg {
    pub(crate) fn new() -> Self {
        MatchedArg {
            occurs: 0,
            ty: None,
            indices: Vec::new(),
            vals: Vec::new(),
            raw_vals: Vec::new(),
            ignore_case: false,
        }
    }

    pub(crate) fn inc_occurrences(&mut self) {
        self.occurs += 1;
    }

    pub(crate) fn get_occurrences(&self) -> u64 {
        self.occurs
    }

    pub(crate) fn indices(&self) -> Cloned<Iter<'_, usize>> {
        self.indices.iter().cloned()
    }

    pub(crate) fn get_index(&self, index: usize) -> Option<usize> {
        self.indices.get(index).cloned()
    }

    pub(crate) fn push_index(&mut self, index: usize) {
        self.indices.push(index)
    }

    #[cfg(test)]
    pub(crate) fn raw_vals(&self) -> Iter<Vec<OsString>> {
        self.raw_vals.iter()
    }

    #[cfg(feature = "unstable-grouped")]
    pub(crate) fn vals(&self) -> Iter<Vec<AnyValue>> {
        self.vals.iter()
    }

    pub(crate) fn vals_flatten(&self) -> Flatten<Iter<Vec<AnyValue>>> {
        self.vals.iter().flatten()
    }

    pub(crate) fn raw_vals_flatten(&self) -> Flatten<Iter<Vec<OsString>>> {
        self.raw_vals.iter().flatten()
    }

    pub(crate) fn first(&self) -> Option<&AnyValue> {
        self.vals_flatten().next()
    }

    #[cfg(test)]
    pub(crate) fn first_raw(&self) -> Option<&OsString> {
        self.raw_vals_flatten().next()
    }

    pub(crate) fn push_val(&mut self, val: AnyValue, raw_val: OsString) {
        self.vals.push(vec![val]);
        self.raw_vals.push(vec![raw_val]);
    }

    pub(crate) fn new_val_group(&mut self) {
        self.vals.push(vec![]);
        self.raw_vals.push(vec![]);
    }

    pub(crate) fn append_val(&mut self, val: AnyValue, raw_val: OsString) {
        // We assume there is always a group created before.
        self.vals.last_mut().expect(INTERNAL_ERROR_MSG).push(val);
        self.raw_vals
            .last_mut()
            .expect(INTERNAL_ERROR_MSG)
            .push(raw_val);
    }

    pub(crate) fn num_vals(&self) -> usize {
        self.vals.iter().flatten().count()
    }

    // Will be used later
    #[allow(dead_code)]
    pub(crate) fn num_vals_last_group(&self) -> usize {
        self.vals.last().map(|x| x.len()).unwrap_or(0)
    }

    pub(crate) fn all_val_groups_empty(&self) -> bool {
        self.vals.iter().flatten().count() == 0
    }

    pub(crate) fn has_val_groups(&self) -> bool {
        !self.vals.is_empty()
    }

    pub(crate) fn check_explicit(&self, predicate: ArgPredicate) -> bool {
        if self.ty == Some(ValueSource::DefaultValue) {
            return false;
        }

        match predicate {
            ArgPredicate::Equals(val) => self.raw_vals_flatten().any(|v| {
                if self.ignore_case {
                    // If `v` isn't utf8, it can't match `val`, so `OsStr::to_str` should be fine
                    eq_ignore_case(&v.to_string_lossy(), &val.to_string_lossy())
                } else {
                    OsString::as_os_str(v) == OsStr::new(val)
                }
            }),
            ArgPredicate::IsPresent => true,
        }
    }

    pub(crate) fn source(&self) -> Option<ValueSource> {
        self.ty
    }

    pub(crate) fn update_ty(&mut self, ty: ValueSource) {
        if let Some(existing) = self.ty {
            self.ty = Some(existing.max(ty));
        } else {
            self.ty = Some(ty)
        }
    }

    pub(crate) fn set_ignore_case(&mut self, yes: bool) {
        self.ignore_case = yes;
    }
}

impl Default for MatchedArg {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for MatchedArg {
    fn eq(&self, other: &MatchedArg) -> bool {
        let MatchedArg {
            occurs: self_occurs,
            ty: self_ty,
            indices: self_indices,
            vals: _,
            raw_vals: self_raw_vals,
            ignore_case: self_ignore_case,
        } = self;
        let MatchedArg {
            occurs: other_occurs,
            ty: other_ty,
            indices: other_indices,
            vals: _,
            raw_vals: other_raw_vals,
            ignore_case: other_ignore_case,
        } = other;
        self_occurs == other_occurs
            && self_ty == other_ty
            && self_indices == other_indices
            && self_raw_vals == other_raw_vals
            && self_ignore_case == other_ignore_case
    }
}

impl Eq for MatchedArg {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grouped_vals_first() {
        let mut m = MatchedArg::new();
        m.new_val_group();
        m.new_val_group();
        m.append_val(AnyValue::new(String::from("bbb")), "bbb".into());
        m.append_val(AnyValue::new(String::from("ccc")), "ccc".into());
        assert_eq!(m.first_raw(), Some(&OsString::from("bbb")));
    }

    #[test]
    fn test_grouped_vals_push_and_append() {
        let mut m = MatchedArg::new();
        m.push_val(AnyValue::new(String::from("aaa")), "aaa".into());
        m.new_val_group();
        m.append_val(AnyValue::new(String::from("bbb")), "bbb".into());
        m.append_val(AnyValue::new(String::from("ccc")), "ccc".into());
        m.new_val_group();
        m.append_val(AnyValue::new(String::from("ddd")), "ddd".into());
        m.push_val(AnyValue::new(String::from("eee")), "eee".into());
        m.new_val_group();
        m.append_val(AnyValue::new(String::from("fff")), "fff".into());
        m.append_val(AnyValue::new(String::from("ggg")), "ggg".into());
        m.append_val(AnyValue::new(String::from("hhh")), "hhh".into());
        m.append_val(AnyValue::new(String::from("iii")), "iii".into());

        let vals: Vec<&Vec<OsString>> = m.raw_vals().collect();
        assert_eq!(
            vals,
            vec![
                &vec![OsString::from("aaa")],
                &vec![OsString::from("bbb"), OsString::from("ccc"),],
                &vec![OsString::from("ddd")],
                &vec![OsString::from("eee")],
                &vec![
                    OsString::from("fff"),
                    OsString::from("ggg"),
                    OsString::from("hhh"),
                    OsString::from("iii"),
                ]
            ]
        )
    }
}
