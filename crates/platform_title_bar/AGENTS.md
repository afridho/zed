# Linux MacTahoe window controls

This fork customizes Linux client-side window controls in `src/platforms/platform_linux.rs`. Keep the root `.rules` instructions in effect.

## Design and behavior

- Render the buttons with GPUI inside Zed's existing titlebar. Do not add a separate server-decorated titlebar or require GTK styling.
- Match the MacTahoe colors used in the owner's Zen Browser customization: close `#e9524a`, minimize `#f1ae1b`, maximize/restore `#59c837`.
- Circles are 14 px across with an 8 px gap. Show dark symbols on hover and darken the background while pressed. Inactive and disabled controls use the theme's disabled icon color.
- Preserve desktop-configured button order and placement; do not hard-code the left side.
- Preserve negotiated supported controls, disabled-action guards, native minimize/maximize/restore calls, and the caller-provided close action. In particular, do not replace the contextual close action with unconditional window destruction.
- Native Wayland is supported. Use `"window_decorations": "client"` to see these controls; server decorations bypass them.
- No extension API, configuration migration, or Wayland backend changes are required for this styling.

## Validation and release build

Run from the repository root:

```sh
cargo fmt --package platform_title_bar --check
CARGO_BUILD_JOBS=4 CARGO_PROFILE_DEV_DEBUG=0 cargo check -p platform_title_bar
CARGO_BUILD_JOBS=8 ZED_RELEASE_CHANNEL=stable CARGO_PROFILE_RELEASE_DEBUG=0 cargo build --release -p zed
```

The release command retains the repository's release optimization and Thin LTO settings while omitting debug symbols. Adjust the job count for available RAM; 8 jobs succeeded on the owner's 16-logical-CPU, approximately 28 GB RAM machine. Release compilation can be quiet for several minutes during the final binary optimization/link step. A successful focused check alone does not produce a runnable binary.

Compile the release channel as `stable` when replacing the installed stable editor. Release builds do not honor the development-only runtime `ZED_RELEASE_CHANNEL` override. Keep automatic updates disabled in the user's settings so an official binary does not overwrite this custom build.

Before replacing an installation, back up its executable, settings, and data. Preserve `~/.config/zed` and `~/.local/share/zed` (including extensions and workspace data). Do not pass a temporary `--user-data-dir` for normal use. Keep the installed CLI/launcher working and remove any previous `WAYLAND_DISPLAY=` override when switching back to native Wayland. Never silently terminate an editor with unsaved user work.

For visual verification, use an isolated `--user-data-dir` with a `config/settings.json` containing `"window_decorations": "client"`. Check active/inactive colors, hover/pressed states, minimize, maximize/restore, close confirmation, titlebar dragging, and fullscreen. The owner has visually tried the development build; repeat these checks when changing the implementation.

## Local installation used for this work

- Release output: `target/release/zed`.
- Installed custom executable: `~/.local/zed.app/libexec/zed-editor-custom`.
- Existing entrypoint: `~/.local/zed.app/libexec/zed-editor` (a shell wrapper forwarding arguments to the custom executable).
- Desktop launcher: `~/.local/share/applications/dev.zed.Zed.desktop`.

These are machine-local paths, not files to commit. Do not copy personal settings, tokens, databases, extension installations, or build artifacts into the repository.
