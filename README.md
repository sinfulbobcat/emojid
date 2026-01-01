# 😄 emojid

[!WARNING] WIP, Clone Compile and Develop, nothing but that!

**Fast, keyboard-first emoji & kaomoji picker for Wayland (KDE Plasma)**

`emojid` is a minimal, instant emoji picker designed for **Wayland**, built in **Rust** using **egui/eframe**.
It opens instantly, lets you search with fuzzy matching, and pastes reliably using `ydotool`.

No Electron. No lag. No mouse required.

---

## ✨ Features

* ⚡ **Instant popup** (launcher-style)
* ⌨️ **Keyboard-first workflow**
* 🔍 **Fuzzy search** (type meaning, not exact characters)
* 😀 **Emoji + Kaomoji support**
* 📂 **Categories** (persistent across launches)
* 🔁 **Live config reload** (`Ctrl + R`)
* 📋 **Reliable paste on Wayland**
* 🧠 **Remembers last category**
* 🪶 **Single native binary**

---

## 🖥️ Screenshots

*(Optional — add later)*

---

## 📦 Dependencies

### Runtime

* `ydotool` (required for paste on Wayland)

### Build

* Rust (stable)
* Cargo

---

## 🔧 Installation (Arch / CachyOS)

### 1️⃣ Install dependencies

```bash
sudo pacman -S rust ydotool
```

Enable ydotool daemon:

```bash
sudo systemctl enable --now ydotool
```

> ⚠️ ydotool requires access to input devices.
> On Arch-based systems, this is handled automatically when the service is running.

---

### 2️⃣ Build emojid

```bash
git clone https://github.com/yourname/emojid.git
cd emojid
cargo build --release
```

### 3️⃣ Install binary

```bash
sudo install -Dm755 target/release/emojid /usr/bin/emojid
```

---

## ⌨️ Keybindings

| Key              | Action                     |
| ---------------- | -------------------------- |
| Type             | Fuzzy search               |
| Enter            | Copy + paste selected item |
| Esc              | Close picker               |
| ↑ / ↓            | Navigate items             |
| A / D            | Switch categories          |
| Tab / Shift+Tab  | Cycle categories           |
| Ctrl + R         | Reload config              |
| Ctrl + Backspace | Clear search               |

---

## 🚀 Usage

Run directly:

```bash
emojid
```

Recommended KDE shortcut:

* **Meta + .**

---

## ⚙️ Configuration

Config file location:

```
~/.config/emojid/config.toml
```

### Example config

```toml
[last]
category = "Kaomoji"

[[category]]
name = "Emoji"
items = ["😀","😂","🔥","✨","❤️"]

[[category]]
name = "Kaomoji"
items = [
  "(╯°□°）╯︵ ┻━┻",
  "¯\\_(ツ)_/¯",
  "(•_•)",
  "(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧"
]
```

### Live reload

After editing the config:

```
Ctrl + R
```

No restart needed.

---

## 🧠 How paste works (Wayland-safe)

1. Emoji is copied to clipboard
2. Picker window closes
3. After a short delay, `ydotool` simulates **Ctrl + V**
4. Paste happens in the previously focused app

This avoids focus issues common on Wayland.

---

## 🛠️ Development

### Fast dev builds

```bash
cargo run
```

Recommended `Cargo.toml` tweaks:

```toml
[profile.dev]
opt-level = 1
incremental = true
```

---

## 🐛 Troubleshooting

### Paste doesn’t work

* Ensure `ydotool` is running:

  ```bash
  systemctl status ydotool
  ```
* Try pasting manually after closing the picker to verify clipboard works

### App doesn’t open

* Run from terminal to see errors:

  ```bash
  emojid
  ```

---

## 🗺️ Roadmap

* Emoji annotations (🔥 → fire)
* Match highlighting
* Custom keybindings
* History / recent emojis
* System tray mode

---

## 📜 License

MIT License

---

## ❤️ Credits

Built with:

* Rust
* egui / eframe
* ydotool


