use std::time::{SystemTime, UNIX_EPOCH};

/// Leap Indicator (LI): This is a two-bit code warning of an impending
/// leap second to be inserted/deleted in the last minute of the current
/// day.  This field is significant only in server messages.
///
/// On startup, servers set this field to 3 (clock not synchronized), and
/// set this field to some other value when synchronized to the primary
/// reference clock.  Once set to a value other than 3, the field is
/// never set to that value again, even if all synchronization sources
/// become unreachable or defective.
pub enum LeapIndicator {
    NoWarning,
    /// last minute has 61 seconds
    LongMinute,
    /// last minute has 59 seconds
    ShortMinute,
    /// clock not synchronized
    AlarmCondition,
}

/// This is a three-bit integer indicating the
/// NTP/SNTP version number, currently 4.  If necessary to distinguish
/// between IPv4, IPv6, and OSI, the encapsulating context must be
/// inspected.
pub struct VersionNumber(u8);

/// This is a three-bit number indicating the protocol mode.
///
///
/// In unicast and manycast modes, the client sets this field to 3
/// (client) in the request, and the server sets it to 4 (server) in the
/// reply.  In broadcast mode, the server sets this field to 5
/// (broadcast).  The other modes are not used by SNTP servers and
/// clients.
pub enum Mode {
    Reserved,
    SymetricActive,
    SymetricPassive,
    Client,
    Server,
    Broadcast,
    /// reserved for NTP control message
    NTPReserved,
    /// reserved for private use
    PrivateReserved,
}

/// This is an eight-bit unsigned integer indicating the
/// stratum.  This field is significant only in SNTP server messages.
pub enum Stratum {
    KissOfDeath,
    PrimaryReference,
    SecondaryReference(u8),
    Reserved(u8),
}

pub struct SendPacket {
    pub leap_indicator: LeapIndicator,
    pub version_number: VersionNumber,
    pub mode: Mode,
    pub stratum: Stratum,
    pub originate_timestamp: Option<u64>,
    pub recieve_timestamp: Option<u64>,
    pub transmit_timestamp: u64,
    // authenticator: u8
}

impl SendPacket {
    pub fn new() -> Self {
        SendPacket {
            leap_indicator: LeapIndicator::NoWarning,
            version_number: VersionNumber(4),
            mode: Mode::Client,
            stratum: Stratum::SecondaryReference(3),
            originate_timestamp: None,
            recieve_timestamp: None,
            transmit_timestamp: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Instant, SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn test_name() {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("{:#?}", timestamp.subsec_nanos());
    }
}
