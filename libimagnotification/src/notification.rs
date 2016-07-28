use std::error::Error;
use std::default::Default;

use notify_rust::Notification;
use notify_rust::NotificationUrgency;

pub struct ErrorNotification {
    trace_depth: usize,
    timeout: i32,
    urgency: NotificationUrgency,
}

impl ErrorNotification {

    /// Create a new ErrorNotification
    ///
    /// trace depth will be 1 by default
    /// timeout will be set by the server by default
    pub fn new() -> ErrorNotification {
        ErrorNotification::default()
    }

    /// Set the trace depth. 0 will trace all the way to the last error
    pub fn with_trace_depth(mut self, u: usize) -> ErrorNotification {
        self.trace_depth = u;
        self
    }

    /// Set the timeout, see the documentation for the `notify-rust` crate for details
    pub fn with_timeout(mut self, i: i32) -> ErrorNotification {
        self.timeout = i;
        self
    }

    pub fn notify(&self, e: &Error) {
        self.trace_notify(e, self.trace_depth)
    }

    fn trace_notify(&self, e: &Error, u: usize) {
        let mut n = Notification::new();
        n.appname("imag");
        n.summary("[Error]");
        n.urgency(self.urgency);
        n.body(e.description());
        n.finalize().show().ok(); // Ignoring error here

        if e.cause().is_some() && u > 0 {
            self.trace_notify(e.cause().unwrap(), u - 1);
        }
    }

}

impl Default for ErrorNotification {

    fn default() -> ErrorNotification {
        ErrorNotification {
            trace_depth: 1,
            timeout: -1,
            urgency: NotificationUrgency::Normal
        }
    }

}
