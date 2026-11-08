#[path = "runtime.rs"]
mod runtime;

fn on_gui_layer_pre() {
    runtime::on_gui_layer_pre();
}
