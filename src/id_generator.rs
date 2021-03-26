// Epoch timestamp in millisecond precision - 41 bits (gives us 69 years with a custom epoch)
// Configured machine id - 10 bits (gives us up to 1024 machines)
// Sequence number - 12 bits (A local counter per machine that rolls over every 4096)

#[derive(Debug, Eq, PartialEq)]
pub struct Id(u64);
const SEQUENCE_BITS: usize = 12;
const MACHINE_BITS: usize = 10;
const TIMESTAMP_SHIFT: usize = MACHINE_BITS + SEQUENCE_BITS;
const MACHINE_SHIFT: usize = SEQUENCE_BITS;
const MACHINE_MASK: u64 = 0x3FF; //10 bits
const SEQUENCE_MASK: u64 = 0xFFF; //12 bits

// const TIMESTAMP_BITS: usize = 41;
// const TIMESTAMP_MASK: u64 = 0xFFF; //12 bits
impl Id {
    pub fn new(epoch_ts: u64, machine_id: u16, sequence_number: u16) -> Id {
        let mut id = epoch_ts << TIMESTAMP_SHIFT;
        id |= (machine_id as u64 & MACHINE_MASK) << MACHINE_SHIFT;
        id |= sequence_number as u64 & SEQUENCE_MASK;
        Id(id)
    }

    fn to_parts(&self) -> (u64, u16, u16) {
        let ts = self.0 >> TIMESTAMP_SHIFT;
        let machine = (self.0 >> MACHINE_SHIFT) & MACHINE_MASK;
        let sequence = self.0 & SEQUENCE_MASK;
        (ts, machine as u16, sequence as u16)
    }
    pub fn as_string_output(&self) -> String {
        self.0.to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_is_empty_if_all_numbers_are_empty() {
        assert_eq!(Id::new(0, 0, 0), Id(0));
    }
    #[test]
    fn it_is_packed_if_all_constituent_parts_are_packed() {
        assert_eq!(Id::new(u64::MAX, u16::MAX, u16::MAX), Id(u64::MAX));
    }

    #[test]
    fn it_handles_other_cases() {
        assert_eq!(Id::new(0, 0, 1), Id(1));
        assert_eq!(Id::new(0, 1, 0), Id(1 << 12));
        assert_eq!(Id::new(1, 0, 0), Id(1 << 22));
    }

    #[test]
    fn it_doesnt_throw_away_information() {
        assert_eq!(Id::new(0, 0, 1).to_parts(), (0, 0, 1));
        assert_eq!(Id::new(0, 1, 0).to_parts(), (0, 1, 0));
        assert_eq!(Id::new(1, 0, 0).to_parts(), (1, 0, 0));
        assert_eq!(
            Id::new(u64::MAX, u16::MAX, u16::MAX).to_parts(),
            (u64::MAX >> 22, u16::MAX & 0x3FF, u16::MAX & 0xFFF)
        );
    }
}
