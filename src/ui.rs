use skyline::{hooks::InlineCtx, patching::nop_data};

pub mod menus;

const LOG_UITEXT: &[u8] = &[0xef, 0xbc, 0xac, 0xef, 0xbd, 0x8f, 0xef, 0xbd, 0x87, 0x0d, 0x0a, 0x00, 0x00]; // Log, but wide
const SEND_UITEXT: &[u8] = &[0xef, 0xbc, 0xae, 0xef, 0xbd, 0x85, 0xef, 0xbd, 0x98, 0xef, 0xbd, 0x94, 0x00, 0x00]; // Next, but wide

#[skyline::hook(offset = 0x00133694, inline)]
fn char_limit_textbox_hook(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = 39;
    }
}

#[skyline::hook(offset = 0x0013376c, inline)]
fn char_limit_log_hook(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = 39;
    }
}

#[skyline::hook(offset = 0x00136e90, inline)]
fn ui_text_log_hook(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[2].x.as_mut() = LOG_UITEXT.as_ptr() as u64;
    }
}

#[skyline::hook(offset = 0x000030f8, inline)]
fn ui_text_send1_hook(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[2].x.as_mut() = SEND_UITEXT.as_ptr() as u64;
    }
}

#[skyline::hook(offset = 0x001331dc, inline)]
fn ui_text_send2_hook(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[2].x.as_mut() = SEND_UITEXT.as_ptr() as u64;
    }
}

#[skyline::hook(offset = 0x001331f0, inline)]
fn ui_text_send3_hook(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[2].x.as_mut() = SEND_UITEXT.as_ptr() as u64;
    }
}

fn nop_patches() {
    // NOP some UI-related instructions
    unsafe {
        nop_data(0x00133694).expect("Should have NOP'd the instruction");
        nop_data(0x0013376c).expect("Should have NOP'd the instruction");
        nop_data(0x003603dc).expect("Should have NOP'd the instruction");
    }
}

pub fn install_hook() {
    nop_patches();

    menus::install_hooks();

    skyline::install_hooks!(char_limit_textbox_hook, char_limit_log_hook);
    skyline::install_hook!(ui_text_log_hook);
    skyline::install_hooks!(ui_text_send1_hook, ui_text_send2_hook, ui_text_send3_hook);
}
