# Tauri Fullscreen Always on Top Release Bug

### Reproduce (Dev mode)

In dev mode
1. clone repo
1. `pnpm i`
1. `pnpm tauri dev`
1. run any app in fullscreen
1. window will join other workspaces

### Reproduce (Release)

In dev mode
1. clone repo
1. `pnpm i`
1. `pnpm tauri build`
1. run the tauri app in `src-tauri/target/release/bundle/macos/release-mode-fullscreen-bug.app`
1. run any app in fullscreen
1. window will not join other workspaces
