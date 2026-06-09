## Yuno Learner
Simple application that outputs an audio file when a letter is clicked.

Goal of the app is to learn how each letter of the Alphabet sounds.
Uses [Dear PyGui](https://github.com/hoffstadt/DearPyGui) toolkit.


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
More specifically, a system using Debian 13.5 as the OS, Hyprland 0.53.3 for the DE and nerdctl pn rootless mode for the containerization tool.
The following steps might not work if these do not match and behave unexpectedly or not start due to errors.

It is not neccesary to follow the app inside a container, but it might be beneficial for development and not install the extra dependencies on the system.
On the actual machine where the app will be used, it is recommended to run the code on the actual application.

Build and start the image:

    nerdctl build -t yuno-leaner:latest -f Dockerfile.dev .
    sh start-dev.sh

Do note that dearpygui requires a display server library in order to show the app.
Thats why the Dockerfile has those lines installing the whole dependencies.
The start-dev.sh will forward the non-root user's display. It requires X11.


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
The license for the actual code of the app is MIT.

### Fonts
This app uses the [Liberation Sans](https://github.com/liberationfonts/liberation-fonts) typeface, which is licensed under the [SIL Open Font License, Version 1.1](https://scripts.sil.org/OFL).

The font files and license are included in `assets/fonts/` for convenience.
