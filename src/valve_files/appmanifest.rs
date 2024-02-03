use serde::Deserialize;

use super::error::ValveFileError;

#[derive(Deserialize, Debug, PartialEq)]
pub struct AppState {
    #[serde(rename = "appid")]
    pub app_id: u64,
    pub universe: bool,
    pub name: String,
    #[serde(rename = "StateFlags")]
    pub state_flags: i32,
    #[serde(rename = "installdir")]
    pub install_dir: String,
    #[serde(rename = "LastUpdated")]
    pub last_updated: i64,
    #[serde(rename = "SizeOnDisk")]
    pub size_on_disk: u128,
    #[serde(rename = "StagingSize")]
    pub staging_size: Option<u128>,
    #[serde(rename = "buildid")]
    pub build_id: u64,
    #[serde(rename = "LastOwner")]
    pub last_owner: u128,
    #[serde(rename = "UpdateResult")]
    pub update_result: Option<u64>,
    #[serde(rename = "BytesToDownload")]
    pub bytes_to_download: Option<u64>,
    #[serde(rename = "BytesDownloaded")]
    pub bytes_downloaded: Option<u64>,
    #[serde(rename = "BytesToStage")]
    pub bytes_to_stage: Option<u64>,
    #[serde(rename = "BytesStaged")]
    pub bytes_staged: Option<u64>,
    #[serde(rename = "TargetBuildID")]
    pub target_build_id: Option<u128>,
    #[serde(rename = "AutoUpdateBehavior")]
    pub auto_update_behaviour: Option<i32>,
    #[serde(rename = "AllowOtherDownloadsWhileRunning")]
    pub allow_other_downloads_while_running: Option<bool>,
    #[serde(rename = "ScheduledAutoUpdate")]
    pub schedule_auto_update: Option<i32>,
}

pub fn from_str(acf_data: &str) -> Result<AppState, ValveFileError> {
    match keyvalues_serde::from_str::<AppState>(acf_data) {
        Ok(x) => Ok(x),
        Err(err) => Err(ValveFileError::KeyValuesParser(err.to_string())),
    }
}

#[test]
fn test_de() {
    use keyvalues_serde::from_str;

    let data = include_str!("../../test-files/appmanifest.acf");

    let vdf = from_str::<AppState>(data);
    assert!(vdf.as_ref().is_ok());
}
