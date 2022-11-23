# Four Dimensional Pong

Pong, but in Four Dimensions

Four Dimensional Pong was coded on [my stream](https://twitch.tv/codingmentalmodels) where I use coding projects to learn and teach mental models from life, math, physics, and other subjects. Follow me for more to come!

## Motivation

Famously, special relativity introduces time as a fourth dimension, but higher dimensional phenomena are all around us.  Even in classical mechanics, if you have multiple particles, you need more than three dimensions.  String Theory might be 9 or 10 or 11 dimensional.  In machine learning, 10s, 100s or even higher dimensional data is common.  Hilbert Space, the setting of Quantum Mechanics, is often considered to be infinite dimensional.  

But frequently when students confront these topics or ideas for the first time, they get stuck.  They ask questions like:
* How can time be a dimension?
* How can there be more than 3 dimensions?  4?  9?  10?  11?  Infinite?
* How many dimensions are there in our universe?
* What is a dimension?
* How can we visualize a fourth dimension?
* How can we add a fourth (spatial) dimension to Pong?

Four Dimensional Pong is meant to be a simple example of a more-than-three dimensional system that can be visualized and used to help understand higher dimensions.  

### Answers

Roughly speaking, these are the answers to the questions above, in a slightly more pedogogical order.

* What is a dimension?  A dimension is an independent number that you need to specify in order to tell me something.  
* How can time be a dimension?  Well, it's just another number.  But it's not **the** fourth dimension.   There could easily be others, it depends on the system.  
* How can there be more than 3 dimensions?  4?  9?  10?  11?  Infinite?  Well, dimensions are just the number of numbers you need to specify for a system, so just specify more.
* How many dimensions are there in our universe?  Well, it depends on which theories of physics you subscribe to, but probably at least four given that special relativity is a fundamental framework for all of the other plausible theories of physics we have.
* How can we visualize a fourth dimension?  Probably you can't, at least not in the way you'd want to.  Our brains are evolutionarily programmed for three dimensions.  
    * Note you can do it by imagining that time is another spatial dimension and stipulating a way to equate time and space, e.g. 1m == 1s.  
    * You can also do it using color!

Given those answers, 
* How can we add a fourth (spatial) dimension to Pong?  
    * What are the objects?
        * Arena
            * 2d Pong: Rectangular Arena
            * 3d Pong: Rectangular Prism Area (Extruded Cube)
            * 4d Pong: 4-dimensional Rectangular Prism (i.e. 4th dimensional cube extruded along one dimension)
        * Paddles
            * 2d Pong: Lines
            * 3d Pong: Planes
            * 4d Pong: 3D-Cubes
        * Ball
            * 2d Pong: Point or a Circle
            * 3d Pong: Point or a Sphere
            * 4d Pong: Point or a 3d-Sphere
        * Goals
            * 2d Pong: Lines
            * 3d Pong: Planes
            * 4d Pong: 3D-Cubes
        * Ball Trajectories
            * 2d Pong: Angled Lines that always go lengthwise (left-right)
            * 3d Pong: Angled Lines that always go lengthwise (forward-backward)
            * 4d Pong: Angle Lines that always go lengthwise
    * How can we represent this, given that we need the time dimension?  
        * Use Color for the "long dimension"!  Blue for the player, red for the opponent.
        * The game is played in a cube of 3 spatial dimensions, with one blue paddle and one red that can't touch each other.  The ball bounces around the cube becoming redder and redder until it's the same as the paddle at which point it can be hit.  Then it beomes bluer and bluer, etc. until someone misses and the opponent gains points.

## Design

### Overview

Implement Pong, but in four spatial dimensions, using color to define the "w" direction.  

### Definitions

* Four spatial dimensions, (w, x, y, z).  
    * x, y, and z will be rendered as x, y, z
    * w will be rendered as color, from Blue to Red.  
* Player 1 == Blue, Player 2 == Red
* Controls:
    * Mouse for moving in the x, y plane
    * W & S or Up and Down for moving in the z direction (towards, away)
* Camera Angle: Slightly back, rotated upwards to have perspective along the z axis.


## Mistakes

I like to log mistakes that I make while working on a project.  It keeps me honest and helps me identify how I can get better.  Here's that log.  I'm sure there were others.

* We spent a long time trying to debug why asset loading had started to fail:
    * But we hadn't written a minimal test to narrow down the issue.  This took 5 minutes and would have led to earlier progress.
    * We didn't recognize that if `load` doesn't block and there's nowhere where we're joining back the results of the asset loading thread (which clearly there isn't since we have no access to it), then we can't expect the state to change within our load function.  
    * We also didn't think through why the code example wouldn't have an `else` case when failing to find the asset in `Assets`.  
* We misunderstood how `iyes_loopless` states were instantiated and spent some time with two versions of those states, leading to confusing error messages.
    * Noticed that `NextState` was being used and thought about what that could be.
    * Bit the bullet on loopless states not being explicitly instantiated and tried to work out how that could be.