use crate::config;
use gettextrs::gettext as tr;
use notify_rust::Notification;

/// If the given expression fails, notifies the user and returns.
/// Else, assigns the variable with given name to the result.
macro_rules! assign_or_return {
    ($var: ident, $expr: expr) => {
        let res = $expr;
        if let Err(e) = res {
            use crate::app::notify::notify_err;
            notify_err(e);
            return;
        }
        let $var = res.expect("Error case should have been taken care of in macro above");
    };
}

/// If the given expression fails, notifies the user.
macro_rules! notify_if_err {
    ($expr: expr) => {
        use crate::app::notify::notify_err;
        if let Err(e) = $expr {
            notify_err(e)
        }
    };
}

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
