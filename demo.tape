# VHS tape: Livediff v3.4.0 Showcase
# High contrast, large font, sleek modern terminal demo

Output demo.gif

Set Shell "bash"
Set FontSize 14
Set FontFamily "JetBrainsMono Nerd Font"
Set Width 1080
Set Height 640
Set Padding 12
Set Theme "Wez"
Set WindowBar Colorful
Set PlaybackSpeed 1.2
Set LineHeight 1.3

# 1. Start Livediff Demo Shell
Hide
Type "./write_examples.sh & ./livediff_demo"
Sleep 500ms
Enter
Show

# 2. Watch High-Throughput Burst Monitoring & Ratio Gauge
Sleep 4.5s

# 3. Smart Navigation across Polyglot Files
Sleep 500ms
Down 3
Sleep 1.2s
Down 3
Sleep 1.2s

# 4. Command Palette (Ctrl+P / :)
Type ":"
Sleep 1.5s
Type "split"
Sleep 1s
Enter
Sleep 2s

# 5. AST Symbol Inspector (o)
Type "o"
Sleep 1.5s
Down 1
Sleep 800ms
Enter
Sleep 1.5s

# 6. In-Diff Live Search (Ctrl+F)
Ctrl+F
Sleep 800ms
Type "pub"
Sleep 1.5s
Enter
Sleep 1s
Escape
Sleep 800ms

# 7. Dynamic Theme Palettes (Catppuccin, Tokyo Night, Nord, Gruvbox)
Type "t"
Sleep 1s
Type "t"
Sleep 1s
Type "t"
Sleep 1s
Type "t"
Sleep 1s

# 8. Multi-Tab Interactive Help Overlay (?)
Type "?"
Sleep 1.5s
Type "2"
Sleep 1s
Type "3"
Sleep 1s
Type "4"
Sleep 1s
Escape
Sleep 800ms

# 9. Quit
Type "q"
Sleep 500ms
Hide
Type "rm -f compiler.rs hooks.tsx server.go analytics.py migration.sql config.json styles.css guide.md Makefile Main.java app.rb mix.exs core.clj main.zig script.sh docker-compose.yml schema.graphql index.html data.csv api.proto tailwind.config.js App.swift Main.kt Application.java Main.cs App.scala Main.pl app.php dynamic.go *.patch"
Enter
Type "clear"
Enter
Show
Sleep 200ms
