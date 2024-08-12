use manganis::*;
pub const V1: manganis::ImageAsset =
  manganis::mg!(image("./public/V1.png").format(ImageType::Avif).preload());
pub const V2: manganis::ImageAsset =
  manganis::mg!(image("./public/V2.png").format(ImageType::Avif).preload());
pub const V3: manganis::ImageAsset =
  manganis::mg!(image("./public/V3.png").format(ImageType::Avif).preload());
