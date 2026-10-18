use base64::{engine::general_purpose, Engine};
use sysinfo::{ProcessRefreshKind, System, UpdateKind};

use crate::{error::ProcessInfoError, rest::LCUClientInfo};

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

/// Finds the League Client and returns its command line.
///
/// Only process names are listed; the command line is read for the League Client process alone,
/// so no other process on the machine is inspected.
pub fn get_league_process_args() -> Option<String> {
    let mut sys = System::new();
    sys.refresh_processes_specifics(ProcessRefreshKind::new());

    let pid = sys
        .processes()
        .iter()
        .find(|(_, process)| process.name().contains(TARGET_PROCESS))
        .map(|(pid, _)| *pid)?;

    if !sys.refresh_process_specifics(pid, ProcessRefreshKind::new().with_cmd(UpdateKind::Always)) {
        return None;
    }

    sys.process(pid).map(|process| process.cmd().join(" "))
}

pub fn get_auth_info(args: String) -> Result<LCUClientInfo, ProcessInfoError> {
    let port = get_arg(PORT_ARG, &args)?;
    let auth_token = get_arg(TOKEN_ARG, &args)?;
    let remoting_port = get_arg(REMOTING_PORT_ARG, &args)?;
    let remoting_token = get_arg(REMOTING_TOKEN_ARG, &args)?;

    Ok(LCUClientInfo {
        port: port.parse::<u16>().unwrap(),
        token: general_purpose::STANDARD.encode(format!("riot:{}", auth_token)),
        remoting_port: remoting_port.parse::<u16>().unwrap(),
        remoting_token: general_purpose::STANDARD.encode(format!("riot:{}", remoting_token)),
    })
}

pub fn get_lcu_client_info() -> Result<LCUClientInfo, ProcessInfoError> {
    get_league_process_args()
        .ok_or(ProcessInfoError::ProcessNotAvailable)
        .and_then(get_auth_info)
}

fn get_arg(arg: &str, args: &str) -> Result<String, ProcessInfoError> {
    args.split(' ')
        .find(|s| s.contains(arg))
        .ok_or(ProcessInfoError::ProcessNotAvailable)
        .map(|s| s.replace(arg, ""))
}
