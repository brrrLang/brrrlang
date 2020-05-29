# Spruce-lang
Spruce-lang is a language based on events, there are no function calls.
## Hello world in Spruce-lang
```spruce
@export std::StartEvent:100 default;

@import 'std';
@require std::io;

@EventHandler ehid = 100, e = std::StartEvent {
    raise std::io::PrintLnEvent("Hello world");
    e.exitCode = 0;
    raise e.FinnishEvent() await default;
}
```

Heres how it works
- `@import` tags import a specific module into the current module, in the case of this example, the std module. This will also auto-require any other sub-modules of 
the module that have been outline in the mod.spruce
- `@require` This imports a submodule of an already imported module or submodule into the current module.
- `@EventHandler ehid = 100, e = std::StartEvent` is the main method of your spruce application. It is a EventHandler for the StartEvent, so everytime a StartEvent 
is 
raised, once the runtime hase been initlised, the StartEvent is raised and this EventHandler is called asyncouriously.
- `raise std::io::PrintLnEvent("Hello world");` This line raises the std::io::PrintLnEvent event with the arguments of "Hello world". This causes all the EventHandlers for 
std::io::PrintLineEvent to be called 
asyncourously. This prints 'hello world' and the newline charcter to the console.
- `e.exitCode = 0;` This sets the value for exitCode for the StartEvent to 0, events are an objects that inherits the event class.
- `raise e.FinnishEvent() await default` This calls the finnish method on the StartEvent, this tells the application that this event's execution has been completed
- `@export` This exports a specific event handler to an alias for example `@export Foo:100 default` will map the Foo event handeler with the id of 100 as the 
default event handler. In this exapmle the runtime raises StartEvent, then awaits the event with the alias default.

## Vaiables 
Variables are mutable by default, they are staticaly typed. You use the let keyword to define them. eg:
```spruce
let a = 0;
```
This will create a int_32 with the value 0;

### Constants
Constants are like variables with immuatable values.
```spruce
let const a = 0;
```

### Manulay defining types
You can manualy define the type of a variable using the : operator
```spruce
let a: int_32;
```
This will create an unitilized int_32

With number primitives, you can define their type using putting the type after the number
```spruce
let a = 16uint_8;
```

## Events
Events are an object that can be raised, and have EventHandlers attached to it. When an event is raised, all of the event handlers registered to it will be 
asyncourously. One event can have many attached handlers, all are called when any given event is raised. The syntax to raise an event is ` raise EventName`.

### Creating an event
Befor you can create event handlers, you first have to create the event. This can be done like so:
```spruce
@Event
class FooEvent {
    pub require {
        int_32 number1;
        int_32 number2;
    }
    pub {
        int_32 result;
    }
}
```

The `@event` tag tells the compiler that this is an event, inherit for Event.

The `pub require` block is a block for all of the infomation that must be passed to the consturctor when you create a object of this type, like in Raise. Think 
of these are your parameters for you event handler

The `pub` block is just public members

### Creating an event handler
Now that we have an event that we can raise, we can create an event handler that will be called when the event is raised
```spruce
@EventHandler ehid = 100, e = FooEvent {
    e.result = number1 * number2;
    raise e.FinnishEvent() await default;
}
```
The `@EventHandler` tells the compiler that this is an event handler, the `ehid` is the event handler id, and is used to distiguish between diffrent event handlers. 
Convention is to start from 100 and increse by 100 for each new handler, although it does not have to be a multiple of 100, if no ehid is specified, the compiler 
will assign the event the next avaliable ehid. The `e = Foo` tells the compiler what event this handler should be registered to.

Any time you want to access infomation from the event inside an event handler, use `e.property`. `e` is the object that was created from raising the event.

To use this event, you simpaly raise Foo, this will asyncoursouly call all the event handler registered to the event Foo, like the one we just created. Heres how we 
would use it:

```spruce
raise FooEvent(1, 100);
```

Now Foo still has some data stored in it that we want to retrieve, the result member contains the result of multiplying 1 and 100, we want to get this data out of 
the Foo event. To do this, we await an event handler. When using an await, you must specify the ehid of the handler that you want to get the data from. The evntent 
handler that you have specified will be called asyncourously while all other handlers will still be called asyncoursouly. This will return the event object from 
that event handler that you have specified. Using the Foo event and handler that we have just created, here is how you get the information out.
```
let multiply = raise FooEvent(50, 10) await 100.result;
```
This will create a variable with the value of e.result from the event handler with the ehid of 100, so in this case, it will have the value of 500.

Here is an expample using all of this together;

```spruce
@export std::StartEvent:100 default;

@import 'std';
@require std::io;

@EventHandler ehid = 100, e = std::StartEvent {
    let number = raise FooEvent(69, 420) await 100.result;
    raise std::io::PrintlnEvent(number);
    e.exitCode = 0;
    raise e.FinnishEvent() await default;    
}

@Event
class FooEvent {
    pub require {
        int_32 number1;
        int_32 number2;
    }  

    pub {
        int_32 result;
    }
}

@EventHandler ehid = 0, e = Foo {
    e.result = e.number1 * e.number2;
    raise e.FinnishEvent() await default;
}
```

Events are also objects so you can store them like a so

```spruce
@import 'std';
@require std::io;

@EventHandler ehid = 100, e = std::StartEvent {
    let fooEvent = raise Foo(69, 420) await 100;
    raise std::io::PrintLnEvent(fooEvent.number);
    e.exitCode = 0;
    raise e.FinnishEvent() await default;    
}
// Rest of program
```

## Compiler overview
```
      +-------------------------+
      |                         |
      |Program is read to string|
      |                         |
      +-----------+-------------+
                  |
                  |
                  v
+-----------------+--------------------+
|                                      |
|Split into lines with scope infomation|XXXXXXXXXX
|                                      |         XXXXX
+-----------------+--------------------+             XX
                  |                                   XX
                  |                                    XX
                  |                                     X
                  |                         +------------------------+
                  |                         |                        |
                  |                         |New thread for each line|
                  |                         |                        |
                  |                         +-----------+------------+
                  |                                     |
                  |                                     |
                  |                                     |
                  |                                     v
                  |                    +----------------+-------------------+
                  |                    |                                    |
                  |                    | Split via keywords (Spaces, +,=,@) |
                  |                    |                                    |
                  |                    +----------------+-------------------+
                  |                                     |
                  |                                     |
                  |                                     v
                  |                  +------------------+-------------------+
                  |                  |                                      |
                  |                  | Create context for the keywords      |
                  |                  | (Variables, loop, if statement, etc) |
                  |                  |                                      |
                  |                  +------------------+-------------------+
                  |                                     |
                  |                                     |
                  |                                     |
                  |                                     v
                  |                      +--------------+-------------------+
                  |                      |                                  |
                  |                      | Use context to convert to tokens |
                  |                      |                                  |
                  |                      +--------------+-------------------+
                  |                                     |
                  |                                     |
                  v                                     |
    +-------------+-----------------+                   v
    |                               |     +-------------+------------------+
    | Wait for threads to compleate |     |                                |
    |                               +<----+    Save data to Arc Multex     |
    +-------------+-----------------+     |                                |
                  |                       +--------------------------------+
                  |
                  v
    +-------------+-------------------+
    |                                 |
    |Convert to low-level instructions|
    |                                 |
    +-------------+-------------------+
                  |
                  |
                  |
                  v
 +----------------+---------------------------+
 |                                            |
 | Convert low-level instructions to assembly |
 | on a per-architecture basis                |
 |                                            |
 +--------------------------------------------+

```