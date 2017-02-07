// Copyright (c) IxMilia.  All Rights Reserved.  Licensed under the Apache License, Version 2.0.  See License.txt in the project root for license information.

// other implementation is in `generated/entities.rs`

use std::io::Write;
use enum_primitive::FromPrimitive;
use itertools::PutBack;

use ::{
    CodePair,
    Color,
    DxfError,
    DxfResult,
    Point,
    Vector,
};

use code_pair_writer::CodePairWriter;
use enums::*;
use entities::*;
use helper_functions::*;

//------------------------------------------------------------------------------
//                                                                           Arc
//------------------------------------------------------------------------------
impl Arc {
    pub fn new(center: Point, radius: f64, start: f64, end: f64) -> Self {
        Arc {
            center: center,
            radius: radius,
            start_angle: start,
            end_angle: end,
            .. Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                        Circle
//------------------------------------------------------------------------------
impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Circle {
            center: center,
            radius: radius,
            .. Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                 DimensionBase
//------------------------------------------------------------------------------
impl DimensionBase {
    fn set_dimension_type(&mut self, val: i16) -> DxfResult<()> {
        self.is_block_reference_referenced_by_this_block_only = (val & 32) == 32;
        self.is_ordinate_x_type = (val & 64) == 64;
        self.is_at_user_defined_location = (val & 128) == 128;
        self.dimension_type = try_result!(DimensionType::from_i16(val & 0x0F)); // only take the lower 4 bits
        Ok(())
    }
    #[doc(hidden)]
    pub fn get_dimension_type(&self) -> i16 {
        let mut val = self.dimension_type as i16;
        if self.is_block_reference_referenced_by_this_block_only {
            val |= 32;
        }
        if self.is_ordinate_x_type {
            val |= 64;
        }
        if self.is_at_user_defined_location {
            val |= 128;
        }
        val
    }
}

//------------------------------------------------------------------------------
//                                                                        Face3D
//------------------------------------------------------------------------------
impl Face3D {
    pub fn new(first_corner: Point, second_corner: Point, third_corner: Point, fourth_corner: Point) -> Self {
        Face3D {
            first_corner: first_corner,
            second_corner: second_corner,
            third_corner: third_corner,
            fourth_corner: fourth_corner,
            .. Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                          Line
//------------------------------------------------------------------------------
impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Line {
            p1: p1,
            p2: p2,
            .. Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                              LwPolylineVertex
//------------------------------------------------------------------------------
/// Represents a single vertex of a `LwPolyline`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LwPolylineVertex {
    pub x: f64,
    pub y: f64,
    pub id: i32,
    pub starting_width: f64,
    pub ending_width: f64,
    pub bulge: f64,
}

//------------------------------------------------------------------------------
//                                                                    ModelPoint
//------------------------------------------------------------------------------
impl ModelPoint {
    pub fn new(p: Point) -> Self {
        ModelPoint {
            location: p,
            .. Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                   ProxyEntity
//------------------------------------------------------------------------------
impl ProxyEntity {
    // lower word
    pub fn get_object_drawing_format_version(&self) -> i32 {
        (self._object_drawing_format & 0xFFFF) as i32
    }
    pub fn set_object_drawing_format_version(&mut self, version: i32) {
        self._object_drawing_format |= version as u32 & 0xFFFF;
    }
    // upper word
    pub fn get_object_maintenance_release_version(&self) -> i32 {
        self._object_drawing_format as i32 >> 4
    }
    pub fn set_object_mainenance_release_version(&mut self, version: i32) {
        self._object_drawing_format = (version << 4) as u32 + (self._object_drawing_format & 0xFFFF);
    }
}

//------------------------------------------------------------------------------
//                                                                         Solid
//------------------------------------------------------------------------------
impl Solid {
    pub fn new(first_corner: Point, second_corner: Point, third_corner: Point, fourth_corner: Point) -> Self {
        Solid {
            first_corner: first_corner,
            second_corner: second_corner,
            third_corner: third_corner,
            fourth_corner: fourth_corner,
            .. Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                         Trace
//------------------------------------------------------------------------------
impl Trace {
    pub fn new(first_corner: Point, second_corner: Point, third_corner: Point, fourth_corner: Point) -> Self {
        Trace {
            first_corner: first_corner,
            second_corner: second_corner,
            third_corner: third_corner,
            fourth_corner: fourth_corner,
            .. Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                        Vertex
//------------------------------------------------------------------------------
impl Vertex {
    pub fn new(location: Point) -> Self {
        Vertex {
            location: location,
            .. Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                    EntityType
//------------------------------------------------------------------------------
impl EntityType {
    fn apply_dimension_code_pair(&mut self, pair: &CodePair) -> DxfResult<bool> {
        match self {
            &mut EntityType::RotatedDimension(ref mut dim) => {
                match pair.code {
                    12 => { dim.insertion_point.x = try!(pair.value.assert_f64()); },
                    22 => { dim.insertion_point.y = try!(pair.value.assert_f64()); },
                    32 => { dim.insertion_point.z = try!(pair.value.assert_f64()); },
                    13 => { dim.definition_point_2.x = try!(pair.value.assert_f64()); },
                    23 => { dim.definition_point_2.y = try!(pair.value.assert_f64()); },
                    33 => { dim.definition_point_2.z = try!(pair.value.assert_f64()); },
                    14 => { dim.definition_point_3.x = try!(pair.value.assert_f64()); },
                    24 => { dim.definition_point_3.y = try!(pair.value.assert_f64()); },
                    34 => { dim.definition_point_3.z = try!(pair.value.assert_f64()); },
                    50 => { dim.rotation_angle = try!(pair.value.assert_f64()); },
                    52 => { dim.extension_line_angle = try!(pair.value.assert_f64()); },
                    _ => { return Ok(false); },
                }
            },
            &mut EntityType::RadialDimension(ref mut dim) => {
                match pair.code {
                    15 => { dim.definition_point_2.x = try!(pair.value.assert_f64()); },
                    25 => { dim.definition_point_2.y = try!(pair.value.assert_f64()); },
                    35 => { dim.definition_point_2.z = try!(pair.value.assert_f64()); },
                    40 => { dim.leader_length = try!(pair.value.assert_f64()); },
                    _ => { return Ok(false); },
                }
            },
            &mut EntityType::DiameterDimension(ref mut dim) => {
                match pair.code {
                    15 => { dim.definition_point_2.x = try!(pair.value.assert_f64()); },
                    25 => { dim.definition_point_2.y = try!(pair.value.assert_f64()); },
                    35 => { dim.definition_point_2.z = try!(pair.value.assert_f64()); },
                    40 => { dim.leader_length = try!(pair.value.assert_f64()); },
                    _ => { return Ok(false); },
                }
            },
            &mut EntityType::AngularThreePointDimension(ref mut dim) => {
                match pair.code {
                    13 => { dim.definition_point_2.x = try!(pair.value.assert_f64()); },
                    23 => { dim.definition_point_2.y = try!(pair.value.assert_f64()); },
                    33 => { dim.definition_point_2.z = try!(pair.value.assert_f64()); },
                    14 => { dim.definition_point_3.x = try!(pair.value.assert_f64()); },
                    24 => { dim.definition_point_3.y = try!(pair.value.assert_f64()); },
                    34 => { dim.definition_point_3.z = try!(pair.value.assert_f64()); },
                    15 => { dim.definition_point_4.x = try!(pair.value.assert_f64()); },
                    25 => { dim.definition_point_4.y = try!(pair.value.assert_f64()); },
                    35 => { dim.definition_point_4.z = try!(pair.value.assert_f64()); },
                    16 => { dim.definition_point_5.x = try!(pair.value.assert_f64()); },
                    26 => { dim.definition_point_5.y = try!(pair.value.assert_f64()); },
                    36 => { dim.definition_point_5.z = try!(pair.value.assert_f64()); },
                    _ => { return Ok(false); },
                }
            },
            &mut EntityType::OrdinateDimension(ref mut dim) => {
                match pair.code {
                    13 => { dim.definition_point_2.x = try!(pair.value.assert_f64()); },
                    23 => { dim.definition_point_2.y = try!(pair.value.assert_f64()); },
                    33 => { dim.definition_point_2.z = try!(pair.value.assert_f64()); },
                    14 => { dim.definition_point_3.x = try!(pair.value.assert_f64()); },
                    24 => { dim.definition_point_3.y = try!(pair.value.assert_f64()); },
                    34 => { dim.definition_point_3.z = try!(pair.value.assert_f64()); },
                    _ => { return Ok(false); },
                }
            },
            _ => { return Err(DxfError::UnexpectedEnumValue); },
        }
        Ok(true)
    }
}

//------------------------------------------------------------------------------
//                                                                        Entity
//------------------------------------------------------------------------------
impl Entity {
    /// Creates a new `Entity` with the default common values.
    pub fn new(specific: EntityType) -> Self {
        Entity {
            common: Default::default(),
            specific: specific,
        }
    }
    #[doc(hidden)]
    pub fn read<I>(iter: &mut PutBack<I>) -> DxfResult<Option<Entity>>
        where I: Iterator<Item = DxfResult<CodePair>> {

        'new_entity: loop {
            match iter.next() {
                // first code pair must be 0/entity-type
                Some(Ok(pair @ CodePair { code: 0, .. })) => {
                    let type_string = try!(pair.value.assert_string());
                    if type_string == "ENDSEC" || type_string == "ENDBLK" {
                        iter.put_back(Ok(pair));
                        return Ok(None);
                    }

                    match &*type_string {
                        "DIMENSION" => {
                            // dimensions require special handling
                            let mut common = EntityCommon::default();
                            let mut dimension_entity: Option<EntityType> = None;
                            let mut dimension_base = DimensionBase::default();
                            loop {
                                match iter.next() {
                                    Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                        // new entity or ENDSEC
                                        iter.put_back(Ok(pair));
                                        break;
                                    },
                                    Some(Ok(pair)) => {
                                        match dimension_entity {
                                            Some(ref mut dim) => {
                                                if !try!(dim.apply_dimension_code_pair(&pair)) {
                                                    try!(common.apply_individual_pair(&pair, iter));
                                                }
                                            },
                                            None => {
                                                match pair.code {
                                                    1 => { dimension_base.text = try!(pair.value.assert_string()); },
                                                    2 => { dimension_base.block_name = try!(pair.value.assert_string()); },
                                                    3 => { dimension_base.dimension_style_name = try!(pair.value.assert_string()); },
                                                    10 => { dimension_base.definition_point_1.x = try!(pair.value.assert_f64()); },
                                                    20 => { dimension_base.definition_point_1.y = try!(pair.value.assert_f64()); },
                                                    30 => { dimension_base.definition_point_1.z = try!(pair.value.assert_f64()); },
                                                    11 => { dimension_base.text_mid_point.x = try!(pair.value.assert_f64()); },
                                                    21 => { dimension_base.text_mid_point.y = try!(pair.value.assert_f64()); },
                                                    31 => { dimension_base.text_mid_point.z = try!(pair.value.assert_f64()); },
                                                    41 => { dimension_base.text_line_spacing_factor = try!(pair.value.assert_f64()); },
                                                    42 => { dimension_base.actual_measurement = try!(pair.value.assert_f64()); },
                                                    51 => { dimension_base.horizontal_direction_angle = try!(pair.value.assert_f64()); },
                                                    53 => { dimension_base.text_rotation_angle = try!(pair.value.assert_f64()); },
                                                    70 => { try!(dimension_base.set_dimension_type(try!(pair.value.assert_i16()))); },
                                                    71 => { dimension_base.attachment_point = try_result!(AttachmentPoint::from_i16(try!(pair.value.assert_i16()))); },
                                                    72 => { dimension_base.text_line_spacing_style = try_result!(TextLineSpacingStyle::from_i16(try!(pair.value.assert_i16()))); },
                                                    210 => { dimension_base.normal.x = try!(pair.value.assert_f64()); },
                                                    220 => { dimension_base.normal.y = try!(pair.value.assert_f64()); },
                                                    230 => { dimension_base.normal.z = try!(pair.value.assert_f64()); },
                                                    280 => { dimension_base.version = try_result!(Version::from_i16(try!(pair.value.assert_i16()))); },
                                                    100 => {
                                                        match &*try!(pair.value.assert_string()) {
                                                            "AcDbAlignedDimension" => { dimension_entity = Some(EntityType::RotatedDimension(RotatedDimension { dimension_base: dimension_base.clone(), .. Default::default() })); },
                                                            "AcDbRadialDimension" => { dimension_entity = Some(EntityType::RadialDimension(RadialDimension { dimension_base: dimension_base.clone(), .. Default::default() })); },
                                                            "AcDbDiametricDimension" => { dimension_entity = Some(EntityType::DiameterDimension(DiameterDimension { dimension_base: dimension_base.clone(), .. Default::default() })); },
                                                            "AcDb3PointAngularDimension" => { dimension_entity = Some(EntityType::AngularThreePointDimension(AngularThreePointDimension { dimension_base: dimension_base.clone(), .. Default::default() })); },
                                                            "AcDbOrdinateDimension" => { dimension_entity = Some(EntityType::OrdinateDimension(OrdinateDimension { dimension_base: dimension_base.clone(), .. Default::default() })); },
                                                            _ => {}, // unexpected dimension type
                                                        }
                                                    },
                                                    _ => { try!(common.apply_individual_pair(&pair, iter)); },
                                                }
                                            },
                                        }
                                    },
                                    Some(Err(e)) => return Err(e),
                                    None => return Err(DxfError::UnexpectedEndOfInput),
                                }
                            }

                            match dimension_entity {
                                Some(dim) => { return Ok(Some(Entity { common: common, specific: dim })); },
                                None => { continue 'new_entity; }, // unsuccessful dimension match
                            }
                        },
                        _ => {
                            match EntityType::from_type_string(&type_string) {
                                Some(e) => {
                                    let mut entity = Entity::new(e);
                                    if !try!(entity.apply_custom_reader(iter)) {
                                        // no custom reader, use the auto-generated one
                                        loop {
                                            match iter.next() {
                                                Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                                    // new entity or ENDSEC
                                                    iter.put_back(Ok(pair));
                                                    break;
                                                },
                                                Some(Ok(pair)) => try!(entity.apply_code_pair(&pair, iter)),
                                                Some(Err(e)) => return Err(e),
                                                None => return Err(DxfError::UnexpectedEndOfInput),
                                            }
                                        }

                                        try!(entity.post_parse());
                                    }

                                    return Ok(Some(entity));
                                },
                                None => {
                                    // swallow unsupported entity
                                    loop {
                                    match iter.next() {
                                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                                // found another entity or ENDSEC
                                                iter.put_back(Ok(pair));
                                                break;
                                            },
                                            Some(Ok(_)) => (), // part of the unsupported entity
                                            Some(Err(e)) => return Err(e),
                                            None => return Err(DxfError::UnexpectedEndOfInput),
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Ok(pair)) => return Err(DxfError::UnexpectedCodePair(pair, String::from("expected 0/entity-type or 0/ENDSEC"))),
                Some(Err(e)) => return Err(e),
                None => return Err(DxfError::UnexpectedEndOfInput),
            }
        }
    }
    fn apply_code_pair<I>(&mut self, pair: &CodePair, iter: &mut PutBack<I>) -> DxfResult<()>
        where I: Iterator<Item = DxfResult<CodePair>> {

        if !try!(self.specific.try_apply_code_pair(&pair)) {
            try!(self.common.apply_individual_pair(&pair, iter));
        }
        Ok(())
    }
    fn post_parse(&mut self) -> DxfResult<()> {
        match self.specific {
            EntityType::Image(ref mut image) => {
                combine_points_2(&mut image._clipping_vertices_x, &mut image._clipping_vertices_y, &mut image.clipping_vertices, Point::new);
            },
            EntityType::Leader(ref mut leader) => {
                combine_points_3(&mut leader._vertices_x, &mut leader._vertices_y, &mut leader._vertices_z, &mut leader.vertices, Point::new);
            },
            EntityType::MLine(ref mut mline) => {
                combine_points_3(&mut mline._vertices_x, &mut mline._vertices_y, &mut mline._vertices_z, &mut mline.vertices, Point::new);
                combine_points_3(&mut mline._segment_direction_x, &mut mline._segment_direction_y, &mut mline._segment_direction_z, &mut mline.segment_directions, Vector::new);
                combine_points_3(&mut mline._miter_direction_x, &mut mline._miter_direction_y, &mut mline._miter_direction_z, &mut mline.miter_directions, Vector::new);
            },
            EntityType::Section(ref mut section) => {
                combine_points_3(&mut section._vertices_x, &mut section._vertices_y, &mut section._vertices_z, &mut section.vertices, Point::new);
                combine_points_3(&mut section._back_line_vertices_x, &mut section._back_line_vertices_y, &mut section._back_line_vertices_z, &mut section.back_line_vertices, Point::new);
            },
            EntityType::Spline(ref mut spline) => {
                combine_points_3(&mut spline._control_point_x, &mut spline._control_point_y, &mut spline._control_point_z, &mut spline.control_points, Point::new);
                combine_points_3(&mut spline._fit_point_x, &mut spline._fit_point_y, &mut spline._fit_point_z, &mut spline.fit_points, Point::new);
            },
            EntityType::DgnUnderlay(ref mut underlay) => {
                combine_points_2(&mut underlay._point_x, &mut underlay._point_y, &mut underlay.points, Point::new);
            },
            EntityType::DwfUnderlay(ref mut underlay) => {
                combine_points_2(&mut underlay._point_x, &mut underlay._point_y, &mut underlay.points, Point::new);
            },
            EntityType::PdfUnderlay(ref mut underlay) => {
                combine_points_2(&mut underlay._point_x, &mut underlay._point_y, &mut underlay.points, Point::new);
            },
            EntityType::Wipeout(ref mut wo) => {
                combine_points_2(&mut wo._clipping_vertices_x, &mut wo._clipping_vertices_y, &mut wo.clipping_vertices, Point::new);
            },
            _ => (),
        }

        Ok(())
    }
    fn apply_custom_reader<I>(&mut self, iter: &mut PutBack<I>) -> DxfResult<bool>
        where I: Iterator<Item = DxfResult<CodePair>> {

        match self.specific {
            EntityType::Attribute(ref mut att) => {
                let xrecord_text = "AcDbXrecord";
                let mut last_subclass_marker = String::new();
                let mut is_version_set = false;
                let mut xrec_code_70_count = 0;
                loop {
                    let pair = next_pair!(iter);
                    match pair.code {
                        100 => { last_subclass_marker = try!(pair.value.assert_string()); },
                        1 => { att.value = try!(pair.value.assert_string()); },
                        2 => {
                            if last_subclass_marker == xrecord_text {
                                att.x_record_tag = try!(pair.value.assert_string());
                            }
                            else {
                                att.attribute_tag = try!(pair.value.assert_string());
                            }
                        },
                        7 => { att.text_style_name = try!(pair.value.assert_string()); },
                        10 => {
                            if last_subclass_marker == xrecord_text {
                                att.alignment_point.x = try!(pair.value.assert_f64());
                            }
                            else {
                                att.location.x = try!(pair.value.assert_f64());
                            }
                        },
                        20 => {
                            if last_subclass_marker == xrecord_text {
                                att.alignment_point.y = try!(pair.value.assert_f64());
                            }
                            else {
                                att.location.y = try!(pair.value.assert_f64());
                            }
                        },
                        30 => {
                            if last_subclass_marker == xrecord_text {
                                att.alignment_point.z = try!(pair.value.assert_f64());
                            }
                            else {
                                att.location.z = try!(pair.value.assert_f64());
                            }
                        },
                        11 => { att.second_alignment_point.x = try!(pair.value.assert_f64()); },
                        21 => { att.second_alignment_point.y = try!(pair.value.assert_f64()); },
                        31 => { att.second_alignment_point.z = try!(pair.value.assert_f64()); },
                        39 => { att.thickness = try!(pair.value.assert_f64()); },
                        40 => {
                            if last_subclass_marker == xrecord_text {
                                att.annotation_scale = try!(pair.value.assert_f64());
                            }
                            else {
                                att.text_height = try!(pair.value.assert_f64());
                            }
                        },
                        41 => { att.relative_x_scale_factor = try!(pair.value.assert_f64()); },
                        50 => { att.rotation = try!(pair.value.assert_f64()); },
                        51 => { att.oblique_angle = try!(pair.value.assert_f64()); },
                        70 => {
                            if last_subclass_marker == xrecord_text {
                                match xrec_code_70_count {
                                    0 => att.m_text_flag = try_result!(MTextFlag::from_i16(try!(pair.value.assert_i16()))),
                                    1 => att.is_really_locked = as_bool(try!(pair.value.assert_i16())),
                                    2 => att._secondary_attribute_count = try!(pair.value.assert_i16()) as i32,
                                    _ => return Err(DxfError::UnexpectedCodePair(pair, String::new())),
                                }
                                xrec_code_70_count += 1;
                            }
                            else {
                                att.flags = try!(pair.value.assert_i16()) as i32;
                            }
                        },
                        71 => { att.text_generation_flags = try!(pair.value.assert_i16()) as i32; },
                        72 => { att.horizontal_text_justification = try_result!(HorizontalTextJustification::from_i16(try!(pair.value.assert_i16()))); },
                        73 => { att.field_length = try!(pair.value.assert_i16()); },
                        74 => { att.vertical_text_justification = try_result!(VerticalTextJustification::from_i16(try!(pair.value.assert_i16()))); },
                        210 => { att.normal.x = try!(pair.value.assert_f64()); },
                        220 => { att.normal.y = try!(pair.value.assert_f64()); },
                        230 => { att.normal.z = try!(pair.value.assert_f64()); },
                        280 => {
                            if last_subclass_marker == xrecord_text {
                                att.keep_duplicate_records = as_bool(try!(pair.value.assert_i16()));
                            }
                            else if !is_version_set {
                                att.version = try_result!(Version::from_i16(try!(pair.value.assert_i16())));
                                is_version_set = true;
                            }
                            else {
                                att.is_locked_in_block = as_bool(try!(pair.value.assert_i16()));
                            }
                        },
                        340 => { att.secondary_attributes.push(try!(as_u32(try!(pair.value.assert_string())))); },
                        -1 => { att.m_text = try!(as_u32(try!(pair.value.assert_string()))); },
                        _ => { try!(self.common.apply_individual_pair(&pair, iter)); },
                    }
                }
            },
            EntityType::AttributeDefinition(ref mut att) => {
                let xrecord_text = "AcDbXrecord";
                let mut last_subclass_marker = String::new();
                let mut is_version_set = false;
                let mut xrec_code_70_count = 0;
                loop {
                    let pair = next_pair!(iter);
                    match pair.code {
                        100 => { last_subclass_marker = try!(pair.value.assert_string()); },
                        1 => { att.value = try!(pair.value.assert_string()); },
                        2 => {
                            if last_subclass_marker == xrecord_text {
                                att.x_record_tag = try!(pair.value.assert_string());
                            }
                            else {
                                att.text_tag = try!(pair.value.assert_string());
                            }
                        },
                        3 => { att.prompt = try!(pair.value.assert_string()); },
                        7 => { att.text_style_name = try!(pair.value.assert_string()); },
                        10 => {
                            if last_subclass_marker == xrecord_text {
                                att.alignment_point.x = try!(pair.value.assert_f64());
                            }
                            else {
                                att.location.x = try!(pair.value.assert_f64());
                            }
                        },
                        20 => {
                            if last_subclass_marker == xrecord_text {
                                att.alignment_point.y = try!(pair.value.assert_f64());
                            }
                            else {
                                att.location.y = try!(pair.value.assert_f64());
                            }
                        },
                        30 => {
                            if last_subclass_marker == xrecord_text {
                                att.alignment_point.z = try!(pair.value.assert_f64());
                            }
                            else {
                                att.location.z = try!(pair.value.assert_f64());
                            }
                        },
                        11 => { att.second_alignment_point.x = try!(pair.value.assert_f64()); },
                        21 => { att.second_alignment_point.y = try!(pair.value.assert_f64()); },
                        31 => { att.second_alignment_point.z = try!(pair.value.assert_f64()); },
                        39 => { att.thickness = try!(pair.value.assert_f64()); },
                        40 => {
                            if last_subclass_marker == xrecord_text {
                                att.annotation_scale = try!(pair.value.assert_f64());
                            }
                            else {
                                att.text_height = try!(pair.value.assert_f64());
                            }
                        },
                        41 => { att.relative_x_scale_factor = try!(pair.value.assert_f64()); },
                        50 => { att.rotation = try!(pair.value.assert_f64()); },
                        51 => { att.oblique_angle = try!(pair.value.assert_f64()); },
                        70 => {
                            if last_subclass_marker == xrecord_text {
                                match xrec_code_70_count {
                                    0 => att.m_text_flag = try_result!(MTextFlag::from_i16(try!(pair.value.assert_i16()))),
                                    1 => att.is_really_locked = as_bool(try!(pair.value.assert_i16())),
                                    2 => att._secondary_attribute_count = try!(pair.value.assert_i16()) as i32,
                                    _ => return Err(DxfError::UnexpectedCodePair(pair, String::new())),
                                }
                                xrec_code_70_count += 1;
                            }
                            else {
                                att.flags = try!(pair.value.assert_i16()) as i32;
                            }
                        },
                        71 => { att.text_generation_flags = try!(pair.value.assert_i16()) as i32; },
                        72 => { att.horizontal_text_justification = try_result!(HorizontalTextJustification::from_i16(try!(pair.value.assert_i16()))); },
                        73 => { att.field_length = try!(pair.value.assert_i16()); },
                        74 => { att.vertical_text_justification = try_result!(VerticalTextJustification::from_i16(try!(pair.value.assert_i16()))); },
                        210 => { att.normal.x = try!(pair.value.assert_f64()); },
                        220 => { att.normal.y = try!(pair.value.assert_f64()); },
                        230 => { att.normal.z = try!(pair.value.assert_f64()); },
                        280 => {
                            if last_subclass_marker == xrecord_text {
                                att.keep_duplicate_records = as_bool(try!(pair.value.assert_i16()));
                            }
                            else if !is_version_set {
                                att.version = try_result!(Version::from_i16(try!(pair.value.assert_i16())));
                                is_version_set = true;
                            }
                            else {
                                att.is_locked_in_block = as_bool(try!(pair.value.assert_i16()));
                            }
                        },
                        340 => { att.secondary_attributes.push(try!(as_u32(try!(pair.value.assert_string())))); },
                        -1 => { att.m_text = try!(as_u32(try!(pair.value.assert_string()))); },
                        _ => { try!(self.common.apply_individual_pair(&pair, iter)); },
                    }
                }
            },
            EntityType::LwPolyline(ref mut poly) => {
                loop {
                    let pair = next_pair!(iter);
                    match pair.code {
                        // vertex-specific pairs
                        10 => {
                            // start a new vertex
                            poly.vertices.push(LwPolylineVertex::default());
                            vec_last!(poly.vertices).x = try!(pair.value.assert_f64());
                        },
                        20 => { vec_last!(poly.vertices).y = try!(pair.value.assert_f64()); },
                        40 => { vec_last!(poly.vertices).starting_width = try!(pair.value.assert_f64()); },
                        41 => { vec_last!(poly.vertices).ending_width = try!(pair.value.assert_f64()); },
                        42 => { vec_last!(poly.vertices).bulge = try!(pair.value.assert_f64()); },
                        91 => { vec_last!(poly.vertices).id = try!(pair.value.assert_i32()); },
                        // other pairs
                        39 => { poly.thickness = try!(pair.value.assert_f64()); },
                        43 => { poly.constant_width = try!(pair.value.assert_f64()); },
                        70 => { poly.flags = try!(pair.value.assert_i16()) as i32; },
                        210 => { poly.extrusion_direction.x = try!(pair.value.assert_f64()); },
                        220 => { poly.extrusion_direction.y = try!(pair.value.assert_f64()); },
                        230 => { poly.extrusion_direction.z = try!(pair.value.assert_f64()); },
                        _ => { try!(self.common.apply_individual_pair(&pair, iter)); },
                    }
                }
            },
            EntityType::MText(ref mut mtext) => {
                let mut reading_column_data = false;
                let mut read_column_count = false;
                loop {
                    let pair = next_pair!(iter);
                    match pair.code {
                        10 => { mtext.insertion_point.x = try!(pair.value.assert_f64()); },
                        20 => { mtext.insertion_point.y = try!(pair.value.assert_f64()); },
                        30 => { mtext.insertion_point.z = try!(pair.value.assert_f64()); },
                        40 => { mtext.initial_text_height = try!(pair.value.assert_f64()); },
                        41 => { mtext.reference_rectangle_width = try!(pair.value.assert_f64()); },
                        71 => { mtext.attachment_point = try_result!(AttachmentPoint::from_i16(try!(pair.value.assert_i16()))); },
                        72 => { mtext.drawing_direction = try_result!(DrawingDirection::from_i16(try!(pair.value.assert_i16()))); },
                        3 => { mtext.extended_text.push(try!(pair.value.assert_string())); },
                        1 => { mtext.text = try!(pair.value.assert_string()); },
                        7 => { mtext.text_style_name = try!(pair.value.assert_string()); },
                        210 => { mtext.extrusion_direction.x = try!(pair.value.assert_f64()); },
                        220 => { mtext.extrusion_direction.y = try!(pair.value.assert_f64()); },
                        230 => { mtext.extrusion_direction.z = try!(pair.value.assert_f64()); },
                        11 => { mtext.x_axis_direction.x = try!(pair.value.assert_f64()); },
                        21 => { mtext.x_axis_direction.y = try!(pair.value.assert_f64()); },
                        31 => { mtext.x_axis_direction.z = try!(pair.value.assert_f64()); },
                        42 => { mtext.horizontal_width = try!(pair.value.assert_f64()); },
                        43 => { mtext.vertical_height = try!(pair.value.assert_f64()); },
                        50 => {
                            if reading_column_data {
                                if read_column_count {
                                    mtext.column_heights.push(try!(pair.value.assert_f64()));
                                }
                                else {
                                    mtext.column_count = try!(pair.value.assert_f64()) as i32;
                                    read_column_count = true;
                                }
                            }
                            else {
                                mtext.rotation_angle = try!(pair.value.assert_f64());
                            }
                        },
                        73 => { mtext.line_spacing_style = try_result!(MTextLineSpacingStyle::from_i16(try!(pair.value.assert_i16()))); },
                        44 => { mtext.line_spacing_factor = try!(pair.value.assert_f64()); },
                        90 => { mtext.background_fill_setting = try_result!(BackgroundFillSetting::from_i32(try!(pair.value.assert_i32()))); },
                        420 => { mtext.background_color_rgb = try!(pair.value.assert_i32()); },
                        430 => { mtext.background_color_name = try!(pair.value.assert_string()); },
                        45 => { mtext.fill_box_scale = try!(pair.value.assert_f64()); },
                        63 => { mtext.background_fill_color = Color::from_raw_value(try!(pair.value.assert_i16())); },
                        441 => { mtext.background_fill_color_transparency = try!(pair.value.assert_i32()); },
                        75 => {
                            mtext.column_type = try!(pair.value.assert_i16());
                            reading_column_data = true;
                        },
                        76 => { mtext.column_count = try!(pair.value.assert_i16()) as i32; },
                        78 => { mtext.is_column_flow_reversed = as_bool(try!(pair.value.assert_i16())); },
                        79 => { mtext.is_column_auto_height = as_bool(try!(pair.value.assert_i16())); },
                        48 => { mtext.column_width = try!(pair.value.assert_f64()); },
                        49 => { mtext.column_gutter = try!(pair.value.assert_f64()); },
                        _ => { try!(self.common.apply_individual_pair(&pair, iter)); },
                    }
                }
            },
            _ => return Ok(false), // no custom reader
        }
    }
    #[doc(hidden)]
    pub fn write<T>(&self, version: &AcadVersion, write_handles: bool, writer: &mut CodePairWriter<T>) -> DxfResult<()>
        where T: Write {

        if self.specific.is_supported_on_version(version) {
            try!(writer.write_code_pair(&CodePair::new_str(0, self.specific.to_type_string())));
            try!(self.common.write(version, write_handles, writer));
            if !try!(self.apply_custom_writer(version, writer)) {
                try!(self.specific.write(&self.common, version, writer));
                try!(self.post_write(&version, write_handles, writer));
            }
            for x in &self.common.x_data {
                try!(x.write(version, writer));
            }
        }

        Ok(())
    }
    fn apply_custom_writer<T>(&self, version: &AcadVersion, writer: &mut CodePairWriter<T>) -> DxfResult<bool>
        where T: Write {

        match self.specific {
            EntityType::RotatedDimension(ref dim) => {
                try!(dim.dimension_base.write(version, writer));
                if version >= &AcadVersion::R13 {
                    try!(writer.write_code_pair(&CodePair::new_str(100, "AcDbAlignedDimension")));
                }
                try!(writer.write_code_pair(&CodePair::new_f64(12, dim.insertion_point.x)));
                try!(writer.write_code_pair(&CodePair::new_f64(22, dim.insertion_point.y)));
                try!(writer.write_code_pair(&CodePair::new_f64(32, dim.insertion_point.z)));
                try!(writer.write_code_pair(&CodePair::new_f64(13, dim.definition_point_2.x)));
                try!(writer.write_code_pair(&CodePair::new_f64(23, dim.definition_point_2.y)));
                try!(writer.write_code_pair(&CodePair::new_f64(33, dim.definition_point_2.z)));
                try!(writer.write_code_pair(&CodePair::new_f64(14, dim.definition_point_3.x)));
                try!(writer.write_code_pair(&CodePair::new_f64(24, dim.definition_point_3.y)));
                try!(writer.write_code_pair(&CodePair::new_f64(34, dim.definition_point_3.z)));
                try!(writer.write_code_pair(&CodePair::new_f64(50, dim.rotation_angle)));
                try!(writer.write_code_pair(&CodePair::new_f64(52, dim.extension_line_angle)));
                if version >= &AcadVersion::R13 {
                    try!(writer.write_code_pair(&CodePair::new_str(100, "AcDbRotatedDimension")));
                }
            },
            EntityType::RadialDimension(ref dim) => {
                try!(dim.dimension_base.write(version, writer));
                try!(writer.write_code_pair(&CodePair::new_str(100, "AcDbRadialDimension")));
                try!(writer.write_code_pair(&CodePair::new_f64(15, dim.definition_point_2.x)));
                try!(writer.write_code_pair(&CodePair::new_f64(25, dim.definition_point_2.y)));
                try!(writer.write_code_pair(&CodePair::new_f64(35, dim.definition_point_2.z)));
                try!(writer.write_code_pair(&CodePair::new_f64(40, dim.leader_length)));
            },
            EntityType::DiameterDimension(ref dim) => {
                try!(dim.dimension_base.write(version, writer));
                try!(writer.write_code_pair(&CodePair::new_str(100, "AcDbDiametricDimension")));
                try!(writer.write_code_pair(&CodePair::new_f64(15, dim.definition_point_2.x)));
                try!(writer.write_code_pair(&CodePair::new_f64(25, dim.definition_point_2.y)));
                try!(writer.write_code_pair(&CodePair::new_f64(35, dim.definition_point_2.z)));
                try!(writer.write_code_pair(&CodePair::new_f64(40, dim.leader_length)));
            },
            EntityType::AngularThreePointDimension(ref dim) => {
                try!(dim.dimension_base.write(version, writer));
                try!(writer.write_code_pair(&CodePair::new_str(100, "AcDb3PointAngularDimension")));
                try!(writer.write_code_pair(&CodePair::new_f64(13, dim.definition_point_2.x)));
                try!(writer.write_code_pair(&CodePair::new_f64(23, dim.definition_point_2.y)));
                try!(writer.write_code_pair(&CodePair::new_f64(33, dim.definition_point_2.z)));
                try!(writer.write_code_pair(&CodePair::new_f64(14, dim.definition_point_3.x)));
                try!(writer.write_code_pair(&CodePair::new_f64(24, dim.definition_point_3.y)));
                try!(writer.write_code_pair(&CodePair::new_f64(34, dim.definition_point_3.z)));
                try!(writer.write_code_pair(&CodePair::new_f64(15, dim.definition_point_4.x)));
                try!(writer.write_code_pair(&CodePair::new_f64(25, dim.definition_point_4.y)));
                try!(writer.write_code_pair(&CodePair::new_f64(35, dim.definition_point_4.z)));
                try!(writer.write_code_pair(&CodePair::new_f64(16, dim.definition_point_5.x)));
                try!(writer.write_code_pair(&CodePair::new_f64(26, dim.definition_point_5.y)));
                try!(writer.write_code_pair(&CodePair::new_f64(36, dim.definition_point_5.z)));
            },
            EntityType::OrdinateDimension(ref dim) => {
                try!(dim.dimension_base.write(version, writer));
                try!(writer.write_code_pair(&CodePair::new_str(100, "AcDbOrdinateDimension")));
                try!(writer.write_code_pair(&CodePair::new_f64(13, dim.definition_point_2.x)));
                try!(writer.write_code_pair(&CodePair::new_f64(23, dim.definition_point_2.y)));
                try!(writer.write_code_pair(&CodePair::new_f64(33, dim.definition_point_2.z)));
                try!(writer.write_code_pair(&CodePair::new_f64(14, dim.definition_point_3.x)));
                try!(writer.write_code_pair(&CodePair::new_f64(24, dim.definition_point_3.y)));
                try!(writer.write_code_pair(&CodePair::new_f64(34, dim.definition_point_3.z)));
            },
            _ => return Ok(false), // no custom writer
        }

        Ok(true)
    }
    fn post_write<T>(&self, version: &AcadVersion, write_handles: bool, writer: &mut CodePairWriter<T>) -> DxfResult<()>
        where T: Write {

        match self.specific {
            // TODO: write trailing MText on Attribute and AttributeDefinition
            EntityType::Polyline(ref poly) => {
                for v in &poly.vertices {
                    let v = Entity { common: Default::default(), specific: EntityType::Vertex(v.clone()) };
                    try!(v.write(&version, write_handles, writer));
                }
                let seqend = Entity { common: Default::default(), specific: EntityType::Seqend(Default::default()) };
                try!(seqend.write(&version, write_handles, writer));
            },
            _ => (),
        }

        Ok(())
    }
}
