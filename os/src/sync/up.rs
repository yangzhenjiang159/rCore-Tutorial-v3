//! 单处理器内部可变原语

use core::cell::{RefCell, RefMut};

/// 在其中包装一个静态数据结构，以便我们
/// 能够在没有任何 `unsafe` 的情况下访问它。
///
/// 我们应该只在单处理器中使用它。
///
/// 为了获取内部数据的可变引用，调用
/// `exclusive_access`。
pub struct UPSafeCell<T> {
    /// inner data
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// 用户有责任保证内部结构仅用于
    /// 单核处理器.
    pub unsafe fn new(value: T) -> Self{
        Self {
            inner: RefCell::new(value),
        }
    }

    /// 独占访问UPSafeCell中的内部数据。如果数据已被借用，则会感到恐慌。
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}