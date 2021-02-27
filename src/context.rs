use crate::sys;
use std::ffi;
use std::io;

/// A trait defining event callbacks.
///
pub trait Delegate {
    ///
    fn update(&mut self, context: &mut Context) {}

    ///
    fn render(&mut self, context: &mut Context) {}
}

/// Builds a Context with custom configuration values.
///
/// # Examples
///
/// ```
/// use kioto::context;
///
/// fn main() {
///   let context = context::Builder::new()
///     .build()
///     .unwrap();
///
///   // use context
/// }
pub struct Builder {
    title: String,
    enable_video: bool,
}

impl Builder {
    /// Create a new builder.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use kioto::context;
    ///
    /// fn main() {
    ///     let builder = context::Builder::new();
    /// }
    /// ```
    pub fn new() -> Builder {
        Self {
            title: "".to_string(),
            enable_video: false,
        }
    }

    /// Sets the title of the context.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use kioto::context;
    ///
    /// fn main() {
    ///     let mut context = context::Builder::new()
    ///         .title("Hello, world!")
    ///         .build()
    ///         .unwrap();
    /// }
    /// ```
    pub fn title<T>(&mut self, title: T) -> &mut Builder
    where
        T: Into<String>,
    {
        self.title = title.into();
        self
    }

    /// Enable the video driver.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use kioto::context;
    ///
    /// fn main() {
    ///     let mut context = context::Builder::new()
    ///         .enable_video()
    ///         .build()
    ///         .unwrap();
    /// }
    /// ```
    pub fn enable_video(&mut self) -> &mut Builder {
        self.enable_video = true;
        self
    }

    /// Build a new context
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use kioto::context;
    ///
    /// fn main() {
    ///     let context = context::Builder::new()
    ///         .build()
    ///         .unwrap();
    /// }
    pub fn build(&mut self) -> io::Result<Context> {
        if self.enable_video {
            let title = ffi::CString::new(self.title.clone())?;
            let is_ready = unsafe {
                sys::init_video(0, 0, title.as_ptr());
                sys::is_video_ready()
            };

            if !is_ready {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Unable to initialize video driver",
                ));
            }
        }

        Ok(Context { running: false })
    }
}

/// A Context handles the application event loop and connects to the various device drivers.
///
/// # Examples
///
/// ```
/// use kioto::context;
///
/// fn main() {
///     let mut context = context::Context::new();
/// }
/// ```
pub struct Context {
    running: bool,
}

impl Context {
    /// Create a new context with the default configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use kioto::context;
    ///
    /// fn main() {
    ///   let context = context::Context::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self { running: false }
    }

    ///
    ///
    pub fn run<D>(&mut self, delegate: &mut D)
    where
        D: Delegate,
    {
        self.running = true;
        while self.running {
            unsafe {
                sys::begin_frame();
            }

            delegate.update(self);
            delegate.render(self);

            unsafe {
                sys::end_frame();
            }
        }
    }

    /// Schedules the loop for termination after the next tick completes.
    ///
    pub fn shutdown(&mut self) {
        self.running = false;
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            if sys::is_video_ready() {
                sys::close_video();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn build_context() {
        let context = Builder::new().build();

        assert!(context.is_ok());
    }
}
