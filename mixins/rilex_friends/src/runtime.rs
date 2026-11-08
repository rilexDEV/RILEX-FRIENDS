use std::sync::Once;

static FIRST_HUD: Once = Once::new();

pub fn on_hud() {
    FIRST_HUD.call_once(|| {
        log::info!("[RILEX FRIENDS] HUD Hook Activated Successfully!");
    });
}
