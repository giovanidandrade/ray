#ifndef SCENE_H
#define SCENE_H

#include "bvh.h"
#include "camera.h"
#include "color.h"
#include "ppm.h"
#include "ray.h"

struct Scanner
{
  int id;
  int width;
  int height;
  int y0;
  int y1;
  int pixelSamples;
};

struct SceneInfo
{
  int width;
  int height;
  int pixelSamples;

  float aspectRatio;
  BVH world;
  Camera camera;

  PPM canvas;
};

SceneInfo
makeSceneInfo();

void
scan(PPM& canvas, const BVH& world, const Camera& camera, Scanner scanner);

#endif