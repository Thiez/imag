use std::error::Error;

use notification::ErrorNotification;

pub trait NotifyResult<T, E: Error> {

    fn notify_err(self, ErrorNotification) -> Result<T, E>;

    fn notify_on_err<F: FnOnce() -> ErrorNotification>(self, f: F) -> Result<T, E>;

}

impl<T, E: Error> NotifyResult<T, E> for Result<T, E> {

    fn notify_err(self, en: ErrorNotification) -> Result<T, E> {
        self.map_err(|e| { en.notify(&e); e })
    }

    fn notify_on_err<F: FnOnce() -> ErrorNotification>(self, f: F) -> Result<T, E> {
        self.map_err(|e| { f().notify(&e); e })
    }

}


