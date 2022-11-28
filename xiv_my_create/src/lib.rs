//! # xiv_my_create
//!
//! xiv_my_create是学习文档的一个项目

/// 将传入的数字加1
///
/// # Example
///
/// ```
/// let arg = 5;
/// let answer = xiv_my_create::add_one(arg);
///
/// assert_eq!(6,answer);
/// ```
///
/// # panics
///
/// 这个是描述可能引发的pannic场景，不想触发pannic，调用者应确保不在该场景下使用
///
/// # Errors
///
/// 当函数作为Result作为参数返回结果时，这里指出可能出现的错误，以便调用者及时出来
///
/// # Safety
///
/// 当函数使用unsafe关键字时，会指出不安全的原因
pub fn add_one(x:i32) -> i32{
 // 生产文档：cargo doc
 // 直接通过浏览器查看生成的文档：cargo doc --open
 x+1
}