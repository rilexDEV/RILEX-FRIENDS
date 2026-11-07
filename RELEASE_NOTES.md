# 🎮 RILEX FRIENDS MOD - Official Release (v1.0.0)

Welcome to the first official **Clean Build** of RILEX FRIENDS, a native friend system built exclusively for **Horizon Custom by Rilex**!

## 🚀 What's New in this Release?
- **Clean Architecture:** Rebuilt from scratch to remove all legacy errors and manifest conflicts.
- **Fixed TOML Parsing:** The `mixins_revision` bug is officially dead. The launcher will compile the mod smoothly.
- **Cloud Backend Integration:** The mod is fully connected to our Render API (`https://rilex-server.onrender.com`). No more local servers or ngrok links needed!
- **Dependencies Added:** Rust will now properly fetch `egui` for the UI and `reqwest` for networking.

## 🛠️ How to Install
1. Open **Horizon Custom by Rilex**.
2. Go to the **Mods** tab.
3. Paste this repository URL: `https://github.com/rilexDEV/RILEX-FRIENDS`
4. Click **Add from GitHub** and then hit **Build**.
5. Click "I trust these mods" to start the compilation.

## ⚠️ Current Status
The mod is currently hooked into the game's block-breaking event (`break_block`) for testing. When you break a block, the console will print a success message. The full UI (Friend List) will be mapped to the `render_frame` hook in the next update!

## 👨‍💻 Developer
- **RilexDEV**
- TikTok: `@rilexthe_1_andonly`

*Stay tuned for the UI update!*
