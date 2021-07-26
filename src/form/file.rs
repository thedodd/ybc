#![allow(clippy::redundant_closure_call)]

use web_sys::File as SysFile;
use yew::events::ChangeData;
use yew::prelude::*;
use yewtil::NeqAssign;

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
pub struct File {
    props: FileProps,
    link: ComponentLink<Self>,
}

impl Component for File {
    type Message = Vec<SysFile>;
    type Properties = FileProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.update.emit(msg);
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("file");
        classes.push(&self.props.classes);
        if self.props.has_name.is_some() {
            classes.push("has-name");
        }
        if self.props.right {
            classes.push("is-right");
        }
        if self.props.fullwidth {
            classes.push("is-fullwidth");
        }
        if self.props.boxed {
            classes.push("is-boxed");
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if let Some(alignment) = &self.props.alignment {
            classes.push(&alignment.to_string());
        }
        let filenames = self
            .props
            .files
            .iter()
            .map(|file| html! {<span class="file-name">{file.name()}</span>})
            .collect::<Vec<_>>();
        html! {
            <div class=classes>
                <label class="file-label">
                    <input
                        type="file"
                        class="file-input"
                        name=self.props.name.clone()
                        multiple=self.props.multiple
                        onchange=self.link.callback(|data: ChangeData| match data {
                            ChangeData::Files(list) => (0..list.length()).into_iter()
                                .filter_map(|idx| list.item(idx))
                                .collect::<Vec<_>>(),
                            _ => unreachable!("invariant violation: received non-file change event from a file input element"),
                        })
                        />
                    <span class="file-cta">
                        <span class="file-icon">
                            {self.props.selector_icon.clone()}
                        </span>
                        <span class="file-label">
                            {self.props.selector_label.clone()}
                        </span>
                    </span>
                    {filenames}
                </label>
            </div>
        }
    }
}
