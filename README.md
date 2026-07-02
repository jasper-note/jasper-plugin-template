# Jasper plugin template

Template for writing [Jasper](https://github.com/xVanTuring/jasper) plugins in Rust.
Plugins compile to `wasm32-unknown-unknown` and run inside the host's wasmi sandbox;
a plugin package (`.jplug`) is a zip of `manifest.toml` + `plugin.wasm` (+ assets).

The template ships a working before-save hook (trims trailing whitespace) with
native unit tests, a validating packager, and CI that builds and releases the
`.jplug` for you.

## Quick start

1. Click **Use this template** on GitHub (or clone this repo).
2. Rename things — `id`/`name`/`description`/`author` in `manifest.toml`, and
   `package.name` in `Cargo.toml` (keep it equal to the manifest `id`).
3. Write your plugin in `src/lib.rs`. The `register!` macro accepts any
   combination of four slots:
   - `before_save: fn(Note) -> Result<Note, String>` — rewrite notes on save
   - `storage: T` where `T: sdk::storage::Storage` — a sync-storage provider
   - `command: fn(&str, Value) -> Result<Value, PluginError>` — editor/sidebar commands
   - `ui: fn(&str, Value) -> Result<Value, PluginError>` — server-driven sidebar
     trees (returns a UiNode, spec §9.3)
4. Develop:

   ```sh
   cargo test                                            # native unit tests
   cargo build --release --target wasm32-unknown-unknown # build the wasm
   python3 scripts/package.py                            # validate + dist/<id>-<version>.jplug
   ```

5. Install it: Jasper top bar → plug icon → Install → pick the `.jplug` → enable
   (the user is asked to consent to each declared capability).

Reference material: [plugin spec](https://github.com/xVanTuring/jasper/blob/main/docs/plugin-spec.md)
(the contract), [example plugins](https://github.com/xVanTuring/jasper/tree/main/plugins-examples)
(hook / storage / command / theme), and the
[`jasper-plugin-sdk` docs](https://docs.rs/jasper-plugin-sdk).

## What the host provides

| Host API (via `sdk::host`) | Capability required |
|---|---|
| `log(level, msg)` | none |
| `now_ms()` — **the only clock in the sandbox** | none |
| `settings_get` / `settings_set` (plugin-scoped KV) | `settings` |
| `http_request` (HTTP(S) proxied by the host) | `host:http` |
| `notes_get` / `notes_search` / `notes_list_folders` | `notes:read` |
| `notes_upsert` / `notes_create` — confirm-first proposals by default (`pending=true` means *not yet written*) | `notes:write` |
| `ai_complete(messages, options)` — host-configured AI, keys never reach the plugin | `host:ai` |

`notes.*` and `ai.complete` are only available while handling a `command`/`ui`
dispatch (hooks and storage calls get `unsupported`).

Limits (enforced by the host): 64 MiB memory / 2 s CPU per call for normal
plugins, 256 MiB / 10 s for storage providers; wall-clock counts CPU only, so
time spent inside `host_call` IO is free. Every capability you declare is shown
to the user for consent at enable time — declare the minimum.

## Wasm-toolchain gotchas

- **No wasm-bindgen.** Dependencies that assume a JS environment can't run in
  the wasmi sandbox. Classic offender: `chrono` with default features — use
  `default-features = false, features = ["std"]`.
- **No clock, no entropy.** Use `sdk::host::now_ms()` for time (SigV4-style
  signing works fine with it); don't generate ids — the host does that.
- **Verify the wasm is clean.** `scripts/package.py` checks the import section:
  anything other than `joplin.host_call` (e.g. `__wbindgen_*`) fails the build.
- Errors from your plugin become API errors, but a plugin failure in
  `before-save` never loses the user's note — the host saves the original.

## Releasing

Bump `version` in `manifest.toml` (and ideally `Cargo.toml`), then:

```sh
git tag v0.1.0 && git push origin v0.1.0
```

The release workflow verifies the tag matches the manifest, runs tests, builds,
packages, and attaches `<id>-<version>.jplug` + its `.sha256` to a GitHub
Release. The zip is built deterministically, so the checksum is reproducible
from source.

## License

The template itself is MIT OR Apache-2.0 — use it freely; your plugin can be
under any license you like. Note that the SDK you link against
(`jasper-plugin-sdk`) is MIT OR Apache-2.0 as well, so no copyleft obligations
flow into your plugin.

---

## 中文速览

模板自带一个可工作的 before-save 插件（去行尾空白）+ 原生单测 + 校验打包脚本 + CI。
用法：GitHub 上点 **Use this template** → 改 `manifest.toml` 的 `id`/`name` 和
`Cargo.toml` 的 `package.name` → 在 `src/lib.rs` 写逻辑（`register!` 四槽可组合：
`before_save` / `storage` / `command` / `ui`）→ `cargo test` → 构建 wasm →
`python3 scripts/package.py` 出 `.jplug` → Jasper 插件面板安装。
发布：改 `manifest.toml` 版本号后推 `v<版本>` tag，CI 自动出 GitHub Release。
契约见 [plugin-spec.md](https://github.com/xVanTuring/jasper/blob/main/docs/plugin-spec.md)，
参考实现见 [plugins-examples](https://github.com/xVanTuring/jasper/tree/main/plugins-examples)。
坑：别引入会带 wasm-bindgen 的依赖（如 chrono 默认 feature）；沙箱无时钟用
`host::now_ms()`；打包脚本会检查 wasm import 是否干净。
