fn rilex_friends_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::rilex_friends::on_key(
        key,
        pressed,
        game.native_on("rilex_friends") && game.native_playing(),
    );
}
