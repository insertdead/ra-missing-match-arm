use thiserror::Error;

fn main() {
    println!("Hello, World!");
}

#[derive(Debug, Error)]
pub enum ShenanigansAfoot {
    #[error("{0}")]
    NormalVariant(String),
    #[error("testing")]
    OtherVariant,
    #[cfg(feature = "feature1")]
    #[error("{0}")]
    FeatureVariant(i32),
    #[cfg(feature = "feature2")]
    #[error("{0}")]
    OtherFeature(u64)
}