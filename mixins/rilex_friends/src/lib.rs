#[path = "runtime.rs"]
mod runtime;

pub fn on_game_tick() {
    runtime::on_game_tick();
}
