## Yuno Learner
YunoLearner is an easy-to-use, lightweight and free desktop app for adults who need to relearn the alphabet.
It is designed for people recovering from aphasia, brain injury, or other conditions affecting literacy.
The minimal interface and large, clear letters are intentional: no animations or distractions like in apps intended for children.

The app is made in Tauri and kept simple and lightweight.

### Downloading
The goal will be for users to install a single binary directly.

For now, I will try uploading a release to codeberg for Linux systems.
It will hopefully include 3 versions:

#### Linux
- **.deb** - Debian/Ubuntu-based distros
```bash
sudo dpkg -i yuno-learner_0.1.0_amd64.deb
```
Launch from your application menu afterward. To uninstall:
```bash
sudo dpkg -r yuno-learner
```

- **.rpm** - Fedora/openSUSE-based distros
```bash
sudo rpm -i yuno-learner-0.1.0-1.x86_64.rpm
```
- **AppImage** - Other Linux distros
```bash
chmod +x yuno-learner_0.1.0_amd64.AppImage
./yuno-learner_0.1.0_amd64.AppImage
```
Or double-click in your file manager after marking it executable.

### Windows
Download `yuno-learner_0.1.0_x64-setup.exe` and run it to install.

Windows may show a security warning on first run. Click on "More info" -> "Run anyway" to proceed.

This repository is [mirrored on Github](https://github.com/alsen-ca/learner), which is used to build the releases on Windows.

## Limitations
The languages of the audios are currently only available in German. This is noted and will hopefully get updated in due time.
Open a Discussion if you were to specifically require this or want to contribute.

## Dev
### Requirements
View the official documentation for details about Tauri's requirements: https://v2.tauri.app/start/prerequisites/

- Rust
- cargo
- npm
- ALSA development files (for linux)
    * Debian/Ubuntu: `libasound2-dev`
    * Fedora: `alsa-lib-devel`
    * Arch: `alsa-lib`

### Running
I prefer to avoid hot-reloading, so I start the server with:

    npm run tauri dev -- --no-watch

But the "correct" way to do it would be:

    npm run tauri dev

### Compiling
For those that can't o do don't wish to install the binary directly, they can compile the app themselves.

    npm run tauri build -- --no-bundle
    cd src-tauri
    target/release/yuno-learner

## Licenses
### Code
Copyright (C) 2026 Sebastian Alsen. Licensed under the [GPL-3.0](LICENSE).

### Audio
Audio assets in `src-tauri/assets/audios/` are proprietary. See `src-tauri/assets/audios/LICENSE`.

### Icons
The frontend uses icons from [Lucide](https://lucide.dev/), licensed under the [ISC License](https://github.com/lucide-icons/lucide/blob/main/LICENSE).
