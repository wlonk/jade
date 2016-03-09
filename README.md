# Jade in Rust

A Rust parser for the [Jade templating language](http://jade-lang.com/).

## Spec

Jade seems not to have a defined spec, only a reference implementation. So here
I will try to lay out my version of the spec. There are some edge cases where
it diverges from the reference implementation.

This currently only defines a subset of Jade, including the features I find
most crucial.

### Nesting

A document is structured like a tree in memory, but is serialized into a number
of lines when represented as a file. Reconciling these two structures is
central to templating.

**This deviates from the reference implementation.**

All lines with zero leading whitespace characters are children of the root node.

All lines with one or more leading whitespace characters are children of the
first line above them with fewer leading whitespace characters.

If two sibling nodes (lines with the same parent per the definition in the
paragraph above) have different amounts of leading whitespace, this is causes
parser error.

This means that the following Jade:

```jade
foo
    bar
  baz
qux
```

is invalid.

(The reference implementation would parse that Jade to:

```html
<foo>
  <bar></bar>
</foo>
<baz></baz>
<qux></qux>
```

which, quite frankly, I think is terrible---I think an error is much more
meaningful.)

Certain tags cannot have children. Including lines indented below them will
cause a parser error.

TODO: enumerate those tags here.

## Doctypes

Per the [Jade documentation on the
matter](http://jade-lang.com/reference/doctype/), there are a number of
shortcuts you can use that will produce various kinds of doctype.

If the first non-empty line in a template is a doctype, that will be used as
the doctype for the whole document, and will change how certain tags are
rendered. If there is no such doctype declaration, the HTML5 doctype
(`<!DOCTYPE html>`) will be used, though if you are rendering a fragmentary
template and need to use a different doctype, you can override this by passing
an appropriate doctype to the render function.

Any subsequent doctype line will be inserted into the document, but will not
affect rendering.

**This is a deviation from the reference implementation,** in which a doctype
tag will determine the rendering of all nodes after it, which is again,
terrible.

## Tags, attributes, classes, IDs

As per standard Jade, the first word on any line is the tag name, unless:

  * The word begins with a `#`, in which case the tag is a `div` and the word
    after the `#` is the ID of that `div`.
  * The word begins with a `.`, in which case the tag is a `div` and the word
    after the `.` is the class of that div.
  * The parent line ends with `.`, in which case the whole block is text.
  * The tag is a reserved word (see below).

## Reserved words

This isn't just a fancy way to write HTML, this is a _templating_ language. So
we have conditionals, loops, and variable substitution.

This means we need a few reserved words:

  * `if`
  * `else`
  * `each`
  * `extends`
  * `block`
  * `|` (pipe)
