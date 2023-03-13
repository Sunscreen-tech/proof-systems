use thiserror::Error;

/// A result type for Snarky errors.
pub type SnarkyResult<T> = std::result::Result<T, SnarkyError>;

/// A result type for Snarky runtime errors.
pub type SnarkyRuntimeResult<T> = std::result::Result<T, SnarkyRuntimeError>;

/// A result type for Snarky compilation errors.
pub type SnarkyCompileResult<T> = std::result::Result<T, SnarkyCompilationError>;

/// Snarky errors can come from either a compilation or runtime error.
#[derive(Debug, Clone, Error)]
pub enum SnarkyError {
    #[error("A compilation error occurred.")]
    CompilationError(SnarkyCompilationError),

    #[error("A runtime error occurred.")]
    RuntimeError(SnarkyRuntimeError),
}

/// Errors that can occur during compilation of a circuit.
#[derive(Debug, Clone, Error)]
pub enum SnarkyCompilationError {
    #[error("delete this")]
    ToDelete(String),
}

/// Errors that can occur during runtime (proving).
#[derive(Debug, Clone, Error)]
pub enum SnarkyRuntimeError {
    #[error(
        "unsatisfied constraint: `{0} * {1} + {2} * {3} + {4} * {5} + {6} * {1} * {3} + {7} != 0`"
    )]
    UnsatisfiedGenericConstraint(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ),

    #[error("unsatisfied constraint: {0} is not a boolean (0 or 1)")]
    UnsatisfiedBooleanConstraint(String),

    #[error("unsatisfied constraint: {0} is not equal to {1}")]
    UnsatisfiedEqualConstraint(String, String),

    #[error("unsatisfied constraint: {0}^2 is not equal to {1}")]
    UnsatisfiedSquareConstraint(String, String),

    #[error("unsatisfied constraint: {0} * {1} is not equal to {2}")]
    UnsatisfiedR1CSConstraint(String, String, String),
}
