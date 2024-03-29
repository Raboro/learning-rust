use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "hello.stpl")]
pub struct HelloT<'a> {
    pub title: &'a str,
}

pub struct Template<'a, T: TemplateOnce> {
    filename: &'a str,
    file_extension: &'a str,
    file_path: Option<&'a str>,
    template: T,
}

impl<'a, T: TemplateOnce> Template<'a, T> {
    pub fn new(
        filename: &'a str,
        file_extension: &'a str,
        file_path: Option<&'a str>,
        template: T,
    ) -> Self {
        Self {
            filename,
            file_extension,
            file_path,
            template,
        }
    }

    pub fn render(self) -> Option<String> {
        self.template.render_once().ok()
    }
}

impl<'a, T: TemplateOnce> std::fmt::Display for Template<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}.{}",
            self.file_path.unwrap_or(""),
            self.filename,
            self.file_extension
        )
    }
}
