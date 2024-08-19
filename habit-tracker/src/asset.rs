use dioxus::prelude::*;
pub const DAILY: manganis::ImageAsset = manganis::mg!(image("./assets/daily.png")
  .format(ImageType::Avif)
  .preload());
pub const WEEKLY: manganis::ImageAsset = manganis::mg!(image("./assets/weekly.png")
  .format(ImageType::Avif)
  .preload());
pub const S1: manganis::ImageAsset =
  manganis::mg!(image("./assets/s1.png").format(ImageType::Avif).preload());
pub const S2: manganis::ImageAsset =
  manganis::mg!(image("./assets/s2.png").format(ImageType::Avif).preload());
pub const S3: manganis::ImageAsset =
  manganis::mg!(image("./assets/s3.png").format(ImageType::Avif).preload());
pub const S4: manganis::ImageAsset =
  manganis::mg!(image("./assets/s4.png").format(ImageType::Avif).preload());
pub const L1: manganis::ImageAsset =
  manganis::mg!(image("./assets/l1.png").format(ImageType::Avif));
pub const L2: manganis::ImageAsset =
  manganis::mg!(image("./assets/l2.png").format(ImageType::Avif).preload());
pub const L3: manganis::ImageAsset =
  manganis::mg!(image("./assets/l3.png").format(ImageType::Avif).preload());
pub const L4: manganis::ImageAsset =
  manganis::mg!(image("./assets/l4.png").format(ImageType::Avif).preload());
