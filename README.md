## Yuno Learner
Simple application that outputs an audio file when a letter is clicked.

Goal of the app is to learn how each letter of the Alphabet sounds.
Uses [Dear PyGui](https://github.com/hoffstadt/DearPyGui) toolkit.

## Limitations
The languages of the audios are currently only available in German. This is noted and will hopefully get updated in due time.

In addition, there are issues regarding the size of the fonts regarding the size of the screen.
The app was created pretty quickly, as the actual logic is just some ~100 lines.
So the size of the font is set automatically instad of being something relative like some `rem`.
Doing this cleanly in python and dearpyhui specifically will not be worth the hassle for me, specially if the app countinues to get updated in the future.
If anything, this app will be rewrittento Tauri while it is still small. Do keep updated regarding that.

## Requirements
The system requires the following dependencies in order to work.
Changing one might cause unexpected behaviour or the compilation to fail.

    python3 (version: 3.13.13)
    pip (version: 26.1.2)
    pygame (version: 2.6.1)
    dearpygui (version: 2.3.1)

The instructions differ depending on Operating system, but for Windows, it goes more or less like this:

    1. Install python: https://www.python.org/downloads/windows/
    2. Confirm that python and pip are installed in the cmd
        python3 --version
        pip --version
    3. Install pygame
        pip install pygame
    4. Install dearpygui
        pip install dearpygui


## Installation
I would recommend cloning the repository.
If using Windows, then using [Git Bash](https://git-scm.com/downloads) is a good option.

1. Open Git Bash and go to the path were the app should be installed.
For example

    cd Documents/

2. Clone the repository

    git clone https://github.com/alsen-ca/yuno_learner.git


## Usage
### Add audio files
To use this Application, one should add the 26 individual audio files for each letter of the Alphabet under the location assets/audios.
If the audios folder does not exist, then it should be created.
For each letter of the alphabet, an individual file must be added.

Files must have the extension .ogg
File names must be named with the uppercase letter of the character, followed by the extension. Examples:

    - A.ogg
    - B.ogg
    - P.ogg
    - X.ogg


This Application does NOT come with the required audio files.
User must add them themselves.

### Starting the application
Assuming, during the 'Installation' steps, one cloned the Repository in the 'Documents/' path, then a folder '/Documents/yuno_learner' should have been created.

1. We open the same Git Bash terminal as before

2. We go to the 'yuno_learner' folder

    cd yuno_learner

3. We start the application
For Windows:

    python main.py

For Linux:

    python3 main.py

## Dev
The setup for the development environment is for Linux.
More specifically, a system using Debian 13.5 as the OS, Hyprland 0.53.3 for the DE, Pulseaduio 17.0 for audio and nerdctl on rootless mode for the containerization tool.
The start-dev.sh is written for this exact setup, so it might not work if these do not match and behave unexpectedly or not start the application at all.

It is not neccesary to have the app inside a container, but it might be beneficial for development and not install the extra dependencies on the system.
I would recommend to run the code natively on the end-user machine, and not use any container.

Build and start the image:

    nerdctl build -t yuno-leaner:latest -f Dockerfile.dev .
    sh start-dev.sh

Do note that dearpygui requires a display server library in order to show the app.
Thats why the Dockerfile has those lines installing the whole dependencies.
The start-dev.sh will forward the non-root user's display. It requires X11.
It will also require to forward the system's audio socket.


## Compatibility issues
Please note that this app has not been extensively tested.
The original version was written in Linux, so it should work semi-reliably there, at least for Fedora and Debian.

On Windows, there is an issue regarding the size of the fonts, which causes the App to crash without errors.
To fix it, change the value of font_big = dpg.add_font("assets/fonts/LiberationSans-Bold.ttf", 500)

to:

    font_big = dpg.add_font("assets/fonts/LiberationSans-Bold.ttf", 400)

That should be enough, but I am not sure whence the error comes from, maybe it isnt even a problem on the OS level, but rather the size of the screen.
If that should not have solved it, then one can decrease the size further and try it again.

It has not been tested for MacOS at all, but there might be some errors.

## Licences
### Code
The source code is licensed under the [MIT License](LICENSE).

### Fonts
This app uses the [Liberation Sans](https://github.com/liberationfonts/liberation-fonts) typeface, which is licensed under the [SIL Open Font License, Version 1.1](https://scripts.sil.org/OFL).

The font files and license are included in `assets/fonts/` for convenience.

### Audio
Audio assets in `assets/audios/` are proprietary. See `assets/audios/LICENSE`.
