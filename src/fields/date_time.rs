use chrono::{
    prelude::*,
    Duration,
};

pub type DateTime = chrono::DateTime<Utc>;
pub type LocalDateTime = chrono::DateTime<Local>;

// Dates
lazy_static! {
    static ref UTC_BASIS: DateTime = Utc.ymd(1989, 12, 31).and_hms(0, 0, 0);
    static ref LOCAL_BASIS: LocalDateTime = Local.ymd(1989, 12, 31).and_hms(0, 0, 0);
}

from_impl!(DateTime, UnsignedInt32 => |v|
    if v < 0x10000000 {
        // TODO time since system boot, not UTC_BASIS
        *UTC_BASIS + Duration::seconds(v as i64)
    } else {
        *UTC_BASIS + Duration::seconds(v as i64)
    }
);

from_impl!(LocalDateTime, UnsignedInt32 => |v|
    if v < 0x10000000 {
        // TODO time since system boot, not LOCAL_BASIS
        *LOCAL_BASIS + Duration::seconds(v as i64)
    } else {
        *LOCAL_BASIS + Duration::seconds(v as i64)
    }
);
