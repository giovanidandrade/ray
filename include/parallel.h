#ifndef PARALLEL_H
#define PARALLEL_H

#include "camera.h"
#include "ppm.h"

struct ThreadingInfo
{
  int width;
  int height;
  int pixelSamples;

  Camera camera;
};

// Changes canvas
void
launchThreads(const ThreadingInfo& info, PPM& canvas);

#endif