/*!
One-line description.

TBD

# Example

TBD

 */

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default)]
pub struct DependencyWriterOptions {
    max_depth: usize,
    use_color: bool,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl DependencyWriterOptions {
    pub fn with_depth_all(self) -> Self {
        Self {
            max_depth: 0,
            ..self
        }
    }

    pub fn with_max_depth(self, max_depth: usize) -> Self {
        Self { max_depth, ..self }
    }

    pub fn with_use_color(self, use_color: bool) -> Self {
        Self { use_color, ..self }
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn use_color(&self) -> bool {
        self.use_color
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "dot")]
pub mod dot;

#[cfg(feature = "rdf")]
pub mod rdf;

pub mod text;
