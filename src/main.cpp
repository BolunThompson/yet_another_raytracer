#include "src/vec3.h"
#define STB_IMAGE_WRITE_IMPLEMENTATION

#include <cstdint>
#include <cstdio>
#include <stb_image_write.h>

#include "color.h"
#include "ray.h"

#define log(...) std::fprintf(stderr, __VA_ARGS__)

struct RenderConfig {
  // image
  const std::string out;
  const float aspect_ratio;
  const int wt;
  const int ht;
  const int imgsize;

  // camera
  const float focal_length;
  const float viewport_height;
  const float viewport_width;
  const point3 camera_center;
  const vec3 viewport_u;
  const vec3 viewport_v;
  const vec3 pixel_delta_u;
  const vec3 pixel_delta_v;
  const vec3 viewport_upper_left;
  const vec3 pixel00_loc;
};

// For convenience's sake both the human-set values and the calculated values
// are in the same struct. It'd be cleaner to separate it out the inputted
// config from the ouptuted calculated config.
static constexpr RenderConfig c = {
    // image
    .out = std::string("out.png"),
    .aspect_ratio = 16.0 / 9.0,
    .wt = 400,
    .ht = std::max(1, int(c.wt / c.aspect_ratio)),

    // camera
    .focal_length = 1.0,
    .viewport_height = 2.0,
    .viewport_width = c.viewport_height * (float(c.wt) / c.ht),
    .camera_center = point3(0, 0, 0),

    // calculated viewport state
    .viewport_u = vec3(c.viewport_width, 0, 0),
    .viewport_v = vec3(0, -c.viewport_height, 0),
    .pixel_delta_u = c.viewport_u / c.wt,
    .pixel_delta_v = c.viewport_v / c.ht,
    .viewport_upper_left = c.camera_center - vec3(0, 0, c.focal_length) -
                           c.viewport_u / 2 - c.viewport_v,
    .pixel00_loc =
        c.viewport_upper_left + 0.5 * (c.pixel_delta_u + c.pixel_delta_v),
};


void save_image(const uint8_t *data) {
  stbi_write_png(c.out.c_str(), c.wt, c.ht, 3, data, c.wt * 3);
}

float hit_sphere(const point3 &center, float radius, const ray &r) {
  // simplified calculation of the intersection points of the ray and the "sphere",
  // treated as a quadratic here.
  vec3 oc = center - r.origin();
  auto a = r.direction().length_squared();
  auto h = dot(r.direction(), oc);
  auto c = oc.length_squared() - radius * radius;
  auto discriminant = h * h - a * c;
  if (discriminant < 0)
    return -1;
  else
    return (h - std::sqrt(discriminant)) / a;
}

color ray_color(const ray &r) {
  auto t = hit_sphere(point3(0, 0, -1), 0.5, r);
  if (t > 0.0) {
    vec3 N = unit_vector(r.at(t) - vec3(0, 0, -1));
    return 0.5*color(N.x() + 1, N.y() + 1, N.z() +1);
  }
  
  vec3 unit_direction = unit_vector(r.direction());
  auto a = 0.5 * (unit_direction.y() + 1.0);
  return (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0);
}

int main() {
  uint8_t image_data[c.wt * c.ht * 3];
  for (unsigned int y = 0; y < c.ht; ++y) {
    for (unsigned int x = 0; x < c.wt; ++x) {
      int i = 3 * (y * c.wt + x);

      auto pixel_center =
          c.pixel00_loc + (x * c.pixel_delta_u) + (y * c.pixel_delta_v);
      auto ray_direction = pixel_center - c.camera_center;
      ray r(c.camera_center, ray_direction);

      auto pixel_color = ray_color(r);
      write_color(image_data, i, pixel_color);
    }
    log("\rScanlines remaining: %d\x1b[0K", c.ht - y - 1);
  }
  log("\nDone! 🎉\n");
  save_image(image_data);
  return 0;
}
