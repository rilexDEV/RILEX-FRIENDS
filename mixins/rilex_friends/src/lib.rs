#[path = "runtime.rs"]
mod runtime;

fn on_render_frame() {
    runtime::on_render_frame();
}
