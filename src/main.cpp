#define STB_IMAGE_WRITE_IMPLEMENTATION

#include <iostream>
#include <stb_image_write.h>

#define FILE "out.png"
constexpr int WT = 160;
constexpr int HT = 160;
constexpr int IMG_SIZE = WT * HT * 3;

void save_image(const uint8_t *data) {
  stbi_write_png(FILE, WT, HT, 3, data, WT * 3);
}

int main() {
  uint8_t image_data[IMG_SIZE];
  for (unsigned int y = 0; y < HT; ++y) {
    for (unsigned int x = 0; x < WT; ++x) {
      int i = 3 * (y * WT + x);
      image_data[i] = 65;
      image_data[i + 1] = 105;
      image_data[i + 2] = 225;
    }
  }
  save_image(image_data);
  return 0;
}
