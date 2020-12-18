use gtk::prelude::*;
use std::convert::TryFrom;

use plan_backend::data::clean_string;

/// Gets the selected string from the tree view.
pub fn get_selection_from_treeview(treeview: gtk::TreeView) -> Option<String> {
    let selection = treeview.get_selection().get_selected();
    if selection.is_none() {
        None
    } else {
        let (model, iter) = selection.expect("We treated the None case above");
        let column = 0;
        let value = model.get_value(&iter, column);
        Some(
            value
                .get::<&str>()
                .expect("Value in list store should be gchararray")
                .expect("Value in list store should be gchararray")
                .to_owned(),
        )
    }
}

/// Gets a selection tree path from a selection index, a list model and a value to search in the
/// model.
pub fn tree_path_from_selection_index(
    selection_index: Option<i32>,
    model: gtk::ListStore,
    look_for_value_in_model: Option<&String>,
) -> gtk::TreePath {
    let selection_index = selection_index.or_else(|| {
        // No row was given. Find the row containing the current entity.
        if let Some(value) = look_for_value_in_model {
            index_of_row_containing(model, value)
        } else {
            None
        }
    });

    match selection_index {
        Some(index) => gtk::TreePath::from_indicesv(&[index]),
        None => gtk::TreePath::new(),
    }
}

/// Returns the index of the first row containing the given value.
fn index_of_row_containing(model: gtk::ListStore, value: &String) -> Option<i32> {
    let iter = model.get_iter_first();
    if iter.is_none() {
        return None;
    }
    let iter = iter.expect("None case treated above");

    let mut index = 0;
    loop {
        let value_model = model.get_value(&iter, 0);
        let value_model2 = value_model.get::<String>();
        if value_model2.unwrap().unwrap() == *value {
            return Some(index);
        }
        if model.iter_next(&iter) == false {
            return None;
        }
        index += 1;
    }
}

/// Returns the cleaned version of the input in an entry.
pub fn cleaned_input<S>(input: S) -> String
where
    S: Into<String>,
{
    let input = input.into();
    if let Ok(clean_input) = clean_string(&input) {
        if clean_input == input.trim() {
            input
        } else {
            clean_input
        }
    } else {
        input
    }
}

/// Given a collection minus a removed element and the position of the last removed element,
/// returns the next element and its position.
///
/// If there is no next element, returns None.
pub fn get_next_element<T>(
    position_of_removed_element: usize,
    collection: Vec<&T>,
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
