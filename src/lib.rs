use skyline::{hooks::InlineCtx, patching::patch_data};

const LOG_UITEXT: &[u8] = &[ 0xef, 0xbc, 0xac, 0xef, 0xbd, 0x8f, 0xef, 0xbd, 0x87, 0x0d, 0x0a, 0x00, 0x00]; // Log, but wide
const SEND_UITEXT: &[u8] = &[ 0xef, 0xbc, 0xae, 0xef, 0xbd, 0x85, 0xef, 0xbd, 0x98, 0xef, 0xbd, 0x94, 0x00, 0x00]; // Next, but wide

#[skyline::main(name = "ArNosurgeDeluxeSwitchEngPatch")]
pub fn main() {
    println!("Hello from skyline plugin");
    unsafe {
        const NOP: u32 = 0xD503201F;
        patch_data(0x001335c4, &NOP).expect("Unable to patch 0x001335c4");
        patch_data(0x0013369c, &NOP).expect("Unable to patch 0x0013369c");
    } 
    skyline::install_hooks!(char_limit_textbox_hook, char_limit_log_hook);
    skyline::install_hooks!(ui_text_log_hook);
    skyline::install_hooks!(ui_text_send1_hook, ui_text_send2_hook);
}

#[skyline::hook(offset = 0x001335c4, inline)]
pub fn char_limit_textbox_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[1].w.as_mut() = 39; }
}

#[skyline::hook(offset = 0x0013369c, inline)]
pub fn char_limit_log_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[1].w.as_mut() = 39; }
}

#[skyline::hook(offset = 0x00136dc0, inline)]
pub fn ui_text_log_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[2].x.as_mut() = LOG_UITEXT.as_ptr() as u64; }
}

#[skyline::hook(offset = 0x000030ec, inline)]
pub fn ui_text_send1_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[2].x.as_mut() = SEND_UITEXT.as_ptr() as u64; }
}

#[skyline::hook(offset = 0x00133100, inline)]
pub fn ui_text_send2_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[20].x.as_mut() = SEND_UITEXT.as_ptr() as u64; }
}