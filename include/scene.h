#ifndef SCENE_H
#define SCENE_H

#include "color.h"
#include "ray.h"
#include "world.h"

World
makeWorld();

Color
getColor(const Ray& ray, const World& world);

#endif