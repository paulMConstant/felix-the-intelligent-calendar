use std::convert::TryFrom;

/// Given a collection minus a removed element and the position of the last removed element,
/// returns the next element and its position.
///
/// If there is no next element, returns None.
pub fn get_next_element<T>(
    position_of_removed_element: usize,
    collection: &Vec<T>,
) -> (Option<T>, Option<i32>)
where
    T: Clone,
{
    let len = collection.len();

    if len == 0 {
        // No entities left
        (None::<T>, None::<i32>)
    } else {
        let position_of_new_current_element = if len <= position_of_removed_element {
            // The removed element was the last. Show the previous one.
            position_of_removed_element - 1
        } else {
            // Show the next element
            position_of_removed_element
        };

        let new_current_element = Some(collection[position_of_new_current_element].clone());
        let position_of_next_element = i32::try_from(position_of_new_current_element)
            .expect("There should not be 2 billion elements, we should be safe");

        (new_current_element, Some(position_of_next_element))
    }
}
