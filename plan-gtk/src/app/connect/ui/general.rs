use super::helpers::cleaned_input;
use crate::app::ui::Ui;

use gtk::prelude::*;

impl Ui {
    pub fn event_clean_entry_content<T>(&self, entry: T)
    where
        T: IsA<gtk::Buildable> + IsA<gtk::Entry>,
    {
        with_blocked_signals!(
            self,
            {
                entry.set_text(&cleaned_input(entry.get_text()));
            },
            entry
        );
    }
}
