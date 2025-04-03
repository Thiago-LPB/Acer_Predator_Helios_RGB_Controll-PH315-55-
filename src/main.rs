use dynamic_effect::Effects;
use libc::stat;
use rusb::{Context, DeviceHandle, UsbContext};
use std::{env, io};
use std::process::Command;
use std::fs::{self, read_to_string};
use std::os::unix::process::CommandExt;
use std::process::ExitCode;

mod static_effect;
mod brightness;
mod dynamic_effect;


struct Color{
    red:u8,
    green:u8,
    blue:u8
}


struct State{
    brightness: u8,
    color: Color,
    effect_type: String,
    dynamic_effect_name: String,
    speed: u8
}

fn write_state_file(color: Color, brightness: u8, effect_type: String, state_path: &String, dynamic_effect_name:String, speed: u8) {
       let op = fs::write(state_path, format!("{}\n{} {} {}\n{}\n{}\n{}",
                                              brightness, color.red,
                                              color.green, color.blue, effect_type,
                                              dynamic_effect_name, speed)); 
        match op {
            Ok(_) => println!("State file created!"),
            Err(e) => eprint!("Error creating state file: {e}")
        }
    }

fn get_state(state_path: &String) -> State{


        let lines:Vec<String> = read_to_string(state_path)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        let current_brightness = lines[0].parse::<u8>().unwrap();
        let current_color = lines[1].split_whitespace().map(|item| item.parse::<u8>().unwrap()).collect::<Vec<_>>();
        let current_effect_type = lines[2].clone();
        let current_dynamic_effect_name = lines[3].clone();
        let current_speed = lines[4].parse::<u8>().unwrap();


        return State{
            brightness: current_brightness,
            color: Color{red: current_color[0], green: current_color[1], blue: current_color[2]},
            effect_type: current_effect_type,
            dynamic_effect_name: current_dynamic_effect_name,
            speed: current_speed
        };
    }


fn main() -> ExitCode {


    let state_path = "/tmp/kb_state.txt".to_string();

    //Request sudo password
    let uid = unsafe {libc::getuid()};

    if uid != 0 {
        println!("Need root permission!");
        
        let err = Command::new("sudo")
        .args(env::args())
        .exec();

        eprint!("Unable to get root acess: {:?}", err);
        return ExitCode::FAILURE;
    }
    //
    
    // Create state file if doesn't exist
    if fs::metadata(&state_path).is_err(){
        write_state_file(Color{red: 25, green: 255, blue: 25}, 12, "static".to_string(), &state_path, "breath_effect".to_string(), 4);
    }
    //

    let mut state = get_state(&state_path);
    let effects = Effects::new();
    
    let args: Vec<String> = env::args().collect(); 

    let vid = 0x04f2; 
    let pid = 0x0117; 

    let context = Context::new().expect("Unable to initialize libusb context");
    let handle: DeviceHandle<_> = context.open_device_with_vid_pid(vid, pid)
        .expect("Device not found");

    // Reivindicar a interface correta (se necessário)
    let claim_result = handle.claim_interface(3);
    
    match claim_result{
        Ok(_) => println!("Interface claimed!"),
        Err(_) => {
            println!("Detaching kernel driver...");
            handle.detach_kernel_driver(3).expect("Falha ao liberar kernel driver");
            handle.claim_interface(3).expect("Unable to claim interface!");

println!("Interface claimed!");
        }
    }

    if args[1] == "brightness"{
        if args.len() != 3 {
            println!("Wrong brightness_set command format");
            return ExitCode::FAILURE;
        }
        


        let result = brightness::set_brightness(args[2].to_string(), handle, &mut state, effects);
        
        if let Err(e) = result {
            eprint!("{e}");
            return ExitCode::FAILURE;
        }

        println!("{}", result.ok().unwrap());

    }else if args[1] == "color"{
        if args.len() != 5 {
            println!("Should be in format R G B");
            return ExitCode::FAILURE;
        }

        state.color = Color{
            red: args[2].parse().unwrap(),
            green: args[3].parse().unwrap(),
            blue: args[4].parse().unwrap()
        };
        
        static_effect::set_color(&handle, &state);

    }else if args[1] == "effect" {
      if args.len() != 5 {
            println!("Should be in format R G B");
            return ExitCode::FAILURE;
        }
        state.color = Color{
            red: args[2].parse().unwrap(),
            green: args[3].parse().unwrap(),
            blue: args[4].parse().unwrap()
        };
        
        println!("Select a effect:\n
            (1) BreathEffect\n
            (2) NeonEffect\n
            (3) RippleEffect\n
            (4) SnowEffect\n
            (5) SnakeEffect\n
            (6) WaveEffect\n
            (7) LightningEffect\n
            (8) RaindropEffect\n
            (9) SpotEffect\n
            (10) StarEffect\n
            (11) FireballEffect\n
            (12) HeartbeatEffect\n
");
        
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).expect("Unable to read input");
        let mut buffer = buffer.trim();

        match buffer{
            "1" => buffer = "breath_effect", 
            "2" => buffer = "neon_effect", 
            "3" => buffer = "ripple_effect", 
            "4" => buffer = "snow_effect", 
            "5" => buffer = "snake_effect", 
            "6" => buffer = "wave_effect", 
            "7" => buffer = "lightning_effect", 
            "8" => buffer = "raindrop_effect", 
            "9" => buffer = "spot_effect", 
            "10"=> buffer = "star_effect", 
            "11"=> buffer = "fireball_effect", 
            "12"=> buffer = "heartbeat_effect", 
            _ => println!("wrong number")
        }
        
        dynamic_effect::set_effect(effects.get_effects(buffer), &handle, &mut state);
        state.dynamic_effect_name = buffer.to_string();

    }else if args[1] == "speed" && args.len() == 3{
    
        if args[2].to_lowercase() == "up"{
            if state.speed <= 1{
                println!("Already at maximum speed!");
                return ExitCode::FAILURE;
            }
            state.speed -= 1;
            dynamic_effect::set_effect(effects.get_effects(&state.dynamic_effect_name), &handle, &mut state);
        } else if args[2].to_lowercase() == "down" {
            if state.speed >= 9 {
                println!("Already at minimum brightness!");
                return ExitCode::FAILURE;
            }
            state.speed += 1;
            dynamic_effect::set_effect(effects.get_effects(&state.dynamic_effect_name), &handle, &mut state);
        } else {
            eprint!("Wrong command. Should be up or down.");
            return ExitCode::FAILURE;
        }


    }else if args[1] == "reset" && args.len() == 2 {
        
        println!("State file reset!");
        write_state_file(Color{red: 25, green: 255, blue: 25}, 12, "static".to_string(), &state_path, "breath_effect".to_string(), 4);
        return ExitCode::SUCCESS;
    
    }else{
        println!("Invalid arguments names. Should be color, brightness.");
        return ExitCode::FAILURE;
    }
    write_state_file(state.color, state.brightness, state.effect_type, &state_path, state.dynamic_effect_name, state.speed);
    return ExitCode::SUCCESS;
   }
