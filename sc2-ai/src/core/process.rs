use std::{
    net::TcpListener,
    path::Path,
    process::{Child, Command, ExitStatus},
};

use bevy::ecs::system::Resource;
use regex::Regex;

use super::client::Client;

const SC2_BINARY: &str = "SC2_x64.exe";
const SC2_SUPPORT: &str = "Support64";
const HOST: &str = "127.0.0.1";

const DEFAULT_SC2_PATH: &str = {
    #[cfg(target_os = "windows")]
    {
        "C:/Program Files (x86)/StarCraft II"
    }
    #[cfg(target_os = "linux")]
    {
        "~/StarCraftII"
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        compile_error!("Unsupported OS");
    }
};

#[derive(Resource, Debug)]
pub struct Process(Child);

impl Drop for Process {
    fn drop(&mut self) {
        self.0.kill().expect("Failed to kill process");
    }
}

impl Process {
    pub fn wait(&mut self) -> std::io::Result<ExitStatus> {
        self.0.wait()
    }

    pub fn kill(&mut self) -> std::io::Result<()> {
        self.0.kill()
    }
}

pub fn launch_client() -> Result<(Process, Client), anyhow::Error> {
    let sc2_path = get_path_to_sc2();
    let (base_version, data_hash) = (get_latest_base_version(&sc2_path), "");

    let mut process = Command::new(format!(
        "{}/Versions/Base{}/{}",
        sc2_path, base_version, SC2_BINARY
    ));

    let port = get_unused_port();

    process
        .current_dir(format!("{}/{}", sc2_path, SC2_SUPPORT))
        .arg("-listen")
        .arg(HOST)
        .arg("-port")
        .arg(port.to_string())
        // 0 - windowed, 1 - fullscreen
        .arg("-displayMode")
        .arg("0");
    if !data_hash.is_empty() {
        process.arg("-dataVersion").arg(data_hash);
    }

    let process = process.spawn().map(Process)?;
    let client = Client::connect("127.0.0.1", port)?;
    Ok((process, client))
}

pub fn map_path(map_name: &str) -> String {
    let sc2_path = get_path_to_sc2();

    let maps = {
        let path = format!("{}/Maps", sc2_path);
        if std::fs::metadata(&path).is_ok() {
            path
        } else {
            let path = format!("{}/maps", sc2_path);
            if std::fs::metadata(&path).is_ok() {
                path
            } else {
                panic!("Can't find maps folder in: {}", sc2_path);
            }
        }
    };
    let map_path = format!("{}/{}.SC2Map", maps, map_name);
    std::fs::metadata(&map_path).unwrap_or_else(|_| panic!("Map doesn't exists: {}", map_path));
    map_path
}

fn get_latest_base_version(sc2_path: &str) -> u32 {
    Path::new(&format!("{}/Versions", sc2_path))
        .read_dir()
        .expect("Can't read `Versions` folder")
        .filter_map(|dir| {
            let dir = dir.unwrap();
            dir.file_type().ok().filter(|ftype| ftype.is_dir()).and(
                dir.file_name()
                    .to_str()
                    .filter(|name| name.starts_with("Base"))
                    .map(|name| name[4..].parse::<u32>().unwrap()),
            )
        })
        .max()
        .unwrap()
}

fn get_unused_port() -> i32 {
    (5000..65535)
        .find(|port| TcpListener::bind((HOST, *port)).is_ok())
        .unwrap() as i32
}

fn get_path_to_sc2() -> String {
    match std::env::var_os("SC2PATH") {
        Some(path) => path.to_str().unwrap().to_string(),
        None => {
            if cfg!(target_os = "windows") {
                let file = std::fs::read_to_string(format!(
                    "{}/Documents/StarCraft II/ExecuteInfo.txt",
                    dirs::home_dir().unwrap().to_str().unwrap(),
                ))
                .expect("Can't read ExecuteInfo.txt");
                let re = Regex::new(r"= (.*)\\Versions")
                    .unwrap()
                    .captures(&file)
                    .unwrap();

                let path = Path::new(&re[1]);
                if path.exists() {
                    return path.to_str().unwrap().replace("\\", "/");
                }
            }
            DEFAULT_SC2_PATH.to_string()
        }
    }
}
