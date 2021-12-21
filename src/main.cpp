#include "ppm.h"
#include <cstdio>

int
main()
{
  int width = 256;
  int height = 256;

  float X_SCALING = 1.0 / (width - 1);
  float Y_SCALING = 1.0 / (height - 1);

  PPM canvas(width, height);

  for (int y = 0; y < height; ++y) {
    for (int x = 0; x < width; ++x) {
      float gray = (x * X_SCALING + y * Y_SCALING) / 2.0;
      canvas.setColor(x, y, gray);
    }
  }

  canvas.dump();
  return 0;
}