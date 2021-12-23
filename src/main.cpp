#include "camera.h"
#include "ppm.h"
#include "rand.h"
#include "scene.h"
#include <thread>

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

  // We're leaving one thread open so we can use the computer
  // while the render is going.
  //
  // If we have no idea about the number of threads the computer
  // can support, let's be conservative and go single threaded.
  int processorCount = std::thread::hardware_concurrency();
  int numThreads = processorCount == 0 ? 1 : processorCount - 1;

  PPM canvas(width, height);
  World world = makeWorld();

  std::vector<std::thread> threads(numThreads);

  int y0 = 0;
  int step = height / numThreads;
  int rem = height % numThreads;
  for (int i = 0; i < numThreads; ++i) {
    Scanner scanner = { .id = i,
                        .width = width,
                        .height = height,
                        .y0 = y0,
                        .y1 = y0 + step + rem,
                        .pixelSamples = pixelSamples };

    threads[i] = std::thread(
      scan,
      // We're sharing memory, but this is safe because each thread has
      // a predetermined slice that doesn't overlap with the others
      std::ref(canvas),
      // Sharing the world and the camera is OK since they're read only
      std::cref(world),
      std::cref(camera),
      scanner);

    y0 += step + rem;
    // We're arbitrarily giving the first thread more work
    // so as far as the other threads are aware,
    // there is no remainder
    rem = 0;
  }

  for (int i = 0; i < numThreads; ++i) {
    threads[i].join();
  }

  canvas.dump();
  return 0;
}