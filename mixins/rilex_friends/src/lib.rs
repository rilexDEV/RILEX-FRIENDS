fn rilex_friends_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut HudBatch,
    corners: &mut Corners,
) {
    let _ = (game, batch, corners);
    crate::game::rilex_friends::on_hud();
}
