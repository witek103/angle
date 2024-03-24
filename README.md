# `angle`

Provides convenient way to store orientation as Angle in `no_std` environment.
Accepts both degrees and radians.
Takes care of basic operations (addition, subtraction, cos, sin) and makes sure
angle value is always normalized (stays within `(-180.0, +180.0)`).
`is_within()` can be used to check if two angles are near each other with given
accuracy.
Implements `Display` with feature `use_std` enabled.
