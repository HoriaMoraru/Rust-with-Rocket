use std::collections::HashMap;
use std::path::PathBuf;
use rocket::{launch, routes, get, post, futures};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use serde::Deserialize;
use heim::process;
use futures::StreamExt as _;
use heim::process::{Pid};
use heim::cpu;
use heim::memory;
use std::process::Command;
use rocket::http::ext::IntoCollection;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Response {
    status: String,
    error: u64
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct SysMemory {
    total: u64,
    usage: u64
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Status {
    cpus: u64,
    memory: SysMemory,
    uptime: u64,
    usage: u64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Starter {
    command: String,
    arguments: Vec<String>,
    environment: HashMap<String, String>

}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct StartResponse {
    status: String,
    stdout: String,
    stderr: String,
    error: u64
}

#[get("/status")]
async fn status() -> Json<Status> {
    let total_memory = memory::memory().await.unwrap().total().value;
    let used_memory = total_memory - memory::memory().await.unwrap().available().value;
    let curr_memory: SysMemory = SysMemory {total: total_memory, usage: used_memory};
    let curr_status: Status = Status {cpus: cpu::logical_count().await.unwrap(), memory: curr_memory, uptime: 0,
     usage: 0};

    Json(curr_status)
}


#[derive(Serialize, Deserialize)]
struct MyMemory {
    resident: u64,
    virtual_: u64,
}

#[derive(Serialize, Deserialize)]
struct Process {
    pid: Pid,
    ppid: Pid,
    command: String,
    arguments: String,
    memory: MyMemory,
}

#[get("/processes")]
async fn processes() -> Json<Vec<Process>> {
    let mut processes = Vec::new();

    let stream = process::processes().await.unwrap();
    futures::pin_mut!(stream);

    while let Some(process) = stream.next().await {
        let process = process.unwrap();
        let pid = process.pid();
        let ppid = process.parent_pid().await.unwrap();
        let command = process.name().await.unwrap();
        let arg_cmd = process.command().await.unwrap();
        let arguments = arg_cmd.to_os_string().to_str().unwrap().to_string();
        let memory = process.memory().await.unwrap();
        let process = Process {
            pid,
            ppid,
            command,
            arguments,
            memory: MyMemory {
                resident: memory.rss().value,
                virtual_: memory.vms().value,
            },
        };
        processes.push(process);
    }
    Json(processes)
}

#[get("/processes/<pid>")]
async fn process_pid(pid: Pid) -> Json<Process> {
    let process = process::get(pid).await.unwrap();
    let pid = process.pid();
    let ppid = process.parent_pid().await.unwrap();
    let command = process.name().await.unwrap();
    let arg_cmd = process.command().await.unwrap();
    let arguments = arg_cmd.to_os_string().to_str().unwrap().to_string();
    let memory = process.memory().await.unwrap();
    let process = Process {
        pid,
        ppid,
        command,
        arguments,
        memory: MyMemory {
            resident: memory.rss().value,
            virtual_: memory.vms().value,
        },
    };
    Json(process)
}

#[get("/cpus")]
async fn cpus() -> Json<Cpudata> {
    let mut sys = System::new();
    let sys = spawn_blocking(move || { sys.refresh_cpu_specifics(CpuRefreshKind::everything()); sys } ).await.unwrap();
    let cpudata = Cpudata {
        model: sys.global_cpu_info().brand().to_string(),
        manufacturer: sys.global_cpu_info().vendor_id().to_string(),
        speed: sys.global_cpu_info().frequency(),
        usage: sys.global_cpu_info().cpu_usage(),
    };
    Json(cpudata)
}

#[get("/cpus/<cpu_number>")]
async fn cpus_num(cpu_number: usize) -> Json<Cpudata> {
    let mut sys = System::new();
    let sys = spawn_blocking(move || { sys.refresh_cpu_specifics(CpuRefreshKind::everything()); sys } ).await.unwrap();
    let mut iter = sys.cpus().iter();
    let cpudata = Cpudata {
        model: iter.nth(cpu_number).unwrap().brand().to_string(),
        manufacturer: iter.nth(cpu_number).unwrap().vendor_id().to_string(),
        speed: iter.nth(cpu_number).unwrap().frequency(),
        usage: iter.nth(cpu_number).unwrap().cpu_usage(),
    };
    Json(cpudata)
    
}

#[get("/processes/kill/<pid>")]
async fn kill_process(pid: u32) -> Result<Json<Response>, String> {
    let _cmd = Command::new("kill")
        .arg("-9")
        .arg(pid.to_string())
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(Json({
        Response {
            status: "ok".to_string(),
            error: 0
        }
    }))
}


#[post("/processes/start", data = "<data>")]
async fn process_start(data: Json<Starter>) -> Result<Json<StartResponse>, String> {
    let _cmd = Command::new(data.command.clone())
        .args(data.arguments.clone())
        //.envs(data.environment.clone())
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(Json({
        StartResponse {
            status: "ok".to_string(),
            stdout: "".to_string(),
            stderr: "".to_string(),
            error: 0
        }
    }))
}


#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![processes, process_pid, status, kill_process, process_start])
}
