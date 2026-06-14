#!/bin/bash

# livediff - Real-time File Monitoring Tool for Developers
# https://github.com/SoCkEt7/livediff

# ---------------------------------------------------------
# USAGE:
#   ./install.sh          - Install livediff globally (using Cargo)
#   ./install.sh remove   - Remove global installation
# ---------------------------------------------------------

VERSION="0.1.0"

# Print banner
print_banner() {
  echo -e "\033[1;36m"
  echo "   ______          __    __                 "
  echo "  / ____/___  ____/ /___/ /__  ____  _____ "
  echo " / /   / __ \/ __  / __  / _ \/ __ \/ ___/ "
  echo "/ /___/ /_/ / /_/ / /_/ /  __/ / / (__  )  "
  echo "\____/\____/\__,_/\__,_/\___/_/ /_/____/   "
  echo -e "\033[0m"
  echo -e "\033[1;32mReal-time file monitoring for developers (v$VERSION) [Rust Edition]\033[0m"
  echo
}

# Cleanup function (removes old installations if they exist)
cleanup_previous() {
  echo "Cleaning up previous installations..."

  # Check for pnpm global bin directory
  if command -v pnpm >/dev/null 2>&1; then
    PNPM_BIN=$(pnpm root -g 2>/dev/null)/../../bin
    for b in Livediff livediff; do
      if [ -L "$PNPM_BIN/$b" ]; then
        echo "  Removing previous pnpm link ($b)..."
        rm -f "$PNPM_BIN/$b"
      fi
    done
  fi

  # Check for npm global installations
  if command -v npm >/dev/null 2>&1; then
    NPM_BIN=$(npm bin -g 2>/dev/null)
    for b in Livediff livediff; do
      if [ -f "$NPM_BIN/$b" ] || [ -L "$NPM_BIN/$b" ]; then
        echo "  Removing previous npm installation ($b)..."
        npm uninstall -g "$b" 2>/dev/null || true
      fi
    done
  fi

  # Remove from common bin locations
  for BIN_DIR in /usr/local/bin ~/.local/bin ~/.local/share/pnpm; do
    for b in Livediff livediff; do
      if [ -L "$BIN_DIR/$b" ] || [ -f "$BIN_DIR/$b" ]; then
        echo "  Removing legacy binary at $BIN_DIR/$b..."
        rm -f "$BIN_DIR/$b"
      fi
    done
  done

  # Remove legacy aliases
  for RC_FILE in ~/.bashrc ~/.zshrc ~/.profile ~/.bash_profile; do
    if [ -f "$RC_FILE" ]; then
      for b in Livediff livediff; do
        if grep -q "alias $b=" "$RC_FILE" 2>/dev/null; then
          echo "  Removing old alias ($b) from $RC_FILE..."
          sed -i.bak "/alias $b=/d" "$RC_FILE" 2>/dev/null || sed -i '' "/alias $b=/d" "$RC_FILE" 2>/dev/null
        fi
      done
    fi
  done

  echo "  Cleanup and alias update complete!"
  echo
}

# Install function
install_livediff() {
  print_banner

  echo "Installing livediff v$VERSION..."
  echo

  # Check for Cargo
  if ! command -v cargo >/dev/null 2>&1; then
    echo -e "\033[1;31mError: Cargo is not installed!\033[0m"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
  fi

  # Get script directory
  SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
  cd "$SCRIPT_DIR"

  # Cleanup legacy installations
  cleanup_previous

  # Install using Cargo
  echo "Building and installing with Cargo..."
  cargo install --path .

  if [ $? -ne 0 ]; then
    echo -e "\033[1;31mError: Failed to install livediff via Cargo\033[0m"
    exit 1
  fi

  echo
  echo -e "\033[1;32m✓ Installation complete!\033[0m"
  echo
  echo -e "\033[1;36mlivediff v$VERSION is now installed globally via Cargo\033[0m"
  echo

  # Test installation
  echo "Testing installation..."
  if command -v livediff >/dev/null 2>&1; then
    echo -e "\033[1;32m✓ livediff command is available\033[0m"
    LIVEDIFF_PATH=$(which livediff)
    echo "  Installed at: $LIVEDIFF_PATH"
  else
    echo -e "\033[1;33m⚠ Warning: livediff command not found in PATH\033[0m"
    echo "  Ensure ~/.cargo/bin is in your PATH."
  fi

  # Setup optional alias
  echo
  echo -e "\033[1;36mOptional: Set up an alias for livediff (e.g., 'ld')\033[0m"
  read -p "Enter alias name [leave blank to skip]: " custom_alias
  
  if [ -n "$custom_alias" ]; then
    ALIAS_CMD="alias $custom_alias='livediff'"
    for RC_FILE in ~/.bashrc ~/.zshrc ~/.profile ~/.bash_profile; do
      if [ -f "$RC_FILE" ]; then
        # Remove any existing alias with the same name
        sed -i.bak "/alias $custom_alias=/d" "$RC_FILE" 2>/dev/null || sed -i '' "/alias $custom_alias=/d" "$RC_FILE" 2>/dev/null
        echo "$ALIAS_CMD" >> "$RC_FILE"
        echo "  Added alias '$custom_alias' to $RC_FILE"
      fi
    done
    echo -e "\033[1;32m✓ Alias set up successfully.\033[0m"
    echo "Please restart your terminal or source your shell config (e.g., 'source ~/.bashrc') to use it."
  fi

  echo
  echo -e "\033[1;33mUsage:\033[0m"
  echo "  cd /path/to/your/project"
  if [ -n "$custom_alias" ]; then
    echo "  $custom_alias"
  else
    echo "  livediff"
  fi
  echo
  echo "Happy coding!"
}

# Remove function
remove_livediff() {
  print_banner

  echo -e "\033[1;31mUninstalling livediff...\033[0m"
  echo

  if ! command -v cargo >/dev/null 2>&1; then
    echo "Cargo not found. Checking binary directly..."
  else
    cargo uninstall livediff
  fi

  echo
  echo -e "\033[1;32mUninstallation complete!\033[0m"
  echo
}

# Version function
show_version() {
  echo "livediff v$VERSION (Rust)"
  echo "https://github.com/SoCkEt7/livediff"
}

# Help function
show_help() {
  print_banner
  echo "livediff - A lightweight file monitoring tool for developers"
  echo
  echo "USAGE:"
  echo "  ./install.sh              Install globally as 'livediff'"
  echo "  ./install.sh remove       Remove global installation"
  echo "  ./install.sh --help       Show this help message"
  echo "  ./install.sh --version    Show version information"
  echo
}

# Main
case "$1" in
  install|"")
    install_livediff
    ;;
  remove)
    remove_livediff
    ;;
  --help|-h)
    show_help
    ;;
  --version|-v)
    show_version
    ;;
  *)
    echo "Unknown command: $1"
    echo "Use --help for usage information"
    exit 1
    ;;
esac

exit 0
