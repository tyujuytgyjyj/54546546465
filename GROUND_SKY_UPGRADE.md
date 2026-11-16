# APEX.drive Wind, Anti-Aliasing, Fence and Sky Fix

- The camera-centred equirectangular cloud UV mapping is unwrapped to remove the 0/1 seam. Cloud masks fade near the zenith to avoid a pole discontinuity.
- 4x MSAA is enabled by default; `--msaa8` requests 8x, while `--no-msaa` disables it. OGRE falls back to no MSAA if the requested mode cannot be created.
- The bridge uploads a full CPU-generated mip chain for ordinary textures, cloud masks and normal maps.
- Fence alpha is preserved as an explicit cutout material tag from `bake_props.py` through `props.rs`, with a dense runtime alpha-sampling fallback.
- Trees, palms, shrubs and plants get height-weighted sway. Grass cards sway in the coastal wind; the same wind changes Gerstner wave direction/speed/strength and advects broken shoreline foam.
- Water reflection rendering, tyre grip, rolling resistance, suspension and sunrise/sunset curves are retained.

The wind model is a lightweight visual field, not a full CFD solver or vehicle wind-force simulation. Source/archive structure has been checked; local MSVC/OGRE compilation and in-game visual tests remain required.
