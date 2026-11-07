use std::sync::Once;

static FIRST_BREAK: Once = Once::new();

pub fn on_break_block() {
    FIRST_BREAK.call_once(|| {
        eprintln!("[RILEX FRIENDS] 2026: break_block hook reached");
    });
}
