## Yuno Learner
YunoLearner is an easy-to-use, lightweight and free desktop app for adults who need to relearn the alphabet.
It is designed for people recovering from aphasia, brain injury, or other conditions affecting literacy.
The minimal interface and large, clear letters are intentional: no animations or distractions like in apps intended for children.

The app is made in Tauri and kept simple and lightweight.

## Limitations
The languages of the audios are currently only available in German. This is noted and will hopefully get updated in due time.
Open a Discussion if you were to 


## Dev
### Requirements

- Rust
- npm

### Versions

- Tauri 4.6.2

### Running
I prefer to avoid hot-reloading, so I start the server with:

    npm run tauri dev -- --no-watch

But the "correct" way to do it is:

    npm run tauri dev

## Installation
I would recommend cloning the repository.
If using Windows, then using [Git Bash](https://git-scm.com/downloads) is a good option.

1. Open Git Bash and go to the path were the app should be installed.
For example

    cd Documents/

2. Clone the repository

    git clone https://github.com/alsen-ca/yuno_learner.git

## Licences
### Code
Copyright (C) 2026 Sebastian Alsen. Licensed under the [GPL-3.0](LICENSE).

### Audio
Audio assets in `src-tauri/assets/audios/` are proprietary. See `src-tauri/assets/audios/`.
