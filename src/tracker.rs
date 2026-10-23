const CONNECT: &str = "CLIENT CONNECTED";
const DISCONNECTED: &str = "CLIENT DISCONNECTED";

#[derive(Default)]
pub struct Info {
    last_known_count: Option<i64>,
}

impl Info {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_line(&mut self, line: &str) {
        // check if we have some exact count
        if let Some(count) = extract_session_count(line) {
            println!(": parsed sessions: count={count}");
            self.last_known_count = Some(count);
        }

        if line.contains(CONNECT) {
            // show message with exact or approximate count
            let count = self.last_known_count.unwrap_or(1);

            crate::notifier::send_connect_alert(count);
            println!(": connect event: count={count}");
        } else if line.contains(DISCONNECTED) {
            // show message with approximate count, but stop at 0
            let mut count = self.last_known_count.unwrap_or(0);
            if count > 0 {
                count -= 1;
            }

            crate::notifier::send_disconnect_alert(count);
            self.last_known_count = Some(count);
            println!(": disconnect event: count={count}");
        }
    }
}

const SESSION_MARKER: &str = "active sessions:";

fn extract_session_count(line: &str) -> Option<i64> {
    let idx = line.find(SESSION_MARKER)?;
    let remainder = &line[idx + SESSION_MARKER.len()..];

    let num_str: String = remainder
        .trim_start()
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();

    num_str.parse::<i64>().ok()
}
