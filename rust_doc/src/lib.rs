//! 这里是整个crate的文档注释, 使用的是INNER行注释的形式

pub use base::house::echo;

/**
 * 这里,我们给base模块添加文档注释.
 * 因为注释紧贴着被注释的项,且在被注释项的外边,所以, 被称为OUTER块文档注释
*/
pub mod base {

    pub mod house {
        /*! 这里是house模块的INNER块文档注释,和///的形式对应 */

        /// 这里是函数echo的OUTER行文档
        ///
        /// # Exapmples
        ///
        /// ```rust
        /// fn echo_7() {
        ///   assert_eq!(rust_doc::base::house::echo(7), 7);
        /// }
        /// ```
        ///
        /// # Errors
        /// This function returns no error
        /// # Panics
        /// This function never panic
        /// # Safety
        pub fn echo(a: i32) -> i32 {
            // 这是一个普通的行注释

            /* 这是一个普通的块注释
             * */

            println!("Received {}, response {}", a, a);
            a
        }
    }
}
