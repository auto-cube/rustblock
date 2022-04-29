use std::process::{Command, Stdio};
use chrono::prelude::*;
use std::thread;
use std::time::Duration;
fn main() {
    loop {
        let battery = get_battery();
        let light = get_light();
        let volume = get_volume();
        let time = get_time();
        let status = battery+&light+&volume+&time;
        Command::new("xsetroot").arg("-name").arg(status).output().expect("xsetroot error");
        thread::sleep(Duration::from_secs(1));
    }

}
fn get_volume()->String{
    let volume_command = Command::new("pamixer")
    .arg("--get-volume")
    .stdout(Stdio::piped())
    .output()
    .expect("Failed to execute command");
    let volume = String::from_utf8_lossy(&volume_command.stdout);
    let mute_command = Command::new("pamixer")
    .arg("--get-mute")
    .stdout(Stdio::piped())
    .output()
    .expect("Failed to execute command");
    let is_mute = String::from_utf8_lossy(&mute_command.stdout);
    if is_mute.eq("false"){
        return String::from("[\u{f026} Mute]");
    }else{
        if volume.trim_end().parse::<i32>().expect("error")<=30{
            return "[\u{f027} ".to_string()+volume.trim_end()+"%]";
        }else{
            return "[\u{f028} ".to_string()+volume.trim_end()+"%]";
        }
    }
}
fn get_time()->String{
    let dt = Local::now().format("%F %a %T");
    "[\u{f017} ".to_string()+&dt.to_string()[..]+"]"
}
fn get_light()->String{
    let light_command = Command::new("light")
    .arg("-G")
    .stdout(Stdio::piped())
    .output()
    .expect("Failed to execute command");
    let light = String::from_utf8_lossy(&light_command.stdout);
    let light = light.trim_end().to_string().parse::<f64>().expect("light_command output trans error") as i64;
    return "[\u{f0eb} ".to_string()+&light.to_string()[..]+"]";

}
fn get_battery()->String{
    let path_to_power_supply = String::from("/sys/class/power_supply/");
    let path_to_bat = path_to_power_supply.clone()+("CMB0/");
    let path_to_ac  = path_to_power_supply.clone()+("ADP1/");
    //
    let status_command = Command::new("cat")
    .arg(path_to_bat.clone()+"status")
    .output()
    .expect("Failed to execute command");
    let status = String::from_utf8_lossy(&status_command.stdout);
    //
    let capacity_command = Command::new("cat")
    .arg(path_to_bat.clone()+"capacity")
    .output()
    .expect("Failed to execute command");
    let capacity = String::from_utf8_lossy(&capacity_command.stdout);
    //
    let is_plugged_command = Command::new("cat")
    .arg(path_to_ac+"online")
    .output()
    .expect("Failed to execute command");
    let is_plugged = String::from_utf8_lossy(&is_plugged_command.stdout);
    let capacity_int = capacity.clone().trim_end().parse::<i32>().expect("capacity parse interger error");
    if status == "Full"{
        return "[\u{f240} ".to_string()+&capacity.trim_end()[..]+"%]"
    }else{
        if is_plugged.trim_end() == "1"{
            return "[\u{f0e7} ".to_string()+&capacity.trim_end()[..]+"%]"
        }else {
            if capacity_int>=95 && capacity_int<=100{
                return "[\u{f240} ".to_string()+&capacity.trim_end()[..]+"%]"
            }else if capacity_int>=75 && capacity_int<95{
                return "[\u{f241} ".to_string()+&capacity.trim_end()[..]+"%]"
            }else if capacity_int>=50 && capacity_int<75{
                return "[\u{f242} ".to_string()+&capacity.trim_end()[..]+"%]"
            }else if capacity_int>=25 && capacity_int<50{
                return "[\u{f243} ".to_string()+&capacity.trim_end()[..]+"%]"
            }else{
                return "[\u{f244} ".to_string()+&capacity.trim_end()[..]+"%]"
            }
        }
    }

}
