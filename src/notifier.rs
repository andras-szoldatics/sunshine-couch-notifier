use notify_rust::{Notification, Urgency};

pub fn send_connect_alert(session_count: i64) {
    let body = body_with_count(session_count);

    let _ = Notification::new()
        .summary("Gamer on Couch - Connected")
        .body(&body)
        .urgency(Urgency::Normal)
        .icon("input-gaming")
        .show();
}

pub fn send_disconnect_alert(session_count: i64) {
    let body = body_with_count(session_count);

    let _ = Notification::new()
        .summary("Gamer on Couch - Disconnected")
        .body(&body)
        .urgency(Urgency::Normal)
        .icon("input-gaming")
        .show();
}

fn body_with_count(session_count: i64) -> String {
    format!("active session count: {session_count}")
}
