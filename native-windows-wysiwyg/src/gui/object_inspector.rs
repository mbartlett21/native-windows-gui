use nwd::NwgPartial;
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};


#[derive(Default)]
#[derive(NwgPartial)]
pub struct ObjectInspector {

    #[nwg_layout(flex_direction: FlexDirection::Column)]
    layout: nwg::FlexboxLayout,

    //
    // Current controls tree
    //
    #[nwg_control(text: "Control Tree", v_align: nwg::VTextAlign::Top)]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(30.0) })]
    controls_label: nwg::Label,

    #[nwg_control(
        list_style: nwg::ListViewStyle::Detailed,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::AUTO_COLUMN_SIZE | nwg::ListViewExFlags::FULL_ROW_SELECT,
    )]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Percent(1.0) })]
    control_list: nwg::ListView,

    //
    // Selected control properties
    //
    #[nwg_control(text: "Properties", v_align: nwg::VTextAlign::Top)]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(30.0) })]
    properties_label: nwg::Label,

    #[nwg_control(
        list_style: nwg::ListViewStyle::Detailed,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::AUTO_COLUMN_SIZE | nwg::ListViewExFlags::FULL_ROW_SELECT,
    )]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Percent(1.0) })]
    properties_list: nwg::ListView,
}

impl ObjectInspector {

    pub(super) fn init(&self) {
        let ctrl = &self.control_list;
        ctrl.set_headers_enabled(true);
        ctrl.insert_column("Name");
        ctrl.insert_column("Type");
    
        let prop = &self.properties_list;
        prop.set_headers_enabled(true);
        prop.insert_column("Name");
        prop.insert_column("Value");

        self.set_enabled(false);
    }

    pub(super) fn set_enabled(&self, enabled: bool) {
        self.properties_list.set_enabled(enabled);
        self.control_list.set_enabled(enabled);
    }
}
