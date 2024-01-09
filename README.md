## Story

In a very hypothetical continuous-flow chemical plant, there
are two kinds of `Unit`s: `Heater`s, which run endothermic
reactions and add heat as needed, and `Cooler`s, which run
exothermic reactions and remove heat as needed.

Each `Unit` has a `nominal` temperature at which its
reaction will run if no heat is added or removed. A `Cooler`
has a `min` temperature it can cool the reaction to. A
`Heater` has a `max` temperature it can heat the reactor to.

The plant is intended to run all the reactions at the same
temperature. The initial job of the `heatflow` controller is
to determine a legal running temperature.

For example, consider this sequence of reactions:

    * Heater: nominal = 20, max = 40
    * Cooler: nominal = 25, min = 15
    * Heater: nominal = 17, max = 100

The setpoint of this system could be anwhere between 20 and
25.

Note that some reactions could be impossible.

## Assignment

Fill in the template at
<https://MasseyRustTraining/heatflow-template> such that the
given tests pass. Add a few more tests.

## Bonus Assignment

Assume that the energy cost of the system is sum, over each
`Unit`, of the square of the difference of that `Unit` from
the `nominal` temperature. Adjust `find_setpoint()` to
return a minimum energy cost setpoint.

You can do this by trying all possible setpoints as long as
there aren't hugely many. If there are, the problem becomes
much more challenging, requiring a least-squares
minimization.
