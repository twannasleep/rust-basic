use text_plugin_api::TextProcessor;

pub struct LowercaseProcessor;

impl TextProcessor for LowercaseProcessor {
    fn name(&self) -> &str {
        "lowercase"
    }

    fn description(&self) -> &str {
        "Converts text to lowercase"
    }

    fn process(&self, input: &str) -> String {
        input.to_lowercase()
    }
}

#[no_mangle]
pub unsafe extern "C" fn create_plugin() -> Box<dyn TextProcessor> {
    Box::new(LowercaseProcessor)
} 