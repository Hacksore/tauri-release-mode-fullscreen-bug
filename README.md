# Tauri Fullscreen Always on Top Release Bug

### Reproduce (Dev mode)

In dev mode
1. clone repo
1. `pnpm i`
1. `pnpm tauri dev`
1. Run any app in fullscreen and all is well

### Reproduce (Release)

In dev mode
1. clone repo
1. `pnpm i`
1. `pnpm tauri build`
1. run the app in `src-tauri/target/release/bundle/macos/release-mode-fullscreen-bug.app`
