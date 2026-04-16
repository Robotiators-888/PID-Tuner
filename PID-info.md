# Research about PID's
A PID in our use case tells a motor what speed to run at in order to stay at the point/speed you tell it to\
The P value is the proportional part of it and it controls the "speed" but it will never actually get the error to 0. It is a constant offset based on how far you are from the goal at the current momment and is proportianal to the error.\
The I value is the intergral and accounts for and accumulates (hence why its an integral) long term error from the past and adjusts for it, for example gravity. It works to get the error to 0\
The D value is the derivitive which predicts the future by looking at the rate of change of the error (hence why its a derivative) and slows down if approaching the setpoint too fast. This reduces oscliation and prevents overshooting the target.\
The P, I, and D values are all added together to get the final output.\
Note:
* The Derivitive is highly sensitive to noise so don't make it too high\
* The PID also heavily depends on context so it can't be the same everywhere\
Sources:
* [https://www.ni.com/en/shop/labview/pid-theory-explained.html](https://www.ni.com/en/shop/labview/pid-theory-explained.html)
* [https://www.youtube.com/watch?v=fv6dLTEvl74](https://www.youtube.com/watch?v=fv6dLTEvl74)
