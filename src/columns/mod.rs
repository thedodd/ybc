use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

pub struct Columns {
    props: ColumnsProps,
}

impl Component for Columns {
    type Message = ();
    type Properties = ColumnsProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("columns");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

pub struct Column {
    props: ColumnProps,
}

impl Component for Column {
    type Message = ();
    type Properties = ColumnProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("column");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}

// //////////////////////////////////////////////////////////////////////////////
// //////////////////////////////////////////////////////////////////////////////

// /// One of the column sizing options.
// #[derive(Clone, Debug, Display, PartialEq)]
// #[display(fmt="is-{}")]
// pub enum ColumnSize {
//     Is(ColumnIs),
//     Narrow(ColumnNarrow),
// }

// /// A column size offset.
// ///
// /// https://bulma.io/documentation/columns/sizes/#offset
// #[derive(Clone, Debug, Display, PartialEq)]
// #[display(fmt="is-offset-{}")]
// pub enum ColumnOffset {
//     Is(ColumnIs),
// }

// /// An enum controlling Bulma's column sizes.
// ///
// /// https://bulma.io/documentation/columns/sizes/
// #[derive(Clone, Debug, Display, PartialEq)]
// pub enum ColumnIs {
//     /// Corresponds to bulma's `is-three-quarters` class.
//     #[display(fmt="three-quarters")]
//     ThreeQuarters,
//     /// Corresponds to bulma's `is-two-thirds` class.
//     #[display(fmt="two-thirds")]
//     TwoThirds,
//     /// Corresponds to bulma's `is-half` class.
//     #[display(fmt="half")]
//     Half,
//     /// Corresponds to bulma's `is-one-third` class.
//     #[display(fmt="one-third")]
//     OneThird,
//     /// Corresponds to bulma's `is-one-quarter` class.
//     #[display(fmt="one-quarter")]
//     OneQuarter,
//     /// Corresponds to bulma's `is-full` class.
//     #[display(fmt="full")]
//     Full,

//     /// Corresponds to bulma's `is-four-fifths` class.
//     #[display(fmt="four-fifths")]
//     FourFifths,
//     /// Corresponds to bulma's `is-three-fifths` class.
//     #[display(fmt="three-fifths")]
//     ThreeFifths,
//     /// Corresponds to bulma's `is-two-fifths` class.
//     #[display(fmt="two-fifths")]
//     TwoFifths,
//     /// Corresponds to bulma's `is-one-fifth` class.
//     #[display(fmt="one-fifth")]
//     OneFifth,

//     /// Corresponds to bulma's `is-1` class.
//     #[display(fmt="1")]
//     One,
//     /// Corresponds to bulma's `is-2` class.
//     #[display(fmt="2")]
//     Two,
//     /// Corresponds to bulma's `is-3` class.
//     #[display(fmt="3")]
//     Three,
//     /// Corresponds to bulma's `is-4` class.
//     #[display(fmt="4")]
//     Four,
//     /// Corresponds to bulma's `is-5` class.
//     #[display(fmt="5")]
//     Five,
//     /// Corresponds to bulma's `is-6` class.
//     #[display(fmt="6")]
//     Six,
//     /// Corresponds to bulma's `is-7` class.
//     #[display(fmt="7")]
//     Seven,
//     /// Corresponds to bulma's `is-8` class.
//     #[display(fmt="8")]
//     Eight,
//     /// Corresponds to bulma's `is-9` class.
//     #[display(fmt="9")]
//     Nine,
//     /// Corresponds to bulma's `is-10` class.
//     #[display(fmt="10")]
//     Ten,
//     /// Corresponds to bulma's `is-11` class.
//     #[display(fmt="11")]
//     Eleven,
//     /// Corresponds to bulma's `is-12` class.
//     #[display(fmt="12")]
//     Twelve,
// }

// /// An enum controlling Bulma's column sizes.
// ///
// /// https://bulma.io/documentation/columns/sizes/
// #[derive(Clone, Debug, Display, PartialEq)]
// pub enum ColumnNarrow {
//     /// Corresponds to bulma's `is-narrow` class.
//     #[display(fmt="narrow")]
//     Narrow,
//     /// Corresponds to bulma's `is-narrow-mobile` class.
//     #[display(fmt="narrow-mobile")]
//     Mobile,
//     /// Corresponds to bulma's `is-narrow-tablet` class.
//     #[display(fmt="narrow-tablet")]
//     Tablet,
//     /// Corresponds to bulma's `is-narrow-touch` class.
//     #[display(fmt="narrow-touch")]
//     Touch,
//     /// Corresponds to bulma's `is-narrow-desktop` class.
//     #[display(fmt="narrow-desktop")]
//     Desktop,
//     /// Corresponds to bulma's `is-narrow-widescreen` class.
//     #[display(fmt="narrow-widescreen")]
//     Widescreen,
//     /// Corresponds to bulma's `is-narrow-fullhd` class.
//     #[display(fmt="narrow-fullhd")]
//     Fullhd,
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     macro_rules! test_column_is_display {
//         ({test=>$name:ident, val=>$val:expr, expected=>$exp:literal}) => {
//             #[test]
//             fn $name() {
//                 let value = format!("{}", ColumnSize::Is($val));
//                 assert_eq!(value.as_str(), $exp);
//             }
//         }
//     }

//     macro_rules! test_column_is_narrow_display {
//         ({test=>$name:ident, val=>$val:expr, expected=>$exp:literal}) => {
//             #[test]
//             fn $name() {
//                 let value = format!("{}", ColumnSize::Narrow($val));
//                 assert_eq!(value.as_str(), $exp);
//             }
//         }
//     }

//     macro_rules! test_column_is_offset_display {
//         ({test=>$name:ident, val=>$val:expr, expected=>$exp:literal}) => {
//             #[test]
//             fn $name() {
//                 let value = format!("{}", ColumnOffset::Is($val));
//                 assert_eq!(value.as_str(), $exp);
//             }
//         }
//     }

//     test_column_is_display!({test=>class_three_quarters, val=>ColumnIs::ThreeQuarters, expected=>"is-three-quarters"});
//     test_column_is_display!({test=>class_two_thirds, val=>ColumnIs::TwoThirds, expected=>"is-two-thirds"});
//     test_column_is_display!({test=>class_half, val=>ColumnIs::Half, expected=>"is-half"});
//     test_column_is_display!({test=>class_one_third, val=>ColumnIs::OneThird, expected=>"is-one-third"});
//     test_column_is_display!({test=>class_one_quarter, val=>ColumnIs::OneQuarter, expected=>"is-one-quarter"});
//     test_column_is_display!({test=>class_full, val=>ColumnIs::Full, expected=>"is-full"});
//     test_column_is_display!({test=>class_four_fifths, val=>ColumnIs::FourFifths, expected=>"is-four-fifths"});
//     test_column_is_display!({test=>class_three_fifths, val=>ColumnIs::ThreeFifths, expected=>"is-three-fifths"});
//     test_column_is_display!({test=>class_two_fifths, val=>ColumnIs::TwoFifths, expected=>"is-two-fifths"});
//     test_column_is_display!({test=>class_one_fifth, val=>ColumnIs::OneFifth, expected=>"is-one-fifth"});
//     test_column_is_display!({test=>class_one, val=>ColumnIs::One, expected=>"is-1"});
//     test_column_is_display!({test=>class_two, val=>ColumnIs::Two, expected=>"is-2"});
//     test_column_is_display!({test=>class_three, val=>ColumnIs::Three, expected=>"is-3"});
//     test_column_is_display!({test=>class_four, val=>ColumnIs::Four, expected=>"is-4"});
//     test_column_is_display!({test=>class_five, val=>ColumnIs::Five, expected=>"is-5"});
//     test_column_is_display!({test=>class_six, val=>ColumnIs::Six, expected=>"is-6"});
//     test_column_is_display!({test=>class_seven, val=>ColumnIs::Seven, expected=>"is-7"});
//     test_column_is_display!({test=>class_eight, val=>ColumnIs::Eight, expected=>"is-8"});
//     test_column_is_display!({test=>class_nine, val=>ColumnIs::Nine, expected=>"is-9"});
//     test_column_is_display!({test=>class_ten, val=>ColumnIs::Ten, expected=>"is-10"});
//     test_column_is_display!({test=>class_eleven, val=>ColumnIs::Eleven, expected=>"is-11"});
//     test_column_is_display!({test=>class_twelve, val=>ColumnIs::Twelve, expected=>"is-12"});

//     test_column_is_narrow_display!({test=>class_narrow, val=>ColumnNarrow::Narrow, expected=>"is-narrow"});
//     test_column_is_narrow_display!({test=>class_mobile, val=>ColumnNarrow::Mobile, expected=>"is-narrow-mobile"});
//     test_column_is_narrow_display!({test=>class_tablet, val=>ColumnNarrow::Tablet, expected=>"is-narrow-tablet"});
//     test_column_is_narrow_display!({test=>class_touch, val=>ColumnNarrow::Touch, expected=>"is-narrow-touch"});
//     test_column_is_narrow_display!({test=>class_desktop, val=>ColumnNarrow::Desktop, expected=>"is-narrow-desktop"});
//     test_column_is_narrow_display!({test=>class_widescreen, val=>ColumnNarrow::Widescreen, expected=>"is-narrow-widescreen"});
//     test_column_is_narrow_display!({test=>class_fullhd, val=>ColumnNarrow::Fullhd, expected=>"is-narrow-fullhd"});

//     test_column_is_offset_display!({test=>class_offset_three_quarters, val=>ColumnIs::ThreeQuarters, expected=>"is-offset-three-quarters"});
//     test_column_is_offset_display!({test=>class_offset_two_thirds, val=>ColumnIs::TwoThirds, expected=>"is-offset-two-thirds"});
//     test_column_is_offset_display!({test=>class_offset_half, val=>ColumnIs::Half, expected=>"is-offset-half"});
//     test_column_is_offset_display!({test=>class_offset_one_third, val=>ColumnIs::OneThird, expected=>"is-offset-one-third"});
//     test_column_is_offset_display!({test=>class_offset_one_quarter, val=>ColumnIs::OneQuarter, expected=>"is-offset-one-quarter"});
//     test_column_is_offset_display!({test=>class_offset_full, val=>ColumnIs::Full, expected=>"is-offset-full"});
//     test_column_is_offset_display!({test=>class_offset_four_fifths, val=>ColumnIs::FourFifths, expected=>"is-offset-four-fifths"});
//     test_column_is_offset_display!({test=>class_offset_three_fifths, val=>ColumnIs::ThreeFifths, expected=>"is-offset-three-fifths"});
//     test_column_is_offset_display!({test=>class_offset_two_fifths, val=>ColumnIs::TwoFifths, expected=>"is-offset-two-fifths"});
//     test_column_is_offset_display!({test=>class_offset_one_fifth, val=>ColumnIs::OneFifth, expected=>"is-offset-one-fifth"});
//     test_column_is_offset_display!({test=>class_offset_one, val=>ColumnIs::One, expected=>"is-offset-1"});
//     test_column_is_offset_display!({test=>class_offset_two, val=>ColumnIs::Two, expected=>"is-offset-2"});
//     test_column_is_offset_display!({test=>class_offset_three, val=>ColumnIs::Three, expected=>"is-offset-3"});
//     test_column_is_offset_display!({test=>class_offset_four, val=>ColumnIs::Four, expected=>"is-offset-4"});
//     test_column_is_offset_display!({test=>class_offset_five, val=>ColumnIs::Five, expected=>"is-offset-5"});
//     test_column_is_offset_display!({test=>class_offset_six, val=>ColumnIs::Six, expected=>"is-offset-6"});
//     test_column_is_offset_display!({test=>class_offset_seven, val=>ColumnIs::Seven, expected=>"is-offset-7"});
//     test_column_is_offset_display!({test=>class_offset_eight, val=>ColumnIs::Eight, expected=>"is-offset-8"});
//     test_column_is_offset_display!({test=>class_offset_nine, val=>ColumnIs::Nine, expected=>"is-offset-9"});
//     test_column_is_offset_display!({test=>class_offset_ten, val=>ColumnIs::Ten, expected=>"is-offset-10"});
//     test_column_is_offset_display!({test=>class_offset_eleven, val=>ColumnIs::Eleven, expected=>"is-offset-11"});
//     test_column_is_offset_display!({test=>class_offset_twelve, val=>ColumnIs::Twelve, expected=>"is-offset-12"});

// }
