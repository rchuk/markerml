# MarkerML

## About
MarkerML stands for Marker Markup Language.
It's a simple language for formatting and layouting
text similar to HTML.

## Technical Description

Project is implemented using Rust.
Parser is written using [pest](https://crates.io/crates/pest) library.
[thiserror](https://crates.io/crates/thiserror) is used for library crates
and [anyhow](https://crates.io/crates/anyhow) is used for cli and tests.
[miette](https://crates.io/crates/miette) is used to display nice human-readable compilation errors.

Code is split into two main crates:
- [markerml_cli]() - CLI for the MarkerML
- [markerml]() - top level library crate

And a couple of auxiliary crates:
- [markerml_backend]() - provides HTML generation
- [markerml_middleend]() - provides Intermediate Representation
- [markerml_frontend]() - provides parser 

First code is parsed from text into an Abstract Syntax Tree.
Then it's converted into an intermediate representation to simplify
further manipulations. It also catches some simple cases of semantic
errors such as duplicated property names. Finally, this intermediate
representation is used for generating HTML, which can be viewed in
a regular browser.

For a complete grammar overview, refer to the [markerml crate](https://docs.rs/markerml/0.1.2/markerml/).

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
        #["//www.google.com"](Link to google)
    }
}
```

## Builtin components

### Box
Name: `box` \
Properties:
- `vertical`
- `horizontal`
- `x_align: string = "start" | "center" | "end"`. Default: `"start"`
- `y_align: string = "start" | "center" | "end"`. Default: `"start"`

### Text
Name: `@` \
Properties:
- `text content`

### Image
Name: `image` \
Properties:
- `default url: string`

### Link
Name: `#` \
Properties:
- `default url: string`
- `text name`

### List
Name: `list` \
Properties:
- `unordered`
- `ordered`
- `children: slot[]`

### Header
Name: `header` \
Properties:
- `default level: integer = 1`

### Paragraph
Name: `paragraph` \
Properties:
- `text content`

## Grammar

```
WHITESPACE = _{ (" " | "\t" | NEWLINE)+ }

COMMENT = _{ "//" ~ (!NEWLINE ~ ANY)* ~ NEWLINE }

integer = @{ "-"? ~ ASCII_DIGIT+ }

bool = @{ "true" | "false" }

identifier = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }

literal_newline = @{ NEWLINE ~ (" " | "\t")* }

string_literal_segment = @{ (!("$" | "\"" | NEWLINE) ~ ANY)+ }

text_literal_segment = @{ (!("$" | ")" | NEWLINE) ~ ANY)+ }

variable_interpolation = { "${" ~ identifier ~ "}" }

string_segment = ${ literal_newline | variable_interpolation | string_literal_segment }

text_segment = ${ literal_newline | variable_interpolation | text_literal_segment }

string = @{ "\"" ~ string_segment* ~ "\"" }

text = @{ "(" ~ text_segment* ~ ")" }

value = { variable_interpolation | bool | string | integer }

component_name = { "@" | "#" | identifier }

default_property = { value }

named_property = { identifier ~ "=" ~ value }

flag_property = { identifier }

property = { named_property | flag_property }

properties_list = _{ property ~ ("," ~ property)* }

properties = { "[" ~ (properties_list | (default_property ~ ("," ~ properties_list)?))?  ~ ","? ~ "]" }

children = { "{" ~ component* ~ "}" }

component = { component_name ~ properties? ~ children? ~ text? }

ty = @{ "string" | "int" | "bool" | "slot[]" | "slot" }

default_property_definition = { "default" ~ identifier ~ ":" ~ ty }

text_property_definition = { "text" ~ identifier }

named_property_definition = { identifier ~ ":" ~ ty ~ ("=" ~ value)? }

property_definition = { default_property_definition | text_property_definition | named_property_definition }

properties_definition_list = _{ property_definition ~ ("," ~ property_definition)* }

properties_definition = { "[" ~ properties_definition_list? ~ "]" }

component_definition = { "component" ~ identifier ~ properties_definition? ~ children? }

module_item = _{ component_definition | component }

module = { SOI ~ module_item* ~ EOI}
```

### Component instantiation

#### Component
```mermaid
---
title: component
---
graph LR;
    START1:::hidden
    END1:::hidden
    START(( ))
    END((  ))
    
    text_component
    structured_component
    
    START --> text_component
    START --> structured_component
    text_component --> END
    structured_component --> END
    
    START1 --> START
    END --> END1
```

#### Structured component
```mermaid
---
title: structured_component
---
graph LR;
    START:::hidden
    END1:::hidden
    END(( ))
    
    identifier
    properties
    children
    
    %% name
    START --> identifier
    identifier --> END
    identifier --> properties
    identifier --> children
    
    %% properties
    properties --> END
    properties --> children
   
    %% children
    children --> END
    
    %% 
    END --> END1

```

#### Text component
```mermaid
---
title: text_component
---
graph LR;
    START:::hidden
    END1:::hidden
    END(( ))
    
    identifier
    properties
    open(("("))
    close((")"))
    text
    
    %% name
    START --> identifier
    identifier --> properties
    identifier --> open
    identifier --> END
    
    %% properties
    properties --> open
    
    %% open
    open --> text
    open --> close
  
    %% text
    text --> close
  
    %% close
    close --> END
    
    %% 
    END --> END1
```

#### Component properties
```mermaid
---
title: properties
---
graph LR;
    START:::hidden
    END:::hidden
    
    open(("["))
    close(("]"))
    comma((","))
    default_property_value
    subgraph property
        property_start(( ))
        property_end(( ))
        identifier
        equals(("="))
        value
    end

    %% open
    START --> open
    open --> property_start
    open --> default_property_value
    open --> close
    
    %% default property value
    default_property_value --> close
    default_property_value --> comma
    
    %% property
    property_start --> identifier
    identifier --> equals
    equals --> value
    value --> property_end
    identifier --> property_end
    property_end ---> comma
    property_end --> close
    
    %% repeat
    comma --> close
    comma --> property_start
   
    %% close
    close --> END
```

#### Component children
```mermaid
---
title: children
---
graph LR;
    START:::hidden
    END:::hidden
    
    open(("{"))
    close(("}"))
    component
    
    %% open
    START --> open
    open --> component
    open --> close
    
    %% component
    component --> component
    component --> close
    
    %% close
    close --> END
```

### Component definition

#### Component
```mermaid
---
title: component_definition
---
graph LR;
    START:::hidden
    END1:::hidden
    END(( ))
    
    component_keyword('component')
    identifier
    properties_definition
    
    %% component keyword
    START --> component_keyword
    component_keyword --> identifier
    
    %% identifier
    identifier --> children
    identifier --> properties_definition
  
    
    %% properties definition
    properties_definition --> children
    properties_definition --> END
    
    %% 
    children --> END
    
    %% 
    END --> END1
```

#### Component properties
```mermaid
---
title: properties_definition
---
graph LR;
    START:::hidden
    END:::hidden
    
    open(("["))
    close(("]"))
    comma((","))
    subgraph property 
        property_start(( ))
        property_end(( ))
        
        default_keyword('default')
        text_keyword('text')
        identifier
        text_identifier[identifier]
        colon((":"))
        type
        equals(("="))
        value
    end

    %% open
    START --> open
    open --> property_start
    open --> close
    
    %% property
    property_start --> default_keyword
    property_start --> identifier
    property_start --> text_keyword
    default_keyword --> identifier
    identifier --> colon
    colon --> type
    type --> equals
    type --> property_end
    equals --> value
    value --> property_end
    identifier --> property_end
    property_end ---> comma
    property_end --> close
    
    text_keyword --> text_identifier
    text_identifier ------> property_end
    
    %% repeat
    comma --> close
    comma --> property_start
   
    %% close
    close --> END
```

#### Type
```mermaid
---
title: type
---
graph LR;
    START1:::hidden
    END1:::hidden
    START(( ))
    END(( ))
    
    bool('bool')
    string('string')
    int('int')
    slot('slot')
    left_bracket(("["))
    right_bracket(("]"))
    
    %% 
    START1 --> START
    
    %% start
    START --> string
    START --> bool
    START --> int
    START --> slot
    
    %% string
    string ----> END
    
    %% bool
    bool ----> END
    
    %% int
    int ---->END
    
    %% slot
    slot --> left_bracket
    left_bracket --> right_bracket
    right_bracket --> END
    slot --> END
    
    %% 
    END --> END1
```

### Literals

#### Boolean
```mermaid
---
title: bool
---
graph LR;
    START1:::hidden
    END1:::hidden
    START(( ))
    END(( ))
    
    true
    false
    
    START1 --> START
    
    START --> true
    START --> false
    true --> END
    false --> END
    
    END --> END1
```


#### Integer
```mermaid
---
title: integer
---
graph LR;
    START1:::hidden
    END1:::hidden
    START(( ))
    END(( ))
    
    minus(("—"))
    digit(("1..9"))
    single_zero((0))
    zero((0))
    
    %% 
    START1 --> START
    
    %% 
    single_zero ----> END
    
    START ---> single_zero
    START --> minus
    START --> digit
    
    minus --> single_zero
    minus ---> digit
    digit --> digit
    digit --> END
    digit --> zero
    zero --> digit
    zero ---> END
    
    %% 
    END --> END1
```


#### String
```mermaid
---
title: string
---
graph LR;
    START:::hidden
    END:::hidden
    
    open(("&quot"))
    close(("&quot"))
    subgraph content
        content_start(( ))
        content_end(( ))
        
        any[any codepoint except &quot, $, \]
        dollar(($))
        curly_open(("{"))
        curly_close(("}")) 
        identifier
    end
    
    %% open
    START --> open
    open --> content_start
     
    %% content
    content_start --> any
    content_start --> dollar
    content_start --> close
    
    any ---> content_end
    dollar --> curly_open
    
    curly_open --> identifier
    identifier --> curly_close
    curly_close --> content_end
    
    
    content_end --> content_start
    content_end --> close
    
    %% close
    close --> END
    
```

#### Text
```mermaid
---
title: text
---
graph LR;
    START1:::hidden
    END1:::hidden
    START(( ))
    END(( ))
   
    any["any codepoint except ), $"]
    dollar(($))
    curly_open(("{"))
    curly_close(("}")) 
    identifier
    
    %% 
    START1 --> START
    
    %% 
    START --> any
    START --> dollar
    START --> END

    any ---> END
    dollar --> curly_open
    
    curly_open --> identifier
    identifier --> curly_close
    curly_close --> END
    
    END --> START
    
    %% 
    END --> END1
```

#### Identifier

```mermaid
---
title: identifier
---
graph LR;
    START1:::hidden
    END1:::hidden
    START(( ))
    END(( ))
    
    underscore(("_"))
    start(ascii alphabetic)
    continue(ascii alphanumeric)
    
    %% 
    START1 --> START
    
    %% 
    START --> start
    START --> underscore
    
    start ---> underscore
    underscore --> continue
    underscore --> underscore
    continue ---> underscore
    continue --> continue
    
    underscore --> END
    continue --> END
    
    %% 
    END --> END1
```

### Values
```mermaid
---
title: value
---
graph LR;
    START1:::hidden
    END1:::hidden
    START(( ))
    END(( ))
    
    string
    bool
    integer
    identifier
    dollar(($))
    open(("{"))
    close(("}"))
    
    %% 
    START1 --> START
    
    %% 
    START --> string
    START --> bool
    START --> integer
    START --> dollar
    
    string ---> END
    bool ---> END
    integer ---> END
    dollar ---> open
    open --> identifier
    identifier --> close
    close --> END
    
    %% 
    END --> END1
```

#### Comment
```mermaid
---
title: comment
---
graph LR;
    START1:::hidden
    END1:::hidden
    START(( ))
    END(( ))
    
    slash0(("/"))
    slash1(("/"))
    any["Any symbol except newline"]
    newline
    
    %% 
    START1 --> START
    
    %% 
    START --> slash0
    slash0 --> slash1
    
    slash1 --> any
    any --> any
    any --> newline
    
    newline --> END
    
    %% 
    END --> END1
```
