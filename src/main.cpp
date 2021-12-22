#include "camera.h"
#include "ppm.h"
#include "scene.h"
#include <cstdio>

int
main()
{
  float aspect_ratio = 16.0 / 9.0;

  int width = 400;
  int height = static_cast<int>(width / aspect_ratio);

  float viewport_height = 2.0;
  float focal_length = 1.0;
  Camera camera(viewport_height, aspect_ratio, focal_length);

  float X_SCALING = 1.0 / (width - 1);
  float Y_SCALING = 1.0 / (height - 1);

  PPM canvas(width, height);
  World world = makeWorld();

  for (int y = 0; y < height; ++y) {
    fprintf(stderr, "Scanlines done: %d\n", y);
    for (int x = 0; x < width; ++x) {
      float u = x * X_SCALING;
      float v = 1 - y * Y_SCALING;

      Ray ray = camera.cast(u, v);
      Color color = getColor(ray, world);

      canvas.setColor(x, y, color);
    }
  }

  fprintf(stderr, "Scanning done. Printing.\n");
  canvas.dump();
  return 0;
}