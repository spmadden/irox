use crate::tile::{TileAddress, TileData};
use std::fmt::Debug;

pub enum DownloadStatus {
    TileComplete(TileAddress),
    TileDataAvailable(TileData),
    ZoomLevelComplete(u8),
    ZoomLevelStarted(u8, u64),
    Done,
}

impl Debug for DownloadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TileComplete(arg0) => f.debug_tuple("TileComplete").field(arg0).finish(),
            Self::TileDataAvailable(arg0) => {
                f.debug_tuple("TileDataAvailable").field(arg0).finish()
            }
            Self::ZoomLevelComplete(arg0) => {
                f.debug_tuple("ZoomLevelComplete").field(arg0).finish()
            }
            Self::ZoomLevelStarted(arg0, arg1) => f
                .debug_tuple("ZoomLevelStarted")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::Done => write!(f, "Done"),
        }
    }
}
