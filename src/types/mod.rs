mod request;
mod response;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct BasicAuth {
    pub user: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Encryption {
    Required,
    Preferred,
    Tolerated,
}

use serde::{Deserialize, Serialize};

pub(crate) use self::request::RpcRequest;
pub use self::request::{
    ArgumentFields, Id, SessionSetArgs, TorrentAction, TorrentAddArgs, TorrentGetField,
    TorrentRenamePathArgs, TorrentSetArgs, TrackerList,
};

pub(crate) use self::response::RpcResponseArgument;
pub use self::response::{
    BlocklistUpdate, FreeSpace, Nothing, PortTest, RpcResponse, SessionClose, SessionGet,
    SessionStats, Torrent, TorrentAddedOrDuplicate, TorrentRenamePath, TorrentStatus, Torrents,
};
