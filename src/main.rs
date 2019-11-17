//JUST NEED TO WAIT FOR CRATES.IO CERT TO BE VALID, THEN BUG TEST, THEN COPY TO LAPTOP WITH "NORMAL", THEN REPLACE BUTTONS
//SHOULD ADD DURATION LOGGING TO RUBY BEFORE REPLACING AND TO THIS TO SEE IF ANY TIME WAs SAVED

use std::process::Command;
use std::str;


//build switches (monitor, laptop, tablet)
static switch: &'static str = "laptop";


fn main() {
    build_and_run_commands();
}

fn build_and_run_commands() {
    //directions and input disable
    let direction: String;
    if switch == "monitor" {
        direction = "right".into();
    } else if switch == "tablet" {
        direction = "left".into();
    } else {
        direction = "normal".into();
    }

    //identifiers
    let screen: String;
    let touchscreen: String;
    if switch != "monitor" {
        screen = "eDP-1".into();
//duplication
        touchscreen = "ATML1000:00 03EB:8C1F".into(); //11
    } else {
        screen = "HDMI-1".into();
//duplication
        touchscreen = "Melfas LGD AIT Touch Controller".into(); //15
    }

    //non touchscreen input devices
    static touchpad: &'static str = "SynPS/2 Synaptics TouchPad";
    static keyboard: &'static str = "AT Translated Set 2 keyboard";

    //this is the program used to rotate the display
    static screen_orientation_command: &'static str = "xrandr";
    //this is the program used to rotate the touchscreen and disable the inputs
    static input_command: &'static str = "xinput";


    let screen_arg_string = &format!("--output {} --rotate {}", screen, direction);
   
    let mut touch_arg_list = vec!["".into()];
    //'inverted', '-1 0 1 0 -1 1 0 0 1'
    if direction == "left" {
        touch_arg_list = vec!["--set-prop".to_string(), format!(r"{}", touchscreen), "--type=float".to_string(), "Coordinate Transformation Matrix".to_string(),  "0".to_string(), "-1".to_string(), "1".to_string(), "1".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "1".to_string()];
    } else if direction == "right" {
        touch_arg_list = vec!["--set-prop".to_string(), format!(r"{}", touchscreen), "--type=float".to_string(), "Coordinate Transformation Matrix".to_string(),  "0".to_string(), "1".to_string(), "0".to_string(), "-1".to_string(), "0".to_string(), "1".to_string(), "0".to_string(), "0".to_string(), "1".to_string()];	
    } else if direction == "normal" {
        touch_arg_list = vec!["--set-prop".to_string(), format!(r"{}", touchscreen), "--type=float".to_string(), "Coordinate Transformation Matrix".to_string(),  "1".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "1".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "1".to_string()];
    }


    if switch != "tablet" {
        Command::new(input_command).arg("enable").arg(keyboard).output().expect("failed to run keyboard lock command");
        Command::new(input_command).arg("enable").arg(touchpad).output().expect("failed to run touchpad lock command");	
    } else {
        Command::new(input_command).arg("disable").arg(keyboard).output().expect("failed to run keyboard lock command");
        Command::new(input_command).arg("disable").arg(touchpad).output().expect("failed to run touchpad lock command");
    }
    
    let screen_arg_list: Vec<&str> = screen_arg_string.split(" ").collect();
    Command::new(screen_orientation_command).args(screen_arg_list).output().expect("failed to run screen orientation command");
    Command::new(input_command).args(touch_arg_list).output().expect("failed to run touch orientation command");
    
    //this loop finds the touch screen IDs for each monitor, which change each time you plug
    let xinput_bytes = Command::new(input_command).output().expect("failed to query xinput").stdout;
    let device_string = str::from_utf8(&xinput_bytes).expect("failed to convert byte output from xinput to string").to_string();
    let device_list = device_string.split("\n");
    let mut laptop_touch_id: String = "".into();
    let mut monitor_touch_id: String = "".into();
    for device in device_list {
//duplication
        if device.contains("ATML1000:00 03EB:8C1F") {
            let position = device.chars().position(|target| target == '=').expect("failed to find position of id number");
            let dirty_id: String = device.chars().skip(position+1).take(2).collect();
            laptop_touch_id = dirty_id.trim().into();
//duplication         
        } else if switch == "monitor" && device.contains("Melfas LGD AIT Touch Controller") {
            let position = device.chars().position(|target| target == '=').expect("failed to find position of id number");
            let dirty_id: String = device.chars().skip(position+1).take(2).collect();
            monitor_touch_id = dirty_id.trim().into();           
        }
    }

    if switch != "monitor" {
	    Command::new(input_command).arg("--map-to-output").arg(laptop_touch_id).arg(screen).output().expect("failed to map laptop only touch to screen");
    } else {
	    Command::new(input_command).arg("--map-to-output").arg(monitor_touch_id).arg(screen).output().expect("failed to map monitor touch to screen");
	    Command::new(input_command).arg("--map-to-output").arg(laptop_touch_id).arg("eDP-1").output().expect("failed to map laptop touch to screen");	
    }

}

