Released android-activity 0.2

Android-activity is a new "glue" layer for building native Rust application on Android
supporting multiple `Activity` base classes (`NativeActivity` and `GameActivity`)

This crate has evolved over a number of experimental iterations initially looking
at how to build Rust applications using the `GameActivity` class
from the Android Game Developer Kit - which solves a number of limitations with using
`NativeActivity`.

I originally needed this while working on [Bluey UI](https://github.com/rib/bluey/tree/main/bluey-ui)
which was an Egui-base UI for testing my cross-platform Bluetooth library Bluey.
For this I needed to work with a custom Activity subclass, and was also looking to
be based on `AndroidAppCompat`


I think the API is at a point now where it'd be good to have more people experimenting
with this glue layer
I think this is probably the first release that