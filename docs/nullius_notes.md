# Nullius reference notes

Notes on the Factorio mod [Nullius](https://mods.factorio.com/mod/nullius), collected as a pattern for this game's resource set and progression.

Source: [github.com/GregorSamsanite/nullius](https://github.com/GregorSamsanite/nullius), files under `nullius/prototypes/` (`technology.lua`, `checkpoint.lua`, `resource.lua`, `item/recipe.lua`, `item/intermediate.lua`).

Caveat: the data was extracted with a regex parser, not Factorio's Lua loader. Details are likely right but unverified against the in-game tech tree. The parsed output is in `docs/nullius_parsed.json` (516 techs, 238 recipes, dependency depth per tech).

## Raw resources

| Resource | Notes |
|---|---|
| Iron ore | Autoplaced. |
| Bauxite | Autoplaced. Mining time 0.8. Source of aluminum. |
| Sandstone | Autoplaced. Mining time 1. Source of silicon, trace titanium. |
| Limestone | Mining time 1. Source of calcium, trace sulfur. (The mod page also says "calcite"; the source uses `nullius-limestone`.) |
| Fumarole | Infinite fluid vent, 10 volcanic gas per mine, with depletion. |
| Geothermal | Separate map-gen control. |
| Air, seawater | Fluid inputs via separation / filtration techs. |
| Copper | Not a map resource. Gated behind `copper-production` (7,000 units), which needs `checkpoint-copper-ore` <- `asteroid-mining-1` <- `astronomy`. |

No coal, oil, or wood.

## Research structure

1. **Checkpoints** (107 zero-pack techs, e.g. `checkpoint-iron-ingot`) gate the next real tech. Descriptions read like "produce 16 iron ingots", so they appear to require demonstrating a material first. The exact unlock trigger was not confirmed.
2. **Science packs chain**, each requiring the previous:
   geology (start) -> climatology -> mechanical (depth 9) -> electrical (24) -> chemical (40) -> physics (61) -> astronomy (85).
   A separate biology branch uses biochemistry, microbiology, botany, nematology, dendrology, ichthyology and zoology packs.
3. Late techs cost hundreds of thousands to millions of units (e.g. `solar-power-4` 2M, `cybernetics-7` 3M).
4. Longest prerequisite chain is about 108 techs deep.

## Early-game chain (depth 0 to 9)

| Depth | Tech | Unlocks / notes |
|---|---|---|
| 0 | `salvage-lab-wreckage` | Triggered by mining the crashed lab (`mine-entity nullius-landing-lab`). |
| 1 | `geology-1` (5 units, no packs) | Geology pack: 4 bauxite + 4 sandstone + 4 iron ore. |
| 2 | `iron-smelting-1` | Iron ingot: 5 iron ore -> 2 ingot + 1 gravel. |
| 2 | `climatology-1` | Climatology pack. |
| 3 | `iron-working-1` | Iron plate: 4 ingot -> 3 plate. Iron rod: 4 ingot -> 5 rod. Iron sheet: 5 plate -> 6 sheet. |
| 3 | `plumbing-1`, `water-filtration-1` | Pipes; seawater filtration. |
| 4 | `air-separation-1`, `electrolysis-1` | Air separation; saline electrolysis. |
| 4 | `iron-working-2` (needs `checkpoint-iron-ingot`) | Iron wire (3 rod -> 4); iron gear (2 plate + 1 rod -> 2). |
| 4 | `masonry-1`, `mining-tools` | Sand and gravel recipes; first mining tool. |
| 5 | `masonry-2` | Block recipes (below). |
| 5 | `carbon-sequestration-1`, `plumbing-2`, `fabrication-tools` | |
| 6 | `organic-chemistry-1`, `carbon-sequestration-2`, `electromagnetism-1` | Plastic; graphite and CO to CO2; motor. |
| 7 | `metallurgy-1`, `filtration-1` | Small furnace; filters. |
| 9 | `mechanical-engineering-1` | Mechanical pack: 1 motor + 3 iron gear. |

### Masonry-2 block recipes (main product plus byproduct)

- Stone block: 2 stone -> 5 brick + 1 gravel
- Sandstone block: 3 sandstone -> 10 brick + 1 sand
- Limestone block: 4 limestone -> 12 brick + 1 crushed limestone

## Mid to late game

- Electrical pack (depth 24): decider combinator, lamp, 2 copper cable, capacitor. Copper is not available that early, so the real recipe at that stage probably differs from what the parse caught.
- Chemical pack (depth 40) from `experimental-chemistry`.
- Physics pack (depth 61): needs the `physics` tech and 8 intermediates (lab, stirling engine, nanofabricator, etc.).
- Volcanic gas checkpoint (33), geothermal power (36), astronomy (85, 5,000 units), asteroid mining (86 to 100).
- Packaging/boxing techs (49 to 56) add "boxed" bulk-logistics recipes; a Factorio-specific mechanic.

## Ideas for this game

- **Byproduct outputs:** nearly every conversion yields a main product plus a byproduct. `TileResource` could carry a main output and an optional byproduct.
- **Gating by demonstration:** unlock a resource in `unlocked_resources` the first time a prerequisite is collected or crafted, like checkpoints.
- **Pack-style progression:** energy could be the first research currency, with each resource tier unlocking the next pack.
- **Payoff-gated resources:** copper is absent until the late game and forces the asteroid-mining chain.
