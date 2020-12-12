use crate::config;
use gettextrs::gettext as tr;
use notify_rust::Notification;

pub fn notify_err(error: Box<dyn std::error::Error>) {
    toast_notify(tr("Uh-Oh..."), error.to_string(), 3000);
}

pub fn toast_notify<S1, S2>(summary: S1, body: S2, timeout_ms: i32)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    // TODO - icon (auto_icon ?)
    if let Err(_e) = Notification::new()
        .summary(summary.as_ref())
        .body(body.as_ref())
        .icon(config::APP_ICON)
        .timeout(timeout_ms)
        .show()
    {
        // TODO - inform user with normal dialog if notifications do not work
    }
}
