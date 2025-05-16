Fehther

A highly configurable wallpaper manager script built around Feh using the open weather API to change your wallpaper based on changes in weather, sunrise and sunset. When the weather changes, so does your wallpaper.

______________________________________________________________________

Modes: 

Cycle Mode: Cycles through the wallpapers available in the configured folder. Cycle mode can be used as a standalone mode or in combination with other modes. 

Daytime Mode: Changes the wallpaper based on sunrise and sunset. After sunrise the wallpaper will be sourced from the configured daytime folder, after sunset the wallpaper will be sourced from the configured nighttime folder. 

Golden Hour Mode: Changes the wallpaper for a set time after sunrise and before sunset. Sunrise and sunset folders can be configured in the config file. Golden hour timer defaults to 60 minutes after sunrise and 60 minutes before sunset, but can be set to any value. 

Weather Mode: Changes wallpaper based on weather. Weather can be grouped together in custom weather groups. The available weather types are: Clear, Scattered Clouds, Few Clouds, Broken Clouds, Overcast Clouds, Drizzle, Rain, Mist, Snow and thunder. Weather mode can be disabled for specific daytimes. For example, weather mode can be active for night and day, but turned off for sunrise and sunset. 

Feh Modes: All available Feh background modes are supported, the modes are: center, fill, max, scale or tile. 

______________________________________________________________________

Getting Started:

1.  Make sure Feh is installed. Feh is available for most linux distro's through their respective application managers. For more info on how to install feh, visit: [https://github.com/derf/feh]

2.  Make sure rustup is installed: 
    ```
    rustup -V
    ```
    Install rustup if not installed: 
    ```
    'curl https://sh.rustup.rs -sSf | sh'
    ```
3.  Create a free open weather account on [https://openweathermap.org] to create your Open Weather API key. 

4.  Download the Fehther repository and cd into the folder: 
```
    ~ $ git clone https://github.com/metamaxo/fehther && cd fether
```
5.  Copy the config.ini file into your .config folder: 
```
    cp -r fehther_config ~/.config/
```
6.  Build fehther: 
```
    cargo build --release 
```
7.  move fehther into .bin and make executable 
```
    $ chmod +x ./target/release/fehther && mv ./target/release/fether /home/<user>/.bin/fehther/
```
8.  execute fehther from wherever is convenient. For example:

  for running fehther from your i3 config add the following line to ~/.config/i3/ :
```
    exec --no-startup-id nohup /.bin/fehther/fehther
```
  for running fehther from bash or zsh add the folling line to the corresponding .zshrc or .bashrc: 
```
    nohup /.bin/fehther/fehther &
```
9. (optional) remove unnecessary files: 
```
    sudo rm -rf ~/fehther
```
10. fehther is now running, for configuration edit the .config/fehter/config.ini file

______________________________________________________________________

Configuration: 

    




