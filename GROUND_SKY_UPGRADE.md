# APEX.drive Sky, Sand and Road Correction

- Cloud UVs now follow viewing direction on an equirectangular sky map. This removes the fixed-altitude projection responsible for radial streaks near the horizon.
- 2K cumulus texture uses warped multi-scale billows, edge erosion and internal light/shadow variation. Cirrus is thinner and less opaque.
- Sand no longer uses directional sine waves. It uses isotropic multi-scale grain with lower normal amplitude and reduced tiling.
- Road asphalt also uses isotropic fine aggregate and a separate, subtle road-normal family.
- Water rendering, vehicle grip, rolling resistance, suspension and sunrise/sunset curves are unchanged.

Source/archive structure was checked. A local Windows/MSVC OGRE build and in-game visual test are still required.
