#![recursion_limit="1024"]

mod columns;
mod common;
mod components;
mod elements;
mod form;
mod layout;

// columns
pub use columns::{
    Column, ColumnProps,
    Columns, ColumnsProps,
};

// common
pub use common::{Alignment, Size};

// components
pub use components::breadcrumb::{
    Breadcrumb, BreadcrumbProps,
    BreadcrumbSeparator, BreadcrumbSize,
};
pub use components::card::{
    Card, CardProps,
    CardImage, CardImageProps,
    CardHeader, CardHeaderProps,
    CardContent, CardContentProps,
    CardFooter, CardFooterProps,
};
pub use components::dropdown::{
    Dropdown, DropdownProps, DropdownMsg,
};
pub use components::menu::{
    Menu, MenuProps,
    MenuLabel, MenuLabelProps,
    MenuList, MenuListProps,
};
pub use components::message::{
    Message, MessageProps,
    MessageBody, MessageBodyProps,
    MessageHeader, MessageHeaderProps,
};
pub use components::modal::{
    Modal, ModalProps, ModalMsg,
    ModalCard, ModalCardProps,
    ModalCloser, ModalCloseMsg,
};
pub use components::navbar::{
    Navbar, NavbarProps, NavbarMsg,
    NavbarDivider, NavbarDividerProps,
    NavbarDropdown, NavbarDropdownProps,
    NavbarFixed, NavbarItemTag,
    NavbarItem, NavbarItemProps,
};
pub use components::pagination::{
    Pagination, PaginationProps,
    PaginationEllipsis,
    PaginationItem, PaginationItemProps,
    PaginationItemRouter,
    PaginationItemType,
};
pub use components::panel::{
    Panel, PanelProps,
    PanelBlock, PanelBlockProps,
    PanelTabs, PanelTabsProps,
};
pub use components::tabs::{Tabs, TabsProps};

// elements
pub use elements::r#box::{Box, BoxProps};
pub use elements::button::{
    Button, ButtonProps,
    Buttons, ButtonsProps,
    ButtonAnchor, ButtonAnchorProps,
    ButtonAnchorRouter, ButtonRouter, ButtonRouterProps,
    ButtonInputReset, ButtonInputResetProps,
    ButtonInputSubmit, ButtonInputSubmitProps,
    ButtonGroupSize,
};
pub use elements::content::{Content, ContentProps};
pub use elements::delete::{Delete, DeleteProps};
pub use elements::icon::{Icon, IconProps};
pub use elements::image::{Image, ImageProps, ImageSize};
pub use elements::notification::{Notification, NotificationProps};
pub use elements::progress::{Progress, ProgressProps};
pub use elements::table::{Table, TableProps};
pub use elements::tag::{
    Tag, TagProps,
    Tags, TagsProps,
};
pub use elements::title::{
    Title, TitleProps,
    Subtitle, SubtitleProps,
    HeaderSize,
};

// form
pub use form::checkbox::{Checkbox, CheckboxProps};
pub use form::control::{Control, ControlProps};
pub use form::field::{
    Field, FieldProps,
    FieldHorizontal, FieldHorizontalProps,
    AddonsAlign, GroupedAlign, LabelSize,
};
pub use form::file::{File, FileProps};
pub use form::input::{Input, InputProps, InputType};
pub use form::radio::{Radio, RadioProps};
pub use form::select::{
    Select, SelectProps,
    MultiSelect, MultiSelectProps,
};
pub use form::textarea::{TextArea, TextAreaProps};

// layout
pub use layout::container::{Container, ContainerProps};
pub use layout::footer::{Footer, FooterProps};
pub use layout::hero::{Hero, HeroProps, HeroSize};
pub use layout::level::{
    Level, LevelProps,
    LevelItem, LevelItemProps,
    LevelLeft, LevelLeftProps,
    LevelRight, LevelRightProps,
};
pub use layout::media::{
    Media, MediaProps,
    MediaContent, MediaContentProps,
    MediaLeft, MediaLeftProps,
    MediaRight, MediaRightProps,
};
pub use layout::section::{Section, SectionProps, SectionSize};
pub use layout::tile::{Tile, TileProps, TileCtx, TileSize};
