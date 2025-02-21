/// Trait that all text processing plugins must implement
pub trait TextProcessor {
    /// Returns the name of the plugin
    fn name(&self) -> &str;

    /// Returns a description of what the plugin does
    fn description(&self) -> &str;

    /// Process the input text and return the modified text
    fn process(&self, input: &str) -> String;
}

/// Type alias for the plugin creation function
pub type CreatePlugin = unsafe fn() -> Box<dyn TextProcessor>;

/// The expected name of the plugin creation function
pub const PLUGIN_CREATOR_FUNCTION: &str = "create_plugin";

#[cfg(test)]
mod tests {
    use super::*;

    struct TestProcessor;

    impl TextProcessor for TestProcessor {
        fn name(&self) -> &str {
            "test"
        }

        fn description(&self) -> &str {
            "A test processor"
        }

        fn process(&self, input: &str) -> String {
            input.to_string()
        }
    }

    #[test]
    fn test_processor() {
        let processor = TestProcessor;
        assert_eq!(processor.name(), "test");
        assert_eq!(processor.description(), "A test processor");
        assert_eq!(processor.process("test"), "test");
    }
} 