#![cfg_attr(feature = "docinclude", feature(external_doc))]
#![cfg_attr(feature = "docinclude", doc(include = "../README.md"))]
#![recursion_limit = "1024"]

mod columns;
mod common;
mod components;
mod elements;
mod form;
mod layout;

// columns
pub use columns::{Column, ColumnProps, Columns, ColumnsProps};

// common
pub use common::{Alignment, Size};

// components
pub use components::breadcrumb::{Breadcrumb, BreadcrumbProps, BreadcrumbSeparator, BreadcrumbSize};
pub use components::card::{
    Card, CardContent, CardContentProps, CardFooter, CardFooterProps, CardHeader, CardHeaderProps, CardImage, CardImageProps, CardProps,
};
pub use components::dropdown::{Dropdown, DropdownMsg, DropdownProps};
pub use components::menu::{Menu, MenuLabel, MenuLabelProps, MenuList, MenuListProps, MenuProps};
pub use components::message::{Message, MessageBody, MessageBodyProps, MessageHeader, MessageHeaderProps, MessageProps};
pub use components::modal::{Modal, ModalCard, ModalCardProps, ModalCloseMsg, ModalCloser, ModalMsg, ModalProps};
pub use components::navbar::{
    Navbar, NavbarDivider, NavbarDividerProps, NavbarDropdown, NavbarDropdownProps, NavbarFixed, NavbarItem, NavbarItemProps, NavbarItemTag,
    NavbarMsg, NavbarProps,
};
pub use components::pagination::{
    Pagination, PaginationEllipsis, PaginationItem, PaginationItemProps, PaginationItemRouter, PaginationItemType, PaginationProps,
};
pub use components::panel::{Panel, PanelBlock, PanelBlockProps, PanelProps, PanelTabs, PanelTabsProps};
pub use components::tabs::{Tabs, TabsProps};

// elements
pub use elements::button::{
    Button, ButtonAnchor, ButtonAnchorProps, ButtonAnchorRouter, ButtonGroupSize, ButtonInputReset, ButtonInputResetProps, ButtonInputSubmit,
    ButtonInputSubmitProps, ButtonProps, ButtonRouter, ButtonRouterProps, Buttons, ButtonsProps,
};
pub use elements::content::{Content, ContentProps};
pub use elements::delete::{Delete, DeleteProps};
pub use elements::icon::{Icon, IconProps};
pub use elements::image::{Image, ImageProps, ImageSize};
pub use elements::notification::{Notification, NotificationProps};
pub use elements::progress::{Progress, ProgressProps};
pub use elements::r#box::{Box, BoxProps};
pub use elements::table::{Table, TableProps};
pub use elements::tag::{Tag, TagProps, Tags, TagsProps};
pub use elements::title::{HeaderSize, Subtitle, SubtitleProps, Title, TitleProps};

// form
pub use form::checkbox::{Checkbox, CheckboxProps};
pub use form::control::{Control, ControlProps};
pub use form::field::{AddonsAlign, Field, FieldProps, GroupedAlign, LabelSize};
pub use form::file::{File, FileProps};
pub use form::input::{Input, InputProps, InputType};
pub use form::radio::{Radio, RadioProps};
pub use form::select::{MultiSelect, MultiSelectProps, Select, SelectProps};
pub use form::textarea::{TextArea, TextAreaProps};

// layout
pub use layout::container::{Container, ContainerProps};
pub use layout::footer::{Footer, FooterProps};
pub use layout::hero::{Hero, HeroProps, HeroSize};
pub use layout::level::{Level, LevelItem, LevelItemProps, LevelLeft, LevelLeftProps, LevelProps, LevelRight, LevelRightProps};
pub use layout::media::{Media, MediaContent, MediaContentProps, MediaLeft, MediaLeftProps, MediaProps, MediaRight, MediaRightProps};
pub use layout::section::{Section, SectionProps, SectionSize};
pub use layout::tile::{Tile, TileCtx, TileProps, TileSize};
