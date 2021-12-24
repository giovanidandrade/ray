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
  Point lookFrom = Point(13, 2, 3);
  Point lookAt = Point(0, 0, 0);
  Vec viewUp = Vec(0, 1, 0);

  float aperture = 0.1;
  float distToFocus = 10;

  Camera camera(
    lookFrom, lookAt, viewUp, 20.0, aspectRatio, aperture, distToFocus);

  return camera;
}

World
makeWorld()
{
  World world;

  auto ground = make_shared<Lambertian>(Color(0.5, 0.5, 0.5));
  auto glass = make_shared<Dielectric>(1.5);

  world.addObject(make_shared<Sphere>(Point(0, -1000, 0), 1000, ground));

  for (int x = -11; x < 11; ++x) {
    for (int z = -11; z < 11; ++z) {
      float materialRandom = randomFloat();

      Point center =
        Point(x + 0.9 * randomFloat(), 0.2, z + 0.9 * randomFloat());

      float distance = (center - Point(4, 0.2, 0)).lenSquared();
      if (distance > 0.9) {
        Rc<Material> material;

        if (materialRandom < 0.8) {
          // Lambertian
          Color albedo = randomColor() * randomColor();
          material = make_shared<Lambertian>(albedo);
          world.addObject(make_shared<Sphere>(center, 0.2, material));
        } else if (materialRandom < 0.95) {
          // Metal
          Color color = randomColor() * 0.5 + 0.5;
          float fuzz = randomFloat(0, 0.5);

          material = make_shared<Metal>(color, fuzz);
          world.addObject(make_shared<Sphere>(center, 0.2, material));
        } else {
          // Glass
          world.addObject(make_shared<Sphere>(center, 0.2, glass));
        }
      }
    }
  }

  // And now the big boys
  world.addObject(make_shared<Sphere>(Point(0, 1, 0), 1, glass));

  auto diffuse = make_shared<Lambertian>(Color(0.4, 0.2, 0.1));
  world.addObject(make_shared<Sphere>(Point(-4, 1, 0), 1, diffuse));

  auto metal = make_shared<Metal>(Color(0.7, 0.6, 0.5));
  world.addObject(make_shared<Sphere>(Point(4, 1, 0), 1, metal));

  return world;
}

SceneInfo
makeSceneInfo()
{
  float aspectRatio = 3.0 / 2.0;

  int width = 1200;
  int height = static_cast<int>(width / aspectRatio);

  int pixelSamples = 500;
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