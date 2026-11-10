# VHS tape for the cyberpunk look demo

Output examples/vhs/checkbox_cyberpunk.gif

Set FontSize 20
Set Width 1200
Set Height 820
Set Padding 0
Set Theme "Catppuccin Mocha"

Hide
Type "clear && cargo run --release --example checkbox_cyberpunk"
Enter
Sleep 5s
Show
Sleep 1.5s

# Toggle a few systems (glitch + decrypt on every toggle)
Space
Sleep 1s
Down
Space
Sleep 1s
Down
Space
Sleep 1s

# Select all, from the indeterminate state
Up
Up
Up
Sleep 0.5s
Space
Sleep 1.2s
Space
Sleep 1.2s

# Cycle chamfers
Type "c"
Sleep 0.8s
Type "c"
Sleep 0.8s
Type "c"
Sleep 0.8s

# Hard shadow, then the block size
Type "h"
Sleep 1s
Type "s"
Sleep 1.5s
Down
Space
Sleep 1.2s

# Palettes: Neon, Arasaka, Synthwave
Type "p"
Sleep 1.2s
Type "p"
Sleep 1.2s
Type "p"
Sleep 1.2s
Down
Space
Sleep 1.5s

# Flat: cut corners, no shadow, no fills behind labels
Type "f"
Sleep 1.5s
Type "s"
Sleep 1.5s
Up
Sleep 1.5s

Hide
Type "q"
Sleep 0.3s
