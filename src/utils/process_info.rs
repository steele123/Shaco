use base64::{engine::general_purpose, Engine};
use sysinfo::{ProcessExt, System, SystemExt};

use crate::error::ProcessInfoError;

#[cfg(target_os = "windows")]
const TARGET_PROCESS: &str = "LeagueClientUx.exe";
#[cfg(target_os = "linux")]
const TARGET_PROCESS: &str = "LeagueClientUx.";
#[cfg(target_os = "macos")]
const TARGET_PROCESS: &str = "LeagueClientUx";

const PORT_ARG: &str = "--riotclient-app-port=";

const TOKEN_ARG: &str = "--riotclient-auth-token=";

pub(crate) fn get_auth_info() -> Result<(String, String), ProcessInfoError> {
    let mut sys = System::new_all();
    sys.refresh_processes();

    let args = sys
        .processes()
        .values()
        .find(|p| p.name() == TARGET_PROCESS)
        .map(|p| p.cmd())
        .ok_or(ProcessInfoError::ProcessNotAvailable)?;

    let port = args
        .iter()
        .find(|arg| arg.starts_with(PORT_ARG))
        .map(|arg| arg.strip_prefix(PORT_ARG).unwrap().to_string())
        .ok_or(ProcessInfoError::PortNotFound)?;
    let auth_token = args
        .iter()
        .find(|arg| arg.starts_with(TOKEN_ARG))
        .map(|arg| {
            arg.strip_prefix(TOKEN_ARG)
                .unwrap()
                .to_string()
        })
        .ok_or(ProcessInfoError::AuthTokenNotFound)?;

    Ok((
        general_purpose::STANDARD.encode(format!("riot:{}", auth_token)),
        port,
    ))
}
