use std::io::Write;

use skyline::{hooks::InlineCtx, patching::Patch};

use crate::reg_x;

pub mod menus;

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

#[skyline::hook(offset = 0x00379f0, inline)]
fn button_help_label_hook(ctx: &mut InlineCtx) {
    // Make sure the string we received is not empty, so we don't panic if it is
    if let Ok(label) = unsafe { skyline::try_from_c_str(reg_x!(ctx, 1) as _) } {
        // Turn the string into a crc32 early for reusability
        let crc32 = crc32fast::hash(label.as_bytes());

        let new_label: &str = match crc32 {
            // Sample
            // 0x69 => "Schlong",
            0x96b58f65 => "<IM02><IM00>Next",
            0x899f17bd => "<IM06>Log",
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
                label_file
                    .write(format!("ButtonHelp label '{}' was received, matching CRC32: {:#x}\n", &label, crc32).as_bytes())
                    .expect("Could not write ButtonHelp label to file");

                // Return the original label
                &label
            },
        };

        unsafe {
            // Return either the original label or our custom one, with a null-terminator
            let c_label = std::ffi::CString::new(new_label).unwrap();

            // TODO: Free the original pointer
            *ctx.registers[1].x.as_mut() = c_label.as_ptr() as u64;

            // Tell the compiler not to free our string
            std::mem::forget(c_label);
        }
    }
}

fn nop_patches() {
    // NOP some UI-related instructions
    Patch::in_text(0x00133694).nop().expect("Should have NOP'd the instruction");
    Patch::in_text(0x0013376c).nop().expect("Should have NOP'd the instruction");
    Patch::in_text(0x003603dc).nop().expect("Should have NOP'd the instruction");
}

pub fn install_hook() {
    nop_patches();

    menus::install_hooks();

    skyline::install_hooks!(char_limit_textbox_hook, char_limit_log_hook);
    skyline::install_hook!(button_help_label_hook);
}
