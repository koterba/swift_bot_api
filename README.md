# SwiftBot Rust Library

This is the library for controlling the SwiftBot.


## Getting Started

To start coding your SwiftBot, type ``cargo add swift_bot_api`` add the following lines of code to the start of your program.
```rust
use swift_bot_api::*;
```
You will also have to create an instance of the SwiftBot, like this:
```rust
let mut bot = SwiftBot::new().expect("Unable to initialise SwiftBot");
```

## Buttons

SwiftBot has four buttons to its rear, labelled A, B, X, and Y.
These can be read using the `is_pressed()` function, which accepts the ``Button`` enum that consists of the following varients:

* `Button::A`
* `Button::B`
* `Button::X`
* `Button::Y`

For example, to read the A button you would write:

```rust
if is_pressed(Button::A) {
    // perform action
}
```


## Button LEDs

Next to each button on the SwiftBot is a white LED. These can be controlled using the `set_button_light()` function, which also accepts the ``Button`` enum like the ``is_pressed()`` method above, followed by a number between `0.0` and `1.0`.

For example, to set the button B light to half brightness you would write:

```rust
bot.set_button_light(Button::B, 0.5);
```

To turn the light off you would set the value to ``0.0``:

```rust
bot.set_button_light(Button::B, 0.0);
```


## Motors

The SwiftBot features two motors with independent control, enabling [differential steering](https://en.wikipedia.org/wiki/Differential_steering), whereby the speed of one motor can be controlled separately of the other.

There are several ways these motors can be commanded from code:

### Simple Movements
```rust
bot.forward()
bot.backward()
```

The above functions will drive SwiftBot at full speed.

To stop SwiftBot from moving, simply call `bot.stop()`. This will make the robot stop sharply.

### Advanced Movements

To get more control over SwiftBot's movements, each motor can be individually controlled using `set_motor_speed()`. This takes the ``Motor`` enum and a number between `-1.0` and `1.0`, where positive values will drive the motor forward and negative values will drive the motor backward. The below example will have Swiftbot curving slowly to the right.

```rust
bot.set_motor_speed(Motor::Left, 1.0);
bot.set_motor_speed(Motor::Right, 0.5);
```


## Underlighting
The SwiftBot also has a six-zone RGB underlighting.

### Set colour

```rust
bot.set_underlight(255, 0, 0) // red
bot.show_underlight() // make the lights show
```


### Disable lights

There may be the case where you want to turn off the underlights, for example to save power, but have them remember what colour you last set when turned back on. For this the `bot.clear_underlight()` can be used.



## Distance Sensor

SwiftBot features a front mounted ultrasonic distance sensor. This sensor can be read using `bot.distance()`, which will return a measured distance in centrimetres, like so:
```rust
let distance = bot.distance();
println!("Distance: {}cm", distance);
```
