# APEX.drive Ground & Sky Correction

## Corrected from the in-game screenshots
- Removed the fixed-altitude ground-plane projection for cloud UVs. It caused the long radial streaks and excessive repetition near the horizon.
- Clouds now use direction-based equirectangular UVs, with separate scales and slow drift for cumulus and cirrus.
- Reworked the 2K cumulus mask with multi-scale domain warping, rounded billow lobes, edge erosion, and baked internal light/shadow variation. Cirrus is now a thinner, less opaque filament layer.
- Removed directional sine waves from the sand normal map. The old wave terms were the source of ruler-straight beach stripes.
- Sand now uses isotropic multi-scale grain and a much lower normal amplitude/tile multiplier.
- Road asphalt retains its own surface family and does not use the sand normal family.

## Preserved
- Water code and compositor are unchanged.
- Vehicle grip, rolling resistance and suspension are unchanged.
- Sunrise/sunset colour curves are unchanged.

## Validation
Source-level changes and archive structure are checked. A local Windows/MSVC OGRE build and in-game screenshot test are still required; no claim of verified runtime quality is made.
