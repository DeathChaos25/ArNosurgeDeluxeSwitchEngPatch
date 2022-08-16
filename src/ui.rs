use skyline::{hooks::InlineCtx, patching::{patch_data, nop_data, patch_str}};

const LOG_UITEXT: &[u8] = &[ 0xef, 0xbc, 0xac, 0xef, 0xbd, 0x8f, 0xef, 0xbd, 0x87, 0x0d, 0x0a, 0x00, 0x00]; // Log, but wide
const SEND_UITEXT: &[u8] = &[ 0xef, 0xbc, 0xae, 0xef, 0xbd, 0x85, 0xef, 0xbd, 0x98, 0xef, 0xbd, 0x94, 0x00, 0x00]; // Next, but wide

#[skyline::hook(offset = 0x00133694, inline)]
fn char_limit_textbox_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[1].w.as_mut() = 39; }
}

#[skyline::hook(offset = 0x0013376c, inline)]
fn char_limit_log_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[1].w.as_mut() = 39; }
}

#[skyline::hook(offset = 0x00136e90, inline)]
fn ui_text_log_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[2].x.as_mut() = LOG_UITEXT.as_ptr() as u64; }
}

#[skyline::hook(offset = 0x000030ec, inline)]
fn ui_text_send1_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[2].x.as_mut() = SEND_UITEXT.as_ptr() as u64; }
}

#[skyline::hook(offset = 0x001331d0, inline)]
fn ui_text_send2_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[20].x.as_mut() = SEND_UITEXT.as_ptr() as u64; }
}

fn nop_patches() { // NOP some UI-related instructions
    unsafe {
        nop_data(0x00133694).expect("Should have NOP'd the instruction");
        nop_data(0x0013376c).expect("Should have NOP'd the instruction");
    }
}

fn overwrite_names() { // Names that can be patched in place
    unsafe {
        patch_str(0x006182b8, "Ion").expect("Should have patched the string at 0x006182b8");
        patch_str(0x006182bf, "Delta").expect("Should have patched the string at 0x006182bf");
        patch_str(0x006182c9, "Cass").expect("Should have patched the string at 0x006182c9");
    } 
}

pub fn install_hook() {

    nop_patches();
    overwrite_names();

    skyline::install_hooks!(char_limit_textbox_hook, char_limit_log_hook);
    skyline::install_hook!(ui_text_log_hook);
    skyline::install_hooks!(ui_text_send1_hook, ui_text_send2_hook);
}