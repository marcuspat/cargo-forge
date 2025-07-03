# Assets Directory

This directory contains game assets organized by type:

- `models/` - 3D models (.gltf, .obj, .fbx)
- `textures/` - Texture files (.png, .jpg, .hdr)
- `sounds/` - Audio files (.ogg, .wav, .mp3)
- `shaders/` - Custom shaders (.wgsl)

## Asset Loading

Bevy can load assets from this directory using the `asset_server`:

```rust
fn load_assets(asset_server: Res<AssetServer>) {
    let model: Handle<Scene> = asset_server.load("models/character.gltf#Scene0");
    let texture: Handle<Image> = asset_server.load("textures/character.png");
    let sound: Handle<AudioSource> = asset_server.load("sounds/jump.ogg");
}
```

## Supported Formats

- **Models**: GLTF 2.0 (.gltf, .glb) - recommended
- **Textures**: PNG, JPEG, HDR, KTX2, DDS
- **Audio**: OGG Vorbis, FLAC, WAV, MP3
- **Shaders**: WGSL (.wgsl)