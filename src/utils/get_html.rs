use pulldown_cmark::{Options, Parser};

pub fn get_html(content: &str) -> String {
    let options = Options::empty();
    
    let parser = Parser::new_ext(&content, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    
    return html_output;
}
