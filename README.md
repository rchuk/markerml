# MarkerML

## About
Simple markup and templating language, that is transpiled to HTML.

## Features
**TODO**

## Development

Project is implemented using Rust.
Lexer is written from scratch, while parser uses [chumsky](https://crates.io/crates/chumsky) library.

Development is split into a couple of stages:
1. Basic syntax parsing
2. Intermediate Representation and support for builtin components
3. Emitting HTML
4. Live reloading
5. Custom components definitions

## Examples

### Page
```
page[
    title="Sample page"
] {
    box[horizontal] {
        @(Hello world)
        @[bold](Bold text)
    }
    
    box[direction = "vertical"] {
        header[1](Welcome)
        #["https://google.com"](Nice link)
        header[level=2](Blah blah)
        image["https://picsum.photos/id/237/200/300"]
    }
    
    list[ordered] {
        @(First item)
        @(Second item)
    }
    
    list[unordered] {
        @(Something)
        @(Something else)
    }
    
    // By default box has vertical orientation
    box {
        paragraph(
            Lorem ipsum dolor
            sit amet
        )
        paragraph(
            Different paragraph
        )
        some_component[a = "Text", b = "other text"] {
            box {
                @(abc)
            }
            box {
                @(cde)
            }
        }
        other_component {
            @(Well)
        }
    }
}
```

### Creating reusable component
```
component other_component[
    child: slot
] {
    box[horizontal] {
        ${child}
        @(Hello)
    }
}
```

### Creating reusable component with multiple children
```
component some_component[
    default a: string,
    b: string = "abc",
    children: slot[]
] {
    box[horizontal] {
        @(${a})
        @(${b})
        list {
            ${children}
        }
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

### Page
Name: `page` \
Properties:
- `title: string`

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
    
    minus(("-"))
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
        backslash((\))
        escaped_dollar(($))
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
    content_start --> backslash
    
    any ---> content_end
    dollar --> curly_open
    
    curly_open --> identifier
    identifier --> curly_close
    curly_close --> content_end
    
    backslash --> escaped_dollar
    escaped_dollar ----> content_end
    backslash --> content_end
    
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
   
    any["any codepoint except ), $, \"]
    dollar(($))
    backslash((\))
    escaped_dollar(($))
    curly_open(("{"))
    curly_close(("}")) 
    identifier
    
    %% 
    START1 --> START
    
    %% 
    START --> any
    START --> dollar
    START --> END
    START --> backslash
    
    any ---> END
    dollar --> curly_open
    
    curly_open --> identifier
    identifier --> curly_close
    curly_close --> END
    
    backslash --> escaped_dollar
    escaped_dollar ----> END
    backslash --> END
    
    END --> START
    
    %% 
    END --> END1
```

#### Identifier

Parsing occurs according to [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/#D1).
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
    start(XID_Start)
    continue(XID_Continue)
    
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
    
    %% 
    START1 --> START
    
    %% 
    START --> string
    START --> bool
    START --> integer
    START --> identifier
    
    string --> END
    bool --> END
    integer --> END
    identifier --> END
    
    %% 
    END --> END1
```

### Miscellaneous

#### Comment
