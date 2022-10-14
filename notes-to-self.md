# Most Importantly
*get started already! stop being so freakin afraid!*

## 2D arrays
2D arrays are initialized with nested brackets, like this:

```rust
let mut onlyGoodBattleRoyale:[[str;2];3] = [["99";2];3];
```
outputs something like
```
[["99","99"],
 ["99","99"],
 ["99","99"]]
```

*NOTE:* inner = columns, outer = rows

Therefore, to create the game grid...

```rust
let mut grid:[[i32; 10]; 40] = [[0; 10]; 40];
```

_Yes, the grid is 40 blocks tall in Guideline T*tris. This is because of garbage lines._

The 2D arrays themselves are accessed the same way as in Java.
```rust
onlyGoodBattleRoyale[1][1] = "Crab Game"
```
results in
```
[["99","99"],
 ["99","Crab Game"],
 ["99","99"]]
```

Yes, I think Crab Game is a battle royale. Don't @ me.

## Tetrominos
Did you know that filling a rectangle with one set of tetrominoes is impossible?

An idea is to do something similar to Tetrjs, but store rotations as well. Can never have enough bloat.

For example, the T piece would be something like:
```
000  040  000  040
444  044  040  440
040  040  444  040
```

Replacing the 0s with .s to make it clearer:
```
...  .4.  ...  .4.
444  .44  .4.  44.
.4.  .4.  444  .4.
```

Yes, it's ARS! (Arika Rotation System)

This may also be possible by coding specific offsets for specific rotations...

*WOULD-BE-COOL:* Symmetrical ARS. I don't like that there's a right side bias. Yes there is a right side bias in TGM.
_DTET already does something similar, but I'd prefer rotation button decide the horiz offset than rotation state._
