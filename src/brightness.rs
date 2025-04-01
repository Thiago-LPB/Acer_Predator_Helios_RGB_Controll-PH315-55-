use std::time::Duration;
// use std::env;
use rusb::{DeviceHandle, Context};
use crate::{dynamic_effect::{self, Effects}, State};

pub fn set_brightness(option: String, handle: DeviceHandle<Context>, state: &mut State, effects: Effects) -> Result<String, String>{

    // let exec_dir = env::current_exe().expect("Unable to get current_exec_dir");
    // let current_dir = exec_dir.parent().expect("Failed to get current dir");
    
    let brightness_levels:[u8;5] = [0, 12, 25, 37, 51]; 

    let brightness_index = brightness_levels.iter().position(|&x| x == state.brightness).expect("Number not in brightness levels");

    if option == "up"{

        if brightness_index == brightness_levels.len()-1 {

            state.brightness = brightness_levels[brightness_index];
            println!("Already at maximum brightness");

        }else{

            state.brightness = brightness_levels[brightness_index+1];

        }
    }else if option == "down"{

        if brightness_index == 0 {

            state.brightness = brightness_levels[brightness_index];
            println!("Already at minimum brightness");

        }else{

            state.brightness = brightness_levels[brightness_index-1];

        }
    }else{
        return Err("Not valid option for brightness. Available options: up, down".to_string()); 
    }

    if state.effect_type.to_lowercase() == "static" {
        let precommand_brightness = [0x08, 0x00, 0x33, 0x05, 0x00, 0x00, 0x00, 0xf5];
        let set_brightness:[u8;8] = [8, 0, 51, 5, state.brightness, 1, 1, 189-state.brightness];
    

    let timeout = Duration::from_secs_f64(0.01);

    match handle.write_control(0x21, 0x09, 0x0300, 3, &precommand_brightness, timeout) {
        Ok(bytes) => println!("Enviado {} bytes", bytes),
        Err(e) => return Err(format!("Erro ao enviar: {e:?}Isso cria dinamicamente uma String formatada.")),
    }

    match handle.write_control(0x21, 0x09, 0x0300, 3, &set_brightness, timeout) {
        Ok(bytes) => println!("Enviado {} bytes", bytes),
        Err(e) => return Err(format!("Erro ao enviar: {e:?}")),
    }
    } else if state.effect_type.to_lowercase() == "dynamic" {
        dynamic_effect::set_effect(effects.get_effects(state.dynamic_effect_name.as_str()), &handle, state);
    }

    return Ok("Brilho alterado!".to_string());
}
