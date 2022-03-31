This is just a simple program to detect duplicate archetypes in FiveM which causes the crash `asparagus-artist-bluebird`

The usage here is simple, using CodeWalkers RPF Explorer search for .ytyp (with no mods or anything else in the GTA V folder)

Select All -> Export XML -> export to data/default_maps

After that close CodeWalker, make a folder called "A" (just because it shows at the top) and put the folder(s) that have your map into this folder

After all of that reopen CodeWalker RPF Explorer, search for .ytyp and Select the .ytyp that have the "A" path and export them to "maps"

After that run "archetypechecker.exe" or do "cargo run" if you have rust installed

It will output a list of all of the maps that have an overlapping CMloArchetypeDef, you will have to delete these or fix them (don't ask me how, but I would assume removing duplicate defs :P)

Most of these will be exact copies of the .ytyp from GTA 5 so deleting them wont affect anything.