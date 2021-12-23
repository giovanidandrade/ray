#include "camera.h"
#include "ppm.h"
#include "rand.h"
#include "scene.h"
#include <cstdio>

int
main()
{
  float aspectRatio = 16.0 / 9.0;

  int width = 400;
  int height = static_cast<int>(width / aspectRatio);

  float viewportHeight = 2.0;
  float focalLength = 1.0;
  Camera camera(viewportHeight, aspectRatio, focalLength);

  int pixelSamples = 100;
  float PIXEL_SCALING = 1.0 / pixelSamples;

  float X_SCALING = 1.0 / (width - 1);
  float Y_SCALING = 1.0 / (height - 1);

  PPM canvas(width, height);
  World world = makeWorld();

  for (int y = 0; y < height; ++y) {
    fprintf(stderr, "Scanlines done: %d\n", y);
    for (int x = 0; x < width; ++x) {
      Color color = Color(0, 0, 0);

      for (int s = 0; s < pixelSamples; ++s) {
        float u = jitter(x) * X_SCALING;
        float v = 1 - jitter(y) * Y_SCALING;

        Ray ray = camera.cast(u, v);
        color = color + getColor(ray, world);
      }

      canvas.setColor(x, y, color * PIXEL_SCALING);
    }
  }

  fprintf(stderr, "Scanning done. Printing.\n");
  canvas.dump();
  return 0;
}