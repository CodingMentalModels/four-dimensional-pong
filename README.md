# Four Dimensional Pong

Pong, but in Four Dimensions


## Motivation

Understand what it means when we say there's a n-th dimension.

Answer questions like:
* How can time be a dimension?
* How can there be more than 3 dimensions?  Four?  9?  10?  11?
* How many dimensions are there in our universe?
* What is a dimension?
* How can we visualize a fourth dimension?
* How can we add a fourth (spatial) dimension to Pong?


### Answers

* What is a dimension?  A dimension is an independent number that you need to specify in order to tell me something.  
* How can time be a dimension?  Well, it's just another number.  But it's not **the** fourth dimension.   There could easily be others, it depends on the system.  
* How can there be more than 3 dimensions?  Four?  9?  10?  11?  Well, dimensions are just the number of numbers you need to specify for a system, so just specify more.
* How many dimensions are there in our universe?  Well, it depends on which theories of physics you subscribe to, but probably at least four.
* How can we visualize a fourth dimension?  Probably you can't, at least not in the way you'd want to.  Our brains are evolutionary programmed for three dimensions.  
    * Note you can do it by imagining that time is another spatial dimension and stipulating that e.g. 1m == 1s.  
    * You can also do it color!
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

### Entity Component System (ECS) Implementation

Bevy Game Engine

#### The Problem: Nasty Inheritance Hierarchies

* Game Object
    * Character
        * Mario
        * Goomba
        * Yoshi
    * MapElement
        * Platform
* Mario
    * Walk
    * Jump
    * Collide
    * Input
* Goomba
    * Walk
    * Jump
    * Collide
    * AI
* Yoshi
    * Walk
    * Jump
    * Input
    * Collide
    * AI
* Platform
    * AI
    * Input
    * Collide


#### The Solution: ECS Overview
* Entity -- A container for components
    * A unique entity within your world, e.g. Mario
    * Doesn't have any properties itself
* Components -- A bit of data that's attached to one or more entities
    * e.g. Health, Position, IsColliding
* Systems -- Logic that queries all entities with certain components and mutates them
    * e.g. Collision Logic, AI, Input Handling
    * DamageSystem might query for entities with Health, Collision and then see if they've collided and deduct health accordingly.

#### Entities
* Ball
* Paddles
* Goals
* Wall


#### Components
* Tags for each entity, e.g.
    * Ball
    * Paddle
    * Goal
    * Wall
* Position(w, x, y, z)
* Velocity(w, x, y, z)
* IsCollidable
* Score


#### Systems
* Game
    * UserInputSystem: Keypresses, Mouse -> Position of Blue Paddle
    * OpponentAISystem: Position of Ball, Velocity of Ball -> Position of Red Paddle
    * BallMovementSystem: Position of Ball, Velocity of Ball -> Position
    * CollisionSystem: Position of Ball, Position of Y -> Velocity of Ball, Score Events
    * ScoreSystem: Score -> Updating the UI
    * RenderSystem: Positions -> Color of Ball, Transforms in Scene
* Pause
* Gameover / Victory


## Design Questions / Decisions
* AssetServer for the Blender Scene -- Currently doing the easy, simple thing (asset_server.load) but might want to use the more thorough approach later.
* How can we use asset_server to load or create sub-entities from the glb?


## Mistakes

