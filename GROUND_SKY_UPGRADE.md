# APEX.drive Ground & Sky Surface Upgrade

## Scope
The current upgrade targets the OGRE Next 3.0 bridge and the Rust terrain meshes.

- New 2K alpha-textured cumulus and cirrus layers are drawn by OGRE Unlit on separate sky queues.
- The established atmospheric gradient, sun, stars, moon and sunrise/sunset colour functions are kept.
- PBS detail-normal maps are generated for sand, dirt/gravel, grass terrain, asphalt roads and grass blades.
- Terrain albedo remains the baked 4K colour map; detail normals provide microstructure and roughness response.
- Surface ID 4 identifies sand. It reuses dirt/gravel grip and rolling resistance, so vehicle physics values stay unchanged.
- Water rendering is intentionally untouched; `water.rs`, the water compositor and reflection path are preserved.

## Files
- `ogre_bridge.cpp`: sky texture overlays and PBS ground surface families.
- `level.zip`: terrain layer batching, beach classification, grass alpha texture, and sand surface mapping.
- `tools/bake_west_coast.py`, `tools/bake_roads.py`: preserve sand as surface ID 4 when baking maps.

## Install
Replace the bridge archive and level archive, then rebuild both the C++ bridge and Rust client using the same OGRE Next 3.0 SDK/toolchain. When re-baking West Coast, use the updated scripts.

The code/archive structure has been checked. No MSVC build or in-game screenshot test was available, so report any compile output before treating the upgrade as final.

The cloud layers are textured GPU sky overlays, not volumetric ray-marched clouds. The ground maps add procedural PBR normal detail; authored material/albedo packs can be layered in a later pass.
