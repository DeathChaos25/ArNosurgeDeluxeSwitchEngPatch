use skyline::hooks::InlineCtx;

#[skyline::hook(offset = 0x23df90, inline)]
fn left_window_title_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[1].x.as_mut() = skyline::c_str("Characters\0") as u64; }
}

#[skyline::hook(offset = 0x23c780, inline)]
fn chara_description_hook(ctx: &mut InlineCtx) {
    // Character length needs fixing through the layout file, or a UINode hook. Preferably in code?
    unsafe { *ctx.registers[20].x.as_mut() = b"Reach enlightenment. Seek the thighs. Avoid the socks.\0\0".as_ptr() as u64; }

    todo!("Rework the character description table hook");
}

pub fn install_hook() {
    skyline::install_hooks!(left_window_title_hook, chara_description_hook);
}