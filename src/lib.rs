use std::borrow::{Borrow, Cow};
use serde::Serialize;
use tinytemplate::TinyTemplate;

const APP_VERSION_TEMPLATE_NAME: &str = "app_version_template";

fn get_elm_version_file_template<'lt>() -> Cow<'lt, str> {
    let bytes = include_bytes!("templates/AppVersion.elm");
    return String::from_utf8_lossy(bytes);
}

pub struct Templates<'lt> {
    elm_app_version_template_contents: Cow<'lt, str>,
}

impl<'lt> Templates<'lt> {
    pub fn load() -> Templates<'lt> {
        Templates {
            elm_app_version_template_contents: get_elm_version_file_template()
        }
    }
}

pub struct TemplateEngine<'lt> {
    //container: &'lt Templates<'lt>,
    tiny_template: TinyTemplate<'lt>,
}

impl<'lt> TemplateEngine<'lt> {
    /// Creates a new TemplateEngine with the needed templates. Returns an Err with an error message when the initialization fails.
    pub fn new(templates: &'lt Templates<'lt>) -> Result<TemplateEngine<'lt>, String>
    {
        let mut tt: TinyTemplate<'lt> = TinyTemplate::new();

        match tt.add_template(APP_VERSION_TEMPLATE_NAME, templates.elm_app_version_template_contents.borrow())
        {
            Ok(_) => {
                let template_engine = TemplateEngine {
                    tiny_template: tt,
                };

                Ok(template_engine)
            }
            Err(e) => { Err(e.to_string()) }
        }
    }

    pub fn render(&self, app_version: AppVersion) -> Result<String, String> {
        match self.tiny_template.render(APP_VERSION_TEMPLATE_NAME, app_version.borrow())
        {
            Ok(rendered_str) => { Ok(rendered_str) }
            Err(render_err) => { Err(render_err.to_string()) }
        }
    }
}

#[derive(Serialize)]
pub struct AppVersion {
    pub version: String,
    pub build_number: u32,
    pub hash: String,
    pub timestamp: i64,
    pub source: String,
}

#[cfg(test)]
use spectral::prelude::*;

#[test]
fn load_default_elm_version_file() {

    //Arrange & Act
    let templates = Templates::load();

    //Assert
    assert_that!(templates.elm_app_version_template_contents.into_owned()).starts_with("module ");
}

#[test]
fn run_template() {
    //Arrange
    let templates = Templates::load();
    let engine = TemplateEngine::new(templates.borrow()).unwrap();

    let model = AppVersion {
        version: "World".to_string(),
        build_number: 1,
        hash: "123456".to_string(),
        timestamp: 0,
        source: "rust unit test".to_string(),
    };

    let rendered = engine.render(model).unwrap();
    println!("Rendered: {}", rendered);
    //println!("Rendered: {}", "henk");
}
