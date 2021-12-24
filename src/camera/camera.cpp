#include "camera.h"
#include "rand.h"
#include <cmath>

Camera::Camera(Point lookFrom,
               Point lookAt,
               Vec viewUp,
               float fov,
               float aspectRatio,
               float aperture,
               float focusDistance)
{
  float theta = fov * M_PI / 180;
  float fovHeight = tan(theta / 2);

  float viewportHeight = 2 * fovHeight;
  float viewportWidth = aspectRatio * viewportHeight;

  this->lookVector = (lookFrom - lookAt).normalize();
  this->unitHorizontal = viewUp.cross(lookVector).normalize();
  this->unitVertical = lookVector.cross(unitHorizontal);

  this->origin = lookFrom;
  this->horizontal = focusDistance * viewportWidth * unitHorizontal;
  this->vertical = focusDistance * viewportHeight * unitVertical;
  this->lowerLeftCorner =
    origin - horizontal / 2.0 - vertical / 2.0 - focusDistance * lookVector;

  this->lensRadius = aperture / 2.0;
}

Ray
Camera::cast(float s, float t) const
{
  Vec random = lensRadius * randomInUnitDisk();
  Vec offset = unitHorizontal * random.x + unitVertical * random.y;

  return Ray(origin + offset,
             lowerLeftCorner + s * horizontal + t * vertical - origin - offset);
}