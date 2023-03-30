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

const REMOTING_PORT_ARG: &str = "--app-port=";

const REMOTING_TOKEN_ARG: &str = "--remoting-auth-token=";

pub(crate) fn get_auth_info() -> Result<(String, String, String, String), ProcessInfoError> {
    let mut sys = System::new_all();
    sys.refresh_processes();

    let args = sys
        .processes()
        .values()
        .find(|p| p.name() == TARGET_PROCESS)
        .map(|p| p.cmd())
        .ok_or(ProcessInfoError::ProcessNotAvailable)?;

    let port = get_arg(PORT_ARG, args)?;
    let auth_token = get_arg(TOKEN_ARG, args)?;
    let remoting_port = get_arg(REMOTING_PORT_ARG, args)?;
    let remoting_token = get_arg(REMOTING_TOKEN_ARG, args)?;

    Ok((
        general_purpose::STANDARD.encode(format!("riot:{}", auth_token)),
        port,
        general_purpose::STANDARD.encode(format!("riot:{}", remoting_token)),
        remoting_port,
    ))
}

fn get_arg(arg: &str, args: &[String]) -> Result<String, ProcessInfoError> {
    args.iter()
        .find(|a| a.starts_with(arg))
        .map(|a| a.strip_prefix(arg).unwrap().to_string())
        .ok_or(ProcessInfoError::AuthTokenNotFound)
}