use text_plugin_api::TextProcessor;

pub struct UppercaseProcessor;

impl TextProcessor for UppercaseProcessor {
    fn name(&self) -> &str {
        "uppercase"
    }

    fn description(&self) -> &str {
        "Converts text to uppercase"
    }

    fn process(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

#[no_mangle]
pub unsafe extern "C" fn create_plugin() -> Box<dyn TextProcessor> {
    Box::new(UppercaseProcessor)
} 