pub mod parser;
pub mod ast;
pub mod lexer;
pub mod token;
pub mod common;

/// Performs complete parsing of the source code
///
/// # Example
/// ```
/// let code = r#"
///       page[
///         title = "Sample page"
///       ] {
///         header[1](Hello World)
///         paragraph(
///             Content
///         )
///       }
///     "#;
///
///     let (module, errors) = markerml_frontend::parse(&code);
///     println!("Module: {:#?}", module);
///     for err in errors {
///         println!("Parse error: {}. At line {} column {}", err, err.span().start.line, err.span().start.column);
///     }
/// ```
pub fn parse(code: impl AsRef<str>) -> (Option<ast::Module>, Vec<parser::ParserError>) {
    use chumsky::{Stream, Parser};

    let (tokens, eof) = lexer::Lexer::new(&code).lex();
    let parser = parser::parser();
    let stream = Stream::from_iter(eof, tokens.into_iter());
    parser.parse_recovery(stream)

    // TODO: Check some language invariants in additional pass
    // TODO: Implement better error handling. Use miette crate
}

