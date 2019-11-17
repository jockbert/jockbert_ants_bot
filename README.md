# Jockbert ants bot

Bot in the game [Ants AI Challenge](http://ants.aichallenge.org/).



## Rough architecture

![rough architecture](doc/rough_architecture.png)


## Future improvements roadmap
1. Fix food planning bug: Take the nearest food instead of the one first in the food-list.
1. Remember food/hills/opponents etc. in fog of war.
1. Reuse old search paths if no new obstacle are in the way.
1. Implement ant-on-ant-battle strategy.
1. Iterative search length cutoff, first iteration using Manhattan. End before timeout.
1. Extract to run strategies on other thread.
1. Parallelize the search effort - use multiple threads.


## Changes up to 2019-11-17
* "Multiple A*" in search when finding food. For single search start position, reuses partial results for next search target.
* Add strategy SpreadOutScout, taking precedence over RandomWalk. It uses a low resolusion grid spreading over he entire map, as ant targets.
* Add strategy HillRaiser, using enemy hills as ant targets.

```
        Current strategy stack
        1. HillRaiser
        2. GatherFood
        3. SpreadOutScout
        4. RandomWalk
```

## Older changes up to 2019-10-03

* Add Breadth-first-search for finding parths from one start position and several targets.
* Add RandomWalk strategy.
* Add GatherFood strategy.
* Add Searck length cut off limit in search.
* Add AvoidWater filter when enumerating possible ant orders.
