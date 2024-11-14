# MarkerML

## General

This library is a top level crate for MarkerML,
which provides function that attempts to convert given MarkerML
code to HTML. MarkerML stands for Marker Markup Language.
It's a simple language for formatting and layouting
text similar to HTML.

Syntax is described in details on [docs.rs]("https://docs.rs/markerml/latest/markerml/").

## Example
```markerml
box {
    header[1](Hello, world!)
    paragraph(Some text)
    box[horizontal] {
        @(Wow)
    }
    box[vertical, x_align = "center"] {
        @(Wow)
    }
}
```
