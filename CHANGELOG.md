changelog
=========
This changelog follows the patterns described here: https://keepachangelog.com/en/1.0.0/.

## Unreleased

## 0.1.7
### added
- Added field `id` to compenents missing the field.

## 0.1.6
### changed
- Updates to the `Navbar` component:
    - `navbrand`, `navstart`, `navend` are now all optional.
    - a new `navburger: bool` property has been added. This bool controls whether or not a `navbar-burger` will be rendered inside of the navbar when being rendered within smaller viewports. This value defaults to `true`, maintaining backwards compatibility.

## 0.1.5
### fixed
- Fixed a few of the button & button-like components to use the HTML `disabled` attribute instead of the Bulma `is-disabled` CSS class. The latter has been deprecated for some time.
- Update docs in the README to point to the latest Bulma 0.9.1 release.

## 0.1.4
### added
- Add prop `padded` to Navbar. Setting this to true will wrap the contents of the navbar in a container to add padding to the contents under some circumstances.
- The default tag type for NavbarItems is now `div`.
- Added the `href`, `rel` & `target` props to the `NavbarItem` component. They will only be used when the `NavbarItemTag::A` is being used for the component.
- Added the `rel` & `target` props to the `ButtonAnchor` component.
- Adds an additional size for heros: `is-fullheight-with-navbar`. This one is present in the Bulma docs, but is slightly hidden.

## 0.1.3
### fixed
- Removed `Children` props from Hero & made header & footer optional.

### added
- Added the `centered`, `multiline` & `vcentered` props to the `Columns` component, corresponding to the standard Bulma classes bearing the same name prefixed with `is-`.

## 0.1.0
- Initialize release. See the [release notes on Github](https://github.com/thedodd/ybc/releases/tag/v0.1.0) for more info.
