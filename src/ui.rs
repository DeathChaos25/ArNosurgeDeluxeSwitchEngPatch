use std::io::Write;

use skyline::{hooks::InlineCtx, patching::nop_data};

use crate::reg_x;

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

#[skyline::hook(offset = 0x00379f0, inline)]
fn button_help_label_hook(ctx: &mut InlineCtx) {
    // Make sure the string we received is not empty, so we don't panic if it is
    if let Ok(label) = unsafe { skyline::try_from_c_str(reg_x!(ctx, 1) as _) } {
        // Turn the string into a crc32 early for reusability
        let crc32 = crc32fast::hash(label.as_bytes());


        let new_label: &str = match crc32 {
            // Sample
            // 0x69 => "Schlong",
            0xd79d3a50 => "<IM06>Menu",
            0xe0b77043 => "<IM30>Dash ON",
            0xf1984651 => "<IM02>Jump",
            0xa235d8ba => "<IM00>Save",
            0xe380ef16 => "<IM26>Move",
            0xeeb15882 => "<IM02>Leave",
            0xef15d583 => "<IM00>Confirm",
            _ => {
                // If we're not handling that crc32, open a file at the root of the SD with append permissions.
                let mut label_file = std::fs::File::options()
                .create(true)
                .append(true)
                .open("sd:/arnosurge_buttonhelp_labels.txt")
                .expect("Should have opened the arnosurge_buttonhelp_label.txt file");

                // Add the label and matching crc32 to the file
                label_file.write(format!("ButtonHelp label '{}' was received, matching CRC32: {:#x}\n", &label, crc32).as_bytes())
                .expect("Could not write ButtonHelp label to file");

                // Return the original label
                &label
            },
        };

        unsafe {
            // Return either the original label or our custom one, with a null-terminator
            *ctx.registers[1].x.as_mut() = skyline::c_str(&new_label.to_owned())) as u64;
        }
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
    skyline::install_hook!(button_help_label_hook);
    // skyline::install_hooks!(ui_text_log_hook, ui_text_send1_hook, ui_text_send2_hook, ui_text_send3_hook);
}
