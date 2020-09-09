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

**NOTE WELL:** this library is based on the [Yew branch for optional parameters here](https://github.com/yewstack/yew/pull/1537). As such, it is probably best to wait until that branch has landed before using this crate for production apps ... but that's up to you 🛑.

## getting started
First, add this library to your Yew project's `Cargo.toml`.

```toml
[dependencies]
ybc = "*"
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
