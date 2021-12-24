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

  ThreadingInfo info = { .width = width,
                         .height = height,
                         .pixelSamples = pixelSamples,
                         .aspectRatio = aspectRatio };
  launchThreads(info, canvas);

  canvas.dump();
  return 0;
}