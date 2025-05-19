![Image](https://github.com/user-attachments/assets/6f94c942-e916-4cee-a4d8-0b593c965395)
# Fehther

A highly configurable wallpaper manager script built around Feh, using the OpenWeatherMap API to change your wallpaper based on weather conditions, sunrise, and sunset. When the weather changes, so does your wallpaper.

## Table of Contents

* [Modes](#modes)
* [Getting Started](#getting-started)
* [Configuration](#configuration)
    * [Settings](#settings)
    * [Modes](#modes-1)
    * [Weather Groups](#weather-groups)
    * [Folders](#folders)
* [Extra Information](#extra-information)
* [Troubleshooting](#troubleshooting)
* [Contributing](#contributing)
* [License](#license)

## Modes

Fehther supports several modes, which can be used individually or in combination:

###   Cycle Mode:

* Cycles through the wallpapers available in the configured folder.
* Can be used as a standalone mode or in combination with other modes.

###   Daytime Mode:

* Changes the wallpaper based on sunrise and sunset.
* After sunrise, the wallpaper is sourced from the configured daytime folder.
* After sunset, the wallpaper is sourced from the configured nighttime folder.

###   Golden Hour Mode:

* Changes the wallpaper for a set time after sunrise and before sunset.
* Sunrise and sunset folders can be configured in the config file.
* Golden hour timer defaults to 60 minutes after sunrise and 60 minutes before sunset but can be set to any value.

###   Weather Mode:

* Changes the wallpaper based on weather conditions.
* Weather conditions can be grouped into custom weather groups.
* Available weather types are: Clear, Scattered Clouds, Few Clouds, Broken Clouds, Overcast Clouds, Drizzle, Rain, Mist, Snow, and Thunder.
* Weather mode can be disabled for specific daytimes (e.g., enabled for day and night, but disabled for sunrise and sunset).

## Getting Started

1.  **Install Feh:**

    * Make sure Feh is installed. Feh is available for most Linux distributions through their respective package managers.
    * For more information on how to install Feh, visit: <https://github.com/derf/feh>

2.  **Install Rustup:**

    * Make sure rustup is installed:

        ```
        rustup -V
        ```

    * Install rustup if not installed:

        ```
        curl https://sh.rustup.rs -sSf | sh
        ```

3.  **Create an OpenWeatherMap Account:**

    * Create a free OpenWeatherMap account on <https://openweathermap.org> to obtain your OpenWeatherMap API key.

4.  **Download Fehther:**

    * Download the Fehther repository and navigate to the folder:

        ```
        git clone https://github.com/metamaxo/fehther && cd fehther
        ```

5.  **Copy the Configuration File:**

    * Copy the `config.ini` file into your configuration folder:

        ```
        sudo cp -r ./config.ini ~/.config/fehther/
        ```

6.  **Install the Executable:**

    * Move the fehther executable to a standard location and make it executable:

        ```
        sudo chmod +x ./fehther/fehther && sudo cp ./fehther/fehther /usr/bin
        ```

        *(Preferred: System-wide installation)*

        OR

        ```
        sudo chmod +x ./fehther/fehther && mkdir -p ~/.local/bin && sudo cp ./fehther/fehther ~/.local/bin
        ```

        *(Alternative: User-specific installation)*

7.  **Run Fehther:**

    * Execute Fehther from a convenient location. Ensure that the directory containing the `fehther` executable (e.g., `/usr/bin` or `$HOME/.local/bin`) is in your system's `PATH`.

    * For running Fehther from your i3 configuration, add the following line to `~/.config/i3/config`:

        ```
        exec --no-startup-id fehther
        ```

    * For running Fehther from Bash or Zsh, add the following line to the corresponding `.zshrc` or `.bashrc`:

        ```
        nohup fehther &
        ```

        * Add a note about the importance of sourcing the shell configuration file. This is crucial for the \`PATH\` changes to take effect.

8.  **(Optional) Remove Unnecessary Files:**

    * Remove the cloned repository:

        ```
        sudo rm -rf ~/fehther
        ```

9.  **Configuration:**

    * Fehther is now running. Edit the configuration file at \`~/.config/fehther/config.ini\` to customize its behavior.

## Configuration

Fehther offers many configuration options, most of which are optional. Here's a detailed explanation of each option. Ensure your folder layout matches the configured modes. The basic folder structure starts from your main wallpaper folder:

For example, a complete folder structure for day and night might look like this:

```bash
-home/user/wallpapers/
    ├── day/
    │   ├── clear/
    │   ├── mist/
    │   ├── rain/
    │   ├── slightly-cloudy/
    │   ├── very-cloudy/
    │   ├── snow/
    │   └── thunder/
    └── night/
        ├── clear/
        ├── mist/
        ├── rain/
        ├── slightly-cloudy/
        ├── very-cloudy/
        ├── snow/
        └── thunder/
```


###   Settings

* `key`:  **(Required)** Replace this with your OpenWeatherMap API key. A free key can be obtained from <https://openweathermap.org>.

* `city`:  **(Required)** Replace this with your current city for accurate weather data.

* `country`:  **(Required)** Use the two-letter country code for your location (e.g., "US", "CA", "GB").

* `path`:  **(Required)** Replace this with the absolute path to your main wallpaper folder. This is the root directory where Fehther will look for subfolders.

###   Modes

All modes can be combined. For example, you can use both weather mode and cycle mode simultaneously. Fehther will then cycle through wallpapers within the appropriate weather folder.

* `feh-mode`: Sets the Feh display mode. Available modes are:

    * `center`: Centers the image on the screen.
    * `fill`: Fills the entire screen, preserving aspect ratio.
    * `max`: Scales the image to the largest size that fits within the screen dimensions, preserving aspect ratio.
    * `scale`: Scales the image to fit the screen.
    * `tile`: Tiles the image to fill the screen.

* `daytime-mode`: If set to `true`, your wallpaper will change based on day and night.

* `golden-hour-mode`: If set to `true`, the wallpaper will change at sunrise and sunset.

* `golden-hour-time`: The duration (in minutes) after sunrise and before sunset that defines the "golden hour." Default is 60 minutes.

* `weather-mode`: If set to `true`, the wallpaper will change based on the current weather conditions.

* `disabled-daytime-modes`: A comma-separated list of daytimes for which weather mode should be disabled. For example, if you only want weather-based wallpapers during the day, set this to `sunrise sunset night`. Valid options are: `sunrise`, `day`, `sunset`, and `night`.

* `cycle-mode`: If set to `true`, Fehther will cycle through the wallpapers in the current folder.

* `cycle-timer`: Sets the interval (in minutes) for cycling through wallpapers in cycle mode. For example, setting this to `5` will change the wallpaper every 5 minutes.

###   Weather Groups

* `weather-groups`: Set to `true` to enable custom grouping of weather conditions.

    Fehther allows you to group weather types for more flexible wallpaper selection. You can define custom groups, and each weather type can belong to only one group. Groups are defined in the format `group-name = weather types`. For example:

    ```
    rainy = drizzle rain
    cloudy = broken-clouds overcast-clouds scattered-clouds
    clear = clear
    ```

    In this example, if the weather is "drizzle" or "rain", Fehther will use wallpapers from the \`rainy\` folder.

    **Folder Structure Example:**

    Given the `weather-groups` example above, and assuming `wallpaper_folder` is set to `/home/user/wallpapers`, your wallpaper folder structure might look like this:

    ```
    /home/user/wallpapers/
    ├── day/
    │   ├── clear/
    │   ├── cloudy/
    │   ├── rainy/
    │   └── ...
    └── night/
        ├── clear/
        ├── cloudy/
        ├── rainy/
        └── ...
    ```

    You can also rename single weather conditions to match your folder names:

    ```
    mist = foggy
    ```

###   Folders

Each folder must be located within the main wallpaper folder specified by the `path` setting.

* `custom-folder-names`: Set to `true` to use custom folder names for daytimes. If `false`, the default names (`day`, `night`, `sunrise`, `sunset`) will be used.

* `daytime-folder-name`: Custom folder name for daytime. Default is `day`.

* `nighttime-folder-name`: Custom folder name for nighttime. Default is `night`.

* `sunrise-folder-name`: Custom folder name for sunrise. Default is `sunrise`.

* `sunset-folder-name`: Custom folder name for sunset. Default is `sunset`.

## Extra Information

If you're having trouble finding high-resolution wallpapers to match your needs, I recommend using [unsplash.com](https://unsplash.com). There's no shortage of nice, free-to-use wallpapers there. If you have any questions or issues, please feel free to contact me. I'll try to respond as soon as possible.

## Troubleshooting

* **Permissions Issues:** If you encounter errors related to file permissions, ensure that you have the necessary permissions to read the wallpaper files and write to the configuration directory (`~/.config/fehther`). Using `sudo` for the copy and move commands, as shown in the installation instructions, should resolve most permission problems.

* **Feh Not Found:** If you get an error that `feh` cannot be found, double-check that it is correctly installed and that it's in your system's `PATH`. You can verify this by running `which feh` in your terminal. If it doesn't output a path, you'll need to install `feh` or add its installation directory to your `PATH`.

* **Config file not found**: If Fehther complains about the config file not being found, make sure that the file is located at `~/.config/fehther/config.ini`.

* **Wallpaper doesn't change**: If the wallpaper doesn't change, double-check that the paths in your config file are correct, and that the folder structure matches what you have configured.

* **Internet connection errors**: If you get errors about not being able to connect to the internet, check your internet connection. Fehther needs the connection to get the weather data.

## Contributing

If you'd like to contribute to Fehther, please feel free to submit bug reports, feature requests, or pull requests. When submitting a pull request:

1.  Fork the repository.
2.  Create a new branch for your changes.
3.  Make your changes and commit them with clear, descriptive commit messages.
4.  Push your branch to your fork.
5.  Submit a pull request to the main repository's \`main\` branch.

Please follow the existing code style and include relevant tests when possible.

