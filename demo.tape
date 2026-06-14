Output assets/screenshot.gif

Require cargo
Require tmux
Require echo

Set FontSize 15
Set Width 1200
Set Height 600
Set TypingSpeed 60ms
Set Theme "Dracula"
Set WindowBar Colorful
Set BorderRadius 10
Set Margin 20
Set MarginFill "#6B50FF"

# Setup the environment
Hide
Type "cargo build --release"
Enter
Type "mkdir -p demo_dir/src"
Enter
Type "echo 'fn main() {}' > demo_dir/src/main.rs"
Enter
Type "clear"
Enter
Show

# Start tmux
Type "tmux new -s livediff-demo"
Enter
Sleep 1s

# Split the window vertically (Ctrl+B, then %)
Ctrl+B
Type "%"
Sleep 1s

# Now we are in the right pane. Let's switch back to the left pane to start Livediff.
Ctrl+B
Left
Sleep 500ms

# Pane 0: Start Livediff
Type "./target/release/Livediff demo_dir"
Enter
Sleep 2s

# Switch back to the right pane
Ctrl+B
Right
Sleep 500ms

# Pane 1: Edit files
Type "echo '// Adding some comments' >> demo_dir/src/main.rs"
Enter
Sleep 2s

Type "echo 'fn helper() {}' >> demo_dir/src/main.rs"
Enter
Sleep 3s

Type "echo 'pub mod utils;' > demo_dir/src/lib.rs"
Enter
Sleep 2s

Type "echo 'pub fn process() {}' > demo_dir/src/utils.rs"
Enter
Sleep 4s

# Clean up
Hide
Ctrl+B
Type "d"
Sleep 500ms
Type "tmux kill-session -t livediff-demo"
Enter
Type "rm -rf demo_dir"
Enter
Show
