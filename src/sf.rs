use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "hello.stpl")]
pub struct HelloT<'a> {
    pub title: &'a str,
}
