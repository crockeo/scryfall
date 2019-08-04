/// The error object that accompanies a 4xx or a 5xx from the server.
pub struct Error {
    /// An integer HTTP status code for this error.
    pub error: u16,

    /// A computer-friendly string representing the appropriate HTTP status code.
    pub code: String,

    /// A human-readable string explaining the error.
    pub details: String,

    /// A computer-friendly string that provides additional context for the main error. For example, an endpoint many
    /// generate HTTP 404 errors for different kinds of input. This field will provide a label for the specific kind
    /// of 404 failure, such as ambiguous.
    pub error_type: Option<String>,

    /// If your input also generated non-failure warnings, they will be provided as human-readable strings in this
    /// array.
    pub warnings: Option<Vec<String>>,
}
