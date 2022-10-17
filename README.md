# organic-roguelike

The goal of this project is to build a simple roguelike game with organic dungeon layout.

This means I will not be using square or rectangular rooms, or even a grid based model as found in many other games in the genre:
I instead aim to build a layout that feels more organic, more tightly packed, with room of very diverse shapes and sizes.

## Inspiration

I will be basing my approach on this previous work by Joel Simon (https://www.joelsimon.net/evo_floorplans.html) which produces results like this :
![Optimized school](https://www.joelsimon.net/imgs/evo_plans/results_bottom.jpeg)

Of course this approach is built for school floors and optimizing flux so I will have to adapt it.

## Plan

- [ ] Drawing arbitrary polygons with bevy
- [ ] Implementation of Fortune algorithm for Voronoi diagrams
- [ ] Genetic algorithm for organic level generation
- [ ] Making the actual game
