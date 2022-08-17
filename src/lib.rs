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

#[skyline::hook(offset = 0x19dd60)]
pub fn get_battle_name_by_id(idx: u32) -> *const u8 {
    let result = call_original!(idx);

    let name = unsafe { CStr::from_ptr(result as _) };
    
    println!("Battle name requested: {}", name.to_str().unwrap());
    
    result
}

/// Don't use this for now
#[skyline::hook(offset = 0x19de04, inline)]
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
        b"Human Soldier\0".as_ptr(),
        b"Cass_Enemy\0".as_ptr(),
        b"Ion_Enemy\0".as_ptr(),
        b"Delta_Enemy\0".as_ptr(),
        b"Avatar_Enemy\0".as_ptr(),
        b"Kanon_Boss\0".as_ptr(),
        b"Kanon_Boss2\0".as_ptr(),
        b"Nay_Boss\0".as_ptr(),
        b"Zill_Boss\0".as_ptr(),
        b"Zill_Boss2\0".as_ptr(),
        b"Nelico_Boss\0".as_ptr(),
        b"Nelo_Boss\0".as_ptr(),
        b"Cosal_Boss\0".as_ptr(),
        b"Cosal_Boss2\0".as_ptr(),
        b"Prim_Boss\0".as_ptr(),
        b"Prim_BossFinal\0".as_ptr(),
        b"Hidden Boss\0".as_ptr(),
        b"Hidden Boss (Extra)\0".as_ptr(),
        b"Hidden Boss (Extra - Core)\0".as_ptr(),
        b"Soreil (Delta Battle)\0".as_ptr(),
        b"Soreil (Avatar Battle)\0".as_ptr(),
        b"8th Dimensional\0".as_ptr(),
        b"Felion Subspecies\0".as_ptr(),
        b"Neregracos Subspecies\0".as_ptr(),
        b"Awinir Subspecies\0".as_ptr(),
        b"Dimensional\0".as_ptr(),
        b"Intangible\0".as_ptr(),
        b"Chimon Limited\0".as_ptr(),
        b"BLGRD\0".as_ptr(),
        b"GSK-99\0".as_ptr(),
        b"HR-50\0".as_ptr(),
        b"Pururururu\0".as_ptr(),
        b"Ogregute\0".as_ptr(),
        b"Godbard\0".as_ptr(),
        b"Meclich\0".as_ptr(),
        b"Zeneral\0".as_ptr(),
        b"Aipida\0".as_ptr(),
        b"Nelo\0".as_ptr(),
        b"Nelo\0".as_ptr(),
        b"8th Dimensional\0".as_ptr(),
        b"8th Dimensional\0".as_ptr(),
        b"Human Soldier_Male(Gun)\0".as_ptr(),
        b"Human Soldier_Male(Gun)\0".as_ptr(),
        b"Prim\0".as_ptr(),
        b"New_8th Dimensional\0".as_ptr(),
        b"Man A\0".as_ptr(),
        b"Man B\0".as_ptr(),
        b"Man C\0".as_ptr(),
        b"Man D\0".as_ptr(),
        b"Man E\0".as_ptr(),
        b"Woman A\0".as_ptr(),
        b"Woman B\0".as_ptr(),
        b"Woman C\0".as_ptr(),
        b"Woman D\0".as_ptr(),
        b"Woman E\0".as_ptr(),
        b"Boy A\0".as_ptr(),
        b"Boy B\0".as_ptr(),
        b"Boy C\0".as_ptr(),
        b"Boy D\0".as_ptr(),
        b"Boy E\0".as_ptr(),
        b"Girl A\0".as_ptr(),
        b"Girl B\0".as_ptr(),
        b"Girl C\0".as_ptr(),
        b"Girl D\0".as_ptr(),
        b"Girl E\0".as_ptr(),
        b"Older Man A\0".as_ptr(),
        b"Older Man B\0".as_ptr(),
        b"Older Man C\0".as_ptr(),
        b"Older Man D\0".as_ptr(),
        b"Older Man E\0".as_ptr(),
        b"Older Woman A\0".as_ptr(),
        b"Older Woman B\0".as_ptr(),
        b"Older Woman C\0".as_ptr(),
        b"Older Woman D\0".as_ptr(),
        b"Older Woman E\0".as_ptr(),
        b"Soldier A\0".as_ptr(),
        b"Soldier B\0".as_ptr(),
        b"Soldier C\0".as_ptr(),
        b"Soldier D\0".as_ptr(),
        b"Soldier E\0".as_ptr(),
        b"Sharl_1A\0".as_ptr(),
        b"Sharl_1B\0".as_ptr(),
        b"Sharl_1C\0".as_ptr(),
        b"Sharl_1D\0".as_ptr(),
        b"Sharl_1E\0".as_ptr(),
        b"Sharl_2A\0".as_ptr(),
        b"Sharl_2B\0".as_ptr(),
        b"Sharl_2C\0".as_ptr(),
        b"Sharl_2D\0".as_ptr(),
        b"Sharl_2E\0".as_ptr(),
        b"Sharl_3A\0".as_ptr(),
        b"Sharl_3B\0".as_ptr(),
        b"Sharl_3C\0".as_ptr(),
        b"Sharl_3D\0".as_ptr(),
        b"Sharl_3E\0".as_ptr(),
        b"Teru Tribe A\0".as_ptr(),
        b"Teru Tribe B\0".as_ptr(),
        b"Teru Tribe C\0".as_ptr(),
        b"Teru Tribe D\0".as_ptr(),
        b"Teru Tribe E\0".as_ptr(),
        b"PLASMA Bios Shop\0".as_ptr(),
        b"PLASMA General Store\0".as_ptr(),
        b"PLASMA General Store 2\0".as_ptr(),
        b"Quanturv General Store\0".as_ptr(),
        b"Quanturv Bios Shop\0".as_ptr(),
        b"Honokano Rare Shop\0".as_ptr(),
        b"Honokano General Store\0".as_ptr(),
        b"Honokano Bios Shop\0".as_ptr(),
        b"Tenryo General Store\0".as_ptr(),
        b"Tenryo Bios Shop\0".as_ptr(),
        b"Kurt General Store\0".as_ptr(),
        b"Kurt Bios Shop\0".as_ptr(),
        b"Sharl Ticket_NPC Position\0".as_ptr(),
        b"Sharl Ticket_NPC Position\0".as_ptr(),
        b"Sharl Pythagoraria Ban NPC\0".as_ptr(),
        b"Sharl Elder_NPC Position\0".as_ptr(),
        b"NPC_C12_3_020\0".as_ptr(),
        b"NPC_C12_3_100\0".as_ptr(),
        b"Sharl Pythagoraria Ban NPC_2\0".as_ptr(),
        b"Solius of the Storm\0".as_ptr(),
        b"Sickly Child (Felie)\0".as_ptr(),
        b"Sonwe\0".as_ptr(),
        b"Loria\0".as_ptr(),
        b"Faylon\0".as_ptr(),
        b"Sharl Chief\0".as_ptr(),
        b"Sharl Elder\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr(),
        b"NPC Frame\0".as_ptr()
        ];
    let original_list = unsafe { std::slice::from_raw_parts_mut(reg_x_mut!(ctx, 8) as *mut *const u8, names.len()) };

    original_list.copy_from_slice(&names);
}


#[skyline::hook(offset = 0x23df90, inline)]
pub fn encyclopedia_characters_met_left_title_string_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[1].x.as_mut() = skyline::c_str("Characters\0") as u64; }
}

// TODO: Move the bio stuff in a /ui/menus/bio.rs module at this point
#[skyline::hook(offset = 0x23c780, inline)]
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