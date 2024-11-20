# Demo car dashboard

# Build for Raspberry Pi 4/5

```shell
 cross build \
  --release \
  --bin car-dashboard \
  --target=aarch64-unknown-linux-gnu \
  --features slint/renderer-skia,slint/backend-linuxkms-noseat 
```

