use super::Encryption;
use serde::Deserialize;
use serde_repr::*;

#[derive(Deserialize, Debug)]
pub struct RpcResponse<T: RpcResponseArgument> {
    pub arguments: T,
    pub result: String,
}

impl<T: RpcResponseArgument> RpcResponse<T> {
    pub fn is_ok(&self) -> bool {
        self.result == "success"
    }
}
pub trait RpcResponseArgument {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct SessionGet {
    pub alt_speed_down: i64,
    pub alt_speed_enabled: bool,
    pub alt_speed_time_begin: i64,
    pub alt_speed_time_delay: i64,
    pub alt_speed_time_enabled: bool,
    pub alt_speed_time_end: i64,
    pub blocklist_enabled: bool,
    pub blocklist_size: usize,
    pub blocklist_url: String,
    pub cache_size_mb: isize,
    pub config_dir: String,
    pub default_trackers: Vec<String>,
    pub dht_enabled: bool,
    pub download_dir: String,
    pub download_queue_enabled: bool,
    pub download_queue_size: isize,
    pub encryption: Encryption,
    pub idle_seeding_limit_enabled: bool,
    pub idle_seeding_limit: isize,
    pub incomplete_dir_enabled: bool,
    pub incomplete_dir: String,
    pub lpd_enabled: bool,
    pub peer_limit_global: isize,
    pub peer_limit_per_torrent: isize,
    pub peer_port_random_on_start: bool,
    pub peer_port: u16,
    pub pex_enabled: bool,
    pub port_forwarding_enabled: bool,
    pub queue_stalled_enabled: bool,
    pub queue_stalled_minutes: isize,
    pub rename_partial_files: bool,
    pub rpc_version: i32,
    pub rpc_version_minimum: i32,
    pub rpc_version_semver: semver::Version,
    pub script_torrent_added_enabled: bool,
    pub script_torrent_added_filename: String,
    pub script_torrent_done_enabled: bool,
    pub script_torrent_done_filename: String,
    pub script_torrent_done_seeding_enabled: bool,
    pub script_torrent_done_seeding_filename: String,
    pub seed_queue_enabled: bool,
    pub seed_queue_size: isize,
    pub seed_ratio_limit: f64,
    pub seed_ratio_limited: bool,
    pub speed_limit_down: isize,
    pub speed_limit_down_enabled: bool,
    pub speed_limit_up: isize,
    pub speed_limit_up_enabled: bool,
    pub start_added_torrents: bool,
    pub trash_original_torrent_files: bool,
    pub utp_enabled: bool,
    pub version: String,
}

impl RpcResponseArgument for SessionGet {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionStats {
    pub torrent_count: i32,
    pub active_torrent_count: i32,
    pub paused_torrent_count: i32,
    pub download_speed: i64,
    pub upload_speed: i64,
    #[serde(rename = "current-stats")]
    pub current_stats: Stats,
    #[serde(rename = "cumulative-stats")]
    pub cumulative_stats: Stats,
}
impl RpcResponseArgument for SessionStats {}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionClose {}
impl RpcResponseArgument for SessionClose {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct BlocklistUpdate {
    pub blocklist_size: Option<i32>,
}
impl RpcResponseArgument for BlocklistUpdate {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct FreeSpace {
    pub path: String,
    pub size_bytes: i64,
}
impl RpcResponseArgument for FreeSpace {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PortTest {
    pub port_is_open: bool,
}
impl RpcResponseArgument for PortTest {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum TorrentStatus {
    Stopped = 0,
    QueuedToVerify = 1,
    Verifying = 2,
    QueuedToDownload = 3,
    Downloading = 4,
    QueuedToSeed = 5,
    Seeding = 6,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Torrent {
    pub activity_date: Option<i64>,
    pub added_date: Option<i64>,
    pub done_date: Option<i64>,
    pub download_dir: Option<String>,
    pub edit_date: Option<i64>,
    pub error: Option<i64>,
    pub error_string: Option<String>,
    pub eta: Option<i64>,
    pub id: Option<i64>,
    pub is_finished: Option<bool>,
    pub is_private: Option<bool>,
    pub is_stalled: Option<bool>,
    pub labels: Option<Vec<String>>,
    pub left_until_done: Option<i64>,
    pub metadata_percent_complete: Option<f32>,
    pub name: Option<String>,
    pub hash_string: Option<String>,
    pub peers_connected: Option<i64>,
    pub peers_getting_from_us: Option<i64>,
    pub peers_sending_to_us: Option<i64>,
    pub percent_done: Option<f32>,
    pub rate_download: Option<i64>,
    pub rate_upload: Option<i64>,
    pub recheck_progress: Option<f32>,
    pub seconds_seeding: Option<i64>,
    pub seed_ratio_limit: Option<f32>,
    pub size_when_done: Option<i64>,
    pub status: Option<TorrentStatus>,
    pub torrent_file: Option<String>,
    pub total_size: Option<i64>,
    pub trackers: Option<Vec<Trackers>>,
    pub upload_ratio: Option<f32>,
    pub uploaded_ever: Option<i64>,
    pub files: Option<Vec<File>>,
    /// for each file in files, whether or not they will be downloaded (0 or 1)
    pub wanted: Option<Vec<i8>>,
    /// for each file in files, their download priority (low:-1,normal:0,high:1)
    pub priorities: Option<Vec<i8>>,
    pub file_stats: Option<Vec<FileStat>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub files_added: i32,
    pub downloaded_bytes: i64,
    pub uploaded_bytes: i64,
    pub seconds_active: i64,
    pub session_count: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Torrents<T> {
    pub torrents: Vec<T>,
}
impl RpcResponseArgument for Torrents<Torrent> {}

#[derive(Deserialize, Debug, Clone)]
pub struct Trackers {
    pub id: i32,
    pub announce: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub length: i64,
    pub bytes_completed: i64,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileStat {
    pub bytes_completed: i64,
    pub wanted: bool,
    /// low: -1, normal: 0, high: 1
    pub priority: i8,
}

#[derive(Deserialize, Debug)]
pub struct Nothing {}
impl RpcResponseArgument for Nothing {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TorrentAddedOrDuplicate {
    TorrentDuplicate(Torrent),
    TorrentAdded(Torrent),
}
impl RpcResponseArgument for TorrentAddedOrDuplicate {}

#[derive(Deserialize, Debug)]
pub struct TorrentRenamePath {
    pub path: Option<String>,
    pub name: Option<String>,
    pub id: Option<i64>,
}
impl RpcResponseArgument for TorrentRenamePath {}
