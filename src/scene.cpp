#include "scene.h"
#include "rand.h"
#include "sphere.h"
#include "vec.h"
#include <cstdio>
#include <limits>

using std::make_shared;

const float infinity = std::numeric_limits<float>::infinity();

World
makeWorld()
{
  WorldList objects = WorldList(2);

  objects[0] = make_shared<Sphere>(Point(0, 0, -1), 0.5);
  objects[1] = make_shared<Sphere>(Point(0, -100.5, -1), 100);

  return World(objects);
}

Color
getColor(const Ray& ray, const World& world)
{
  if (auto obs = world.hit(ray, 0, infinity)) {
    return 0.5 * (obs->normal.toColor() + Color(1, 1, 1));
  }

  Vec unit = ray.direction.normalize();

  float t = 0.5 * (unit.y + 1.0);
  return t * Color(0.5, 0.7, 1.0) + (1.0 - t) * Color(1, 1, 1);
}

void
scan(PPM& canvas, const World& world, const Camera& camera, Scanner scanner)
{
  int id = scanner.id;

  int y0 = scanner.y0;
  int y1 = scanner.y1;
  int width = scanner.width;
  int pixelSamples = scanner.pixelSamples;

  float X_SCALING = 1.0 / width;
  float Y_SCALING = 1.0 / scanner.height;
  float PIXEL_SCALING = 1.0 / pixelSamples;

  for (int y = y0; y < y1; ++y) {
    fprintf(stderr, "Thread %d - Scanlines done: %d\n", id, y - y0);
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
}