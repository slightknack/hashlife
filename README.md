# Hashlife, but in Rust
> Otherwise known as *Gosper's Algorithm*
> (but in Rust).
>
> Also WIP.

Hashlife is a fairly complex yet efficient algorithm used for accelerating
cellular automata.
This implementation is pretty basic as it only covers Conway's game of life on
an infinite 2d grid.

# Is it any good?
Well yes, but actually no.

## What do you mean?
This is a fairly fast and efficient implementation.
However, it sucks on three fronts:

1. It's rigid — hardcoded and optimized.
2. The code isn't modular — it's repetitive.
   I would use macros, but it's not really worth it to me.
4. It hasn't been tested.
   I implemented it this afternoon.
   It's correct, but only if you take my word for it.
3. There's no UI.

The last one is partially a joke.
I plan to implement a TUI eventually, but I haven't gotten around to it yet.
For now, it's just the core algorithm.

# What is Hashlife?
If you haven't already, Google it.
Many other people have written better explanations that I ever could've.
In case you're wondering, this implementation
is loosely based on [this post](http://jennyhasahat.github.io/hashlife.html).

## Macrocells
Hashlife recursively represents grids of cells as directed acyclic graphs of
immutable macrocells.
Each macrocell functions as a quadtree pointing to four sub-cells.
Additionally, each macrocell points to a center cell containing the
result of that cell after n steps, where n is half the size of the cell.

## Basecells
Basically, a base cell contains a small quantifiable region of grid.
This is where the actual calculation takes place.
The Basecells are 4 by 4; in this program they're
calculated in groups of 8 by 8, with a minimum step size of 2.

## Hashing
To speed up calculation, cells are cached.
When a new cell is constructed, if it already exists, a reference to the
previously constructed cell is returned.
This cell already contains the calculated result sub-cell,
which saves computation.

**In essence, Hashlife is an algorithm that spatially memoizes a temporally
directed acyclic graph emulating a quadtree.**

Hashlife is amazing, but it's not that fun.
If you'd like to try something fast *and* fun,
try [implementing CGOL using comonads](https://samtay.github.io/posts/comonadic-game-of-life.html).

# Contributing
Don't — If you do — good luck.
It'd probably be a better exercise to just to implement your own version
from scratch.

---

**Sincerely,  
Isaac Clayton (`@slightknack`)**

<img height="128px" src="https://i2.kym-cdn.com/entries/icons/original/000/020/478/088.png">
</img>

*Certified Internet Meme Master*  
