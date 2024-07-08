use bevy_app::prelude::*;
use bevy_asset::{
    prelude::*,
    processor::LoadTransformAndSave,
    saver::{AssetSaver, SavedAsset},
    transformer::{AssetTransformer, TransformedAsset},
    AssetLoader, AsyncReadExt as _, AsyncWriteExt as _,
};
use bevy_ecs::prelude::*;
use bevy_reflect::TypePath;
use bevy_render::{
    prelude::*,
    render_asset::RenderAssetUsages,
    renderer::RenderDevice,
    texture::{
        CompressedImageFormats, ImageFormat, ImageFormatSetting, ImageLoader, ImageLoaderSettings,
        ImageSampler, ImageType,
    },
};
use bevy_utils::BoxedFuture;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct SvgProcessorPlugin(());

impl Plugin for SvgProcessorPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SvgAsset>();

        app.register_asset_loader(SvgLoader);

        let svg_procssor = SvgToPngProcessor::from_world(app.world_mut());

        type PreprocessedSvgToPng = LoadTransformAndSave<SvgLoader, SvgToPngProcessor, PngSaver>;

        app.register_asset_processor::<PreprocessedSvgToPng>(LoadTransformAndSave::new(
            svg_procssor,
            PngSaver,
        ));
        app.set_default_asset_processor::<PreprocessedSvgToPng>("svg");
    }
}

#[derive(Asset, TypePath)]
struct SvgAsset {
    svg_tree: usvg::Tree,
    output_size: (u32, u32),
}

#[derive(Serialize, Deserialize)]
struct SvgLoaderSettings {
    output_size: (u32, u32),
}

impl Default for SvgLoaderSettings {
    fn default() -> Self {
        Self {
            output_size: if cfg!(feature = "default_8x8") {
                (8, 8)
            } else if cfg!(feature = "default_16x16") {
                (16, 16)
            } else if cfg!(feature = "default_32x32") {
                (32, 32)
            } else if cfg!(feature = "default_64x64") {
                (64, 64)
            } else if cfg!(feature = "default_128x128") {
                (128, 128)
            } else if cfg!(feature = "default_256x256") {
                (256, 256)
            } else if cfg!(feature = "default_512x512") {
                (512, 512)
            } else if cfg!(feature = "default_1024x1024") {
                (1024, 1024)
            } else {
                panic!("You must enable one of the default size features for bevy_svg_processor")
            },
        }
    }
}

struct SvgLoader;

impl AssetLoader for SvgLoader {
    type Asset = SvgAsset;

    type Settings = SvgLoaderSettings;

    type Error = std::io::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut bevy_asset::io::Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy_asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let svg_tree = usvg::Tree::from_data(&bytes, &usvg::Options::default()).unwrap();
        Ok(SvgAsset {
            svg_tree,
            output_size: _settings.output_size,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["svg"]
    }
}

struct SvgToPngProcessor {
    supported_compressed_formats: CompressedImageFormats,
}

#[derive(Serialize, Deserialize)]
struct SvgToPngProcessorSettings {
    pub format: ImageFormatSetting,
    pub is_srgb: bool,
    pub sampler: ImageSampler,
    pub asset_usage: RenderAssetUsages,
}

impl Default for SvgToPngProcessorSettings {
    fn default() -> Self {
        Self {
            format: ImageFormatSetting::default(),
            is_srgb: true,
            sampler: ImageSampler::Default,
            asset_usage: RenderAssetUsages::default(),
        }
    }
}

impl FromWorld for SvgToPngProcessor {
    fn from_world(world: &mut World) -> Self {
        let supported_compressed_formats = match world.get_resource::<RenderDevice>() {
            Some(render_device) => CompressedImageFormats::from_features(render_device.features()),

            None => CompressedImageFormats::NONE,
        };
        Self {
            supported_compressed_formats,
        }
    }
}

impl AssetTransformer for SvgToPngProcessor {
    type AssetInput = SvgAsset;
    type AssetOutput = Image;

    type Settings = SvgToPngProcessorSettings;

    type Error = std::io::Error;

    async fn transform<'a>(
        &'a self,
        asset: TransformedAsset<Self::AssetInput>,
        settings: &'a Self::Settings,
    ) -> Result<TransformedAsset<Self::AssetOutput>, Self::Error> {
        let tree = &asset.svg_tree;

        let tree_size = tree.size();
        let (width, height) = asset.output_size;

        let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
        let tns = tiny_skia::Transform::from_scale(
            width as f32 / tree_size.width(),
            height as f32 / tree_size.height(),
        );
        resvg::render(tree, tns, &mut pixmap.as_mut());
        let png = pixmap.encode_png().unwrap();

        let img = Image::from_buffer(
            &png,
            ImageType::Extension("png"),
            self.supported_compressed_formats,
            settings.is_srgb,
            settings.sampler.clone(),
            settings.asset_usage,
        )
        .unwrap();

        let asset = asset.replace_asset(img);

        Ok(asset)
    }
}

struct PngSaver;

impl AssetSaver for PngSaver {
    type Asset = Image;

    type Settings = ();

    type OutputLoader = ImageLoader;

    type Error = std::io::Error;

    async fn save<'a>(
        &'a self,
        writer: &'a mut bevy_asset::io::Writer,
        asset: SavedAsset<'a, Self::Asset>,
        _settings: &'a Self::Settings,
    ) -> Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error> {
        let mut bytes_buf = std::io::Cursor::new(Vec::new());

        image::write_buffer_with_format(
            &mut bytes_buf,
            &asset.data,
            asset.width(),
            asset.height(),
            image::ExtendedColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();

        writer.write_all(&bytes_buf.into_inner()).await?;

        Ok(ImageLoaderSettings {
            format: ImageFormatSetting::Format(ImageFormat::Png),
            is_srgb: asset.texture_descriptor.format.is_srgb(),
            sampler: asset.sampler.clone(),
            asset_usage: asset.asset_usage,
        })
    }
}
