use crate::error::Error;
pub(crate) use byteorder::{LittleEndian, ReadBytesExt};

/// Tries to read X bytes from the cursor, if reading fails, captures position nicely.
macro_rules! try_read {
    ($cursor: ident, u8) => {
        $cursor
            .read_u8()
            .map_err(|e| Error::io(e, $cursor.position()))?;
    };

    ($cursor: ident, i8) => {
        $cursor
            .read_i8()
            .map_err(|e| Error::io(e, $cursor.position()))?;
    };

    ($cursor: ident, u16) => {
        $cursor
            .read_u16::<byteorder::LittleEndian>()
            .map_err(|e| Error::io(e, $cursor.position()))?;
    };

    ($cursor: ident, i16) => {
        $cursor
            .read_i16::<byteorder::LittleEndian>()
            .map_err(|e| Error::io(e, $cursor.position()))?;
    };

    ($cursor: ident, i32) => {
        $cursor
            .read_i32::<byteorder::LittleEndian>()
            .map_err(|e| Error::io(e, $cursor.position()))?;
    };

    ($cursor: ident, u32) => {
        $cursor
            .read_u32::<byteorder::LittleEndian>()
            .map_err(|e| Error::io(e, $cursor.position()))?;
    };

    ($cursor: ident, i64) => {
        $cursor
            .read_i64::<byteorder::LittleEndian>()
            .map_err(|e| Error::io(e, $cursor.position()))?;
    };

    ($cursor: ident, u64) => {
        $cursor
            .read_u64::<byteorder::LittleEndian>()
            .map_err(|e| Error::io(e, $cursor.position()))?;
    };
}
