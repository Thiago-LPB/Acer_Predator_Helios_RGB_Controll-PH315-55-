use std::time::Duration;
use libc::stat;
use rusb::{DeviceHandle, Context};
use crate::{brightness, Color, State};


pub struct Effects {
    breath_effect: [u8;8],
    neon_effect: [u8;8],
    ripple_effect: [u8;8],
    snow_effect: [u8;8],
    snake_effect: [u8;8],
    wave_effect: [u8;8],
    lightning_effect: [u8;8],
    raindrop_effect: [u8;8],
    spot_effect: [u8;8],
    star_effect: [u8;8],
    fireball_effect: [u8;8],
    heartbeat_effect: [u8;8],
}

impl Effects {
    
    pub fn new() -> Self{

        const BREATH_EFFECT:[u8;8] = [8, 0, 2, 1, 12, 1, 0, 224];
        const NEON_EFFECT:[u8;8] = [8, 0, 8, 5, 12, 1, 0, 221];
        const RIPPLE_EFFECT:[u8;8] = [8, 0, 6, 9, 12, 1, 1, 213];
        const SNOW_EFFECT:[u8;8] = [8, 0, 40, 9, 12, 1, 0, 213];
        const SNAKE_EFFECT:[u8;8] = [8, 0, 5, 9, 12, 1, 0, 213];
        const WAVE_EFFECT:[u8;8] = [8, 0, 3, 1, 12, 1, 5, 221];
        const LIGHTNING_EFFECT:[u8;8] = [8, 0, 18, 14, 12, 1, 0, 213];
        const RAINDROP_EFFECT:[u8;8] = [8, 0, 10, 9, 12, 1, 5, 213];
        const SPOT_EFFECT:[u8;8] = [8, 0, 37, 9, 12, 1, 5, 213];
        const STAR_EFFECT:[u8;8] = [8, 0, 38, 9, 12, 1, 5, 213];
        const FIREBALL_EFFECT:[u8;8] = [8, 0, 39, 9, 12, 1, 5, 213];
        const HEARTBEAT_EFFECT:[u8;8] = [8, 0, 41, 9, 12, 1, 5, 213];


        Self {
        breath_effect : BREATH_EFFECT,
        neon_effect : NEON_EFFECT,
        ripple_effect : RIPPLE_EFFECT,
        snow_effect : SNOW_EFFECT,
        snake_effect : SNAKE_EFFECT,
        wave_effect : WAVE_EFFECT,
        lightning_effect : LIGHTNING_EFFECT,
        raindrop_effect : RAINDROP_EFFECT,
        spot_effect : SPOT_EFFECT,
        star_effect : STAR_EFFECT,
        fireball_effect : FIREBALL_EFFECT,
        heartbeat_effect : HEARTBEAT_EFFECT,
    }




}
    pub fn get_effects(&self, name: &str) -> [u8;8]{

        match name {
            "breath_effect" => self.breath_effect,
            "neon_effect" => self.neon_effect,
            "snow_effect" => self.snow_effect,
            "ripple_effect" => self.ripple_effect,
            "snake_effect" => self.snake_effect,
            "wave_effect" => self.wave_effect,
            "lightning_effect" => self.lightning_effect,
            "raindrop_effect" => self.raindrop_effect,
            "spot_effect" => self.spot_effect,
            "star_effect" => self.star_effect,
            "fireball_effect" => self.fireball_effect,
            "heartbeat_effect" => self.heartbeat_effect,
            _ => self.breath_effect
        }
    }
}




pub fn set_effect(mut effect: [u8;8], handle:&DeviceHandle<Context>, state: &mut State) {

    /*
        Wave directions:
            0,1 -> right
            2 -> left
            3 -> top
            4 -> bottom
            5 -> spin right
            6 -> spin left
     */

    let precommand_effects:[u8;16] = [8, 2, 0, 0, 0, 0, 0, 245, 20, 0, 0, state.color.red, state.color.green, state.color.blue, 0, 0];

    effect[7] = if state.speed<effect[3] {effect[7]+(effect[3]-state.speed)} else {effect[7]-(state.speed - effect[3])};
    effect[7] = if state.brightness<effect[4] {effect[7]+(effect[4]-state.brightness)} else {effect[7]-(state.brightness-effect[4])};

    effect[3] = state.speed;

    effect[4] = state.brightness;
    
    println!("{:?}", effect);

    let timeout = Duration::from_secs_f64(0.01);

    for i in 0..2{
        match handle.write_control(0x21, 0x09, 0x0300, 3, &precommand_effects[i*8..(i+1)*8], timeout){
            Ok(bytes) => println!("{bytes} bytes sent"),
            Err(e) => eprint!("Erro ao enviar: {:?}", e),
        }
    }

    //std::thread::sleep(Duration::from_secs_f64(0.57));

    match handle.write_control(0x21, 0x09, 0x0300, 3, &effect, timeout) {
        Ok(bytes) => println!("{} bytes sent", bytes),
        Err(e) => eprintln!("Erro ao enviar: {:?}", e),
    }
} 
