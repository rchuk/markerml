
fn main() {
    // TODO: Create cli

    let code = r#"
      component custom[
        default smth: string = "wow"
      ] {
        header[1](Hello)
        box[horizontal, prop = "hi"] {
           @(google.com ${a} com)
        }
      }
    "#;

    let (module, errors) = markerml_frontend::parse(&code);
    println!("Module: {:#?}", module);
    for err in errors {
        println!("Parse error: {}. At line {} column {}", err, err.span().start.line, err.span().start.column);
    }
}
