//! Progress-Bar implementation.

/// Progress-bar structure.
pub struct Bar {
    /// Graphics.
    pb: indicatif::ProgressBar,
}

impl Bar {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(msg: &str, total: u64) -> Self {
        debug_assert!(total > 0);

        let pb = indicatif::ProgressBar::new(total);
        pb.set_message(msg);

        pb.set_style(
            indicatif::ProgressStyle::default_bar()
            .template("{spinner:.cyan} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}")
            .progress_chars("\\/")
        );

        Self { pb }
    }

    /// Tick the bar forward once.
    #[inline]
    pub fn tick(&mut self) {
        self.pb.inc(1);
    }

    /// Increment the bar forward a given amount.
    #[inline]
    pub fn inc(&mut self, n: u64) {
        debug_assert!(n > 0);

        self.pb.inc(n);
    }

    /// Finish with a message.
    #[inline]
    pub fn finish_with_message(&mut self, msg: &'static str) {
        self.pb.finish_with_message(msg);
    }
}
