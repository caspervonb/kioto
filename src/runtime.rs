use crate::sys;
use std::io;

/// Builds a Runtime with custom configuration values.
///
/// # Examples
///
/// ```
/// use kioto::runtime;
///
/// fn main() {
///   let runtime = runtime::Builder::new()
///     .build()
///     .unwrap();
///
///   // use runtime
/// }
pub struct Builder {}

impl Builder {
    /// Create a new builder.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kioto::runtime;
    ///
    /// fn main() {
    ///     let builder = runtime::Builder::new();
    /// }
    /// ```
    pub fn new() -> Builder {
        Self {}
    }

    /// Build a new runtime
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kioto::runtime;
    ///
    /// fn main() {
    ///     let runtime = runtime::Builder::new()
    ///         .build()
    ///         .unwrap();
    /// }
    pub fn build(&mut self) -> io::Result<Runtime> {
        Ok(Runtime { running: false })
    }
}

/// A Runtime handles the application event loop and connects to the various device drivers.
///
/// # Examples
///
/// ```
/// use kioto::runtime;
///
/// fn main() {
///     let mut runtime = runtime::Runtime::new();
///     runtime.run_with(|runtime| {
///       runtime.shutdown();
///
///       Ok(())
///     });
/// }
/// ```
pub struct Runtime {
    running: bool,
}

impl Runtime {
    /// Create a new runtime with the default configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kioto::runtime;
    ///
    /// fn main() {
    ///   let runtime = runtime::Runtime::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self { running: false }
    }

    /// Run the runtime loop with the given callback which is called once per tick until shutdown.
    ///
    /// # Examples
    /// ```rust
    /// use kioto::runtime;
    ///
    /// fn main() {
    ///   let mut runtime = runtime::Runtime::new();
    ///   runtime.run_with(|runtime| {
    ///     runtime.shutdown();
    ///
    ///     Ok(())
    ///   });
    /// }
    /// ```
    pub fn run_with<F>(&mut self, callback: F) -> Result<(), io::Error>
    where
        F: Fn(&mut Runtime) -> Result<(), io::Error>,
    {
        self.running = true;
        let mut result = callback(self);

        while self.running && result.is_ok() {
            result = callback(self);
        }

        result
    }

    /// Schedules the loop for termination after the next tick completes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kioto::runtime;
    ///
    /// fn main() {
    ///     let mut runtime = runtime::Runtime::new();
    ///     runtime.run_with(|runtime| {
    ///         runtime.shutdown();
    ///
    ///         Ok(())
    ///     });
    /// }
    /// ```
    pub fn shutdown(&mut self) {
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn build_runtime() {
        let runtime = Builder::new().build();

        assert!(runtime.is_ok());
    }

    #[test]
    pub fn run_with() {
        let mut runtime = Runtime::new();
        let result = runtime.run_with(|runtime| {
            runtime.shutdown();

            Ok(())
        });

        assert!(result.is_ok());
    }
}
