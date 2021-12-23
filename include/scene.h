#ifndef SCENE_H
#define SCENE_H

#include "camera.h"
#include "color.h"
#include "ppm.h"
#include "ray.h"
#include "world.h"

struct Scanner
{
  int id;
  int width;
  int height;
  int y0;
  int y1;
  int pixelSamples;
};

World
makeWorld();

void
scan(PPM& canvas, const World& world, const Camera& camera, Scanner scanner);

#endif