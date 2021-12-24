#include "parallel.h"

#include "scene.h"
#include <thread>

int
getNumThreads()
{
  // We're leaving one thread open so we can use the computer
  // while the render is going.
  //
  // If we have no idea about the number of threads the computer
  // can support, let's be conservative and go single threaded.
  int processorCount = std::thread::hardware_concurrency();
  int numThreads = processorCount == 0 ? 1 : processorCount - 1;

  return numThreads;
}

void
launchThreads(const ThreadingInfo& info, PPM& canvas)
{
  int width = info.width;
  int height = info.height;

  World world = makeWorld();
  Camera camera = makeCamera(info.aspectRatio);

  int numThreads = getNumThreads();
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
                        .pixelSamples = info.pixelSamples };

    threads[i] = std::thread(
      scan,
      // We're sharing memory, but this is safe because each thread has
      // a predetermined slice that doesn't overlap with the others
      std::ref(canvas),
      // Sharing the world and the camera is OK since they're read only
      std::cref(world),
      std::cref(camera),
      scanner);

    // We're arbitrarily giving the first thread more work
    // so as far as the other threads are aware,
    // there is no remainder
    y0 += step + rem;
    rem = 0;
  }

  for (int i = 0; i < numThreads; ++i) {
    threads[i].join();
  }
}