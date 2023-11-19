# Rustlab for Robotics Competitios

## Agenda

- Workshop goals
- This repo
- Something about me
- Robotics competitions
- Embedded Rust 101
- Meet your robot
- Embassy
- Hello Blink
- Hello World
- Digital read
- Analog read
- PWM output
- Tasks and messages (Ping Pong)
- WIFI: list access points
- WIFI: join a network
- TCP client
- Robotic sumo
- More on async embedded rust

## Workshop goals

- giving you a taste of embedded Rust
- with concrete tasks
- learning by writing code (minimal frontal teaching)
- having *fun*

# This repo

- bot: the robot code (code that runs on the robot)
- bot-msg: a message broker that we can use to coordinate the competition (runs on the PC)
- bot-3d: the 3d plans for the robot body

## Something about me

- passionate software engineer
- mostly did systems programming
- also as a JIT compiler engineer (Mono, Unity3D, V8 in the Google team)
- for me Rust is a natural choice :-)

## Robotics competitions

- definitely *not* the RC-controlled "destroy the opponent" kind
- robots are autonomous
- hobby: relatively cheap off the shelf components
- the competition is friendly
- high educational value
- I learned Rust this way!

## Embedded Rust 101

- say no to:
  - operating systems
  - imposed runtime environments
  - the standard library
- say yes to:
  - bare-metal programming
  - complete hardware control
  - the core library
- your code stays safe

## Meet your robot

- let's call it Froggy
- maybe you'll curse it as "Miopic Frog"...
- Raspberry PI Pico W
  - rp2040 CPU
  - a WIFI-capable "coprocessor"
- a board with:
  - a power regulator
  - two motor drivers
  - no-soldering connectors
- two IR-reflection digital distance sensors
- one IR-reflection analog "color" (BW) sensor
- two motors and two wheels
- one power back as power supply

## Embassy

- Embassy what?
  - an async framework for embedded Rust
- async why?
  - well... only Embassy has a driver for this WIFI chip
  - I preferred doing this workshop with "mainstream" Rust
  - I ended up *loving* Embassy!
- we'll see why

## Hello Blink

Let's blink a led

## Hello World

Let's write something over the USB wire so we can read it on our development machine
(note that we do not use a debugging probe)

## Digital read

Let's read the status of a digital pin

## Analog read

Let's read the status of an analog pin

## PWM output

Let's generate an electrical signal that can stimulate the motor drivers so that the motors will move

## Tasks and messages (Ping Pong)

Async-based cooperative multitasking, synchronizing tasks using messages

## WIFI: list access points

Test the WIFI hardware by listing the mearby access points

## WIFI: join a network

Test the WIFI hardware by connecting to a network

## TCP client

Connect to a TCP server and write something to it

## And now?

- get back to our original goal: make these robots fight each other
- no robot will be harmed in the process
- understand robotic sumo
- have fun!
- also, understand async embedded rust by writing logging code
