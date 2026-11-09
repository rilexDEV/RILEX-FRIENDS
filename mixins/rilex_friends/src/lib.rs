fn rilex_friends_hud(game: &mut crate::game::Game<'_>, batch: &mut HudBatch) {
    if !crate::game::rilex_friends::is_open(
        game.native_on("rilex_friends") && game.native_playing(),
    ) {
        return;
    }
    let viewport = [batch.width, batch.height];
    crate::game::rilex_friends_gui::draw(
        batch,
        viewport,
        |text| game.native_text_width(text),
        |batch, rect, argb| game.native_fill(batch, rect, argb),
        |batch, text, at, rgb| game.native_text(batch, text, at, rgb, false),
    );
}
