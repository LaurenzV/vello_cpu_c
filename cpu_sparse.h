#ifndef CPU_SPARSE_H
#define CPU_SPARSE_H

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class vc_extend {
  Pad,
  Repeat,
  Reflect,
};

enum class vc_fill_rule {
  Winding,
  EvenOdd,
};

enum class vc_image_quality {
  Low,
  Medium,
  High,
};

struct vc_arc_pixmap;

struct vc_argb;

struct vc_context;

struct vc_image;

struct vc_linear_gradient;

struct vc_path;

struct vc_pixmap;

struct vc_radial_gradient;

struct vc_sweep_gradient;

struct vc_transform {
  double sx;
  double kx;
  double ky;
  double sy;
  double tx;
  double ty;
};

struct vc_point {
  double x;
  double y;
};

struct vc_rect {
  double x0;
  double y0;
  double x1;
  double y1;
};

struct vc_color {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
};

struct vc_paint {
  enum class Tag {
    Color,
    LinearGradient,
    RadialGradient,
    SweepGradient,
    Image,
  };

  struct Color_Body {
    vc_color _0;
  };

  struct LinearGradient_Body {
    vc_linear_gradient *_0;
  };

  struct RadialGradient_Body {
    vc_radial_gradient *_0;
  };

  struct SweepGradient_Body {
    vc_sweep_gradient *_0;
  };

  struct Image_Body {
    vc_image *_0;
  };

  Tag tag;
  union {
    Color_Body color;
    LinearGradient_Body linear_gradient;
    RadialGradient_Body radial_gradient;
    SweepGradient_Body sweep_gradient;
    Image_Body image;
  };
};

struct vc_stroke {
  double width;
};

struct vc_gradient_stop {
  double offset;
  vc_color color;
};

extern "C" {

vc_transform vc_transform_identity();

vc_transform vc_transform_scale(double sx, double sy);

vc_transform vc_transform_translate(double tx, double ty);

vc_transform vc_transform_rotate(double angle);

vc_transform vc_transform_rotate_at(double angle, double cx, double cy);

vc_transform vc_transform_combine(vc_transform t1, vc_transform t2);

vc_path *vc_path_create();

void vc_move_to(vc_path *path, vc_point p);

void vc_line_to(vc_path *path, vc_point p);

void vc_quad_to(vc_path *path, vc_point p0, vc_point p1);

void vc_cubic_to(vc_path *path, vc_point p0, vc_point p1, vc_point p2);

void vc_close(vc_path *path);

vc_path *vc_rounded_rect(vc_rect rect, double r);

void vc_path_destroy(vc_path *b);

vc_context *vc_context_create(uint32_t width, uint32_t height);

void vc_context_destroy(vc_context *ctx);

vc_pixmap *vc_pixmap_create(uint32_t width, uint32_t height);

void vc_pixmap_destroy(vc_pixmap *pixmap);

void vc_arc_pixmap_destroy(vc_arc_pixmap *pixmap);

void vc_render_to_pixmap(vc_pixmap *pixmap, vc_context *context);

void vc_set_transform(vc_context *ctx, vc_transform transform);

void vc_set_paint_transform(vc_context *ctx, vc_transform transform);

void vc_reset_paint_transform(vc_context *ctx);

void vc_set_fill_rule(vc_context *ctx, vc_fill_rule fill_rule);

void vc_set_paint(vc_context *ctx, vc_paint paint);

void vc_set_stroke(vc_context *ctx, vc_stroke stroke);

void vc_fill_path(vc_context *ctx, const vc_path *path);

void vc_stroke_path(vc_context *ctx, const vc_path *path);

void vc_fill_rect(vc_context *ctx, vc_rect rect);

vc_argb *vc_data(vc_pixmap *pixmap);

const uint8_t *vc_argb_data(const vc_argb *data);

void vc_argb_destroy(vc_argb *data);

void vc_stroke_rect(vc_context *ctx, vc_rect rect);

vc_linear_gradient *vc_linear_gradient_create(vc_point start, vc_point end, vc_extend extend);

vc_radial_gradient *vc_radial_gradient_create(vc_point center0,
                                              double radius0,
                                              vc_point center1,
                                              double radius1,
                                              vc_extend extend);

void vc_linear_gradient_push_stop(vc_linear_gradient *gradient, vc_gradient_stop stop);

void vc_radial_gradient_push_stop(vc_radial_gradient *gradient, vc_gradient_stop stop);

void vc_linear_gradient_destroy(vc_linear_gradient *gradient);

void vc_radial_gradient_destroy(vc_radial_gradient *gradient);

vc_sweep_gradient *vc_sweep_gradient_create(vc_point center,
                                            double start_angle,
                                            double end_angle,
                                            vc_extend extend);

void vc_sweep_gradient_push_stop(vc_sweep_gradient *gradient, vc_gradient_stop stop);

void vc_sweep_gradient_destroy(vc_sweep_gradient *gradient);

vc_arc_pixmap *vc_pixmap_from_data(const uint8_t *data, uint32_t width, uint32_t height);

vc_image *vc_image_create(vc_arc_pixmap *pixmap,
                          vc_extend x_extend,
                          vc_extend y_extend,
                          vc_image_quality quality);

void vc_image_destroy(vc_image *image);

}  // extern "C"

#endif  // CPU_SPARSE_H
