use std::time::{SystemTime, UNIX_EPOCH};

pub struct Worker {
    pub id: u64,
    pub machine_id: u16,
    pub last_timestamp: u128,
}
impl Worker {
    pub fn new(machine_id: u16) -> Worker {
        let id = 0;
        let last_timestamp = Worker::get_current_timestamp_in_millis();

        Worker {
            id,
            last_timestamp,
            machine_id,
        }
    }

    pub fn next_id_and_timestamp(&mut self) -> (u64, u128) {
        let now = Worker::get_current_timestamp_in_millis();
        let sequence = self.next_id(now);
        (sequence, now)
    }

    fn duration_in_millis(ts: SystemTime) -> u128 {
        ts.duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }

    fn get_current_timestamp_in_millis() -> u128 {
        let ts = SystemTime::now();
        Worker::duration_in_millis(ts)
    }
    fn next_id(&mut self, now: u128) -> u64 {
        let sequence: u64 = if self.last_timestamp == now {
            let current = self.id;
            self.id += 1;
            current
        } else {
            self.id = 1;
            self.last_timestamp = now;
            0
        };
        sequence
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_increments_if_no_time_has_passed() {
        let mut worker = Worker::new(0);
        for i in 0..10_000 {
            assert_eq!(worker.next_id(1), i)
        }
    }

    #[test]
    fn it_resets_if_time_has_passed() {
        let mut worker = Worker::new(0);
        assert_eq!(worker.next_id(1), 0);
        assert_eq!(worker.next_id(1), 1);
        assert_eq!(worker.next_id(2), 0);
        assert_eq!(worker.next_id(2), 1);
        assert_eq!(worker.next_id(2), 2);
        assert_eq!(worker.next_id(2), 3);
        assert_eq!(worker.next_id(3), 0);
    }
}
