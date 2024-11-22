# Demo car dashboard

# Build for Raspberry Pi 4/5

```shell
 cross build \
  --release \
  --bin car-dashboard \
  --target=aarch64-unknown-linux-gnu \
  --features slint/renderer-skia,slint/backend-linuxkms-noseat 
```

# Build for Web

```shell
wasm-pack build --release -d ../_site/pkg --target web dashboard/
```

# Upload

```shell
scp target/aarch64-unknown-linux-gnu/release/car-dashboard <rpi-user>@<rpi-ip>:.
```

# For debug

```plantuml
SLINT_DEBUG_PERFORMANCE=refresh_full_speed,overlay
```