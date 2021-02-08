use crate::config;
use gettextrs::gettext as tr;
use notify_rust::Notification;

pub fn notify_err(error: Box<dyn std::error::Error>) {
    toast_notify(tr("Uh-Oh..."), error.to_string());
}

fn toast_notify<S1, S2>(summary: S1, body: S2)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    // TODO - icon (auto_icon ?)
    if let Err(_e) = Notification::new()
        .summary(summary.as_ref())
        .body(body.as_ref())
        .icon(config::APP_ICON)
        .timeout(config::NOTIFICATION_TIMEOUT_MS)
        .show()
    {
        // TODO - inform user with normal dialog if notifications do not work
    }
}