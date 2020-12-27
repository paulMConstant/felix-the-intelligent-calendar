use plan_backend::data::clean_string;

use glib::clone;
use gtk::prelude::*;

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

/// Formats a spin button to display a time with 2 digits.
pub fn format_time_spin_button(spin: &gtk::SpinButton) {
    spin.connect_output(
        clone!(@weak spin => @default-return gtk::Inhibit(true), move |_| {
        let adjustment = spin.get_adjustment();
        spin.set_text(&format!("{:#02}", adjustment.get_value()));
        gtk::Inhibit(true)
        }),
    );
}
