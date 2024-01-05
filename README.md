This is an example project showcasing how to control the speed of a bevy rapier physics simulation.

![image](https://github.com/rlidwka/bevy-rapier-sim-time/assets/999113/bed9e0c4-e35e-4732-8c72-63cd0b79bffb)

With the provided interface, user can:

 - pause the simulation
 - run the simulation step-by-step
 - fast-forward the simulation with maximum possible speed
 - restart the entire simulation from the beginning

In order to do that, I created `PhysicsSchedule` (direct equivalent of `FixedUpdate`) and `PhysicsTime` (direct equivalent of `Time<Fixed>`), which I can pause or run whenever is necessary.
