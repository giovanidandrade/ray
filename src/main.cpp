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
      float r = x * X_SCALING;
      float g = y * Y_SCALING;
      float b = 0.25;

      canvas.setColor(x, y, Color(r, g, b));
    }
  }

  canvas.dump();
  return 0;
}