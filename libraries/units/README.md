IROX-Units
============
*The little Units Library that could*

Module Structure:
------------------

* [`bounds`](src/bounds.rs) - Bounding Boxes and Range Checks
* [`shapes`](src/shapes/) - Ways to define and describe shapes
    * [`circular`](src/shapes/circular.rs) - `CircularAspect` enum and `CircularDimension` struct, describes a circle by radius or diameter with appropriate length units.
    * [`elliptical`](src/shapes/elliptical.rs) - `Ellipse` struct, describes an ellipse using two `CircularDimension` axes and an optional `CompassDirection` orientation of the first axis
* [`units`](src/units) - Physical Quantities
  * [`angle`](src/units/angle.rs) -  Angle Types, `Angle`, `AngleUnits` for `Degrees` and `Radians`
  * [`compass`](src/units/compass.rs) - Compass Types, 
    * `Compass`, and the absolute types: `Heading`, `Track`, `Bearing`, `Course`, `Azimuth`,
    * `CompassOffest`, and the relative types: `RelativeBearing`
  * [`datasize`](src/units/datasize.rs) - Computer Data Sizes, `Bytes`, `Kilobytes`, etc
  * [`duration`](src/units/duration.rs) - Contains `Duration`, describing a fixed amount of seconds / minutes / hours / milliseconds / etc
  * [`length`](src/units/length.rs) - The SI `Length` quantity, `Meters`, `Feet`, etc
  * [`speed`](src/units/speed.rs) - The SI `Speed` quantity, `MetersPerSecond`, `Knots`, etc
  * [`temperature`](src/units/temperature.rs) - The SI `Temperature` quantity, `Celcius`, `Kelvin`, etc