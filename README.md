# SwiftBot Rust Library

This is the library for controlling the SwiftBot.


## Getting Started

To start coding your SwiftBot, type ``cargo add swift_bot_api`` add the following lines of code to the start of your program.
```rust
use swift_bot_api::*;
```
You will also have to create an instance of the robot, like this:
```rust
let mut bot = SwiftBot::new();
```

## Buttons

Trilobot has four buttons to its rear, labelled A, B, X, and Y.
These can be read using the `is_pressed()` function, which accepts the ``Button`` enum that consists of the following varients:

* `Button::A`
* `Button::B`
* `Button::X`
* `Button::Y`

For example, to read the A button you would write:

```python
state_a = tbot.read_button(BUTTON_A)
```

You can also get the number of buttons using the `NUM_BUTTONS` constant.


## Button LEDs

Next to each button on Trilobot is a mono LED. These can be controlled using the `set_button_led(button_led, value)` function, which accepts a button number from `0` to `3` like the buttons, followed by either `True` for On, `False` for Off, or a number between `0.0` and `1.0` for any brightness in between.

For convenience, each LED can be referred to using the same constants as the buttons.

For example, to set the LED next to button B to half brightness you would write:

```python
tbot.set_button_led(BUTTON_B, 0.5)
```

You can get the number of LEDs using the same constant as the buttons.


## Motors

Trilobot features two motors with independent control, enabling [differential steering](https://en.wikipedia.org/wiki/Differential_steering), whereby the speed of one motor can be controlled separately of the other.

There are several ways these motors can be commanded from code:

### Simple Movements

* `forward(speed=1.0)`
* `backward(speed=1.0)`
* `turn_left(speed=1.0)`
* `turn_right(speed=1.0)`
* `curve_forward_left(speed=1.0)`
* `curve_forward_right(speed=1.0)`
* `curve_backward_left(speed=1.0)`
* `curve_backward_right(speed=1.0)`

Each of the above functions will drive Trilobot at full speed. To slow the robot down, include a number between `0.0` and `1.0` within the brackets. For example `forward(0.5)` will drive forward at half speed.

To stop Trilobot from moving, simply call `stop()`. This will make the robot stop sharply. If a more gradual stop is wanted, call `coast()` instead.

### Advanced Movements

To get more control over Trilobot's movements, each motor can be individually controlled using `set_left_speed(speed)` or `set_right_speed(speed)`. These take a number between `-1.0` and `1.0`, where positive values will drive the motor forward and negative values will drive the motor backward. The below example will have Trilobot curving slowly to the right.

```python
tbot.set_left_speed(1.0)
tbot.set_right_speed(0.5)
```

If the speeds of both motors are regularly set together, then this can be shortened to a single call to `set_motor_speeds(l_speed, r_speed)`. This simplifies the curving right example to:

```python
tbot.set_motor_speeds(1.0, 0.5)
```

A final way the motors can be controlled is by using `set_motor_speed(motor, speed)`, which accepts a motor number from `0` to `1`, followed by a number between `-1.0` and `1.0`.

For convenience, each motor can be referred to using these constants.
* `MOTOR_LEFT` = 0
* `MOTOR_RIGHT` = 1

For example, to set the left motor to half forward speed you would write:

```python
tbot.set_motor_speed(MOTOR_LEFT, 0.5)
```

You can also get the number of motors using the `NUM_MOTORS` constant.

To disable the motors, call `disable_motors()`. This stops their signals and disables the motor driver. Setting any motor speed will enable the driver again. Note that calling `coast()` is the same as disabling the motors.


## Underlighting

One of the funnest features to play with of Trilobot is the six-zone RGB underlighting.

There are several ways these lights can be commanded from code:

### Single Light

A single underlight can be controlled using either `set_underlight(light, r_color, g=None, b=None, show=True)`, which all accept a light number from `0` to `5`, followed by the colour either as separate red, green, and blue values between `0` and `255`, a list/tuple of three numbers, or a hex colour code. Similarly `set_underlight_hsv(light, h, s=1, v=1, show=True)` lets you provide a colour as hue, saturation, and value numbers between `0.0` and `1.0`, and `clear_underlight(light, show=True)` sets the colour to zero.

For convenience, each light can be referred to using these constants.
* `LIGHT_FRONT_RIGHT` = 0
* `LIGHT_FRONT_LEFT` = 1
* `LIGHT_MIDDLE_LEFT` = 2
* `LIGHT_REAR_LEFT` = 3
* `LIGHT_REAR_RIGHT` = 4
* `LIGHT_MIDDLE_RIGHT` = 5

As an example, here are several individual lights being set in various ways:

```python
tbot.set_underlight(LIGHT_FRONT_LEFT, 0, 255, 0)  # Green
tbot.set_underlight(LIGHT_FRONT_RIGHT, '#0000ff')  # Blue
tbot.set_underlight_hsv(LIGHT_REAR_LEFT, 0.0, 1.0, 1.0)  # Red
tbot.clear_underlight(LIGHT_REAR_RIGHT)  # Off
```

You can also set a colour using a variable:

```python
YELLOW = (255, 255, 0)
tbot.set_underlight(LIGHT_MIDDLE_LEFT, YELLOW)  # Yellow
```


### All Lights

To set all of the underlights to a colour at once, `fill_underlighting(r_color, g=None, b=None, show=True)` and `fill_underlighting_hsv(h, s=1, v=1, show=True)` can be used. These accept a colour either as RGB or HSV in the same format as the single light functions. Similarly, `clear_underlighting(show=True)` sets all the LEDs to zero.

### Grouped Lights

To make some animations easier to create, several underlights at once using the `set_underlights(lights, r_color, g=None, b=None, show=True)`, `set_underlights_hsv(ights, h, s=1, v=1, show=True)`, and `clear_underlights(lights, show=True)` functions. Rather than a single number for the light, they instead take a list or tuple of the light numbers.

As an example, here are several lights being set in various ways:

```python
lights = (LIGHT_FRONT_LEFT, LIGHT_MIDDLE_LEFT)
tbot.set_underlights(lights, 0, 255, 0)  # Green
tbot.set_underlights(lights, '#0000ff')  # Blue
tbot.set_underlights_hsv(lights, 0.0, 1.0, 1.0)  # Red
tbot.clear_underlights(lights)  # Off
```

For convenience, several groups have been pre-defined.
* `LIGHTS_LEFT`
* `LIGHTS_RIGHT`
* `LIGHTS_FRONT`
* `LIGHTS_MIDDLE`
* `LIGHTS_REAR`
* `LIGHTS_LEFT_DIAGONAL`
* `LIGHTS_RIGHT_DIAGONAL`

### Show Underlighting

By default all underlight functions will apply the colour to the underlight(s) it is for immediately. This is because they all have an optional `show` parameter that is `True` by default.

For some effects this behaviour may not be wanted, so including `show=False` when calling the function will delay the applying of the colour to the LEDs until the next function that has `show=True`, or `show_underlighting()` can be called explicitly.

In the example below, each light is set to red in a loop, then they are all shown with the end call to `show_underlighting()`:

```python
for led in range(NUM_UNDERLIGHTS):
    tbot.set_underlight(led, 255, 0, 0, show=False)  # Red

tbot.show_underlighting()
```

### Disable Underlighting

There may be the case where you want to turn off the underlights, for example to save power, but have them remember what colour you last set when turned back on. For this the `disable_underlighting()` and `show_underlighting()` functions can be used.

In the example below, the underlights are filled with a dim white but not shown. Then in the loop they are repeatedly shown then disabled.

```python
tbot.fill_underlighting(127, 127, 127, show=False)
while True:
    tbot.show_underlighting()
    time.sleep(0.1)
    tbot.disable_underlighting()
    time.sleep(0.5)
```

## Distance Sensor

Trilobot features a front mounted ultrasonic distance sensor. This sensor can be read using the `read_distance(timeout=50, samples=3, offset=190000)` function, which will return a measured distance in centrimetres.

Some default values are used to get these readings, which can be overwritten with the following parameters:

* `timeout`: the total time in milliseconds to try to get distance reading
* `samples`: how many readings to take before returning an average
* `offset`: the time in nanosections the measurement takes (prevents over estimates)

The default `offset` is set to a value that is suitable for the Raspberry Pi 4, but may need adjusting if you are using a different model of Raspberry Pi.

To give more stable readings, this method will attempt to take several readings and return the average distance. You can set the maximum time you want it to take before returning a result so you have control over how long this method ties up your program. It takes as many readings up to the requested number of samples set as it can before the timeout total is reached. It then returns the average distance measured. Any readings where the single reading takes more than the timeout is ignored so these do not distort the average distance measured.

If no valid readings are taken before the timeout then it returns zero. You can choose parameters to get faster but less accurate readings or take longer to get more samples to average before it returns. The timeout effectively limits the maximum distance the sensor can measure because if the sound pulse takes longer to return over the distance than the timeout set then this method returns zero rather than waiting. So to extend the distance that can be measured, use a larger timeout.
