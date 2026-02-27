# binary

```
- xcode-select --install
- brew install node
- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- rustup target add aarch64-apple-darwin
- brew install zig
- cargo install cargo-zigbuild
- npm install -g pnpm
```
```
cd frontend
pnpm install
pnpm run build
cd ..
```
```
cargo build --release --target x86_64-apple-darwin --bin server
cargo build --release --target aarch64-apple-darwin --bin server
lipo -create target/x86_64-apple-darwin/release/server target/aarch64-apple-darwin/release/server -output ./target/vibe-kanban
```
```
➜  vibe-kanban git:(release/v0.0.160) ls -al target/x86_64-apple-darwin/release/server
-rwxr-xr-x  1 zzh  staff  73866196 Jan 26 23:27 target/x86_64-apple-darwin/release/server
➜  vibe-kanban git:(release/v0.0.160) ls -al target/aarch64-apple-darwin/release/server
-rwxr-xr-x  1 zzh  staff  72366000 Jan 27 09:15 target/aarch64-apple-darwin/release/server
➜  vibe-kanban git:(release/v0.0.160) ls -al target/vibe-kanban 
-rwxr-xr-x  1 zzh  staff  146241456 Jan 27 09:20 target/vibe-kanban
➜  vibe-kanban git:(release/v0.0.160) 
```
```
full: VIBE_KANBAN_DATA_DIR=/Users/zzh/Downloads/dkcp/vibe-kanban HOST=0.0.0.0 PORT=3000 VK_ALLOWED_ORIGINS="http://192.168.2.185:3000" vibe-kanban
used: VIBE_KANBAN_DATA_DIR=/Users/zzh/Downloads/dkcp/vibe-kanban HOST=0.0.0.0 PORT=3000 vibe-kanban
```
```
- https://github.com/BloopAI/vibe-kanban/issues/770
- https://github.com/korjavin/vibe-kanban/commit/96860f3de6425500ac8f1219580db71f7ab8195e#diff-da6498268e99511d9ba0df3c13e439d10556a812881c9d03955b2ef7c6c1c655
```
