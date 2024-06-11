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

### License

Except where noted, all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

#### Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
