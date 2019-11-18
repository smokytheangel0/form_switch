# form_switch
a complicated script to manage two touchscreens where one is convertible

Please build three binaries from this source changing the switch variable for each condition.
Feel free to change the direction variable set for each switch condition, mine is set
to a rather unconventional portrait for the monitor (right), and I have also not implemented the
inverted direction, but the binary whatever that xinput takes to scale the touch area is there for it.

what you do with those binaries is for you to discover, but I have them bound to two keys, and an icon.

//TODO
add consistent pointer slaving options, as it stands, the one touchscreens pointer is slaved to the master and one not
which one is which depends on Circumstances.
https://unix.stackexchange.com/questions/129339/touchscreen-and-mouse-as-separate-inputs

I would enjoy adding this next to make this more predictable, I enjoy having the monitor pointer unslaved, and the
laptop slaved, but I might add options to make all three combinations possible.

The reason for slaving the touchscreens at all is because my desktop environment doesnt take input from unslaved pointers
though many programs do, otherwise I would slave only the touchpad to the pointer, and keep the touchscreens free as seems
to be the case in windows 10.
