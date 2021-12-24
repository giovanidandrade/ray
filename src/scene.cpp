#include "scene.h"
#include "dielectric.h"
#include "lambertian.h"
#include "metal.h"
#include "rand.h"
#include "sphere.h"
#include "vec.h"
#include <cmath>
#include <cstdio>
#include <limits>

using std::make_shared;

const float infinity = std::numeric_limits<float>::infinity();
const Color BLACK = Color(0, 0, 0);

Camera
makeCamera(float aspectRatio)
{
  Point lookFrom = Point(3, 3, 2);
  Point lookAt = Point(0, 0, -1);
  Vec viewUp = Vec(0, 1, 0);

  float aperture = 2;
  float distToFocus = (lookFrom - lookAt).len();

  Camera camera(
    lookFrom, lookAt, viewUp, 20.0, aspectRatio, aperture, distToFocus);

  return camera;
}

World
makeWorld()
{
  WorldList objects = WorldList(5);

  auto ground = make_shared<Lambertian>(Color(0.8, 0.8, 0.0));
  auto blue = make_shared<Lambertian>(Color(0.1, 0.2, 0.5));
  auto glass = make_shared<Dielectric>(1.5);
  auto gold = make_shared<Metal>(Color(0.8, 0.6, 0.2));

  objects[0] = make_shared<Sphere>(Point(0, -100.5, -1), 100, ground);
  objects[1] = make_shared<Sphere>(Point(0, 0, -1), 0.5, blue);
  objects[2] = make_shared<Sphere>(Point(-1, 0, -1), 0.5, glass);
  objects[3] = make_shared<Sphere>(Point(-1, 0, -1), -0.45, glass);
  objects[4] = make_shared<Sphere>(Point(1, 0, -1), 0.5, gold);

  return World(objects);
}

SceneInfo
makeSceneInfo()
{
  float aspectRatio = 16.0 / 9.0;

  int width = 400;
  int height = static_cast<int>(width / aspectRatio);

  int pixelSamples = 100;
  World world = makeWorld();
  Camera camera = makeCamera(aspectRatio);

  PPM canvas(width, height);

  SceneInfo info = { .width = width,
                     .height = height,
                     .pixelSamples = pixelSamples,
                     .aspectRatio = aspectRatio,
                     .world = world,
                     .camera = camera,
                     .canvas = canvas };

  return info;
}

Color
getColor(const Ray& ray, const World& world, int depth)
{
  if (depth <= 0) {
    return BLACK;
  }

  if (auto obs = world.hit(ray, 1e-3, infinity)) {
    if (auto pair = obs->material->scatter(ray, *obs)) {
      Color attenuation = pair->second;
      Ray scattered = pair->first;

      return attenuation * getColor(scattered, world, depth - 1);
    }

    return BLACK;
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

  // Let's arbitrate the max depth
  int maxDepth = 25;

  for (int y = y0; y < y1; ++y) {
    fprintf(stderr, "Thread %d - Scanlines done: %d\n", id, y - y0);
    for (int x = 0; x < width; ++x) {
      Color color = Color(0, 0, 0);

      for (int s = 0; s < pixelSamples; ++s) {
        float u = jitter(x) * X_SCALING;
        float v = 1 - jitter(y) * Y_SCALING;

        Ray ray = camera.cast(u, v);
        color = color + getColor(ray, world, maxDepth);
      }

      canvas.setColor(x, y, color * PIXEL_SCALING);
    }
  }
}