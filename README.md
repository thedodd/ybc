<h1 align="center">ybc</h1>
<div align="center">

[![Build Status](https://github.com/thedodd/ybc/workflows/ci/badge.svg?branch=master)](https://github.com/thedodd/ybc/actions)
[![Crates.io](https://img.shields.io/crates/v/ybc.svg)](https://crates.io/crates/ybc)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE)
![Crates.io](https://img.shields.io/crates/d/ybc.svg)
![Crates.io](https://img.shields.io/crates/dv/ybc.svg)

  <strong>
    A <a href="https://yew.rs">Yew</a> component library based on the <a href="https://bulma.io">Bulma CSS</a> framework.
  </strong>
</div>
<br/>

YBC encapsulates all of the structure, style and functionality of the Bulma CSS framework as a set of Yew components. YBC also ships with support for the Yew Router, adding Bulma-styled components which wrap the Yew Router components for clean integration.

As a guiding principal, YBC does not attempt to encapsulate every single Bulma style as a Rust type, let alone the many valid style combinations. That would be far too complex, and probably limiting to the user in many ways. Instead, YBC handles strucutre, required classes, functionality, sane defaults and every component can be customized with any additional classes for an exact look and feel.

## getting started
First, add this library to your Yew project's `Cargo.toml`.

```toml
[dependencies]
ybc = "0.1" # NOTE: this release is coming soon.
```

Next, you've got a few options. This project will work perfectly well if you just include the Bulma CSS in your HTML, [as described here](https://bulma.io/documentation/overview/start/). The following link in your HTML head should do the trick: `<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.0/css/bulma.min.css"/>`.

However, if you want to customize Bulma, then you will need to have a copy of the Bulma SASS locally, and then import Bulma after you've defined your customizations, [as described here](https://bulma.io/documentation/customize/).

```scss
// index.scss

// Set your brand colors
$purple: #8A4D76;
$pink: #FA7C91;
$brown: #757763;
$beige-light: #D0D1CD;
$beige-lighter: #EFF0EB;

// Import the rest of Bulma
@import "path/to/bulma";
```

This pattern will work perfectly with the [Trunk project](https://github.com/thedodd/trunk). Simply point to your `index.scss` from your `index.html` file, and Trunk will handle the rest.

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1"/>
    <link rel="stylesheet" href="index.sass"/>
  </head>
  <body>
    ...
  </body>
</html>
```
