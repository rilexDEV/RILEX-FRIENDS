fn rilex_friends_register(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: "rilex_friends".into(),
        title: "RILEX FRIENDS 1.1.4".into(),
        description: "Right Shift opens or closes the Friends window. Escape closes it.".into(),
        keys: Vec::new(),
    });
}
