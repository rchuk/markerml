use markerml_frontend::ast::Module;
use markerml_frontend::common::span::Span;

fn parse(code: &str) -> miette::Result<Module<Span>> {
    let module = markerml_frontend::parse(&code)?;

    Ok(module)
}

fn main() -> miette::Result<()> {
    // TODO: Create cli

    /*
    let code = r#"
      component custom[
        default smth: string = "wow",
        smth: integer
      ] {
        header[1](Hello)
        box[horizontal, prop = "hi"] {
           @(google.com ${a} com)
        }
      }
    "#;
    */
    let code = r#"
      component custom[
        default smth: string,
        text else,
        wow: slot[],
        wow1: slot[]
      ] {
        header[1](Hello)
        box[horizontal, prop = "hi"] {
           @(google.com ${a} com)
        }
      }
    "#;

    let module = parse(code).map_err(|err| err.with_source_code(code))?;

    println!("Module: {:#?}", module);
    /*
    match markerml_frontend::parse(&code) {
        Ok(module) => println!("Module: {:#?}", module),
        Err(error) => println!("Error: {}", error),
    };
    */

    Ok(())
}
