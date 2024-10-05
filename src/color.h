#pragma once

#include "vec3.h"

#include <cstdint>

using color = vec3;

inline void write_color(uint8_t data[], const size_t loc, const color& pixel_color) {
    auto r = pixel_color.x();
    auto g = pixel_color.y();
    auto b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    int rbyte = int(255.999 * r);
    int gbyte = int(255.999 * g);
    int bbyte = int(255.999 * b);
    
    // Write out the pixel color components.
    data[loc] = rbyte;
    data[loc + 1] = gbyte;
    data[loc + 2] = bbyte;
}
