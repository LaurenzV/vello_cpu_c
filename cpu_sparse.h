#ifndef CPU_SPARSE_H
#define CPU_SPARSE_H

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class sp_fill_rule {
  Winding,
  EvenOdd,
};

struct sp_argb;

struct sp_context;

struct sp_path;

struct sp_pixmap;

struct sp_transform {
  double sx;
  double kx;
  double ky;
  double sy;
  double tx;
  double ty;
};

struct sp_point {
  double x;
  double y;
};

struct sp_rect {
  double x0;
  double y0;
  double x1;
  double y1;
};

struct sp_color {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
};

struct sp_paint {
  enum class Tag {
    Color,
  };

  struct Color_Body {
    sp_color _0;
  };

  Tag tag;
  union {
    Color_Body color;
  };
};

struct sp_stroke {
  double width;
};

extern "C" {

sp_transform sp_transform_identity();

sp_transform sp_transform_scale(double sx, double sy);

sp_transform sp_transform_translate(double tx, double ty);

sp_transform sp_transform_rotate(double angle);

sp_transform sp_transform_rotate_at(double angle, double cx, double cy);

sp_path *sp_path_create();

void sp_move_to(sp_path *path, sp_point p);

void sp_line_to(sp_path *path, sp_point p);

void sp_quad_to(sp_path *path, sp_point p0, sp_point p1);

void sp_cubic_to(sp_path *path, sp_point p0, sp_point p1, sp_point p2);

void sp_close(sp_path *path);

sp_path *sp_rounded_rect(sp_rect rect, double r);

void sp_path_destroy(sp_path *b);

sp_context *sp_context_create(uint32_t width, uint32_t height);

void sp_context_destroy(sp_context *ctx);

sp_pixmap *sp_pixmap_create(uint32_t width, uint32_t height);

void sp_pixmap_destroy(sp_pixmap *pixmap);

void sp_render_to_pixmap(sp_pixmap *pixmap, sp_context *context);

void sp_set_transform(sp_context *ctx, sp_transform transform);

void sp_set_fill_rule(sp_context *ctx, sp_fill_rule fill_rule);

void sp_set_paint(sp_context *ctx, sp_paint paint);

void sp_set_stroke(sp_context *ctx, sp_stroke stroke);

void sp_fill_path(sp_context *ctx, const sp_path *path);

void sp_stroke_path(sp_context *ctx, const sp_path *path);

void sp_fill_rect(sp_context *ctx, sp_rect rect);

sp_argb *sp_data(sp_pixmap *pixmap);

const uint8_t *sp_argb_data(const sp_argb *data);

void sp_argb_destroy(sp_argb *data);

void sp_stroke_rect(sp_context *ctx, sp_rect rect);

}  // extern "C"

#endif  // CPU_SPARSE_H
