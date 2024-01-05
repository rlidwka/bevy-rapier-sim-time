This is an example project showcasing how to control the speed of a bevy rapier physics simulation.

With the provided interface, user can:

 - pause the simulation
 - run the simulation step-by-step
 - fast-forward the simulation with maximum possible speed
 - restart the entire simulation from the beginning

In order to do that, I created `PhysicsSchedule` (direct equivalent of `FixedUpdate`) and `PhysicsTime` (direct equivalent of `Time<Fixed>`), which I can pause or run whenever is necessary.
