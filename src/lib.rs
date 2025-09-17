#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use vello_cpu::color::{AlphaColor, PremulRgba8, Srgb};
use vello_cpu::kurbo::{Affine, BezPath, Cap, Join, Point, Rect, RoundedRectRadii, Shape, Stroke};
use vello_cpu::peniko::{Fill, Gradient, GradientKind, ColorStop, ColorStops, Extend, ImageQuality, SweepGradientPosition, RadialGradientPosition, LinearGradientPosition};
use vello_cpu::{PaintType, Pixmap, RenderContext, RenderMode, Image, RenderSettings, Level, ImageSource};
use vello_cpu::color::DynamicColor;
use std::sync::Arc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc_point {
    x: f64,
    y: f64,
}

impl From<Point> for vc_point {
    fn from(value: Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<vc_point> for Point {
    fn from(value: vc_point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc_transform {
    sx: f64,
    kx: f64,
    ky: f64,
    sy: f64,
    tx: f64,
    ty: f64,
}

impl From<Affine> for vc_transform {
    fn from(value: Affine) -> Self {
        let components = value.as_coeffs();
        Self {
            sx: components[0],
            kx: components[1],
            ky: components[2],
            sy: components[3],
            tx: components[4],
            ty: components[5],
        }
    }
}

impl From<vc_transform> for Affine {
    fn from(value: vc_transform) -> Self {
        Affine::new([value.sx, value.kx, value.ky, value.sy, value.tx, value.ty])
    }
}

pub struct vc_path(BezPath);

#[repr(C)]
#[derive(Debug)]
pub struct vc_color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<vc_color> for AlphaColor<Srgb> {
    fn from(value: vc_color) -> Self {
        Self::from_rgba8(value.r, value.g, value.b, value.a)
    }
}

#[repr(C)]
pub enum vc_fill_rule {
    Winding,
    EvenOdd,
}

impl From<vc_fill_rule> for Fill {
    fn from(value: vc_fill_rule) -> Self {
        match value {
            vc_fill_rule::Winding => Fill::NonZero,
            vc_fill_rule::EvenOdd => Fill::EvenOdd,
        }
    }
}

#[repr(C)]
pub struct vc_rect {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl From<vc_rect> for Rect {
    fn from(value: vc_rect) -> Self {
        Rect::from_points((value.x0, value.y0), (value.x1, value.y1))
    }
}

#[no_mangle]
pub extern "C" fn vc_transform_identity() -> vc_transform {
    vc_transform {
        sx: 1.0,
        kx: 0.0,
        ky: 0.0,
        sy: 1.0,
        tx: 0.0,
        ty: 0.0,
    }
}

#[no_mangle]
pub extern "C" fn vc_transform_scale(sx: f64, sy: f64) -> vc_transform {
    vc_transform {
        sx,
        kx: 0.0,
        ky: 0.0,
        sy,
        tx: 0.0,
        ty: 0.0,
    }
}

#[no_mangle]
pub extern "C" fn vc_transform_translate(tx: f64, ty: f64) -> vc_transform {
    vc_transform {
        sx: 1.0,
        kx: 0.0,
        ky: 0.0,
        sy: 1.0,
        tx,
        ty,
    }
}

#[no_mangle]
pub extern "C" fn vc_transform_rotate(angle: f64) -> vc_transform {
    Affine::rotate(angle).into()
}

#[no_mangle]
pub extern "C" fn vc_transform_rotate_at(angle: f64, cx: f64, cy: f64) -> vc_transform {
    Affine::rotate_about(angle, Point::new(cx, cy)).into()
}

#[no_mangle]
pub extern "C" fn vc_transform_combine(t1: vc_transform, t2: vc_transform) -> vc_transform {
    let a1: Affine = t1.into();
    let a2: Affine = t2.into();
    (a1 * a2).into()
}

#[no_mangle]
pub unsafe extern "C" fn vc_path_create() -> *mut vc_path {
    Box::into_raw(Box::new(vc_path(BezPath::new())))
}

#[no_mangle]
pub unsafe extern "C" fn vc_move_to(path: *mut vc_path, p: vc_point) {
    (*path).0.move_to(p);
}

#[no_mangle]
pub unsafe extern "C" fn vc_line_to(path: *mut vc_path, p: vc_point) {
    (*path).0.line_to(p);
}

#[no_mangle]
pub unsafe extern "C" fn vc_quad_to(path: *mut vc_path, p0: vc_point, p1: vc_point) {
    (*path).0.quad_to(p0, p1);
}

#[no_mangle]
pub unsafe extern "C" fn vc_cubic_to(path: *mut vc_path, p0: vc_point, p1: vc_point, p2: vc_point) {
    (*path).0.curve_to(p0, p1, p2);
}

#[no_mangle]
pub unsafe extern "C" fn vc_close(path: *mut vc_path) {
    (*path).0.close_path();
}

#[no_mangle]
pub unsafe extern "C" fn vc_rounded_rect(rect: vc_rect, r: f64) -> *mut vc_path {
    let rect: Rect = rect.into();
    let rounded = rect.to_rounded_rect(RoundedRectRadii::from(r));

    Box::into_raw(Box::new(vc_path(rounded.to_path(0.1))))
}

#[no_mangle]
pub unsafe extern "C" fn vc_path_destroy(b: *mut vc_path) {
    let _ = Box::from_raw(b);
}

pub struct vc_context(RenderContext);

#[no_mangle]
pub unsafe extern "C" fn vc_context_create(width: u32, height: u32, num_threads: u32) -> *mut vc_context {
    let settings = RenderSettings {
        level: Level::new(),
        num_threads: num_threads as u16,
        render_mode: RenderMode::OptimizeSpeed
    };
    
    let ctx = RenderContext::new_with(width as u16, height as u16, settings);
    Box::into_raw(Box::new(vc_context(ctx)))
}

#[no_mangle]
pub unsafe extern "C" fn vc_context_reset(ctx: *mut vc_context) {
    (*ctx).0.reset();   
}

#[no_mangle]
pub unsafe extern "C" fn vc_context_resize(ctx: *mut vc_context, width: u32, height: u32, num_threads: u32) {
    if (*ctx).0.width() != width as u16 || (*ctx).0.height() != height as u16 {
        let settings = RenderSettings {
            level: Level::new(),
            num_threads: num_threads as u16,
            render_mode: RenderMode::OptimizeSpeed
        };
        
        (*ctx).0 = RenderContext::new_with(width as u16, height as u16, settings);
    }   
}

#[no_mangle]
pub unsafe extern "C" fn vc_context_destroy(ctx: *mut vc_context) {
    let _ = Box::from_raw(ctx);
}

pub struct vc_pixmap(Pixmap);

pub struct vc_arc_pixmap(Arc<Pixmap>);

#[no_mangle]
pub unsafe extern "C" fn vc_pixmap_create(width: u32, height: u32) -> *mut vc_pixmap {
    let pixmap = Pixmap::new(width as u16, height as u16);
    Box::into_raw(Box::new(vc_pixmap(pixmap)))
}

#[no_mangle]
pub unsafe extern "C" fn vc_pixmap_destroy(pixmap: *mut vc_pixmap) {
    let _ = Box::from_raw(pixmap);
}

#[no_mangle]
pub unsafe extern "C" fn vc_arc_pixmap_destroy(pixmap: *mut vc_arc_pixmap) {
    let _ = Box::from_raw(pixmap);
}

#[no_mangle]
pub unsafe extern "C" fn vc_flush(context: *mut vc_context) {
    (*context)
        .0
        .flush();
}

#[no_mangle]
pub unsafe extern "C" fn vc_render_to_pixmap(pixmap: *mut vc_pixmap, context: *mut vc_context) {
    (*context)
        .0
        .render_to_pixmap(&mut (*pixmap).0);
}

#[no_mangle]
pub unsafe extern "C" fn vc_set_transform(ctx: *mut vc_context, transform: vc_transform) {
    (*ctx).0.set_transform(transform.into())
}

#[no_mangle]
pub unsafe extern "C" fn vc_set_paint_transform(ctx: *mut vc_context, transform: vc_transform) {
    (*ctx).0.set_paint_transform(transform.into())
}

#[no_mangle]
pub unsafe extern "C" fn vc_reset_paint_transform(ctx: *mut vc_context) {
    (*ctx).0.reset_paint_transform()
}

#[no_mangle]
pub unsafe extern "C" fn vc_set_fill_rule(ctx: *mut vc_context, fill_rule: vc_fill_rule) {
    (*ctx).0.set_fill_rule(fill_rule.into());
}

#[no_mangle]
pub unsafe extern "C" fn vc_set_paint(ctx: *mut vc_context, paint: vc_paint) {
    (*ctx).0.set_paint(convert_paint(paint));
}

#[no_mangle]
pub unsafe extern "C" fn vc_set_stroke(ctx: *mut vc_context, stroke: vc_stroke) {
    (*ctx).0.set_stroke(stroke.into());
}

#[no_mangle]
pub unsafe extern "C" fn vc_fill_path(ctx: *mut vc_context, path: *const vc_path) {
    (*ctx).0.fill_path(&(*path).0);
}

#[no_mangle]
pub unsafe extern "C" fn vc_stroke_path(ctx: *mut vc_context, path: *const vc_path) {
    (*ctx).0.stroke_path(&(*path).0);
}

#[no_mangle]
pub unsafe extern "C" fn vc_fill_rect(ctx: *mut vc_context, rect: vc_rect) {
    (*ctx).0.fill_rect(&rect.into());
}

pub struct vc_argb(Vec<u8>);

#[no_mangle]
pub unsafe extern "C" fn vc_data(pixmap: *mut vc_pixmap) -> *mut vc_argb {
    let mut buffer = Vec::with_capacity((*pixmap).0.data().len());

    for pixel in (*pixmap).0.data_as_u8_slice().chunks_exact(4) {
        buffer.extend_from_slice(&[pixel[2], pixel[1], pixel[0], pixel[3]]);
    }

    Box::into_raw(Box::new(vc_argb(buffer)))
}

#[no_mangle]
pub unsafe extern "C" fn vc_argb_data(data: *const vc_argb) -> *const u8 {
    (*data).0.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn vc_argb_destroy(data: *mut vc_argb) {
    let _ = Box::from_raw(data);
}

#[repr(C)]
pub struct vc_stroke {
    width: f64,
}

impl From<vc_stroke> for Stroke {
    fn from(value: vc_stroke) -> Self {
        Self {
            width: value.width,
            join: Join::Bevel,
            start_cap: Cap::Butt,
            end_cap: Cap::Butt,
            ..Default::default()
        }
    }
}

#[derive(Clone)]
pub struct vc_image {
    image: Image,
}

impl From<vc_image> for PaintType {
    fn from(value: vc_image) -> Self {
        PaintType::Image(value.image)
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vc_image_quality {
    Low,
    Medium,
    High,
}

impl From<vc_image_quality> for ImageQuality {
    fn from(value: vc_image_quality) -> Self {
        match value {
            vc_image_quality::Low => ImageQuality::Low,
            vc_image_quality::Medium => ImageQuality::Medium,
            vc_image_quality::High => ImageQuality::High,
        }
    }
}

#[repr(C)]
pub enum vc_paint {
    Color(vc_color),
    LinearGradient(*mut vc_linear_gradient),
    RadialGradient(*mut vc_radial_gradient),
    SweepGradient(*mut vc_sweep_gradient),
    Image(*mut vc_image),
}

unsafe fn convert_paint(paint: vc_paint) -> PaintType {
    match paint {
        vc_paint::Color(color) => {
            let c: AlphaColor<Srgb> = color.into();
            c.into()
        }
        vc_paint::LinearGradient(g) => {
            (*g).clone().into()
        }
        vc_paint::RadialGradient(g) => {
            (*g).clone().into()
        }
        vc_paint::SweepGradient(g) => {
            (*g).clone().into()
        }
        vc_paint::Image(img) => {
            (*img).clone().into()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn vc_stroke_rect(ctx: *mut vc_context, rect: vc_rect) {
    (*ctx).0.stroke_rect(&rect.into());
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vc_extend {
    Pad,
    Repeat,
    Reflect,
}

impl From<vc_extend> for Extend {
    fn from(value: vc_extend) -> Self {
        match value {
            vc_extend::Pad => Extend::Pad,
            vc_extend::Repeat => Extend::Repeat,
            vc_extend::Reflect => Extend::Reflect,
        }
    }
}

#[repr(C)]
pub struct vc_gradient_stop {
    offset: f64,
    color: vc_color,
}

impl From<vc_gradient_stop> for ColorStop {
    fn from(value: vc_gradient_stop) -> Self {
        let alpha_color: AlphaColor<Srgb> = value.color.into();
        ColorStop {
            offset: value.offset as f32,
            color: DynamicColor::from_alpha_color(alpha_color),
        }
    }
}

#[derive(Clone)]
pub struct vc_linear_gradient {
    start: vc_point,
    end: vc_point,
    stops: Vec<ColorStop>,
    extend: Extend,
}

impl From<vc_linear_gradient> for PaintType {
    fn from(value: vc_linear_gradient) -> Self {
        let gradient = Gradient {
            kind: GradientKind::Linear(LinearGradientPosition {
                start: value.start.into(),
                end: value.end.into(),
            }),
            stops: ColorStops(value.stops.into()),
            extend: value.extend,
            ..Default::default()
        };
        PaintType::Gradient(gradient)
    }
}

#[derive(Clone)]
pub struct vc_radial_gradient {
    center0: vc_point,
    radius0: f64,
    center1: vc_point,
    radius1: f64,
    stops: Vec<ColorStop>,
    extend: Extend,
}

impl From<vc_radial_gradient> for PaintType {
    fn from(value: vc_radial_gradient) -> Self {
        let gradient = Gradient {
            kind: GradientKind::Radial(RadialGradientPosition {
                start_center: value.center0.into(),
                start_radius: value.radius0 as f32,
                end_center: value.center1.into(),
                end_radius: value.radius1 as f32,
            }),
            stops: ColorStops(value.stops.into()),
            extend: value.extend,
            ..Default::default()
        };
        PaintType::Gradient(gradient)
    }
}

#[derive(Clone)]
pub struct vc_sweep_gradient {
    center: vc_point,
    start_angle: f64,
    end_angle: f64,
    stops: Vec<ColorStop>,
    extend: Extend,
}

impl From<vc_sweep_gradient> for PaintType {
    fn from(value: vc_sweep_gradient) -> Self {
        let gradient = Gradient {
            kind: GradientKind::Sweep(SweepGradientPosition {
                center: value.center.into(),
                start_angle: value.start_angle as f32,
                end_angle: value.end_angle as f32,
            }),
            stops: ColorStops(value.stops.into()),
            extend: value.extend,
            ..Default::default()
        };
        PaintType::Gradient(gradient)
    }
}

#[no_mangle]
pub unsafe extern "C" fn vc_linear_gradient_create(
    start: vc_point,
    end: vc_point,
    extend: vc_extend,
) -> *mut vc_linear_gradient {
    Box::into_raw(Box::new(vc_linear_gradient {
        start,
        end,
        stops: Vec::new(),
        extend: extend.into(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn vc_radial_gradient_create(
    center0: vc_point,
    radius0: f64,
    center1: vc_point,
    radius1: f64,
    extend: vc_extend,
) -> *mut vc_radial_gradient {
    Box::into_raw(Box::new(vc_radial_gradient {
        center0,
        radius0,
        center1,
        radius1,
        stops: Vec::new(),
        extend: extend.into(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn vc_linear_gradient_push_stop(
    gradient: *mut vc_linear_gradient,
    stop: vc_gradient_stop,
) {
    (*gradient).stops.push(stop.into());
}

#[no_mangle]
pub unsafe extern "C" fn vc_radial_gradient_push_stop(
    gradient: *mut vc_radial_gradient,
    stop: vc_gradient_stop,
) {
    (*gradient).stops.push(stop.into());
}

#[no_mangle]
pub unsafe extern "C" fn vc_linear_gradient_destroy(gradient: *mut vc_linear_gradient) {
    let _ = Box::from_raw(gradient);
}

#[no_mangle]
pub unsafe extern "C" fn vc_radial_gradient_destroy(gradient: *mut vc_radial_gradient) {
    let _ = Box::from_raw(gradient);
}

#[no_mangle]
pub unsafe extern "C" fn vc_sweep_gradient_create(
    center: vc_point,
    start_angle: f64,
    end_angle: f64,
    extend: vc_extend,
) -> *mut vc_sweep_gradient {
    Box::into_raw(Box::new(vc_sweep_gradient {
        center,
        start_angle,
        end_angle,
        stops: Vec::new(),
        extend: extend.into(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn vc_sweep_gradient_push_stop(
    gradient: *mut vc_sweep_gradient,
    stop: vc_gradient_stop,
) {
    (*gradient).stops.push(stop.into());
}

#[no_mangle]
pub unsafe extern "C" fn vc_sweep_gradient_destroy(gradient: *mut vc_sweep_gradient) {
    let _ = Box::from_raw(gradient);
}

#[no_mangle]
pub unsafe extern "C" fn vc_pixmap_from_data(
    data: *const u8,
    width: u32,
    height: u32,
) -> *mut vc_arc_pixmap {
    let size = (width * height * 4) as usize;
    let data_slice = std::slice::from_raw_parts(data, size);
    let casted: &[PremulRgba8] = bytemuck::cast_slice(data_slice);

    Box::into_raw(Box::new(vc_arc_pixmap(Arc::new(Pixmap::from_parts(casted.to_vec(), width as u16, height as u16)))))
}

#[no_mangle]
pub unsafe extern "C" fn vc_image_create(
    pixmap: *mut vc_arc_pixmap,
    x_extend: vc_extend,
    y_extend: vc_extend,
    quality: vc_image_quality,
) -> *mut vc_image {
    let pixmap_ref = &(*pixmap).0;
    
    let image = Image {
        source: ImageSource::Pixmap(pixmap_ref.clone()),
        x_extend: x_extend.into(),
        y_extend: y_extend.into(),
        quality: quality.into(),
    };
    
    Box::into_raw(Box::new(vc_image { image }))
}

#[no_mangle]
pub unsafe extern "C" fn vc_image_destroy(image: *mut vc_image) {
    let _ = Box::from_raw(image);
}
