# Makefile for Fehther Installation

# Variables
CONFIG_DIR := ~/.config/fehther
BIN_DIR := ~/.bin
FEHTHER_REPO := https://github.com/metamaxo/fehther
FEHTHER_DIR := fehther  # Name of the cloned directory
FEHTHER_EXEC := $(FEHTHER_DIR)/fehther # Path to the executable within the repo

# Default target
all: install

# 1. Install Feh
feh:
	@echo "1. Make sure Feh is installed."
	@echo "   Feh is available for most Linux distributions through their respective package managers."
	@echo "   For more info on how to install Feh, visit: https://github.com/derf/feh"
	@echo "   If you do not have feh installed, please install it and run 'make install' again."

# 2. Install rustup
rustup:
	@echo "2. Make sure rustup is installed:"
	@command -v rustup >/dev/null 2>&1 || ( \
		echo "   Installing rustup..." && \
		curl https://sh.rustup.rs -sSf | sh -s -- -y && \
		source $$HOME/.cargo/env \
	) && echo "   rustup is installed." || echo "   Failed to install rustup."
	rustup -V # verify installation


# 3. Download the Fehther repository
clone:
	@echo "3. Downloading the Fehther repository..."
	git clone $(FEHTHER_REPO) && cd $(FEHTHER_DIR) || { \
		echo "   Failed to clone the repository.  Please check your internet connection and try again, or clone manually and run make install from within the repo." ; exit 1; \
	}
	@echo "   Repository cloned."

# 4. Copy the config.ini file
config: clone
	@echo "4. Copying the config.ini file..."
	mkdir -p $(CONFIG_DIR) #create the directory if it does not exist
	sudo cp -r $(FEHTHER_DIR)/config.ini $(CONFIG_DIR)/ || { \
	  echo "   Failed to copy config.ini.  Please ensure you have permissions to write to $(CONFIG_DIR) and try again." ; exit 1; \
	}
	@echo "   config.ini copied to $(CONFIG_DIR)"

# 5. move fehther into .bin and make executable
move_executable: clone
	@echo "5. Moving fehther into $(BIN_DIR) and making it executable..."
	sudo chmod +x $(FEHTHER_EXEC) || { echo "Failed to change permissions on the executable"; exit 1; }
	mkdir -p $(BIN_DIR) # create the directory
	sudo cp $(FEHTHER_EXEC) $(BIN_DIR)/fehther || { echo "Failed to copy the executable"; exit 1; }
	@echo "   fehther moved to $(BIN_DIR)"

# 6.  i3 and zsh/bash instructions
config_instructions:
	@echo "6.  Next steps:"
	@echo "   -  Execute fehther from wherever is convenient."
	@echo "   -  For running fehther from your i3 config, add the following line to ~/.config/i3/config :"
	@echo "     exec --no-startup-id  $(BIN_DIR)/fehther"
	@echo "   -  For running fehther from bash or zsh, add the following line to the corresponding .zshrc or .bashrc:"
	@echo "     nohup $(BIN_DIR)/fehther &"
	@echo "   -  Make sure to source your zsh/bash config after adding the line (e.g., 'source ~/.zshrc')."

# 7. (optional) remove unnecessary files
remove_files: clone
	@echo "7. (Optional) Removing unnecessary files..."
	sudo rm -rf $(FEHTHER_DIR)
	@echo "   Removed cloned repository."

# 8. Finish message
finish:
	@echo "8. Fehther is now ready to run. For configuration, edit $(CONFIG_DIR)/config.ini"

# Installation target
install: feh rustup openweathermap clone config move_executable config_instructions finish

.PHONY: all feh rustup openweathermap clone config move_executable config_instructions remove_files finish install
