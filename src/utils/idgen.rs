use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

struct IDGenConfig {
    machine_id_mask: u64,
    timestamp_mask: u64,
    timestamp_shift: u8,
    // Max seq number size is 64 - timestamp bits - machine id bits
    // Max seq number is 2^(max key size) - 1
    // Maximum seq number size is 64 - 41 - 1 = 22, so maximum sequence number is 4194303
    // Making it u64 to avoid conversion at comparison/BitOr, and also now it can be used as seq no mask
    max_seq_no: u64,
}

struct IDGenState {
    current_seq_no: u64,
    since: u64,
}

pub struct IDGen {
    config: IDGenConfig,
    state: Mutex<IDGenState>,
}

impl IDGen {
    pub fn new(machine_id: u8) -> Self {
        IDGen::new_with_config(machine_id, 8, 41)
    }

    pub fn new_with_config(machine_id: u8, machine_id_bits: u8, timestamp_bits: u8) -> Self {
        let config = IDGenConfig::new(machine_id, machine_id_bits, timestamp_bits);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        IDGen {
            config,
            state: Mutex::new(IDGenState {
                current_seq_no: 0,
                since: now,
            }),
        }
    }

    pub fn new_id(&self) -> i64 {
        let mut now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let mut state = self.state.lock().unwrap();

        // As system time *may* go backwards, forcefully synchronizing to avoid potentially duplicate ids
        if state.since > now {
            std::thread::sleep(Duration::new(0, ((state.since - now) as u32) * 1000));
            now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
        }

        if state.since == now {
            state.current_seq_no = (state.current_seq_no + 1) & self.config.max_seq_no;

            let hundred_micros = Duration::new(0, 100000);
            if state.current_seq_no == 0 {
                while state.since == now {
                    // Sleep for hundred microseconds until timestamp changes
                    std::thread::sleep(hundred_micros);
                    now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                }
                state.since = now;
                state.current_seq_no = 0
            }
        } else {
            state.since = now;
            state.current_seq_no = 0
        }

       let id= ((self.config.timestamp_mask & state.since) << self.config.timestamp_shift)
            | self.config.machine_id_mask
            | state.current_seq_no;
        id as i64
    }
}

impl IDGenConfig {
    fn new(machine_id: u8, machine_id_bits: u8, timestamp_bits: u8) -> Self {
        assert!(0 < machine_id_bits && machine_id_bits <= 8);
        assert!(machine_id < ((1 << machine_id_bits) as u16 - 1) as u8);
        assert!(41 <= timestamp_bits && timestamp_bits <= 43);
        let max_seq_bits = 64 - timestamp_bits - machine_id_bits;
        IDGenConfig {
            machine_id_mask: (machine_id as u64) << (64 - machine_id_bits),
            timestamp_mask: ((1 as u64) << timestamp_bits) - 1,
            timestamp_shift: 64 - timestamp_bits - machine_id_bits,
            max_seq_no: ((1 as u64) << max_seq_bits) - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IDGen;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::thread;
    use std::time::SystemTime;

    #[test]
    fn test_uniq() {
        let idgen = IDGen::new(128);

        let start = SystemTime::now();
        let range: Vec<u64> = (0..1000000).map(|_i| idgen.new_id()).collect();
        let mut uniq = HashSet::new();
        let all_ids_unique = range.into_iter().all(|id| uniq.insert(id));
        assert!(all_ids_unique);
        let stop = SystemTime::now();
        println!("{}", stop.duration_since(start).unwrap().as_millis());
    }

    #[test]
    fn test_threads() {
        let idgen = Arc::new(IDGen::new(128));
        let mut handles = vec![];

        for _ in 0..8 {
            let idgen = Arc::clone(&idgen);
            let handle = thread::spawn(move || {
                let _range: Vec<u64> = (0..1000000).map(|_i| idgen.new_id()).collect();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}