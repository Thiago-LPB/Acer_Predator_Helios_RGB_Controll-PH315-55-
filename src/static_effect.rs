use std::time::Duration;
use rusb::{DeviceHandle, Context};
use crate::State;


pub fn set_color(handle:&DeviceHandle<Context>, state: &State){
        
    let precommand_color:[u8;8]= [0x12, 0x0, 0x0, 0x8, 0x0, 0x0, 0x0, 0xe5];

    let mut color_type1= [0u8; 512];

    let mut i = 1;

    let ident_loc = [30, 34, 38, 58, 122, 146, 170, 194, 218, 266, 318, 350, 354, 366, 370, 374, 378, 390, 394, 454, 478, 486, 502];

    while i < color_type1.len()
    {
        if let Some(_) = ident_loc.iter().position(|&x| x == i){
            color_type1[i] = 0;
            color_type1[i+1] = 174;
            color_type1[i+2] = 199;
        }else{
            color_type1[i] = state.color.red;
            color_type1[i+1] = state.color.green;
            color_type1[i+2] = state.color.blue;
        }
        i += 4;
    }
     
    let set_color:[u8;8] = [8, 2, 51, 5, state.brightness, 8, 1, 180-state.brightness];
    
    let timeout = Duration::from_secs_f64(0.01);

    match handle.write_control(0x21, 0x09, 0x0300, 3, &precommand_color, timeout) {
        Ok(bytes) => println!("{} bytes sent", bytes),
        Err(e) => eprintln!("Erro ao enviar: {:?}", e),
    }

    for i in 0..8{
        //println!("Index: {i}, Color_type Size: {}", &color_type1[i*64..(i+1)*64].len());
        match handle.write_interrupt(4, &color_type1[i*64..(i+1)*64], timeout){
            Ok(bytes) => println!("{bytes} bytes sent"),
            Err(e) => eprint!("Erro ao enviar: {:?}", e),
        }
    }

    //std::thread::sleep(Duration::from_secs_f64(0.57));

    match handle.write_control(0x21, 0x09, 0x0300, 3, &set_color, timeout) {
        Ok(bytes) => println!("{} bytes sent", bytes),
        Err(e) => eprintln!("Erro ao enviar: {:?}", e),
    }
}
