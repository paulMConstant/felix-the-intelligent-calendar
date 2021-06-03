use gettextrs::gettext as tr;
use gtk::prelude::*;

use crate::app::App;

impl App {
    pub fn connect_export(&self) {
        self.connect_export_pdf();
    }

    fn connect_export_pdf(&self) {
        fetch_from!(
            self.ui.borrow(),
            pdf_export_button,
            main_window,
            export_popover
        );

        let ui = self.ui.clone();
        let data = self.data.clone();
        app_register_signal!(
            self,
            pdf_export_button,
            pdf_export_button.connect_clicked(move |_| {
                export_popover.hide();

                let dialog = gtk::FileChooserDialog::new(
                    Some(&tr("Select directory to store the pdfs")),
                    Some(&main_window),
                    gtk::FileChooserAction::SelectFolder,
                );
                dialog.add_buttons(&[
                    (&tr("Cancel"), gtk::ResponseType::Cancel),
                    (&tr("Select PDF directory"), gtk::ResponseType::Ok),
                ]);

                if dialog.run() == gtk::ResponseType::Ok {
                    if let Some(folder) = dialog.get_filename() {
                        data.borrow().export_as_pdf(folder.clone());
                        ui.borrow().notify_str(&tr(format!(
                            "PDFs generated in {}.",
                            folder.to_string_lossy()
                        )));
                    }
                }

                dialog.close();
            })
        );
    }
}
