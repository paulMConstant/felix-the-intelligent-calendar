use gtk::prelude::*;

/// Gets the selected string from the tree view.
pub fn get_selection_from_treeview(treeview: &gtk::TreeView, column: i32) -> Option<String> {
    let selection = treeview.get_selection().get_selected();
    if selection.is_none() {
        None
    } else {
        let (model, iter) = selection.expect("We treated the None case above");
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
    look_for_value_in_model: Option<String>,
) -> gtk::TreePath {
    let selection_index = selection_index.or_else(|| {
        // No row was given. Find the row containing the current entity.
        if let Some(value) = look_for_value_in_model {
            index_of_row_containing(model, &value)
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
fn index_of_row_containing(model: gtk::ListStore, value: &str) -> Option<i32> {
    let iter = model.get_iter_first();
    iter.as_ref()?;
    // Equivalen to
    //if iter.is_none() {
    //return None;
    //}

    let iter = iter.expect("None case treated above");

    let mut index = 0;
    loop {
        let value_model = model.get_value(&iter, 0);
        let value_model2 = value_model.get::<String>();
        if value_model2.unwrap().unwrap() == *value {
            return Some(index);
        }
        if !model.iter_next(&iter) {
            return None;
        }
        index += 1;
    }
}
