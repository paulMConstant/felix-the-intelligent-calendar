use crate::structs::{
    autoinsertion::{AutoinsertionThreadHandle, Tree},
    ActivityBeginningMinutes, ActivityComputationStaticData,
};

pub fn autoinsert(
    static_data: &[ActivityComputationStaticData],
    current_insertions: &[ActivityBeginningMinutes],
) -> AutoinsertionThreadHandle {
    Tree::new(static_data, current_insertions)
}
