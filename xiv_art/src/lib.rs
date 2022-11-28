//! # Art
//!
//! 一个用来建模艺术概念的代码库

// 发布成对外的公共API
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds{
    /// RYB 颜色模型的三原色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// RYB 模型的调和色
    pub enum SecondaryColor{
        Orange,
        Green,
        Purple,
    }
}

pub mod utils{
    use crate::kinds::*;

    /// 将两种等量的原色混合生成调和色
    pub fn mix(_c1:PrimaryColor,c2:SecondaryColor) -> SecondaryColor{
        c2
    }
}
