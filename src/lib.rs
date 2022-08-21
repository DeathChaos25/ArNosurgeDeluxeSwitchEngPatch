use std::ffi::CStr;

use skyline::hooks::InlineCtx;

mod ui;
mod data;

#[macro_export]
macro_rules! reg_x {
    ($ctx:ident, $no:expr) => {
        *$ctx.registers[$no].x.as_ref()
    };
}

#[macro_export]
macro_rules! reg_w {
    ($ctx:ident, $no:expr) => {
        *$ctx.registers[$no].w.as_ref()
    };
}

#[macro_export]
macro_rules! reg_x_mut {
    ($ctx:ident, $no:expr) => {
        *$ctx.registers[$no].x.as_mut()
    };
}

#[macro_export]
macro_rules! reg_w_mut {
    ($ctx:ident, $no:expr) => {
        *$ctx.registers[$no].w.as_mut()
    };
}

#[skyline::hook(offset = 0x19dd60)]
pub fn get_battle_name_by_id(idx: u32) -> *const u8 {
    let result = call_original!(idx);

    let name = unsafe { CStr::from_ptr(result as _) };
    
    println!("Battle name requested: {}", name.to_str().unwrap());
    
    result
}

#[skyline::hook(offset = 0x3d07d0, inline)]
pub fn sixty_fps_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[1].x.as_mut() = 0 as u64; }
}

#[skyline::main(name = "nosurge")]
pub fn main() {
    // Install panic hook to display errors on the screen, since we don't have a network logger for now
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let err_msg = format!("thread has panicked at '{}', {}", msg, location);

        skyline::error::show_error(
            69,
            "Skyline plugin as panicked! Please open the details and send a screenshot to the developer, then close the game.\n",
            err_msg.as_str(),
        );
    }));

    ui::install_hook();
    data::install_hook();

    skyline::install_hooks!(get_battle_name_by_id, sixty_fps_hook);

    println!("Ar noSurge English Patch is now installed");
}