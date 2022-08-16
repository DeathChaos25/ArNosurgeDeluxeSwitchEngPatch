use std::ffi::CStr;

use skyline::{hooks::InlineCtx, println};

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

#[skyline::hook(offset = 0x19dc90)]
pub fn get_battle_name_by_id(idx: u32) -> *const u8 {
    let result = call_original!(idx);

    let name = unsafe { CStr::from_ptr(result as _) };
    
    println!("Battle name requested: {}", name.to_str().unwrap());
    
    result
}

/// Don't use this for now
#[skyline::hook(offset = 0x19dd30, inline)]
pub fn battle_name_hook(ctx: &mut InlineCtx) {
    let names = [
        b"Cass\0".as_ptr() , 
        b"Ion\0".as_ptr(), 
        b"Delta\0".as_ptr(),
        b"Earthes\0".as_ptr(),
        b"Kanon\0".as_ptr(),
        b"Nay\0".as_ptr(),
        b"Zill\0".as_ptr(),
        b"Nelico\0".as_ptr(),
        b"Nelo\0".as_ptr(),
        b"Cosal\0".as_ptr(),
        b"Prim\0".as_ptr(),
        b"Renall\0".as_ptr(),
        b"Undou\0".as_ptr(),
        b"Shirotaka\0".as_ptr(),
        b"Tattoria\0".as_ptr(),
        b"Sarly\0".as_ptr(),
        b"Ayatane\0".as_ptr(),
        b"Shurelia\0".as_ptr(),
        b"Cass_Powerup\0".as_ptr(),
        b"Cass_Purification\0".as_ptr(),
        b"Cass_Purification(Sharl)\0".as_ptr(),
        b"Ion_Powerup\0".as_ptr(),
        b"Ion_Purification1\0".as_ptr(),
        b"Ion_Purification2\0".as_ptr(),
        b"Delta_Powerup\0".as_ptr(),
        b"Delta_Purification1\0".as_ptr(),
        b"Delta_Costume4(Unnumbered)\0".as_ptr(),
        b"Avatar_Powerup\0".as_ptr(),
        b"Avatar_Costume3(Unnumbered)\0".as_ptr(),
        b"Avatar_Costume4(Unnumbered)\0".as_ptr(),
        b"Kanon_Purification\0".as_ptr(),
        b"Nay_Purification\0".as_ptr(),
        b"Quantum Nova\0".as_ptr(),
        b"Tsunderain\0".as_ptr(),
        b"Nyurokiller Z1\0".as_ptr(),
        b"Nyurokiller Z2\0".as_ptr(),
        b"Nyurokiller Z3\0".as_ptr(),
        b"Spartan Liv\0".as_ptr(),
        b"Feminisat\0".as_ptr(),
        b"Toy Installer\0".as_ptr(),
        b"Bridal Launch\0".as_ptr(),
        b"Ribo Pandemics\0".as_ptr(),
        b"Schizoidrophia\0".as_ptr(),
        b"Ar Ciel Sphere\0".as_ptr(),
        b"Heart of Light\0".as_ptr(),
        b"Chunpi\0".as_ptr(),
        b"I Have No Royalty Income\0".as_ptr(),
        b"Artificial Flower\0".as_ptr(),
        b"Armored Adehime\0".as_ptr(),
        b"Proof of Yuuki\0".as_ptr(),
        b"Amenomurakumo Mikuji\0".as_ptr(),
        b"Lighthouse Keeper's Night\0".as_ptr(),
        b"Tokikagura x Tentouki\0".as_ptr(),
        b"7th Apocalypse\0".as_ptr(),
        b"Sharl_Fairy\0".as_ptr(),
        b"Sharl_Fairy\0".as_ptr(),
        b"Sharl_Fairy\0".as_ptr(),
        b"Sharl_Rockwing\0".as_ptr(),
        b"Sharl_Rockwing\0".as_ptr(),
        b"Sharl_Rockwing\0".as_ptr(),
        b"Sharl_Mermaid\0".as_ptr(),
        b"Sharl_Mermaid\0".as_ptr(),
        b"Sharl_Mermaid\0".as_ptr(),
        b"Hologram\0".as_ptr(),
        b"Hologram\0".as_ptr(),
        b"Hologram\0".as_ptr(),
        b"Chimon Aircraft\0".as_ptr(),
        b"Chimon Aircraft\0".as_ptr(),
        b"Chimon Aircraft\0".as_ptr(),
        b"Chimon Robot\0".as_ptr(),
        b"Chimon Robot\0".as_ptr(),
        b"Chimon Robot\0".as_ptr(),
        b"Beagle\0".as_ptr(),
        b"Beagle\0".as_ptr(),
        b"Beagle\0".as_ptr(),
        b"Tenmon Mecha\0".as_ptr(),
        b"Tenmon Mecha\0".as_ptr(),
        b"Tenmon Mecha\0".as_ptr(),
        b"Tenmon Robot\0".as_ptr(),
        b"Tenmon Robot\0".as_ptr(),
        b"Tenmon Robot\0".as_ptr(),
        b"Slime\0".as_ptr(),
        b"Slime\0".as_ptr(),
        b"Slime\0".as_ptr(),
        b"Buffalo\0".as_ptr(),
        b"Buffalo\0".as_ptr(),
        b"Buffalo\0".as_ptr(),
        b"Bird\0".as_ptr(),
        b"Bird\0".as_ptr(),
        b"Bird\0".as_ptr(),
        b"Phantom\0".as_ptr(),
        b"Phantom\0".as_ptr(),
        b"Phantom\0".as_ptr(),
        b"Human Soldier_Male_Gun\0".as_ptr(),
        b"Human Soldier_Male_Gun\0".as_ptr(),
        b"Human Soldier_Male_Gun\0".as_ptr(),
        b"Human Soldier_Female_Magic\0".as_ptr(),
        b"Human Soldier_Female_Magic\0".as_ptr(),
        b"Human Soldier_Female_Magic\0".as_ptr(),
        b"Mob Robot\0".as_ptr(),
        b"Mob Robot\0".as_ptr(),
        b"Slime\0".as_ptr(),
        b"Mob Priest\0".as_ptr(),
        b"Mob Soldier\0".as_ptr(),
        b"Mob Robot\0".as_ptr(),
        b"Mob Robot\0".as_ptr(),
        b"Sharl A - Lv1\0".as_ptr(),
        b"Sharl B - Lv1\0".as_ptr(),
        b"Sharl C - Lv1\0".as_ptr(),
        b"Slime_Nelico Battle\0".as_ptr(),
        b"Chimon Aircraft\0".as_ptr(),
        b"Chimon Robot\0".as_ptr(),
        b"Sharl A - Lv2\0".as_ptr(),
        b"Sharl B - Lv2\0".as_ptr(),
        b"Sharl C - Lv2\0".as_ptr(),
        b"Mob Soldier\0".as_ptr(),
        b"Songstress\0".as_ptr(),
        b"PLASMA Troop\0".as_ptr(),
        b"Tenmon Robot\0".as_ptr(),
        b"Sharl A - Lv3\0".as_ptr(),
        b"Sharl B - Lv3\0".as_ptr(),
        b"Sharl C - Lv3\0".as_ptr(),
        b"Human Soldier\0".as_ptr()
        ];
    let original_list = unsafe { std::slice::from_raw_parts_mut(reg_x_mut!(ctx, 8) as *mut *const u8, names.len()) };

    original_list.copy_from_slice(&names);
}


#[skyline::hook(offset = 0x23dbb0, inline)]
pub fn encyclopedia_characters_met_left_title_string_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[1].x.as_mut() = skyline::c_str("Characters\0") as u64; }
}

// TODO: Move the bio stuff in a /ui/menus/bio.rs module at this point
#[skyline::hook(offset = 0x23c3a0, inline)]
pub fn chara_bio_description_hook(ctx: &mut InlineCtx) {
    // Character length needs fixing through the layout file.
    unsafe { *ctx.registers[20].x.as_mut() = b"Reach enlightenment. Seek the thighs. Avoid the socks.\0\0".as_ptr() as u64; }
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
    skyline::install_hooks!(encyclopedia_characters_met_left_title_string_hook, chara_bio_description_hook, get_battle_name_by_id, battle_name_hook);

    println!("Ar noSurge English Patch is now installed");
}