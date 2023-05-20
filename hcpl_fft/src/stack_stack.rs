use std::mem::MaybeUninit;

const BUF_SIZE: usize = 50;

/// A stack-allocated stack
pub(crate) struct StackStack<T> {
    buffer: [MaybeUninit<T>; BUF_SIZE],
    len: usize,
}

impl<T> StackStack<T> {
    pub(crate) fn new() -> Self {
        Self {
            buffer: unsafe {
                MaybeUninit::<[MaybeUninit<T>; BUF_SIZE]>::uninit().assume_init()
            },
            len: 0,
        }
    }

    pub(crate) fn push(&mut self, val: T) -> Option<()> {
        if self.len == BUF_SIZE {
            return None;
        }

        self.buffer[self.len].write(val);
        self.len += 1;

        Some(())
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        // SAFETY: this element will have been written to exactly once, and won't be read again
        Some(unsafe { self.buffer[self.len].assume_init_read() })
    }
}
