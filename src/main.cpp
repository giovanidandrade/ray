#include "camera.h"
#include "parallel.h"
#include "ppm.h"
#include "rand.h"

int
main()
{
  float aspectRatio = 16.0 / 9.0;

  int width = 400;
  int height = static_cast<int>(width / aspectRatio);

  PPM canvas(width, height);

  int pixelSamples = 100;

  Point lookFrom = Point(3, 3, 2);
  Point lookAt = Point(0, 0, -1);
  Vec viewUp = Vec(0, 1, 0);

  float aperture = 2;
  float distToFocus = (lookFrom - lookAt).len();

  Camera camera(
    lookFrom, lookAt, viewUp, 20.0, aspectRatio, aperture, distToFocus);

  ThreadingInfo info = {
    .width = width,
    .height = height,
    .pixelSamples = pixelSamples,
    .camera = camera,
  };
  launchThreads(info, canvas);

  canvas.dump();
  return 0;
}