use ckb_types::{packed, prelude::*};
use afl::fuzz;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(msg) = packed::SyncMessage::from_slice(data) {
            msg.to_enum();
        }

        if let Ok(msg) = packed::RelayMessage::from_slice(&data) {
            msg.to_enum();
        }

        let _ = packed::CellbaseWitness::from_slice(&data);

        let _ = packed::IdentifyReader::from_slice(&data);

        let _ = packed::TimeReader::from_slice(&data);
    });
}
