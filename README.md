<h1 align="center">ybc</h1>
<div align="center">

[![Build Status](https://github.com/thedodd/ybc/workflows/ci/badge.svg?branch=master)](https://github.com/thedodd/ybc/actions)
[![Crates.io](https://img.shields.io/crates/v/ybc.svg?style=flat-square)](https://crates.io/crates/ybc)
[![docs.rs](https://docs.rs/ybc/badge.svg?style=flat-square)](https://docs.rs/ybc)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square)](LICENSE)
[![Discord Chat](https://img.shields.io/discord/793890238267260958?logo=discord&style=flat-square)](https://discord.gg/JEPdBujTDr)
![Crates.io](https://img.shields.io/crates/d/ybc.svg?style=flat-square)

  <strong>
    A <a href="https://yew.rs">Yew</a> component library based on the <a href="https://bulma.io">Bulma CSS</a> framework.
  </strong>
</div>
<br/>

YBC encapsulates all of the structure, style and functionality of the Bulma CSS framework as a set of Yew components. YBC also ships with support for the Yew Router, adding Bulma-styled components which wrap the Yew Router components for clean integration.

As a guiding principal, YBC does not attempt to encapsulate every single Bulma style as a Rust type, let alone the many valid style combinations. That would be far too complex, and probably limiting to the user in many ways. Instead, YBC handles strucutre, required classes, functionality, sane defaults and every component can be customized with any additional classes for an exact look and feel.

What does it look like to use YBC? The following is a snippet of a component's `view` method rendering a navbar, a fluid container, and some tiles.

```rust
use ybc::NavbarFixed::Top;
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::Four;
use yew::prelude::*;

struct App; // An application component.

impl Component for App {
  /* .. snip .. */
  fn view(&self) -> Html {
    html!{
      <>
      <ybc::Navbar fixed=Top /* .. your navbar content here .. *//>
      <ybc::Container fluid=true>
        <ybc::Tile ctx=Ancestor>
          <ybc::Tile ctx=Parent vertical=true size=Four>
            <ybc::Tile ctx=Child classes="box">
              <p>{"Lorem ipsum dolor sit amet ..."}</p>
            </ybc::Tile>
            /* .. snip .. more tiles here .. */
          </ybc::Tile>
        </ybc::Tile>
      </ybc::Container>
      </>
    }
  }
}
```

## getting started
### add ybc dependency
First, add this library to your `Cargo.toml` dependencies.

```toml
[dependencies]
ybc = "*"
```

### add bulma
#### add bulma css (no customizations)
This project works perfectly well if you just include the Bulma CSS in your HTML, [as described here](https://bulma.io/documentation/overview/start/). The following link in your HTML head should do the trick: `<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.1/css/bulma.min.css"/>`.

#### add bulma sass (allows customization & themes)
However, if you want to customize Bulma to match your style guidelines, then you will need to have a copy of the Bulma SASS locally, and then import Bulma after you've defined your customizations, [as described here](https://bulma.io/documentation/customize/).

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

If you are using [Trunk](https://github.com/thedodd/trunk) to build your application and bundle its assets, then simply point to your `index.scss` from your `index.html` file, and Trunk will handle compling your application, your sass, and will make everything available in your `dist` dir.

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1"/>
    <link rel="stylesheet" href="index.sass"/>
  </head>
  <body>
    <!-- ... snip ... -->
  </body>
</html>
```

Now just execute `trunk serve --open`, and your application will be built and opened in your browser.

If you are not using [Trunk](https://github.com/thedodd/trunk), you will need to use another mechanism for building your Rust WASM application and its assets.

## web-sys & stdweb
Currently, this library only supports the web-sys backend. Support for stdweb is not currently planned. If that is problematic, please open an issue describing why. Cheers!
