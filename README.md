# Svg -> Png processor for Bevy

Converts `.svg` files to `.png` through the Bevy `asset_processor` feature. 

### Installation

```sh
cargo add --git https://github.com/tbillington/bevy_svg_processor -F default_128x128
```

### Usage

```rust
// Enable AssetMode::Processed and add the SvgProcessorPlugin
app.add_plugins((
    DefaultPlugins.set(AssetPlugin {
        mode: AssetMode::Processed,
        ..default()
    }),
    SvgProcessorPlugin::default(),
));

// Spawn your svg!
commands.spawn(SpriteBundle {
    texture: asset_server.load("warrior.svg"),
    ..default()
});
```

After the first run Bevy will have generated an accompanying `.meta` files for your `.svg` assets.

To customise the dimensions of the rasterised asset open it's associated `*.svg.meta` file and modify `output_size` in the `loader_settings`. It will be regenerated in the next run.

```rust
(
    asset: Process(
        processor: "..",
        settings: (
            loader_settings: (
                output_size: (128, 128),
            ),
            ..
        ),
    ),
)
```

To set the default dimensions for new assets enable the relevant feature in `bevy_svg_processor`. The full list is in the [Cargo.toml](./Cargo.toml).
