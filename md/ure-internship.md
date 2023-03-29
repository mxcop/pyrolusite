URE Internship
29/03/2023

A log of my time at URE.

---

# My [URE](https://www.universityracing.nl/) Internship Log

### Week `1`, `2`, `3`, & `4` <sup>(Jan 30 - Feb 26)</sup>
During the first four weeks I've been working with MCUXpresso and C.<br>
This was a bit of a challenge in the beginning because I had never really done embedded programming before.<br>
So A lot of the concepts and terms were a bit of a mystery.<br>

However after the first week of doing a lot of research I learned a lot.<br>
And I was able to get CAN and some LEDs working.<br>
CAN *(Controller Area Network)* is used in most vehicles, it allows seperate modules to communicate with one another.<br>
While only requiring two wires, CAN High and CAN Low.<br>

After the first two weeks of experimenting and researching, I started work on my 2 week assigment.<br>
Making a HAL *(Hardware Abstraction Layer)* for the new chips that we've received from NXP.<br>
The chip in question is the LPC54628, it's rocking a single core ARM Cortex processor.<br>
With a powerful set of peripherals, including of course CAN.

The HAL was to include:
* `ADC`  *(Analog to Digital)*
* `CAN`  *(Controller Area Network)*
* `GPIO` *(General Purpose IO)*
* `PWM`  *(Pulse Width Modulation)*
* `RIT`  *(Repetitive Interrupt Timer)*
* `SPI`  *(Serial Peripheral Interface)*
* `WDT`  *(Watchdog Timer)*

I managed to implement all of them, however some more thoroughly then others.<br>
Primarly due to the limited time I had. *(Nevertheless I'm still very happy with the result)*

<!-- *Pictures of the LPC54628 Devboard & Can Transceiver*<br>
<img src="./LPC54628.jpg" width=49% style="max-width: 500px">
<img src="./CAN_Transceiver.jpg" width=49% style="max-width: 500px"> -->

<br>

### Week `5` <sup>(Feb 27 - Mar 05)</sup>
During this week I worked on the Framework which is at the core of the car.<br>
It's the brain of the car, and also where the autonomous driving happens.<br>
We're trying to get rid of some unnecessary parts to reduce weight and complexity.<br>

I got too work a little more together with Erik which was fun.<br>
Working together with others makes time pass a lot more quickly and feels more rewarding to me.<br>
I hope that when I'm done with my internship I'll have left behind something useful for everyone at URE.

<br>

### Week `6` <sup>(Mar 06 - Mar 12)</sup>
This week I've started the monsterous task of migrating the ECU logic from Simulink to C++.<br>
Simulink is a way to write logic/code using blocks and connections between those blocks.<br>
The problem is that Simulink is a bit of a mess, and difficult to oversee / maintain.<br>
It's also less efficient then using C++ because Simulink has to make everything generic.<br>

The ECU is the core/brain of the car, as I explained in my previous report.<br>
However in previous years the car has had **two** ECUs which intercommunicate using CAN.<br>
This means more weight and more complexity, however with this migration we will be able to merge them.<br>
Which will save roughly 2.5Kg of weight, and hopefully, make the code easier to maintain.<br>

<br>

### Week `7` <sup>(Mar 13 - Mar 19)</sup>
This week is rather uneventful, I've continued my work migrating the ECU logic.<br>
One fun thing that did happen was I did some research into RFID tags.<br>
So we can build one of these into the monocoque, containing a URL to the URE website.<br>

RFID stands for Radio Frequency ID, it's usually a blue plastic droplet which contains a coil and a chip.<br>
Once a reader *(like your phone)* comes close enough to the tag, the tag wirelessly receives power through the coil.<br>
When powered up the tag will start transmitting it's contents immidiately.<br>
That's how these tags can function without requiring their own battery.<br>

<br>

### Week `8` <sup>(Mar 20 - Mar 26)</sup>
This week I managed to finish the task of rewriting the ESC (Electronic Systems Controller).<br>
I also participated in a sponsor event, where the board gave a presentation for our sponsors.<br>
It was very nice to see a more macro overview of the car and progress so far.<br>

Afterall when you're working on just some software bits you don't really get a good overview of the whole car.<br>
Besides that this weekend we received the satellites and they passed our tests!<br>
*The satellites are PCBs that are placed ontop of the battery cells to monitor their voltage & temperature.*<br>
This is great news because without satellites the car won't be able to drive.<br>

<br>

### Week `9` <sup>(Mar 27 - Apr 02)</sup>
...