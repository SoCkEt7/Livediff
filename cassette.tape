# Output settings
Output assets/screenshot.gif

Set Shell "bash"
Set FontSize 16
Set Width 1200
Set Height 600
Set Padding 20
Set Theme "Dracula"
Set PlaybackSpeed 1.0

# 1. Type installation command
Type "cargo install --path ."
Sleep 500ms
Enter
# Allow time to build and install
Sleep 6s

# 2. Clear screen
Type "clear"
Enter
Sleep 1s

# 3. Set up the background trigger to modify scratch.rs
Type "(sleep 2 && echo -e '\n// This change triggers livediff dynamically!' >> scratch.rs) &"
Enter
Sleep 500ms

# 4. Launch livediff
Type "livediff"
Enter

# 5. Let livediff run and show the change for 5 seconds
Sleep 5s

# 6. Quit livediff
Type "q"
Sleep 1s

# 7. Clean up background job and scratch.rs
Type "rm -f scratch.rs"
Enter
Sleep 1s
