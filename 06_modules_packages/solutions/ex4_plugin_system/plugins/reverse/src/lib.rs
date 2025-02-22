use text_plugin_api::TextProcessor;

pub struct ReverseProcessor;

impl TextProcessor for ReverseProcessor {
    fn name(&self) -> &str {
        "reverse"
    }

    fn description(&self) -> &str {
        "Reverses the input text"
    }

    fn process(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}

#[no_mangle]
pub unsafe extern "C" fn create_plugin() -> Box<dyn TextProcessor> {
    Box::new(ReverseProcessor)
} 