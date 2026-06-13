## Yuno Learner
YunoLearner is an easy-to-use, lightweight and free desktop app for adults who need to relearn the alphabet.
It is designed for people recovering from aphasia, brain injury, or other conditions affecting literacy.
The minimal interface and large, clear letters are intentional: no animations or distractions like in apps intended for children.

The app is made in Tauri and kept simple and lightweight.

### Downloading
The goal will be for users to install a single binary directly.
This would be platform-dependent and systems will probably not be supported directly.

The app would be downloadable from either GitHubs releases or a separate website; still to be decided.

## Limitations
The languages of the audios are currently only available in German. This is noted and will hopefully get updated in due time.
Open a Discussion if you were to specifically require this or want to contribute.

## Dev
### Requirements

- Rust
- cargo
- npm

View the official documentation for more details: https://v2.tauri.app/start/prerequisites/

### Running
I prefer to avoid hot-reloading, so I start the server with:

    npm run tauri dev -- --no-watch

But the "correct" way to do it would be:

    npm run tauri dev

### Compiling
For those that can't o do don't wish to install the binary directly, they can compile the app themselves.
When I would be at the point, I will give instructions here on how to do that.

## Licenses
### Code
Copyright (C) 2026 Sebastian Alsen. Licensed under the [GPL-3.0](LICENSE).

### Audio
Audio assets in `src-tauri/assets/audios/` are proprietary. See `src-tauri/assets/audios/LICENSE`.

### Icons
The frontend uses icons from [Lucide](https://lucide.dev/), licensed under the [ISC License](https://github.com/lucide-icons/lucide/blob/main/LICENSE).
