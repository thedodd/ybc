#![allow(clippy::redundant_closure_call)]

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{File as SysFile, HtmlInputElement};
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FileProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled form value for the currently selected files.
    pub files: Vec<SysFile>,
    /// The callback to be used for propagating changes to this form element.
    pub update: Callback<Vec<SysFile>>,

    /// The display text for the file selector.
    #[prop_or_else(|| "Choose a file...".into())]
    pub selector_label: String,
    /// The HTML contents to use for the file selector icon.
    #[prop_or_default]
    pub selector_icon: Html,

    #[prop_or_default]
    pub classes: Option<Classes>,
    /// An option to control if file names will be displayed; if a value is provided, then the
    /// `has-name` class will be added to this form element and the given value will be used as a
    /// placeholder until files are selected.
    #[prop_or_default]
    pub has_name: Option<String>,
    /// Move the CTA element to the right side of the component.
    #[prop_or_default]
    pub right: bool,
    /// Expand the file display name to the full width of the parent.
    #[prop_or_default]
    pub fullwidth: bool,
    /// Display as a boxed block.
    #[prop_or_default]
    pub boxed: bool,
    /// Allow multiple files to be selected.
    #[prop_or_default]
    pub multiple: bool,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this component within its parent.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
}

/// A custom file upload input.
///
/// [https://bulma.io/documentation/form/file/](https://bulma.io/documentation/form/file/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[function_component(File)]
pub fn file(props: &FileProps) -> Html {
    let class = classes!(
        "file",
        props.classes.clone(),
        props.has_name.is_some().then(|| "has-name"),
        props.right.then(|| "is-right"),
        props.fullwidth.then(|| "is-fullwidth"),
        props.boxed.then(|| "is-boxed"),
        props.size.as_ref().map(|size| size.to_string()),
        props.alignment.as_ref().map(|alignment| alignment.to_string()),
    );
    let filenames = props
        .files
        .iter()
        .map(|file| html! {<span class="file-name">{file.name()}</span>})
        .collect::<Vec<_>>();
    let onchange = props.update.reform(|ev: web_sys::Event| {
        let target = ev.target().expect_throw("event should have a target");
        let input: HtmlInputElement = target.dyn_into().expect_throw("event target should be an input");
        let list = input.files().expect_throw("input should have a file list");
        (0..list.length())
            .into_iter()
            .filter_map(|idx| list.item(idx))
            .collect::<Vec<_>>()
    });
    html! {
        <div {class}>
            <label class="file-label">
                <input
                    type="file"
                    class="file-input"
                    name={props.name.clone()}
                    multiple={props.multiple}
                    {onchange}
                    />
                <span class="file-cta">
                    <span class="file-icon">
                        {props.selector_icon.clone()}
                    </span>
                    <span class="file-label">
                        {props.selector_label.clone()}
                    </span>
                </span>
                {filenames}
            </label>
        </div>
    }
}
