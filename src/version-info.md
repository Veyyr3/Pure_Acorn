Hi!

Version of ACORN-way is 0.2.0

What's new:
* Instead of nothing to setup Acorn now we have `acorn_zsetup` (where you register functions) and `acorn_gsetup` (where you add new fields for all functions into AcornGlobalContext).
* Acorn Functions have 2 arguments: World (from bevy_ecs) and GlobalContext to use global variables (ex: score, player inventory etc.)
* Deleted Bevy schedule because it's useless.