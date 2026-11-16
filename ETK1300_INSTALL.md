# ETK 1300 856tx AWD — Installation

1. Copy **etk1300_model_pack.zip** unchanged into the game's `mods` folder.
2. Replace `main.zip`, `level.zip`, `ogre_bridge.zip` and `ogre_bridge.cpp` with the updated files from this repository.
3. Rebuild the Rust client and the OGRE Next 3.0 bridge.
4. Open the vehicle selector and choose **ETK 1300 856tx AWD**.

The model pack contains `etk1300_856tx_A.glb` with 17 embedded PNG images. Twenty-five of the 66 glTF materials bind base-colour textures, including the original ETK interior, glass, gauge, engine and lighting maps; subtle bundled maps provide seat upholstery, tyre rubber and alloy-wheel finish where the source archive has no dedicated diffuse image. The GLB is a full-detail conversion of the supplied 270-node Collada source: 470,656 vertices and 158,322 triangles before runtime batching.

The selected source configuration is `etk856tx_A`, listed in the supplied mod as AWD with a 3.0L I6, 247 kW, 320 Nm and 1,640 kg. The APEX drivetrain uses the corresponding 8-speed gear ratios and a 3.23 final-drive reference.

**Simulation boundary:** this vehicle uses APEX.drive's existing soft-body chassis, Pacejka tire forces, suspension, 4-wheel AWD torque distribution, automatic gearbox, ABS, traction control, brake/tire thermal model and aerodynamic model. It does not yet run BeamNG's original Lua controllers or reproduce every JBeam node/beam, active differential, ESC/DSE mode, sound controller or exact deformable flexbody behavior. The uploaded ZIP contains references to common BeamNG/ETK800 resources, so its complete original BeamNG configuration is not self-contained. The GLB rendering asset is self-contained; the game-side physics integration is the native APEX model described above.
