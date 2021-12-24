#include "parallel.h"
#include "scene.h"

int
main()
{
  SceneInfo info = makeSceneInfo();
  launchThreads(info);

  info.canvas.dump();
  return 0;
}