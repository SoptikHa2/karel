# Karel in Rust
Karel language interpreter in Rust. This version of Karel is slightly altered to protect sanity of innocent people.

## Overview

From [wikipedia](https://en.wikipedia.org/wiki/Karel_(programming_language)):

> A program in Karel is used to control a simple robot named Karel that lives in an environment consisting of a grid of streets (left-right) and avenues (up-down). Karel understands five basic instructions: move (Karel moves by one square in the direction he is facing), turnLeft (Karel turns 90 ° left), putBeeper (Karel puts a beeper on the square he is standing at), pickBeeper (Karel lifts a beeper off the square he is standing at), and turnoff (Karel switches himself off, the program ends). Karel can also perform boolean queries about his immediate environment, asking whether there is a beeper where he is standing, whether there are barriers next to him, and about the direction he is facing. A programmer can create additional instructions by defining them in terms of the five basic instructions, and by using conditional control flow statements if and while with environment queries, and by using the iterate construct. 

## Syntax

The syntax is slightly altered to protect innocent people.

Example program:
```karel
def main # This is where program starts
  turn-left # Python-like indent is recommended, but not necessary
  move      # Go forward
  put       # Put a beeper here
  if beeper # If there is a beeper here
    take  # Take the beeper
    repeat 3 # Repeat following commands 3 times
      turn left
    endrepeat # We must end each block
  endif
  die # Exit program
enddef
```

You can call other methods like this (including recursive calls):
```karel
def main
  call testmethod
  ...
  die
enddef

def testmethod # Method names cannot contain any whitespace character
  move
enddef
```

While loop looks like this
```karel
def goUpToWall
  while! wall # While there is no wall in front of me (the "!" at end of while negates the condition)
    move
  endwhile
enddef
```

### Conditions

|Condition|Description|
|---|---|
| wall | Is there a wall in front of the robot? |
| north | Is robot facing north? |
| south| ... |
| east|...|
|west|...|
|beeper| Is there at least one beeper here? |

### Instructions

|Instruction|Description|
|---|---|
|move|Move forward|
|turn-left| Turn left|
| take | Take a beeper on this tile |
| put | Put a beeper to this tile |
| die | Turn off the robot |

## Errors

Following situations are considered to be errors and will terminate the robot:

- Move hits a wall or overflows from allowed area (default: 10x10)

- Take is called when there is no beeper on current tile

- Put is called when there are 8 beepers on current tile

- Robot does not die when the program ends and has to be stopped by force. 


# Why????

That's not a very nice question.
