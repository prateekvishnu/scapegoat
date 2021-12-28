#![no_main]
#![feature(map_first_last)]

use std::collections::BTreeSet;
use std::fmt::Debug;
use std::iter::FromIterator;

use libfuzzer_sys::{arbitrary::Arbitrary, fuzz_target};

use scapegoat::SgSet;

const CAPACITY: usize = 2048;

#[derive(Arbitrary, Debug)]
enum SetMethod<T: Ord + Debug> {
    // APIs ------------------------------------------------------------------------------------------------------------
    Append { other: Vec<T> },
    // capacity() returns a constant. Omitted, irrelevant coverage.
    Clear,
    Contains { value: T },
    Difference { other: Vec<T> },
    First,
    Get { value: T },
    Insert { value: T },
    Intersection { other: Vec<T> },
    IsDisjoint { other: Vec<T> },
    IsEmpty,
    IsSubset { other: Vec<T> },
    IsSuperset { other: Vec<T> },
    Iter,
    Last,
    Len,
    New,
    PopFirst,
    PopLast,
    Remove { value: T },
    Replace { value: T },
    Retain { rand_value: T },
    SplitOff { value: T },
    SymmetricDifference { other: Vec<T> },
    Take { value: T },
    Union { other: Vec<T> },
    // Trait Equivalence -----------------------------------------------------------------------------------------------
    Clone,
    Debug,
    Extend { other: Vec<T> },
    Ord { other: Vec<T> },
}

// Harness Helpers -----------------------------------------------------------------------------------------------------

fn checked_get_len<T: Ord + Default, const N: usize>(
    sg_set: &SgSet<T, N>,
    bt_set: &BTreeSet<T>,
) -> usize {
    let len = sg_set.len();
    assert_eq!(len, bt_set.len());

    len
}

fn assert_len_unchanged<T: Ord + Default, const N: usize>(
    sg_set: &SgSet<T, N>,
    bt_set: &BTreeSet<T>,
    old_len: usize,
) {
    assert_eq!(checked_get_len(&sg_set, &bt_set), old_len);
}

// Harness -------------------------------------------------------------------------------------------------------------

// Differential fuzzing harness
fuzz_target!(|methods: Vec<SetMethod<usize>>| {
    let mut sg_set = SgSet::new(); // Data structure under test
    let mut bt_set = BTreeSet::new(); // Reference data structure

    for m in methods {
        match m {
            // API Equivalence -----------------------------------------------------------------------------------------
            SetMethod::Append { other } => {
                if other.len() > CAPACITY {
                    continue;
                }

                let mut sg_other = SgSet::from_iter(other.clone());
                let mut bt_other = BTreeSet::from_iter(other);
                let len_old = checked_get_len(&sg_set, &bt_set);

                assert_eq!(sg_other.len(), bt_other.len());
                if (len_old + sg_other.len()) <= CAPACITY {
                    sg_set.append(&mut sg_other);
                    bt_set.append(&mut bt_other);

                    assert!(sg_other.is_empty());
                    assert!(bt_other.is_empty());

                    assert!(checked_get_len(&sg_set, &bt_set) >= len_old);
                }
            }
            SetMethod::Clear => {
                sg_set.clear();
                bt_set.clear();

                assert!(sg_set.is_empty());
                assert!(bt_set.is_empty());

                assert_eq!(sg_set.len(), 0);
                assert_eq!(bt_set.len(), 0);
            }
            SetMethod::Contains { value } => {
                assert_eq!(sg_set.contains(&value), bt_set.contains(&value));
            }
            SetMethod::Difference { other } => {
                if other.len() > CAPACITY {
                    continue;
                }

                let sg_diff: Vec<_> = sg_set
                    .difference(&SgSet::from_iter(other.clone()))
                    .cloned()
                    .collect();

                let bt_diff: Vec<_> = bt_set
                    .difference(&BTreeSet::from_iter(other))
                    .cloned()
                    .collect();

                assert_eq!(sg_diff, bt_diff);
                assert!(sg_diff.len() <= sg_set.len());
            }
            SetMethod::First => {
                let len_old = checked_get_len(&sg_set, &bt_set);

                assert_eq!(sg_set.first(), bt_set.first());

                assert_len_unchanged(&sg_set, &bt_set, len_old);
            }
            SetMethod::Get { value } => {
                let len_old = checked_get_len(&sg_set, &bt_set);

                assert_eq!(sg_set.get(&value), bt_set.get(&value));

                assert_len_unchanged(&sg_set, &bt_set, len_old);
            }
            SetMethod::Insert { value } => {
                let len_old = checked_get_len(&sg_set, &bt_set);
                if len_old < CAPACITY {
                    assert_eq!(sg_set.insert(value), bt_set.insert(value));

                    assert!(checked_get_len(&sg_set, &bt_set) >= len_old);
                }
            }
            SetMethod::Intersection { other } => {
                if other.len() > CAPACITY {
                    continue;
                }

                let sg_inter: Vec<_> = sg_set
                    .intersection(&SgSet::from_iter(other.clone()))
                    .cloned()
                    .collect();

                let bt_inter: Vec<_> = bt_set
                    .intersection(&BTreeSet::from_iter(other))
                    .cloned()
                    .collect();

                assert_eq!(sg_inter, bt_inter);
                assert!(sg_inter.len() <= sg_set.len());
            }
            SetMethod::IsDisjoint { other } => {
                if other.len() > CAPACITY {
                    continue;
                }

                assert_eq!(
                    sg_set.is_disjoint(&SgSet::from_iter(other.clone())),
                    bt_set.is_disjoint(&BTreeSet::from_iter(other))
                );
            }
            SetMethod::IsEmpty => {
                assert_eq!(sg_set.is_empty(), bt_set.is_empty(),);
            }
            SetMethod::IsSubset { other } => {
                if other.len() > CAPACITY {
                    continue;
                }

                assert_eq!(
                    sg_set.is_subset(&SgSet::from_iter(other.clone())),
                    bt_set.is_subset(&BTreeSet::from_iter(other))
                );
            }
            SetMethod::Iter => {
                assert!(sg_set.iter().eq(bt_set.iter()));
            }
            SetMethod::IsSuperset { other } => {
                if other.len() > CAPACITY {
                    continue;
                }

                assert_eq!(
                    sg_set.is_superset(&SgSet::from_iter(other.clone())),
                    bt_set.is_superset(&BTreeSet::from_iter(other))
                );
            }
            SetMethod::Last => {
                let len_old = checked_get_len(&sg_set, &bt_set);

                assert_eq!(sg_set.last(), bt_set.last());

                assert_len_unchanged(&sg_set, &bt_set, len_old);
            }
            SetMethod::Len => {
                assert_eq!(sg_set.len(), bt_set.len());
            }
            SetMethod::New => {
                sg_set = SgSet::new();
                bt_set = BTreeSet::new();
            }
            SetMethod::PopFirst => {
                let len_old = checked_get_len(&sg_set, &bt_set);

                assert_eq!(sg_set.pop_first(), bt_set.pop_first());

                assert!(checked_get_len(&sg_set, &bt_set) <= len_old);
            }
            SetMethod::PopLast => {
                let len_old = checked_get_len(&sg_set, &bt_set);

                assert_eq!(sg_set.pop_last(), bt_set.pop_last());

                assert!(checked_get_len(&sg_set, &bt_set) <= len_old);
            }
            SetMethod::Remove { value } => {
                let len_old = checked_get_len(&sg_set, &bt_set);

                assert_eq!(sg_set.remove(&value), bt_set.remove(&value));

                assert!(checked_get_len(&sg_set, &bt_set) <= len_old);
            }
            SetMethod::Replace { value } => {
                let len_old = checked_get_len(&sg_set, &bt_set);
                if len_old < CAPACITY {
                    assert_eq!(sg_set.replace(value), bt_set.replace(value));

                    assert!(checked_get_len(&sg_set, &bt_set) >= len_old);
                }
            }
            SetMethod::Retain { rand_value } => {
                let len_old = checked_get_len(&sg_set, &bt_set);

                sg_set.retain(|&k| (k.wrapping_add(rand_value) % 2 == 0));
                bt_set.retain(|&k| (k.wrapping_add(rand_value) % 2 == 0));

                assert!(sg_set.iter().eq(bt_set.iter()));
                assert!(checked_get_len(&sg_set, &bt_set) <= len_old);
            }
            SetMethod::SplitOff { value } => {
                let len_old = checked_get_len(&sg_set, &bt_set);

                assert!(sg_set
                    .split_off(&value)
                    .iter()
                    .eq(bt_set.split_off(&value).iter()));

                assert!(sg_set.iter().eq(bt_set.iter()));
                assert!(checked_get_len(&sg_set, &bt_set) <= len_old);
            }
            SetMethod::SymmetricDifference { other } => {
                if other.len() > CAPACITY {
                    continue;
                }

                let sg_sym_diff: Vec<_> = sg_set
                    .symmetric_difference(&SgSet::from_iter(other.clone()))
                    .cloned()
                    .collect();

                let bt_sym_diff: Vec<_> = bt_set
                    .symmetric_difference(&BTreeSet::from_iter(other))
                    .cloned()
                    .collect();

                assert_eq!(sg_sym_diff, bt_sym_diff);
            }
            SetMethod::Take { value } => {
                let len_old = checked_get_len(&sg_set, &bt_set);

                assert_eq!(sg_set.take(&value), bt_set.take(&value));

                assert!(checked_get_len(&sg_set, &bt_set) <= len_old);
            }
            SetMethod::Union { other } => {
                if other.len() > CAPACITY {
                    continue;
                }

                let sg_union: Vec<_> = sg_set
                    .union(&SgSet::from_iter(other.clone()))
                    .cloned()
                    .collect();

                let bt_union: Vec<_> = bt_set.union(&BTreeSet::from_iter(other)).cloned().collect();

                assert_eq!(sg_union, bt_union);
                assert!(sg_union.len() >= sg_set.len());
            }
            // Trait Equivalence ---------------------------------------------------------------------------------------
            SetMethod::Clone => {
                assert!(sg_set.clone().iter().eq(bt_set.clone().iter()));
            }
            SetMethod::Debug => {
                assert_eq!(format!("{:?}", sg_set), format!("{:?}", bt_set),);
            }
            SetMethod::Extend { other } => {
                let len_old = checked_get_len(&sg_set, &bt_set);

                if (len_old + other.len()) <= CAPACITY {
                    sg_set.extend(other.clone().into_iter());
                    bt_set.extend(other.into_iter());

                    assert!(sg_set.iter().eq(bt_set.iter()));
                    assert!(checked_get_len(&sg_set, &bt_set) >= len_old);
                }
            }
            SetMethod::Ord { other } => {
                if other.len() > CAPACITY {
                    continue;
                }

                let sg_set_new = SgSet::<_, CAPACITY>::from_iter(other.clone().into_iter());
                let bt_set_new = BTreeSet::from_iter(other.into_iter());

                assert_eq!(sg_set.cmp(&sg_set_new), bt_set.cmp(&bt_set_new),);
            }
        }
    }
});
