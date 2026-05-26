# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces , unused_parens)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] # [allow (clippy :: overly_complex_bool_expr)] mod slint_generatedAppWindow {
     use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#TabInfo {
         pub r#active : bool , pub r#title : sp :: SharedString , pub r#url : sp :: SharedString }
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#TextStyle {
         pub r#font_size : f32 , pub r#font_weight : i32 }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_127 {
         r#dark_color_scheme : sp :: Property < bool > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerColorSchemeSelector_127 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerFluentPalette_128 {
         r#text_control_border : sp :: Property < slint :: Brush > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerFluentPalette_128 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_128 :: FIELD_OFFSETS . r#text_control_border }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (352321535f64 as u32) , position : ((99.98f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (2332033023f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (2332033023f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ])) }
                     else {
                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (251658240f64 as u32) , position : ((99.99f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (1929379840f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (1929379840f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ]))) as _ }
                    ) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerStyleMetrics_129 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerStyleMetrics_129 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerLineEditBase_root_107 {
         r#root_107 : sp :: r#Empty , r#root_clip_108 : sp :: r#Clip , r#i_placeholder_109 : sp :: r#Text , r#i_text_input_110 : sp :: r#TextInput , r#root_107_has_focus : sp :: Property < bool > , r#root_107_height : sp :: Property < sp :: LogicalLength > , r#root_107_i_placeholder_109_horizontal_stretch : sp :: Property < f32 > , r#root_107_i_placeholder_109_max_height : sp :: Property < sp :: LogicalLength > , r#root_107_i_placeholder_109_max_width : sp :: Property < sp :: LogicalLength > , r#root_107_i_placeholder_109_min_height : sp :: Property < sp :: LogicalLength > , r#root_107_i_placeholder_109_min_width : sp :: Property < sp :: LogicalLength > , r#root_107_i_placeholder_109_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_107_i_placeholder_109_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_107_i_placeholder_109_vertical_stretch : sp :: Property < f32 > , r#root_107_i_placeholder_109_x : sp :: Property < sp :: LogicalLength > , r#root_107_i_placeholder_109_y : sp :: Property < sp :: LogicalLength > , r#root_107_i_text_input_110_computed_x : sp :: Property < sp :: LogicalLength > , r#root_107_i_text_input_110_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_107_i_text_input_110_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_107_i_text_input_110_x : sp :: Property < sp :: LogicalLength > , r#root_107_i_text_input_110_y : sp :: Property < sp :: LogicalLength > , r#root_107_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_107_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_107_margin : sp :: Property < sp :: LogicalLength > , r#root_107_placeholder_color : sp :: Property < slint :: Brush > , r#root_107_placeholder_text : sp :: Property < sp :: SharedString > , r#root_107_root_clip_108_x : sp :: Property < sp :: LogicalLength > , r#root_107_root_clip_108_y : sp :: Property < sp :: LogicalLength > , r#root_107_text_color : sp :: Property < slint :: Brush > , r#root_107_width : sp :: Property < sp :: LogicalLength > , r#root_107_x : sp :: Property < sp :: LogicalLength > , r#root_107_y : sp :: Property < sp :: LogicalLength > , r#root_107_accepted : sp :: Callback < (sp :: SharedString ,) , () > , r#root_107_edited : sp :: Callback < (sp :: SharedString ,) , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEditBase_root_107 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEditBase_root_107 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((((({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as sp :: Coord) . max ((({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_computed_x }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_clip_108 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_placeholder_color }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from ("")))) && (((({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#preedit_text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from (""))))) {
                         ({
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_placeholder_text }
                        ) . apply_pin (_self) . get () }
                     else {
                         (sp :: SharedString :: from ("")) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#accepted) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_accepted }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_text_color }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x as f64) + (({
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_computed_x }
                            ) . apply_pin (_self) . get () . get () as f64)) as f64) < (({
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_margin }
                            ) . apply_pin (_self) . get () . get () as f64)) {
                                 {
                                     ({
                                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_computed_x }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- (args . 0 . clone ()) . r#x as f64) + (({
                                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_margin }
                                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#x as f64) + (({
                                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_computed_x }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) > (((((({
                                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_width }
                                ) . apply_pin (_self) . get () . get () as f64) - (({
                                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_margin }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_computed_x }
                                        ) . apply_pin (_self) . set (sp :: LogicalLength :: new (((((((({
                                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#x as f64)) as f64) - (({
                                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_margin }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as sp :: Coord) as _) }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_edited }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Color :: from_argb_encoded (4286611584f64 as u32)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (1f64 as f64)) as sp :: Coord) . max ((({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_root_clip_108_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_root_clip_108_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_clip_108 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_clip_108 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_clip_108 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_clip_108 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_clip_108 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_clip_108 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_min_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_root_clip_108_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_root_clip_108_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (((1f64 as f64) * (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ((1f64 as f64) * (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_width }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (((1f64 as f64) * (({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (2u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , _ => :: core :: default :: Default :: default () , }
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_selection (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                ) . apply_pin (_self) . r#clear_selection ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_copy (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                ) . apply_pin (_self) . r#copy ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_cut (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                ) . apply_pin (_self) . r#cut ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_paste (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                ) . apply_pin (_self) . r#paste ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_select_all (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                ) . apply_pin (_self) . r#select_all ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_set_selection_offsets (self : :: core :: pin :: Pin < & Self > , arg_0 : i32 , arg_1 : i32 ,) -> () {
             let _self = self ;
             let args = (arg_0 , arg_1 ,) ;
             ({
                 ({
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                ) . apply_pin (_self) . set_selection_offsets ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1) , args . 0 . clone () as i32 , args . 1 . clone () as i32) }
            ) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerLineEdit_root_111 {
         r#root_111 : sp :: r#Empty , r#i_background_112 : sp :: r#BasicBorderRectangle , r#i_focus_border_115 : sp :: r#Rectangle , r#i_base_114 : InnerLineEditBase_root_107 , r#root_111_height : sp :: Property < sp :: LogicalLength > , r#root_111_horizontal_stretch : sp :: Property < f32 > , r#root_111_i_background_112_width : sp :: Property < sp :: LogicalLength > , r#root_111_i_background_112_x : sp :: Property < sp :: LogicalLength > , r#root_111_i_background_112_y : sp :: Property < sp :: LogicalLength > , r#root_111_i_layout_113_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_111_i_layout_113_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_111_i_layout_113_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_111_state : sp :: Property < i32 > , r#root_111_width : sp :: Property < sp :: LogicalLength > , r#root_111_x : sp :: Property < sp :: LogicalLength > , r#root_111_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEdit_root_111 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEdit_root_111 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerLineEditBase_root_107 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_base_114 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 2u32 - 1 , tree_index_of_first_child + 4u32 - 1) ;
             ({
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_horizontal_stretch }
            ) . apply_pin (_self) . set ({
                 (1f64) as f32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_background_112_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                                 + {
                                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                                         InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                                     + {
                                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_min_width }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_background_112_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                             + {
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                                 + {
                                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_min_width }
                                ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                             + {
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                                 + {
                                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_preferred_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                     + {
                         * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                         + {
                             * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_has_focus }
                        ) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (0f64) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_background_112 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_111_state = ({
                             * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_111_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (184549375f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1308228089f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_111_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (3005095454f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (268435455f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (3019898879f64 as u32)) as _ }
                                )) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_background_112 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_111_state = ({
                             * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_111_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_111_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                                ) }
                             else {
                                 (InnerFluentPalette_128 :: FIELD_OFFSETS . r#text_control_border . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_FluentPalette_128 . as_ref ()) . get ()) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_background_112 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_background_112 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                 + {
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1.0766f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (400f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                 + {
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_margin }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                 + {
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_placeholder_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_111_state = ({
                             * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_111_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (1593835519f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1577058304f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_111_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (2332033023f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (3388997631f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (2566914048f64 as u32)) as _ }
                                )) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4278221012f64 as u32)) . color ()) as sp :: Color }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                 + {
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (2281701375f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                        ) . color () }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                        ) . color ()) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                 + {
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (1593835519f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1577058304f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                 + {
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                 + {
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_focus_border_115 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_state }
                    ) . apply_pin (_self) . get () as f64) == (2f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4284534271f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278214584f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_background_112_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_background_112_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_background_112 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_background_112 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_placeholder_109_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_i_text_input_110_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_margin }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_root_clip_108_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_root_clip_108_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerLineEditBase_root_107 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_base_114 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                             * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                             * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_background_112_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_background_112_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                 + {
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (2f64 as sp :: Coord , ((({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_width }
                ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as sp :: Coord , 4f64 as sp :: Coord , ((({
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_height }
                ) . apply_pin (_self) . get () . get () as f64) - (2f64 as f64)) as sp :: Coord ,) , 4u32 ..= 6u32 => return {
                     * & Self :: FIELD_OFFSETS . r#i_base_114 }
                 . apply_pin (_self) . item_geometry (index - 4u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_base_114 }
                 . apply_pin (_self) . accessible_role (0) , 4u32 ..= 6u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_base_114 }
                 . apply_pin (_self) . accessible_role (index - 4u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (2u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_base_114 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (4u32 ..= 6u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_base_114 }
                 . apply_pin (_self) . accessible_string_property (index - 4u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerScrollBar_root_116 {
         r#root_116 : sp :: r#BasicBorderRectangle , r#i_thumb_117 : sp :: r#BasicBorderRectangle , r#i_touch_area_118 : sp :: r#TouchArea , r#i_up_scroll_button_opacity_119 : sp :: r#Opacity , r#i_up_scroll_button_120 : sp :: r#TouchArea , r#i_icon_opacity_121 : sp :: r#Opacity , r#i_icon_122 : sp :: r#ImageItem , r#i_down_scroll_button_opacity_123 : sp :: r#Opacity , r#i_down_scroll_button_124 : sp :: r#TouchArea , r#i_icon_opacity_125 : sp :: r#Opacity , r#i_icon_126 : sp :: r#ImageItem , r#root_116_height : sp :: Property < sp :: LogicalLength > , r#root_116_horizontal : sp :: Property < bool > , r#root_116_i_down_scroll_button_124_icon : sp :: Property < sp :: Image > , r#root_116_i_down_scroll_button_124_state : sp :: Property < i32 > , r#root_116_i_down_scroll_button_124_x : sp :: Property < sp :: LogicalLength > , r#root_116_i_down_scroll_button_124_y : sp :: Property < sp :: LogicalLength > , r#root_116_i_down_scroll_button_opacity_123_dummy : sp :: Property < sp :: LogicalLength > , r#root_116_i_icon_opacity_121_dummy : sp :: Property < sp :: LogicalLength > , r#root_116_i_icon_opacity_125_dummy : sp :: Property < sp :: LogicalLength > , r#root_116_i_thumb_117_height : sp :: Property < sp :: LogicalLength > , r#root_116_i_thumb_117_width : sp :: Property < sp :: LogicalLength > , r#root_116_i_thumb_117_x : sp :: Property < sp :: LogicalLength > , r#root_116_i_thumb_117_y : sp :: Property < sp :: LogicalLength > , r#root_116_i_touch_area_118_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_116_i_touch_area_118_x : sp :: Property < sp :: LogicalLength > , r#root_116_i_touch_area_118_y : sp :: Property < sp :: LogicalLength > , r#root_116_i_up_scroll_button_120_icon : sp :: Property < sp :: Image > , r#root_116_i_up_scroll_button_120_state : sp :: Property < i32 > , r#root_116_i_up_scroll_button_120_x : sp :: Property < sp :: LogicalLength > , r#root_116_i_up_scroll_button_120_y : sp :: Property < sp :: LogicalLength > , r#root_116_i_up_scroll_button_opacity_119_dummy : sp :: Property < sp :: LogicalLength > , r#root_116_maximum : sp :: Property < sp :: LogicalLength > , r#root_116_page_size : sp :: Property < sp :: LogicalLength > , r#root_116_size : sp :: Property < sp :: LogicalLength > , r#root_116_state : sp :: Property < i32 > , r#root_116_track_size : sp :: Property < sp :: LogicalLength > , r#root_116_value : sp :: Property < sp :: LogicalLength > , r#root_116_width : sp :: Property < sp :: LogicalLength > , r#root_116_x : sp :: Property < sp :: LogicalLength > , r#root_116_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerScrollBar_root_116 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerScrollBar_root_116 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4281084972f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4293980400f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_124_icon }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg")) }
                     else {
                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_124_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_124 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_124 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (0f64) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_124_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) - (4f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_124_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) - (4f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         ({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_size }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         ({
                             let r#tmp_root_116_maximum = ({
                                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_root_116_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_root_116_page_size = ({
                                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_page_size }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((16f64 as sp :: Coord) . max ((((({
                                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (((r#tmp_root_116_page_size . clone () as f64) / (((r#tmp_root_116_maximum . clone () as f64) + (r#tmp_root_116_page_size . clone () as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         ({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_size }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         ({
                             let r#tmp_root_116_maximum = ({
                                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_root_116_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_root_116_page_size = ({
                                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_page_size }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((16f64 as sp :: Coord) . max ((((((({
                                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (r#tmp_root_116_page_size . clone () as f64)) as f64) / (((r#tmp_root_116_maximum . clone () as f64) + (r#tmp_root_116_page_size . clone () as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) - (({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_width }
                        ) . apply_pin (_self) . get () . get () as f64)) }
                     else {
                         (((16f64 as f64) + (((((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                        ) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) - (({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_height }
                        ) . apply_pin (_self) . get () . get () as f64)) }
                     else {
                         (((16f64 as f64) + (((((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                        ) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_touch_area_118_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_touch_area_118_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_120_icon }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) }
                     else {
                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_120_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_120 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_120 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (0f64) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_120_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (4f64) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_120_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (4f64) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         6f64 }
                     else {
                         (2f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ()) || (({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_124 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ()))) || (({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_120 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) {
                         1f64 }
                     else {
                         (0f64) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_track_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         ((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (32f64 as f64)) }
                     else {
                         (((({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (32f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_thumb_117 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_thumb_117 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         ({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_height }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         (({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_width }
                        ) . apply_pin (_self) . get () . get ()) as _ }
                     as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true) && (({
                                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 {
                                     ({
                                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (- (0f64 as sp :: Coord) . max (((({
                                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_touch_area_118_pressed_value }
                                    ) . apply_pin (_self) . get () . get () as f64) + (if ({
                                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                                    ) . apply_pin (_self) . get () {
                                         ((((({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_width }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) }
                                     else {
                                         (((((({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_height }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                                     as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button) == (sp :: r#PointerEventButton :: r#Left))) && ((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Down)))) {
                                 {
                                     ({
                                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_touch_area_118_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (- ({
                                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression0 = {
                                 let r#return_check_merge0 = if ((({
                                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                                ) . apply_pin (_self) . get ()) && ((((args . 0 . clone ()) . r#delta_x as f64) != (0f64 as f64)))) {
                                     (false , {
                                         ({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                                        ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                                        ) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_x as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,) }
                                 else {
                                     (if ! ((! ({
                                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                                    ) . apply_pin (_self) . get ()) && ((((args . 0 . clone ()) . r#delta_y as f64) != (0f64 as f64)))) {
                                         {
                                             {
                                                 }
                                             ;
                                             (true , sp :: r#EventResult :: r#Reject ,) }
                                         }
                                     else {
                                         ((false , {
                                             ({
                                                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                                            ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                                            ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                                            ) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_y as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,)) as _ }
                                    ) as _ }
                                 ;
                                 ;
                                 if (r#return_check_merge0 . clone ()) . 0 {
                                     ({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,) }
                                 else {
                                     ((sp :: r#EventResult :: r#Reject , false , (r#return_check_merge0 . clone ()) . 1 ,)) as _ }
                                 }
                             ;
                             ;
                             if (r#returned_expression0 . clone ()) . 1 {
                                 (r#returned_expression0 . clone ()) . 0 }
                             else {
                                 ((r#returned_expression0 . clone ()) . 2) as _ }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_opacity_119 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         1f64 }
                     else {
                         (0f64) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_120 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                            ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min ((((({
                                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                            ) . apply_pin (_self) . get () . get () as f64) + (10f64 as f64)) as sp :: Coord)) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_120 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_opacity_121 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_120_state }
                    ) . apply_pin (_self) . get () as f64) == (2f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (3388997631f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (2566914048f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = ({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_120_icon }
                        ) . apply_pin (_self) . get () . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) }
                     else {
                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_120_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         6f64 }
                     else {
                         (8f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_opacity_123 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         1f64 }
                     else {
                         (0f64) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_124 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                            ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                            ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((((({
                                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
                            ) . apply_pin (_self) . get () . get () as f64) - (10f64 as f64)) as sp :: Coord)) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_124 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_opacity_125 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_124_state }
                    ) . apply_pin (_self) . get () as f64) == (2f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (3388997631f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (2566914048f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = ({
                             * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_124_icon }
                        ) . apply_pin (_self) . get () . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
                    ) . apply_pin (_self) . get () {
                         sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg")) }
                     else {
                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((({
                         * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_124_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         6f64 }
                     else {
                         (8f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_opacity_123_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_121_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_125_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_opacity_119_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_thumb_117 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_thumb_117 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_120 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_120 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_124 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_124 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_thumb_117_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_touch_area_118_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_touch_area_118_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (8f64 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_120_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_120_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => (8f64 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_124_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_124_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => (8f64 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_opacity_119_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_opacity_119_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 6u32 => (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((8f64 as f64) - (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((8f64 as f64) - (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 7u32 => (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_121_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_121_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 8u32 => (8f64 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_opacity_123_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_opacity_123_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 9u32 => (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((8f64 as f64) - (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((8f64 as f64) - (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 10u32 => (({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_125_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_125_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerAppWindow {
         r#root_1 : sp :: r#WindowItem , r#rectangle_3 : sp :: r#Rectangle , r#rectangle_4 : sp :: r#Rectangle , r#text_6 : sp :: r#Text , r#rectangle_7 : sp :: r#Rectangle , r#empty_8 : sp :: r#Empty , r#rectangle_9 : sp :: r#BasicBorderRectangle , r#back_touch_10 : sp :: r#TouchArea , r#text_11 : sp :: r#Text , r#rectangle_12 : sp :: r#BasicBorderRectangle , r#fwd_touch_13 : sp :: r#TouchArea , r#text_14 : sp :: r#Text , r#rectangle_15 : sp :: r#BasicBorderRectangle , r#reload_touch_16 : sp :: r#TouchArea , r#text_17 : sp :: r#Text , r#rectangle_19 : sp :: r#BasicBorderRectangle , r#go_touch_20 : sp :: r#TouchArea , r#text_21 : sp :: r#Text , r#rectangle_22 : sp :: r#BasicBorderRectangle , r#full_touch_23 : sp :: r#TouchArea , r#text_24 : sp :: r#Text , r#rectangle_25 : sp :: r#BasicBorderRectangle , r#add_tab_touch_26 : sp :: r#TouchArea , r#text_27 : sp :: r#Text , r#rectangle_28 : sp :: r#Rectangle , r#rectangle_40 : sp :: r#Rectangle , r#url_input_18 : InnerLineEdit_root_111 , r#root_1_active_tab_index : sp :: Property < i32 > , r#root_1_add_tab_touch_26_x : sp :: Property < sp :: LogicalLength > , r#root_1_add_tab_touch_26_y : sp :: Property < sp :: LogicalLength > , r#root_1_back_touch_10_x : sp :: Property < sp :: LogicalLength > , r#root_1_back_touch_10_y : sp :: Property < sp :: LogicalLength > , r#root_1_current_url : sp :: Property < sp :: SharedString > , r#root_1_empty_2_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_2_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_2_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_29_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_29_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_29_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_5_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_5_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_5_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_8_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_8_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_8_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_full_touch_23_x : sp :: Property < sp :: LogicalLength > , r#root_1_full_touch_23_y : sp :: Property < sp :: LogicalLength > , r#root_1_fwd_touch_13_x : sp :: Property < sp :: LogicalLength > , r#root_1_fwd_touch_13_y : sp :: Property < sp :: LogicalLength > , r#root_1_go_touch_20_x : sp :: Property < sp :: LogicalLength > , r#root_1_go_touch_20_y : sp :: Property < sp :: LogicalLength > , r#root_1_is_loading : sp :: Property < bool > , r#root_1_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_page_content : sp :: Property < sp :: SharedString > , r#root_1_page_title : sp :: Property < sp :: SharedString > , r#root_1_rectangle_12_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_12_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_12_y : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_15_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_15_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_15_y : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_19_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_19_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_22_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_22_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_25_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_25_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_28_width : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_28_x : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_3_width : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_3_x : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_4_x : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_40_height : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_40_width : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_40_x : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_9_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_9_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_9_y : sp :: Property < sp :: LogicalLength > , r#root_1_reload_touch_16_x : sp :: Property < sp :: LogicalLength > , r#root_1_reload_touch_16_y : sp :: Property < sp :: LogicalLength > , r#root_1_tabs : sp :: Property < sp :: ModelRc < r#TabInfo > > , r#root_1_text_11_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_11_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_11_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_11_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_14_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_14_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_14_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_14_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_17_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_17_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_17_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_17_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_21_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_21_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_21_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_21_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_24_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_24_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_24_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_24_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_27_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_27_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_27_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_27_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , r#root_1_add_tab : sp :: Callback < () , () > , r#root_1_back : sp :: Callback < () , () > , r#root_1_close_tab : sp :: Callback < (i32 ,) , () > , r#root_1_forward : sp :: Callback < () , () > , r#root_1_navigate : sp :: Callback < (sp :: SharedString ,) , () > , r#root_1_open_external : sp :: Callback < (sp :: SharedString ,) , () > , r#root_1_quick_link_clicked : sp :: Callback < (sp :: SharedString ,) , () > , r#root_1_reload : sp :: Callback < () , () > , r#root_1_select_tab : sp :: Callback < (i32 ,) , () > , repeater0 : sp :: Repeater < InnerComponent_rectangle_30 > , repeater1 : sp :: Repeater < InnerComponent_empty_41 > , repeater2 : sp :: Repeater < InnerComponent_rectangle_80 > , repeater3 : sp :: Repeater < InnerComponent_rectangle_101 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerAppWindow >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_AppWindow , window_adapter_ : sp :: OnceCell < sp :: WindowAdapterRc > , }
     impl InnerAppWindow {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_tabs }
                    ) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new ({
                         let r#tmp_root_1_current_url = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_current_url }
                        ) . apply_pin (_self) . get () ;
                         ;
                         ((((((((r#tmp_root_1_current_url . clone ()) == (sp :: SharedString :: from ("https://sumer.os")))) || (((r#tmp_root_1_current_url . clone ()) == (sp :: SharedString :: from ("")))))) || (((r#tmp_root_1_current_url . clone ()) == (sp :: SharedString :: from ("https://www.google.com")))))) || (((r#tmp_root_1_current_url . clone ()) == (sp :: SharedString :: from ("google.com"))))) }
                     as bool)) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new ({
                         let r#tmp_root_1_current_url = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_current_url }
                        ) . apply_pin (_self) . get () ;
                         ;
                         ((((((((r#tmp_root_1_current_url . clone ()) != (sp :: SharedString :: from ("https://sumer.os")))) && (((r#tmp_root_1_current_url . clone ()) != (sp :: SharedString :: from ("")))))) && (((r#tmp_root_1_current_url . clone ()) != (sp :: SharedString :: from ("https://www.google.com")))))) && (((r#tmp_root_1_current_url . clone ()) != (sp :: SharedString :: from ("google.com"))))) }
                     as bool)) as _ }
                 }
            ) ;
             _self . repeater3 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_is_loading }
                    ) . apply_pin (_self) . get () as bool)) as _ }
                 }
            ) ;
             InnerLineEdit_root_111 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#url_input_18 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 8u32 - 1 , tree_index_of_first_child + 21u32 - 1) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_active_tab_index }
            ) . apply_pin (_self) . set ({
                 (0f64) as i32 }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4279179050f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_current_url }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("https://sumer.os")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 60f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 60f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_29_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 38f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 38f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_40 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = (({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_h }
                        ) . apply_pin (_self) . get ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = (({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_29_layoutinfo_h }
                        ) . apply_pin (_self) . get ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_40 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 60f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 60f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_29_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 38f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 38f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_40 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_29_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Start as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 10f64 as _ ;
                                 the_struct . r#end = 10f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_28_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : 4f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_29_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 4f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 10f64 as _ ;
                             the_struct . r#end = 10f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Start as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_29_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_7 }
                                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 1f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 1f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_8_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                                 + {
                                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_h }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                                     + {
                                         * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_19_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 75f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 75f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_22_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 95f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 95f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_25_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 35f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 35f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 10f64 as _ ;
                             the_struct . r#end = 10f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_3_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 12f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_7 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 1f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 1f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_8_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                             + {
                                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_h }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                                 + {
                                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_19_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 75f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 75f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_22_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 95f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 95f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_25_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 35f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 35f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 12f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 10f64 as _ ;
                         the_struct . r#end = 10f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_7 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 25f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 25f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_8_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                             + {
                                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_v }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                                 + {
                                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_19_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 35f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 35f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_22_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 35f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 35f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_25_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 35f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 35f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 10f64 as _ ;
                         the_struct . r#end = 10f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_8_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_9_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 32f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 32f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_12_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 32f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 32f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_15_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 32f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 32f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                        ) . apply_pin (_self) . get () [5usize] as _ , r#spacing : 6f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_8_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_9_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 32f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 32f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_12_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 32f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 32f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_15_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 32f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 32f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 6f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_8_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_9_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 32f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 32f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_12_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 32f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 32f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_15_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 32f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 32f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_is_loading }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_page_content }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_page_title }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("بوابة سومر الوطنية")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_12_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_12_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_15_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_15_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_19_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_19_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_22_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_22_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_25_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_25_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_28_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_3_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_4_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_9_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_9_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_tabs }
            ) . apply_pin (_self) . set ({
                 (sp :: ModelRc :: new (sp :: VecModel :: < r#TabInfo > :: from (sp :: vec ! [{
                     let mut the_struct = r#TabInfo :: default () ;
                     the_struct . r#active = true as _ ;
                     the_struct . r#title = sp :: SharedString :: from ("بوابة سومر") as _ ;
                     the_struct . r#url = sp :: SharedString :: from ("https://sumer.os") as _ ;
                     the_struct }
                 as _]))) as sp :: ModelRc < r#TabInfo > }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_11_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_11_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_11_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_11_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_14_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_14_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_14_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_14_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_17_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_17_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_17_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_17_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_21_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_21_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_21_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_21_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_24_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_24_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_24_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_24_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_27_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_27_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_27_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_27_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("متصفح أوروك — Orok Browser")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_3 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4280166715f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_4 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281549141f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294286859f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (18f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (800f64) as i32 }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_empty_5_padding = 10f64 ;
                     ;
                     ((((60f64 as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("⛵ أوروك")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281549141f64 as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_9 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#back_touch_10 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4281549141f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4280166715f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4282865001f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#back_touch_10 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_back }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#back_touch_10 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294047225f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_11_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_11_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("⬅")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_11_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_11_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_12 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#fwd_touch_13 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4281549141f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4280166715f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4282865001f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#fwd_touch_13 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_forward }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#fwd_touch_13 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294047225f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_14_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_14_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("➡")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_14_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_14_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_15 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#reload_touch_16 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4281549141f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4280166715f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_15 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4282865001f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_15 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_15 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#reload_touch_16 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_reload }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#reload_touch_16 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294047225f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_17_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_17_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("↻")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_17_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_17_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                 + {
                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                 + {
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_accepted }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_navigate }
                            ) . apply_pin (_self) . call (& (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                             + {
                                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                             + {
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_empty_5_padding = 10f64 ;
                     ;
                     ((((60f64 as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_horizontal_stretch }
            ) . apply_pin (_self) . set ({
                 (1f64) as f32 }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_placeholder_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("أدخل رابط موقع أو ابحث في الويب...")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                 + {
                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                 + {
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_current_url }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                 + {
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                    ) . apply_pin (_self) . get () [7usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                 + {
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                    ) . apply_pin (_self) . get () [6usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_19 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#go_touch_20 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4292441862f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294286859f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_19 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#go_touch_20 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_navigate }
                            ) . apply_pin (_self) . call (& (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                             + {
                                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                             + {
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#go_touch_20 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4279179050f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (700f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_21_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_21_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("انتقال")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_21_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_21_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_22 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#full_touch_23 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4278355143f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278413729f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#full_touch_23 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_open_external }
                            ) . apply_pin (_self) . call (& (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
                             + {
                                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                             + {
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#full_touch_23 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (700f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_24_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_24_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("🌐 متصفح كامل")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_24_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_24_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_25 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#add_tab_touch_26 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4281549141f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4280166715f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_25 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4282865001f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_25 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_25 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#add_tab_touch_26 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_add_tab }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#add_tab_touch_26 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294286859f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (18f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (700f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_27_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_27_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("+")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_27_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_27_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_28 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4279179050f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_40 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4279310375f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_add_tab_touch_26_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_add_tab_touch_26_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_back_touch_10_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_back_touch_10_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_full_touch_23_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_full_touch_23_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_fwd_touch_13_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_fwd_touch_13_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_go_touch_20_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_go_touch_20_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_12_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_15_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_28_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_3_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_9_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_reload_touch_16_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_reload_touch_16_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_3 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_4 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#back_touch_10 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#back_touch_10 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#fwd_touch_13 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#fwd_touch_13 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_15 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_15 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_15 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#reload_touch_16 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#reload_touch_16 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_horizontal_stretch }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_background_112_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_background_112_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_19 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_19 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_19 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#go_touch_20 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#go_touch_20 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#full_touch_23 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#full_touch_23 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_25 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_25 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_25 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#add_tab_touch_26 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#add_tab_touch_26 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_28 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_40 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerLineEdit_root_111 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#url_input_18 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_41 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_80 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 3u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_101 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater3 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 900f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 650f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_41 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_80 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 3u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_101 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater3 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_41 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_80 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 3u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_101 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater3 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (60f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_3_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (38f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_28_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 4u32 => (1f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_4_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 59f64 as sp :: Coord ,) , 5u32 => ({
                     let r#tmp_empty_5_padding = 10f64 ;
                     ;
                     ((((60f64 as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 10f64 as sp :: Coord ,) , 6u32 => (25f64 as sp :: Coord , 1f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 10f64 as sp :: Coord ,) , 7u32 => ({
                     let r#tmp_empty_5_padding = 10f64 ;
                     ;
                     ((((60f64 as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , 10f64 as sp :: Coord ,) , 8u32 => ({
                     let r#tmp_empty_5_padding = 10f64 ;
                     ;
                     ((((60f64 as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [7usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [6usize] as sp :: Coord , 10f64 as sp :: Coord ,) , 9u32 => (35f64 as sp :: Coord , 75f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [8usize] as sp :: Coord , 10f64 as sp :: Coord ,) , 10u32 => (35f64 as sp :: Coord , 95f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [10usize] as sp :: Coord , 10f64 as sp :: Coord ,) , 11u32 => (35f64 as sp :: Coord , 35f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [12usize] as sp :: Coord , 10f64 as sp :: Coord ,) , 12u32 => (32f64 as sp :: Coord , 32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_8_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_9_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 13u32 => (32f64 as sp :: Coord , 32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_8_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_12_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 14u32 => (32f64 as sp :: Coord , 32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_8_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_15_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 15u32 => (32f64 as sp :: Coord , 32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_back_touch_10_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_back_touch_10_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 16u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((32f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((32f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 17u32 => (32f64 as sp :: Coord , 32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_fwd_touch_13_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_fwd_touch_13_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 18u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((32f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((32f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 19u32 => (32f64 as sp :: Coord , 32f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_reload_touch_16_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_reload_touch_16_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 20u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((32f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((32f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 27u32 => (35f64 as sp :: Coord , 75f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_go_touch_20_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_go_touch_20_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 28u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((75f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((35f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 29u32 => (35f64 as sp :: Coord , 95f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_full_touch_23_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_full_touch_23_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 30u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((95f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((35f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 31u32 => (35f64 as sp :: Coord , 35f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_add_tab_touch_26_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_add_tab_touch_26_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 32u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((35f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((35f64 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 21u32 ..= 26u32 => return {
                     * & Self :: FIELD_OFFSETS . r#url_input_18 }
                 . apply_pin (_self) . item_geometry (index - 21u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 5u32 => sp :: r#AccessibleRole :: r#Text , 16u32 => sp :: r#AccessibleRole :: r#Text , 18u32 => sp :: r#AccessibleRole :: r#Text , 20u32 => sp :: r#AccessibleRole :: r#Text , 28u32 => sp :: r#AccessibleRole :: r#Text , 30u32 => sp :: r#AccessibleRole :: r#Text , 32u32 => sp :: r#AccessibleRole :: r#Text , 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#url_input_18 }
                 . apply_pin (_self) . accessible_role (0) , 21u32 ..= 26u32 => {
                     * & Self :: FIELD_OFFSETS . r#url_input_18 }
                 . apply_pin (_self) . accessible_role (index - 21u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (5u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("⛵ أوروك") , (16u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("⬅") , (18u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("➡") , (20u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("↻") , (28u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("انتقال") , (30u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("🌐 متصفح كامل") , (32u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("+") , (8u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#url_input_18 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (21u32 ..= 26u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#url_input_18 }
                 . apply_pin (_self) . accessible_string_property (index - 21u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_30 {
         r#rectangle_30 : sp :: r#BorderRectangle , r#rectangle_31 : sp :: r#Rectangle , r#tab_touch_32 : sp :: r#TouchArea , r#text_34 : sp :: r#Text , r#model_data : sp :: Property < r#TabInfo > , r#model_index : sp :: Property < i32 > , r#rectangle_30_empty_33_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#rectangle_30_empty_33_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#rectangle_30_empty_33_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#rectangle_30_tab_touch_32_x : sp :: Property < sp :: LogicalLength > , r#rectangle_30_tab_touch_32_y : sp :: Property < sp :: LogicalLength > , r#rectangle_30_text_34_y : sp :: Property < sp :: LogicalLength > , r#rectangle_30_x : sp :: Property < sp :: LogicalLength > , r#rectangle_30_y : sp :: Property < sp :: LogicalLength > , repeater0 : sp :: Repeater < InnerComponent_rectangle_35 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_30 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_30 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (((match & (InnerAppWindow :: FIELD_OFFSETS . r#root_1_tabs) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () {
                         x => {
                             x . model_tracker () . track_row_count_changes () ;
                             x . row_count () as i32 }
                         }
                     as f64) > (1f64 as f64)) as bool)) as _ }
                 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if (({
                         * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) . r#active {
                         sp :: Color :: from_argb_encoded (4280166715f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4279310375f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_empty_33_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
                                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         InnerComponent_rectangle_30 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_35 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 10f64 as _ ;
                                 the_struct . r#end = 8f64 as _ ;
                                 the_struct }
                             as _ , r#size : 170f64 as _ , r#spacing : 8f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_empty_33_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
                                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         InnerComponent_rectangle_30 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_35 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 8f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 10f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_empty_33_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ) ;
                         InnerComponent_rectangle_30 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_35 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let cache = (InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_29_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [0usize] as usize) + ({
                             * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_31 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if (({
                         * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) . r#active {
                         sp :: Color :: from_argb_encoded (4294286859f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (0f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#tab_touch_32 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_1_select_tab) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . call (& (({
                                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#model_index }
                            ) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#tab_touch_32 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if (({
                         * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) . r#active {
                         sp :: Color :: from_argb_encoded (4294047225f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4287931320f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (34f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set ({
                 (sp :: r#TextOverflow :: r#Elide) as sp :: r#TextOverflow }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) . r#title) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_empty_33_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_tab_touch_32_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_tab_touch_32_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_text_34_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#tab_touch_32 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#tab_touch_32 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerComponent_rectangle_30 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_35 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_empty_33_layoutinfo_h }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 170f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 170f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_empty_33_layoutinfo_v }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 34f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 34f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerComponent_rectangle_30 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_35 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerComponent_rectangle_30 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_35 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (34f64 as sp :: Coord , 170f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (2f64 as sp :: Coord , 170f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (34f64 as sp :: Coord , 170f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_tab_touch_32_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_tab_touch_32_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (34f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_empty_33_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_empty_33_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_text_34_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 3u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (3u32 , sp :: AccessibleStringProperty :: r#Label) => (({
                     * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#model_data }
                ) . apply_pin (_self) . get ()) . r#title , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_35 {
         r#rectangle_35 : sp :: r#BasicBorderRectangle , r#close_touch_36 : sp :: r#TouchArea , r#text_37 : sp :: r#Text , r#rectangle_35_close_touch_36_x : sp :: Property < sp :: LogicalLength > , r#rectangle_35_close_touch_36_y : sp :: Property < sp :: LogicalLength > , r#rectangle_35_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#rectangle_35_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#rectangle_35_text_37_min_height : sp :: Property < sp :: LogicalLength > , r#rectangle_35_text_37_min_width : sp :: Property < sp :: LogicalLength > , r#rectangle_35_text_37_preferred_height : sp :: Property < sp :: LogicalLength > , r#rectangle_35_text_37_preferred_width : sp :: Property < sp :: LogicalLength > , r#rectangle_35_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_35 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_30 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_35 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#close_touch_36 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4293870660f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (0f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (9f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_text_37_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_text_37_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_text_37_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_text_37_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#close_touch_36 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_1_close_tab) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref () . parent . upgrade () . unwrap () . as_pin_ref ()) . call (& ((InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#model_index) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#close_touch_36 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#close_touch_36 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4284773515f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (11f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_text_37_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_text_37_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("×")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_text_37_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_text_37_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_close_touch_36_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_close_touch_36_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#close_touch_36 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#close_touch_36 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 18f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 18f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 18f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 18f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (18f64 as sp :: Coord , 18f64 as sp :: Coord , {
                     let cache = (InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30_empty_33_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , ({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (18f64 as sp :: Coord , 18f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_close_touch_36_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35_close_touch_36_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((18f64 as f64) - (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((18f64 as f64) - (({
                     * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (2u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("×") , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_rectangle_35 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_30 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_rectangle_30 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             3usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 2u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_35 , sp :: ItemVTable , sp :: AllowPin > ;
             3usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#rectangle_35 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#close_touch_36 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_35 :: FIELD_OFFSETS . r#text_37 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_35) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_35 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_rectangle_35 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_35 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_35 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_35 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_35 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerComponent_rectangle_30 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             5usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_30 , sp :: ItemVTable , sp :: AllowPin > ;
             4usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_30 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#rectangle_31 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#tab_touch_32 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#text_34 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_30) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_30 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_rectangle_30 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_30 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_30 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 33u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_30 {
         type Data = r#TabInfo ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_rectangle_30 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_30 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_empty_41 {
         r#empty_41 : sp :: r#Empty , r#i_flickable_42 : sp :: r#Flickable , r#i_flickable_viewport_43 : sp :: r#Empty , r#empty_45 : sp :: r#Empty , r#text_46 : sp :: r#Text , r#text_47 : sp :: r#Text , r#rectangle_48 : sp :: r#BasicBorderRectangle , r#text_50 : sp :: r#Text , r#empty_52 : sp :: r#Empty , r#text_53 : sp :: r#Text , r#empty_54 : sp :: r#Empty , r#rectangle_55 : sp :: r#BasicBorderRectangle , r#card1_touch_56 : sp :: r#TouchArea , r#text_58 : sp :: r#Text , r#text_59 : sp :: r#Text , r#rectangle_60 : sp :: r#BasicBorderRectangle , r#card2_touch_61 : sp :: r#TouchArea , r#text_63 : sp :: r#Text , r#text_64 : sp :: r#Text , r#rectangle_65 : sp :: r#BasicBorderRectangle , r#card3_touch_66 : sp :: r#TouchArea , r#text_68 : sp :: r#Text , r#text_69 : sp :: r#Text , r#rectangle_70 : sp :: r#BasicBorderRectangle , r#card4_touch_71 : sp :: r#TouchArea , r#text_73 : sp :: r#Text , r#text_74 : sp :: r#Text , r#i_vertical_bar_visibility_75 : sp :: r#Clip , r#i_horizontal_bar_visibility_77 : sp :: r#Clip , r#search_input_51 : InnerLineEdit_root_111 , r#i_vertical_bar_76 : InnerScrollBar_root_116 , r#i_horizontal_bar_78 : InnerScrollBar_root_116 , r#empty_41_card1_touch_56_x : sp :: Property < sp :: LogicalLength > , r#empty_41_card1_touch_56_y : sp :: Property < sp :: LogicalLength > , r#empty_41_card2_touch_61_x : sp :: Property < sp :: LogicalLength > , r#empty_41_card2_touch_61_y : sp :: Property < sp :: LogicalLength > , r#empty_41_card3_touch_66_x : sp :: Property < sp :: LogicalLength > , r#empty_41_card3_touch_66_y : sp :: Property < sp :: LogicalLength > , r#empty_41_card4_touch_71_x : sp :: Property < sp :: LogicalLength > , r#empty_41_card4_touch_71_y : sp :: Property < sp :: LogicalLength > , r#empty_41_empty_44_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#empty_41_empty_44_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_44_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_45_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#empty_41_empty_45_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_45_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_49_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#empty_41_empty_49_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_49_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_52_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#empty_41_empty_52_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_52_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_52_width : sp :: Property < sp :: LogicalLength > , r#empty_41_empty_54_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#empty_41_empty_54_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_54_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_54_x : sp :: Property < sp :: LogicalLength > , r#empty_41_empty_57_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#empty_41_empty_57_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_57_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_62_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#empty_41_empty_62_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_62_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_67_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#empty_41_empty_67_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_67_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_72_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#empty_41_empty_72_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#empty_41_empty_72_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#empty_41_i_flickable_42_height : sp :: Property < sp :: LogicalLength > , r#empty_41_i_flickable_42_horizontal_stretch : sp :: Property < f32 > , r#empty_41_i_flickable_42_max_height : sp :: Property < sp :: LogicalLength > , r#empty_41_i_flickable_42_max_width : sp :: Property < sp :: LogicalLength > , r#empty_41_i_flickable_42_min_height : sp :: Property < sp :: LogicalLength > , r#empty_41_i_flickable_42_min_width : sp :: Property < sp :: LogicalLength > , r#empty_41_i_flickable_42_preferred_height : sp :: Property < sp :: LogicalLength > , r#empty_41_i_flickable_42_preferred_width : sp :: Property < sp :: LogicalLength > , r#empty_41_i_flickable_42_vertical_stretch : sp :: Property < f32 > , r#empty_41_i_flickable_42_width : sp :: Property < sp :: LogicalLength > , r#empty_41_i_flickable_42_x : sp :: Property < sp :: LogicalLength > , r#empty_41_i_flickable_42_y : sp :: Property < sp :: LogicalLength > , r#empty_41_i_horizontal_bar_78_visible : sp :: Property < bool > , r#empty_41_i_horizontal_bar_visibility_77_height : sp :: Property < sp :: LogicalLength > , r#empty_41_i_horizontal_bar_visibility_77_width : sp :: Property < sp :: LogicalLength > , r#empty_41_i_horizontal_bar_visibility_77_x : sp :: Property < sp :: LogicalLength > , r#empty_41_i_horizontal_bar_visibility_77_y : sp :: Property < sp :: LogicalLength > , r#empty_41_i_vertical_bar_76_visible : sp :: Property < bool > , r#empty_41_i_vertical_bar_visibility_75_height : sp :: Property < sp :: LogicalLength > , r#empty_41_i_vertical_bar_visibility_75_width : sp :: Property < sp :: LogicalLength > , r#empty_41_i_vertical_bar_visibility_75_x : sp :: Property < sp :: LogicalLength > , r#empty_41_i_vertical_bar_visibility_75_y : sp :: Property < sp :: LogicalLength > , r#empty_41_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#empty_41_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#empty_41_rectangle_55_y : sp :: Property < sp :: LogicalLength > , r#empty_41_rectangle_60_y : sp :: Property < sp :: LogicalLength > , r#empty_41_rectangle_65_y : sp :: Property < sp :: LogicalLength > , r#empty_41_rectangle_70_y : sp :: Property < sp :: LogicalLength > , r#empty_41_text_46_x : sp :: Property < sp :: LogicalLength > , r#empty_41_text_47_x : sp :: Property < sp :: LogicalLength > , r#empty_41_text_50_y : sp :: Property < sp :: LogicalLength > , r#empty_41_text_53_x : sp :: Property < sp :: LogicalLength > , r#empty_41_x : sp :: Property < sp :: LogicalLength > , r#empty_41_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_empty_41 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_empty_41 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerLineEdit_root_111 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#search_input_51 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 11u32 - 1 , tree_index_of_first_child + 12u32 - 1) ;
             InnerScrollBar_root_116 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 36u32 - 1 , tree_index_of_first_child + 37u32 - 1) ;
             InnerScrollBar_root_116 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 47u32 - 1 , tree_index_of_first_child + 48u32 - 1) ;
             sp :: Property :: link_two_way (({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
            ) . apply_pin (_self) , ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self)) ;
             sp :: Property :: link_two_way (({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
            ) . apply_pin (_self) , ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self)) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 48f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 48f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 40f64 as _ ;
                             the_struct . r#end = 40f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as _ , r#spacing : 30f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 500f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 500f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 40f64 as _ ;
                         the_struct . r#end = 40f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 48f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 48f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 30f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 40f64 as _ ;
                         the_struct . r#end = 40f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layout_cache }
                        ) . apply_pin (_self) . get () [1usize] as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
                                 + {
                                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_h }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                                         InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
                                     + {
                                         * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 20f64 as _ ;
                             the_struct . r#end = 20f64 as _ ;
                             the_struct }
                         as _ , r#size : 500f64 as _ , r#spacing : 12f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
                             + {
                                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_h }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
                                 + {
                                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 12f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 20f64 as _ ;
                         the_struct . r#end = 20f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
                             + {
                                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_v }
                            ) . apply_pin (_self) . get ())))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
                                 + {
                                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_layout_113_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layout_cache }
                        ) . apply_pin (_self) . get () [5usize] as _ , r#spacing : 12f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 12f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_44_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 140f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 140f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 140f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 140f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 140f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 140f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 140f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 140f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 15f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 140f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 140f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 140f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 140f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 140f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 140f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 140f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 140f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 15f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 95f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 95f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 95f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 95f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 95f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 95f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 95f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 95f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : 95f64 as _ , r#spacing : 6f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 6f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : 95f64 as _ , r#spacing : 6f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 6f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : 95f64 as _ , r#spacing : 6f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 6f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : 95f64 as _ , r#spacing : 6f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 6f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [5usize] as f64) - (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_horizontal_bar_78_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_width }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_vertical_bar_76_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_height }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layoutinfo_v }
                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layoutinfo_h }
                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294286859f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (34f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (800f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("⛵ بوابة أوروك للويب")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_44_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4287931320f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("مرحبا\u{64b} بك في متصفح الويب الرسمي لنظام سومر الموثوق")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_44_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_48 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4280166715f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_48 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4282865001f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_48 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_48 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (18f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (48f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("🔍")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
                 + {
                     InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                 + {
                     * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_accepted }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_1_navigate) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . call (& (({
                                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
                             + {
                                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
                             + {
                                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (48f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_horizontal_stretch }
            ) . apply_pin (_self) . set ({
                 (1f64) as f32 }
            ) ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107_placeholder_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("ابحث في محرك البحث سومر...")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
                 + {
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
                 + {
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4284773515f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (700f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("الروابط السريعة الموصى بها:")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_44_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_55 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card1_touch_56 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4280640491f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4280166715f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_55 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281549141f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_55 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_55 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card1_touch_56 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_1_quick_link_clicked) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . call (& (sp :: SharedString :: from ("https://sumer.os") as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card1_touch_56 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("🦁")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_empty_57_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_57_padding . clone () as f64)) as f64) - (r#tmp_empty_57_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (700f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("بوابة سومر")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_empty_57_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_57_padding . clone () as f64)) as f64) - (r#tmp_empty_57_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_60 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card2_touch_61 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4278556265f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4280166715f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281549141f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card2_touch_61 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_1_quick_link_clicked) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . call (& (sp :: SharedString :: from ("https://google.com") as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card2_touch_61 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("🔍")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_empty_62_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_62_padding . clone () as f64)) as f64) - (r#tmp_empty_62_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (700f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("بحث جوجل")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_empty_62_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_62_padding . clone () as f64)) as f64) - (r#tmp_empty_62_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_65 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card3_touch_66 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4286331629f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4280166715f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_65 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281549141f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_65 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_65 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card3_touch_66 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_1_quick_link_clicked) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . call (& (sp :: SharedString :: from ("https://wikipedia.org") as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card3_touch_66 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("📖")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_empty_67_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_67_padding . clone () as f64)) as f64) - (r#tmp_empty_67_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (700f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("ويكيبيديا")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_empty_67_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_67_padding . clone () as f64)) as f64) - (r#tmp_empty_67_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_70 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card4_touch_71 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4282865001f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4280166715f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_70 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281549141f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_70 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_70 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card4_touch_71 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_1_quick_link_clicked) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . call (& (sp :: SharedString :: from ("https://github.com") as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card4_touch_71 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("🐙")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_empty_72_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_72_padding . clone () as f64)) as f64) - (r#tmp_empty_72_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (700f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("جيت هاب")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_empty_72_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_72_padding . clone () as f64)) as f64) - (r#tmp_empty_72_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_75 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_vertical_bar_76_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_page_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_width }
                    ) . apply_pin (_self) . get () . get () as f64) + (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_x }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) - (14f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_y }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_77 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_horizontal_bar_78_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
            ) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_page_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_x }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_height }
                    ) . apply_pin (_self) . get () . get () as f64) + (({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_y }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) - (14f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card1_touch_56_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card1_touch_56_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card2_touch_61_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card2_touch_61_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card3_touch_66_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card3_touch_66_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card4_touch_71_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card4_touch_71_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_horizontal_bar_visibility_77_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_horizontal_bar_visibility_77_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_horizontal_bar_visibility_77_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_horizontal_bar_visibility_77_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_vertical_bar_visibility_75_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_vertical_bar_visibility_75_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_vertical_bar_visibility_75_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_vertical_bar_visibility_75_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_rectangle_55_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_rectangle_60_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_rectangle_65_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_rectangle_70_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_text_46_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_text_47_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_text_50_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_text_53_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_48 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_48 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_48 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_48 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_horizontal_stretch }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_background_112_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_i_background_112_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_55 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_55 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_55 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card1_touch_56 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card1_touch_56 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card2_touch_61 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card2_touch_61 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_65 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_65 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_65 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card3_touch_66 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card3_touch_66 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_70 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_70 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_70 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card4_touch_71 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card4_touch_71 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_75 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_75 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_75 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_75 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_75 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_75 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_opacity_123_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_121_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_125_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_opacity_119_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_77 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_77 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_77 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_77 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_77 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_77 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_opacity_123_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_121_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_125_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_opacity_119_dummy }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerLineEdit_root_111 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#search_input_51 }
             . apply_pin (x)) ,) ;
             InnerScrollBar_root_116 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             . apply_pin (x)) ,) ;
             InnerScrollBar_root_116 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_vertical_bar_visibility_75_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_vertical_bar_visibility_75_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_vertical_bar_visibility_75_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_vertical_bar_visibility_75_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_horizontal_bar_visibility_77_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_horizontal_bar_visibility_77_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_horizontal_bar_visibility_77_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_horizontal_bar_visibility_77_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_44_padding = 40f64 ;
                     ;
                     ((((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                 as sp :: Coord , 40f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 6u32 => (48f64 as sp :: Coord , 500f64 as sp :: Coord , 40f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 7u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , {
                     let r#tmp_empty_44_padding = 40f64 ;
                     ;
                     ((((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                 as sp :: Coord , 40f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_44_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 8u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_44_padding = 40f64 ;
                     ;
                     ((((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_text_46_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 9u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_44_padding = 40f64 ;
                     ;
                     ((((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_text_47_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_45_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 10u32 => (48f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_text_50_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 11u32 => (48f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_49_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , ({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
                 + {
                     * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 18u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_44_padding = 40f64 ;
                     ;
                     ((((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_text_53_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 19u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_44_padding = 40f64 ;
                     ;
                     ((((({
                         * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_52_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 20u32 => (95f64 as sp :: Coord , 140f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_rectangle_55_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 21u32 => (95f64 as sp :: Coord , 140f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_rectangle_60_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 22u32 => (95f64 as sp :: Coord , 140f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_rectangle_65_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 23u32 => (95f64 as sp :: Coord , 140f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_54_layout_cache }
                ) . apply_pin (_self) . get () [6usize] as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_rectangle_70_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 24u32 => (95f64 as sp :: Coord , 140f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card1_touch_56_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card1_touch_56_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 25u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_57_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_57_padding . clone () as f64)) as f64) - (r#tmp_empty_57_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 26u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_57_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_57_padding . clone () as f64)) as f64) - (r#tmp_empty_57_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_57_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 27u32 => (95f64 as sp :: Coord , 140f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card2_touch_61_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card2_touch_61_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 28u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_62_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_62_padding . clone () as f64)) as f64) - (r#tmp_empty_62_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 29u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_62_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_62_padding . clone () as f64)) as f64) - (r#tmp_empty_62_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_62_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 30u32 => (95f64 as sp :: Coord , 140f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card3_touch_66_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card3_touch_66_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 31u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_67_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_67_padding . clone () as f64)) as f64) - (r#tmp_empty_67_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 32u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_67_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_67_padding . clone () as f64)) as f64) - (r#tmp_empty_67_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_67_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 33u32 => (95f64 as sp :: Coord , 140f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card4_touch_71_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_card4_touch_71_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 34u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_72_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_72_padding . clone () as f64)) as f64) - (r#tmp_empty_72_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 35u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_72_padding = 12f64 ;
                     ;
                     ((((140f64 as f64) - (r#tmp_empty_72_padding . clone () as f64)) as f64) - (r#tmp_empty_72_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_empty_72_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 36u32 => (({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 14f64 as sp :: Coord , ({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 47u32 => (14f64 as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41_i_flickable_42_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 12u32 ..= 17u32 => return {
                     * & Self :: FIELD_OFFSETS . r#search_input_51 }
                 . apply_pin (_self) . item_geometry (index - 12u32 + 1) , 37u32 ..= 46u32 => return {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_76 }
                 . apply_pin (_self) . item_geometry (index - 37u32 + 1) , 48u32 ..= 57u32 => return {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
                 . apply_pin (_self) . item_geometry (index - 48u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 8u32 => sp :: r#AccessibleRole :: r#Text , 9u32 => sp :: r#AccessibleRole :: r#Text , 10u32 => sp :: r#AccessibleRole :: r#Text , 18u32 => sp :: r#AccessibleRole :: r#Text , 25u32 => sp :: r#AccessibleRole :: r#Text , 26u32 => sp :: r#AccessibleRole :: r#Text , 28u32 => sp :: r#AccessibleRole :: r#Text , 29u32 => sp :: r#AccessibleRole :: r#Text , 31u32 => sp :: r#AccessibleRole :: r#Text , 32u32 => sp :: r#AccessibleRole :: r#Text , 34u32 => sp :: r#AccessibleRole :: r#Text , 35u32 => sp :: r#AccessibleRole :: r#Text , 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#search_input_51 }
                 . apply_pin (_self) . accessible_role (0) , 12u32 ..= 17u32 => {
                     * & Self :: FIELD_OFFSETS . r#search_input_51 }
                 . apply_pin (_self) . accessible_role (index - 12u32 + 1) , 36u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_76 }
                 . apply_pin (_self) . accessible_role (0) , 37u32 ..= 46u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_76 }
                 . apply_pin (_self) . accessible_role (index - 37u32 + 1) , 47u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
                 . apply_pin (_self) . accessible_role (0) , 48u32 ..= 57u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
                 . apply_pin (_self) . accessible_role (index - 48u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (8u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("⛵ بوابة أوروك للويب") , (9u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("مرحبا\u{64b} بك في متصفح الويب الرسمي لنظام سومر الموثوق") , (10u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("🔍") , (18u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("الروابط السريعة الموصى بها:") , (25u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("🦁") , (26u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("بوابة سومر") , (28u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("🔍") , (29u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("بحث جوجل") , (31u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("📖") , (32u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("ويكيبيديا") , (34u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("🐙") , (35u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("جيت هاب") , (11u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#search_input_51 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (12u32 ..= 17u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#search_input_51 }
                 . apply_pin (_self) . accessible_string_property (index - 12u32 + 1 , what) , (36u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_76 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (37u32 ..= 46u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_76 }
                 . apply_pin (_self) . accessible_string_property (index - 37u32 + 1 , what) , (47u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (48u32 ..= 57u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
                 . apply_pin (_self) . accessible_string_property (index - 48u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_empty_41 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             58usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 36u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 47u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 5u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 8u32 , parent_index : 4u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 10u32 , parent_index : 4u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 18u32 , parent_index : 4u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 10u32 , parent_index : 5u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 10u32 , parent_index : 5u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 12u32 , parent_index : 6u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 12u32 , parent_index : 6u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 13u32 , parent_index : 11u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 15u32 , parent_index : 12u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 18u32 , parent_index : 12u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 16u32 , parent_index : 13u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 18u32 , parent_index : 15u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 18u32 , parent_index : 15u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 20u32 , parent_index : 7u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 20u32 , parent_index : 7u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 24u32 , parent_index : 19u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 27u32 , parent_index : 19u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 30u32 , parent_index : 19u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 33u32 , parent_index : 19u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 27u32 , parent_index : 20u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 27u32 , parent_index : 20u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 27u32 , parent_index : 20u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 30u32 , parent_index : 21u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 30u32 , parent_index : 21u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 30u32 , parent_index : 21u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 33u32 , parent_index : 22u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 33u32 , parent_index : 22u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 33u32 , parent_index : 22u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 36u32 , parent_index : 23u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 36u32 , parent_index : 23u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 36u32 , parent_index : 23u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 37u32 , parent_index : 2u32 , item_array_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 41u32 , parent_index : 36u32 , item_array_index : 37u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 41u32 , parent_index : 36u32 , item_array_index : 38u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 41u32 , parent_index : 36u32 , item_array_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 44u32 , parent_index : 36u32 , item_array_index : 40u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 42u32 , parent_index : 39u32 , item_array_index : 41u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 43u32 , parent_index : 41u32 , item_array_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 44u32 , parent_index : 42u32 , item_array_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 45u32 , parent_index : 40u32 , item_array_index : 44u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 46u32 , parent_index : 44u32 , item_array_index : 45u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 47u32 , parent_index : 45u32 , item_array_index : 46u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 48u32 , parent_index : 3u32 , item_array_index : 47u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 52u32 , parent_index : 47u32 , item_array_index : 48u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 52u32 , parent_index : 47u32 , item_array_index : 49u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 52u32 , parent_index : 47u32 , item_array_index : 50u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 55u32 , parent_index : 47u32 , item_array_index : 51u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 53u32 , parent_index : 50u32 , item_array_index : 52u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 54u32 , parent_index : 52u32 , item_array_index : 53u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 55u32 , parent_index : 53u32 , item_array_index : 54u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 56u32 , parent_index : 51u32 , item_array_index : 55u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 57u32 , parent_index : 55u32 , item_array_index : 56u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 58u32 , parent_index : 56u32 , item_array_index : 57u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_empty_41 , sp :: ItemVTable , sp :: AllowPin > ;
             58usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_41 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_42 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_75 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_77 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_flickable_viewport_43 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_45 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_48 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_52 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_46 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_47 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_50 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_background_112 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_focus_border_115 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_clip_108 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#search_input_51 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_53 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#empty_54 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_55 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_60 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_65 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#rectangle_70 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card1_touch_56 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_58 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_59 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card2_touch_61 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_63 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_64 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card3_touch_66 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_68 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_69 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#card4_touch_71 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_73 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_empty_41 :: FIELD_OFFSETS . r#text_74 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_thumb_117 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_opacity_119 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_opacity_123 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_120 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_opacity_121 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_124 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_opacity_125 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_vertical_bar_76 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_thumb_117 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_opacity_119 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_opacity_123 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_120 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_opacity_121 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_124 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_opacity_125 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_empty_41 :: FIELD_OFFSETS . r#i_horizontal_bar_78 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_empty_41) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_empty_41 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_empty_41 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_empty_41 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_empty_41 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 34u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_empty_41 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_empty_41 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_80 {
         r#rectangle_80 : sp :: r#Empty , r#rectangle_82 : sp :: r#BasicBorderRectangle , r#text_84 : sp :: r#Text , r#decline_touch_85 : sp :: r#TouchArea , r#text_86 : sp :: r#Text , r#text_87 : sp :: r#Text , r#empty_88 : sp :: r#Empty , r#i_flickable_89 : sp :: r#Flickable , r#i_flickable_viewport_90 : sp :: r#Empty , r#rectangle_91 : sp :: r#BasicBorderRectangle , r#text_93 : sp :: r#Text , r#rectangle_94 : sp :: r#Rectangle , r#text_95 : sp :: r#Text , r#i_vertical_bar_visibility_96 : sp :: r#Clip , r#i_horizontal_bar_visibility_98 : sp :: r#Clip , r#i_vertical_bar_97 : InnerScrollBar_root_116 , r#i_horizontal_bar_99 : InnerScrollBar_root_116 , r#rectangle_80_decline_touch_85_x : sp :: Property < sp :: LogicalLength > , r#rectangle_80_decline_touch_85_y : sp :: Property < sp :: LogicalLength > , r#rectangle_80_empty_81_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#rectangle_80_empty_81_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#rectangle_80_empty_81_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#rectangle_80_empty_81_width : sp :: Property < sp :: LogicalLength > , r#rectangle_80_empty_83_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#rectangle_80_empty_83_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#rectangle_80_empty_83_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#rectangle_80_empty_88_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#rectangle_80_empty_88_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#rectangle_80_empty_88_visible_height : sp :: Property < sp :: LogicalLength > , r#rectangle_80_empty_88_visible_width : sp :: Property < sp :: LogicalLength > , r#rectangle_80_empty_88_x : sp :: Property < sp :: LogicalLength > , r#rectangle_80_empty_92_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#rectangle_80_i_flickable_89_horizontal_stretch : sp :: Property < f32 > , r#rectangle_80_i_flickable_89_max_height : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_flickable_89_max_width : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_flickable_89_min_height : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_flickable_89_min_width : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_flickable_89_preferred_height : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_flickable_89_preferred_width : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_flickable_89_vertical_stretch : sp :: Property < f32 > , r#rectangle_80_i_flickable_89_x : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_flickable_89_y : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_horizontal_bar_99_visible : sp :: Property < bool > , r#rectangle_80_i_horizontal_bar_visibility_98_height : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_horizontal_bar_visibility_98_width : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_horizontal_bar_visibility_98_x : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_horizontal_bar_visibility_98_y : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_vertical_bar_97_visible : sp :: Property < bool > , r#rectangle_80_i_vertical_bar_visibility_96_height : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_vertical_bar_visibility_96_width : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_vertical_bar_visibility_96_x : sp :: Property < sp :: LogicalLength > , r#rectangle_80_i_vertical_bar_visibility_96_y : sp :: Property < sp :: LogicalLength > , r#rectangle_80_rectangle_82_width : sp :: Property < sp :: LogicalLength > , r#rectangle_80_rectangle_82_x : sp :: Property < sp :: LogicalLength > , r#rectangle_80_rectangle_91_x : sp :: Property < sp :: LogicalLength > , r#rectangle_80_rectangle_91_y : sp :: Property < sp :: LogicalLength > , r#rectangle_80_rectangle_94_x : sp :: Property < sp :: LogicalLength > , r#rectangle_80_text_84_y : sp :: Property < sp :: LogicalLength > , r#rectangle_80_text_86_y : sp :: Property < sp :: LogicalLength > , r#rectangle_80_text_87_y : sp :: Property < sp :: LogicalLength > , r#rectangle_80_text_93_x : sp :: Property < sp :: LogicalLength > , r#rectangle_80_text_95_x : sp :: Property < sp :: LogicalLength > , r#rectangle_80_x : sp :: Property < sp :: LogicalLength > , r#rectangle_80_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_80 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_80 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerScrollBar_root_116 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 15u32 - 1 , tree_index_of_first_child + 16u32 - 1) ;
             InnerScrollBar_root_116 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 26u32 - 1 , tree_index_of_first_child + 27u32 - 1) ;
             sp :: Property :: link_two_way (({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
            ) . apply_pin (_self) , ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self)) ;
             sp :: Property :: link_two_way (({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_value }
            ) . apply_pin (_self) , ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self)) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 40f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 40f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 50f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : (InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as _ , r#spacing : 12f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = (({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + (({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layoutinfo_h }
                        ) . apply_pin (_self) . get ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 40f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 40f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 12f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
                                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 15f64 as _ ;
                             the_struct . r#end = 15f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_rectangle_82_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 15f64 as _ ;
                         the_struct . r#end = 15f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_92_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Start as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_94 }
                                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 1f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 1f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as _ , r#spacing : 20f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as f64) - (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_horizontal_bar_99_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_width }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_vertical_bar_97_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_height }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_rectangle_82_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_82 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4280166715f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_82 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281549141f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_82 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_82 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4279286145f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (700f64) as i32 }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("🌐 متصفح آمن (قارئ أوروك الذكي)")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#decline_touch_85 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281549141f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("|")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4287931320f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set ({
                 (sp :: r#TextOverflow :: r#Elide) as sp :: r#TextOverflow }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_current_url) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_91 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4280166715f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_91 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281549141f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_91 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_91 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294286859f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (22f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (800f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_92_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_page_title) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set ({
                 (sp :: r#TextWrap :: r#WordWrap) as sp :: r#TextWrap }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_94 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4282865001f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4293060848f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_92_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_page_content) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set ({
                 (sp :: r#TextWrap :: r#WordWrap) as sp :: r#TextWrap }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_96 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_vertical_bar_97_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_page_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_width }
                    ) . apply_pin (_self) . get () . get () as f64) + (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_x }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) - (14f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_y }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_98 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_horizontal_bar_99_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
            ) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_page_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_x }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_height }
                    ) . apply_pin (_self) . get () . get () as f64) + (({
                         * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_y }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) - (14f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_decline_touch_85_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_decline_touch_85_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_horizontal_bar_visibility_98_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_horizontal_bar_visibility_98_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_horizontal_bar_visibility_98_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_horizontal_bar_visibility_98_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_vertical_bar_visibility_96_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_vertical_bar_visibility_96_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_vertical_bar_visibility_96_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_vertical_bar_visibility_96_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_rectangle_82_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_rectangle_91_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_rectangle_91_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_rectangle_94_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_text_84_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_text_86_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_text_87_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_text_93_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_text_95_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_82 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_82 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_82 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_82 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#decline_touch_85 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#decline_touch_85 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_91 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_91 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_91 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_91 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_94 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_96 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_96 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_96 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_96 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_96 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_96 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_opacity_123_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_121_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_125_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_opacity_119_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_98 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_98 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_98 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_98 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_98 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_98 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_horizontal }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_down_scroll_button_opacity_123_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_121_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_icon_opacity_125_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_i_up_scroll_button_opacity_119_dummy }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerScrollBar_root_116 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             . apply_pin (x)) ,) ;
             InnerScrollBar_root_116 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_layoutinfo_h }
                ) . apply_pin (_self) . get ())) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_layoutinfo_v }
                ) . apply_pin (_self) . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (40f64 as sp :: Coord , (InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_rectangle_82_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , (InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_81_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (40f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_text_84_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => (40f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_text_86_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => (40f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_text_87_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 6u32 => (40f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_83_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_decline_touch_85_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_decline_touch_85_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 7u32 => (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 8u32 => (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_vertical_bar_visibility_96_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_vertical_bar_visibility_96_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_vertical_bar_visibility_96_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_vertical_bar_visibility_96_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 9u32 => (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_horizontal_bar_visibility_98_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_horizontal_bar_visibility_98_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_horizontal_bar_visibility_98_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_horizontal_bar_visibility_98_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 10u32 => (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 11u32 => (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_rectangle_91_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_rectangle_91_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 12u32 => (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_92_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_text_93_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_92_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 13u32 => (1f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_rectangle_94_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_92_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 14u32 => (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_92_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_text_95_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_92_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 15u32 => (({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 14f64 as sp :: Coord , ({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 26u32 => (14f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_empty_88_visible_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80_i_flickable_89_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
                 + {
                     * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 16u32 ..= 25u32 => return {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_97 }
                 . apply_pin (_self) . item_geometry (index - 16u32 + 1) , 27u32 ..= 36u32 => return {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
                 . apply_pin (_self) . item_geometry (index - 27u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 3u32 => sp :: r#AccessibleRole :: r#Text , 4u32 => sp :: r#AccessibleRole :: r#Text , 5u32 => sp :: r#AccessibleRole :: r#Text , 12u32 => sp :: r#AccessibleRole :: r#Text , 14u32 => sp :: r#AccessibleRole :: r#Text , 15u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_97 }
                 . apply_pin (_self) . accessible_role (0) , 16u32 ..= 25u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_97 }
                 . apply_pin (_self) . accessible_role (index - 16u32 + 1) , 26u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
                 . apply_pin (_self) . accessible_role (0) , 27u32 ..= 36u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
                 . apply_pin (_self) . accessible_role (index - 27u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (3u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("🌐 متصفح آمن (قارئ أوروك الذكي)") , (4u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("|") , (5u32 , sp :: AccessibleStringProperty :: r#Label) => (InnerAppWindow :: FIELD_OFFSETS . r#root_1_current_url) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () , (12u32 , sp :: AccessibleStringProperty :: r#Label) => (InnerAppWindow :: FIELD_OFFSETS . r#root_1_page_title) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () , (14u32 , sp :: AccessibleStringProperty :: r#Label) => (InnerAppWindow :: FIELD_OFFSETS . r#root_1_page_content) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () , (15u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_97 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (16u32 ..= 25u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_vertical_bar_97 }
                 . apply_pin (_self) . accessible_string_property (index - 16u32 + 1 , what) , (26u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (27u32 ..= 36u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
                 . apply_pin (_self) . accessible_string_property (index - 27u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_rectangle_80 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             37usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 7u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 1u32 , children_index : 6u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 3u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 10u32 , parent_index : 2u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 15u32 , parent_index : 2u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 26u32 , parent_index : 2u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 11u32 , parent_index : 7u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 12u32 , parent_index : 10u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 15u32 , parent_index : 11u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 15u32 , parent_index : 11u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 15u32 , parent_index : 11u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 16u32 , parent_index : 8u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 20u32 , parent_index : 15u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 20u32 , parent_index : 15u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 20u32 , parent_index : 15u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 23u32 , parent_index : 15u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 21u32 , parent_index : 18u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 22u32 , parent_index : 20u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 23u32 , parent_index : 21u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 24u32 , parent_index : 19u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 25u32 , parent_index : 23u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 26u32 , parent_index : 24u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 27u32 , parent_index : 9u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 26u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 26u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 31u32 , parent_index : 26u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 34u32 , parent_index : 26u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 32u32 , parent_index : 29u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 33u32 , parent_index : 31u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 34u32 , parent_index : 32u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 35u32 , parent_index : 30u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 36u32 , parent_index : 34u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 37u32 , parent_index : 35u32 , item_array_index : 36u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_80 , sp :: ItemVTable , sp :: AllowPin > ;
             37usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_80 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_82 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#empty_88 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_84 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_86 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_87 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#decline_touch_85 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_89 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_96 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_98 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_flickable_viewport_90 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_91 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_93 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#rectangle_94 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#text_95 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_thumb_117 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_opacity_119 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_opacity_123 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_120 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_opacity_121 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_124 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_opacity_125 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_vertical_bar_97 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#root_116 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_thumb_117 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_touch_area_118 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_opacity_119 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_opacity_123 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_up_scroll_button_120 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_opacity_121 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_122 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_down_scroll_button_124 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_opacity_125 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_80 :: FIELD_OFFSETS . r#i_horizontal_bar_99 }
             + {
                 * & InnerScrollBar_root_116 :: FIELD_OFFSETS . r#i_icon_126 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_80) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_80 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_rectangle_80 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_80 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_80 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 35u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_80 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_80 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_101 {
         r#rectangle_101 : sp :: r#Rectangle , r#text_103 : sp :: r#Text , r#text_104 : sp :: r#Text , r#text_105 : sp :: r#Text , r#rectangle_101_empty_102_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#rectangle_101_empty_102_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#rectangle_101_empty_102_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#rectangle_101_text_103_x : sp :: Property < sp :: LogicalLength > , r#rectangle_101_text_104_x : sp :: Property < sp :: LogicalLength > , r#rectangle_101_text_105_x : sp :: Property < sp :: LogicalLength > , r#rectangle_101_x : sp :: Property < sp :: LogicalLength > , r#rectangle_101_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_101 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_101 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (2853115690f64 as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : (InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as _ , r#spacing : 15f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 15f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_127 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (55f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("⛵")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294286859f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (600f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("جاري تحميل الصفحة... يرجى الانتظار")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4284773515f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_current_url) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_text_103_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_text_104_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_text_105_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layoutinfo_h }
                ) . apply_pin (_self) . get ())) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layoutinfo_v }
                ) . apply_pin (_self) . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , (InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_text_103_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , (InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_text_104_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , (InnerAppWindow :: FIELD_OFFSETS . r#root_1_rectangle_40_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_text_105_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101_empty_102_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Text , 2u32 => sp :: r#AccessibleRole :: r#Text , 3u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("⛵") , (2u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("جاري تحميل الصفحة... يرجى الانتظار") , (3u32 , sp :: AccessibleStringProperty :: r#Label) => (InnerAppWindow :: FIELD_OFFSETS . r#root_1_current_url) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_rectangle_101 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             4usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 3u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_101 , sp :: ItemVTable , sp :: AllowPin > ;
             4usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#rectangle_101 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_103 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_104 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_101 :: FIELD_OFFSETS . r#text_105 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_101) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_101 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_rectangle_101 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_101 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_101 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 36u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_101 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_101 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerAppWindow {
         pub fn new () -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & self_rc) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & self_rc , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             37usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 8u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 33u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 34u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 12u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 12u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 12u32 , parent_index : 1u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 12u32 , parent_index : 1u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 21u32 , parent_index : 1u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 27u32 , parent_index : 1u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 29u32 , parent_index : 1u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 31u32 , parent_index : 1u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 15u32 , parent_index : 7u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 17u32 , parent_index : 7u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 19u32 , parent_index : 7u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 17u32 , parent_index : 12u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 17u32 , parent_index : 12u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 19u32 , parent_index : 13u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 19u32 , parent_index : 13u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 21u32 , parent_index : 14u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 21u32 , parent_index : 14u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 22u32 , parent_index : 8u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 24u32 , parent_index : 21u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 27u32 , parent_index : 21u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 25u32 , parent_index : 22u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 27u32 , parent_index : 24u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 27u32 , parent_index : 24u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 29u32 , parent_index : 9u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 29u32 , parent_index : 9u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 10u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 31u32 , parent_index : 10u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 33u32 , parent_index : 11u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 33u32 , parent_index : 11u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 2u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 3u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 3u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3u32 , parent_index : 3u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerAppWindow , sp :: ItemVTable , sp :: AllowPin > ;
             33usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_3 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_28 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_40 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_4 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_7 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_8 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#root_111 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_19 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_22 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_25 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_9 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_12 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_15 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#back_touch_10 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_11 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#fwd_touch_13 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_14 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#reload_touch_16 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_17 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_background_112 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_107 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 * & InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_focus_border_115 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#root_clip_108 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_placeholder_109 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#url_input_18 }
             + {
                 InnerLineEdit_root_111 :: FIELD_OFFSETS . r#i_base_114 }
             + {
                 * & InnerLineEditBase_root_107 :: FIELD_OFFSETS . r#i_text_input_110 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#go_touch_20 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_21 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#full_touch_23 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_24 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#add_tab_touch_26 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_27 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self ,) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter_ . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let self_rc = sp :: VRcMapped :: origin (& self . self_weak . get () . unwrap () . upgrade () . unwrap () ,) ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& self_rc) ;
                 core :: result :: Result :: Ok (adapter) }
            ) }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter_ . get () . cloned () }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerAppWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerAppWindow {
         fn drop (self : core :: pin :: Pin < & mut InnerAppWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerAppWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerAppWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#AppWindow (sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) ;
     impl r#AppWindow {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerAppWindow :: new () ? ;
             inner . globals . global_ColorSchemeSelector_127 . clone () . init (& inner) ;
             inner . globals . global_FluentPalette_128 . clone () . init (& inner) ;
             inner . globals . global_StyleMetrics_129 . clone () . init (& inner) ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn get_active_tab_index (& self) -> i32 {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_active_tab_index }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_active_tab_index (& self , value : i32) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_active_tab_index }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_add_tab (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_add_tab }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_add_tab (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_add_tab }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_back (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_back }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_back (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_back }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_close_tab (& self , arg_0 : i32 ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_close_tab }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_close_tab (& self , mut f : impl FnMut (i32) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_close_tab }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn get_current_url (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_current_url }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_current_url (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_current_url }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_forward (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_forward }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_forward (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_forward }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_is_loading (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_is_loading }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_is_loading (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_is_loading }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_navigate (& self , arg_0 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_navigate }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_navigate (& self , mut f : impl FnMut (sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_navigate }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn invoke_open_external (& self , arg_0 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_open_external }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_open_external (& self , mut f : impl FnMut (sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_open_external }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn get_page_content (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_page_content }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_page_content (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_page_content }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_page_title (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_page_title }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_page_title (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_page_title }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_quick_link_clicked (& self , arg_0 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_quick_link_clicked }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_quick_link_clicked (& self , mut f : impl FnMut (sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_quick_link_clicked }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn invoke_reload (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_reload }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_reload (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_reload }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_select_tab (& self , arg_0 : i32 ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_select_tab }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_select_tab (& self , mut f : impl FnMut (i32) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_select_tab }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn get_tabs (& self) -> sp :: ModelRc < r#TabInfo > {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_tabs }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_tabs (& self , value : sp :: ModelRc < r#TabInfo >) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_tabs }
            ) . apply_pin (_self) . set (value as _) }
         }
     impl From < r#AppWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > {
         fn from (value : r#AppWindow) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#AppWindow {
         type Inner = InnerAppWindow ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) -> Self {
             Self (inner) }
         fn run (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             core :: result :: Result :: Ok (()) }
         fn show (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     # [allow (dead_code)] struct Globals_AppWindow {
         global_ColorSchemeSelector_127 : :: core :: pin :: Pin < sp :: Rc < InnerColorSchemeSelector_127 >> , global_FluentPalette_128 : :: core :: pin :: Pin < sp :: Rc < InnerFluentPalette_128 >> , global_StyleMetrics_129 : :: core :: pin :: Pin < sp :: Rc < InnerStyleMetrics_129 >> , }
     impl :: core :: default :: Default for Globals_AppWindow {
         fn default () -> Self {
             Self {
                 global_ColorSchemeSelector_127 : InnerColorSchemeSelector_127 :: new () , global_FluentPalette_128 : InnerFluentPalette_128 :: new () , global_StyleMetrics_129 : InnerStyleMetrics_129 :: new () , }
             }
         }
     static SLINT_EMBEDDED_RESOURCE_3 : & 'static [u8] = b"<svg width=\"8\" height=\"6\" viewBox=\"0 0 8 6\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0.992188 6C0.854167 6 0.72526 5.97396 0.605469 5.92188C0.485677 5.86719 0.380208 5.79557 0.289062 5.70703C0.200521 5.61589 0.130208 5.51042 0.078125 5.39062C0.0260417 5.26823 0 5.13802 0 5C0 4.89062 0.0143229 4.79036 0.0429688 4.69922C0.0742188 4.60807 0.119792 4.51823 0.179688 4.42969L2.78125 0.644531C2.84896 0.545573 2.92839 0.458333 3.01953 0.382812C3.11068 0.307292 3.20833 0.244792 3.3125 0.195312C3.41927 0.143229 3.53125 0.104167 3.64844 0.078125C3.76562 0.0520833 3.88281 0.0390625 4 0.0390625C4.11719 0.0390625 4.23438 0.0520833 4.35156 0.078125C4.46875 0.104167 4.57943 0.143229 4.68359 0.195312C4.79036 0.244792 4.88932 0.307292 4.98047 0.382812C5.07161 0.458333 5.15104 0.545573 5.21875 0.644531L7.82031 4.42969C7.88021 4.51823 7.92448 4.60807 7.95312 4.69922C7.98438 4.79036 8 4.89062 8 5C8 5.13802 7.97396 5.26823 7.92188 5.39062C7.86979 5.51042 7.79948 5.61589 7.71094 5.70703C7.6224 5.79557 7.51693 5.86719 7.39453 5.92188C7.27474 5.97396 7.14714 6 7.01172 6H0.992188Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = b"<svg width=\"6\" height=\"8\" viewBox=\"0 0 6 8\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 7.00781L0 0.992187C0 0.854167 0.0260417 0.72526 0.078125 0.605469C0.132812 0.485677 0.204427 0.38151 0.292969 0.292969C0.384115 0.201823 0.489583 0.130208 0.609375 0.078125C0.731771 0.0260417 0.861979 0 1 0C1.20573 0 1.39583 0.0598958 1.57031 0.179687L5.35547 2.78125C5.55859 2.92187 5.71615 3.09896 5.82813 3.3125C5.94271 3.52604 6 3.75521 6 4C6 4.24479 5.94271 4.47396 5.82813 4.6875C5.71615 4.90104 5.55859 5.07812 5.35547 5.21875L1.57031 7.82031C1.39583 7.9401 1.20573 8 1 8C0.861979 8 0.731771 7.97396 0.609375 7.92188C0.489583 7.86979 0.384115 7.79948 0.292969 7.71094C0.204427 7.61979 0.132813 7.51432 0.078125 7.39453C0.0260417 7.27474 0 7.14583 0 7.00781Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = b"<svg width=\"8\" height=\"6\" viewBox=\"0 0 8 6\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 1C0 0.864583 0.0260417 0.735677 0.078125 0.613281C0.130208 0.490885 0.200521 0.385417 0.289062 0.296875C0.380208 0.205729 0.485677 0.134115 0.605469 0.0820312C0.72526 0.0273438 0.854167 0 0.992188 0H7.01172C7.14714 0 7.27474 0.0260417 7.39453 0.078125C7.51693 0.130208 7.6224 0.201823 7.71094 0.292969C7.79948 0.384115 7.86979 0.490885 7.92188 0.613281C7.97396 0.733073 8 0.860677 8 0.996094C8 1.10547 7.98438 1.20573 7.95312 1.29688C7.92448 1.38802 7.88021 1.47917 7.82031 1.57031L5.21875 5.35547C5.08073 5.55599 4.90365 5.71354 4.6875 5.82812C4.47396 5.94271 4.24479 6 4 6C3.75521 6 3.52474 5.94271 3.30859 5.82812C3.09505 5.71354 2.91927 5.55599 2.78125 5.35547L0.179688 1.57031C0.119792 1.48177 0.0742188 1.39193 0.0429688 1.30078C0.0143229 1.20964 0 1.10938 0 1Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = b"<svg width=\"6\" height=\"8\" viewBox=\"0 0 6 8\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 4C0 3.75521 0.0572917 3.52604 0.171875 3.3125C0.286458 3.09635 0.44401 2.91927 0.644531 2.78125L4.42969 0.179687C4.51823 0.119792 4.60807 0.0755208 4.69922 0.046875C4.79036 0.015625 4.89062 0 5 0C5.13542 0 5.26432 0.0260417 5.38672 0.078125C5.50911 0.130208 5.61458 0.201823 5.70312 0.292969C5.79427 0.38151 5.86589 0.485677 5.91797 0.605469C5.97266 0.72526 6 0.854167 6 0.992187L6 7.00781C6 7.14583 5.97266 7.27474 5.91797 7.39453C5.86589 7.51432 5.79427 7.61979 5.70313 7.71094C5.61458 7.79948 5.50911 7.86979 5.38672 7.92187C5.26432 7.97396 5.13542 8 5 8C4.79427 8 4.60417 7.9401 4.42969 7.82031L0.644531 5.21875C0.44401 5.08073 0.286458 4.90495 0.171875 4.69141C0.0572917 4.47526 0 4.24479 0 4Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_5_0 = slint :: VersionCheck_1_5_0 ;
     }
 # [allow (unused_imports)] pub use slint_generatedAppWindow :: {
     r#AppWindow , r#TabInfo , r#TextStyle }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
