use skyline::hooks::InlineCtx;

mod ui;

#[macro_export]
macro_rules! reg_x {
    ($ctx:ident, $no:expr) => {
        unsafe { *$ctx.registers[$no].x.as_ref() }
    };
}

#[macro_export]
macro_rules! reg_w {
    ($ctx:ident, $no:expr) => {
        unsafe { *$ctx.registers[$no].w.as_ref() }
    };
}

#[macro_export]
macro_rules! reg_x_mut {
    ($ctx:ident, $no:expr) => {
        unsafe { *$ctx.registers[$no].x.as_mut() }
    };
}

#[macro_export]
macro_rules! reg_w_mut {
    ($ctx:ident, $no:expr) => {
        unsafe { *$ctx.registers[$no].w.as_mut() }
    };
}

#[repr(C)]
pub struct MenuNameEntry {
    pub name: *const u8,
    pub unk1: u32,
    pub unk2: i32,
    pub unk3: u32,
    pub unk4: [f32;4],
    pub unk5: u32,
}

#[skyline::hook(offset = 0x19dd30, inline)]
pub fn battle_name_hook(ctx: &mut InlineCtx) {
    // Don't use this for now
    let names : [*const u8;3] = [b"Cass\0".as_ptr() , b"Cass\0".as_ptr(), b"Cass\0".as_ptr()];
    unsafe { *ctx.registers[8].x.as_mut() = names.as_ptr() as u64; }
}

#[skyline::main(name = "nosurge")]
pub fn main() {
    // Menu name patches
    let text_region = unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8 };
    // TODO: Un-hardcode the offset? Except if there is no update for the game.
    let menu_name_entries = unsafe { std::slice::from_raw_parts_mut(text_region.add(0xc94fc0) as *mut MenuNameEntry, 166) };
    menu_name_entries[5].name = skyline::c_str("Cass\0");
    menu_name_entries[6].name = skyline::c_str("Ion\0");
    menu_name_entries[7].name = skyline::c_str("Delta\0");

    ui::install_hook();
    //skyline::install_hook!(name_hook);

    println!("Ar noSurge English Patch is now installed");
}