# Design Patterns

## Definitions

- An **object** packages both data and procedures (**methods** or **operations**) that operates on that data.
- An object performs an operation when it receives **request** (or **message**) from a **client**.
- **Encapsulation**: requests are the *only* way to get an object to execute an operation, operations are the *only* way to change an object's internal data.
- The operation's name, objects it takes as parameters and the objects return value combined is called the operation's **signature**. A set of defined signatures is called the **interface**.
- A **type** is the name of a particular interface, two objects with the same type need only share parts of their interfaces. A **subtype**'s interface is a superset of its **supertype**.
- **Dynamic binding**: a request is not bounded to a particular implementation, and the implementation can be substituted during run-time, this substitution is called **polymorphism**.
- A **class** is the implementation of an interface, and an object is the **instance** of that class, the object's internal data is made up of **instance variables**.
- **Abstract class**es may defer some or all of its operation implementation to their **subclass**, methods defined but not implemented are called **abstract operation**s.
- Object **aggregation**: an object *owns* another object, or *is part of* another object; object **acquaintance**: an object simply *knows of* another object, or is simply *using* another object. Object aggregation implies that the two objects have identical lifetimes. In diagrams an arrowhead line (<>--->) represents with a diamond base represents the object at the base aggregates (owns) the object at the head; the object at the head acquaintances the object at the base.

### Type vs. Class

*It is very important to understand the difference between type and class*. Types are only concerned of the object's interface, while classes are concerned of the object's internal state and implementation. Same goes with interface inheritance and class inheritance. Class inheritance defines an object's implementation in terms of another object's implementation, it is a mechanism for code and representation sharing; interface inheritance describes when an object can be used in place of another.

## High Level Stuff

*A design choice is a good choice only when it simplifies more than it complicates.*

The hard part about object-oriented design is decomposing a system into objects. Many factors influence the decomposition, often in conflicting ways. There will always be disagreement on which approach is best.

Strict modeling may reflect today's needs but not tomorrow, abstraction is key to making design flexible.

The key to maximizing reuse lies in anticipating new change, design patterns help us do that by ensuring the system can change in specific ways.

*An object-oriented program's run-time structure often bears little resemblance to its code structure.* Code structure is frozen at compile-time, but object structure is a complicated network of objects. Do not attempt to understand one from another. The aggregation or acquaintance relationship can be difficult to differentiate by reading code. Because of that, *the code won't reveal everything about how the system will work*, the run-time structure must be imposed more by the designer than the language. Many design patterns aren't obvious from the code itself unless the code reader understands the patter.

It is important to design the system to limit its platform dependencies.

## Program to an interface, not an implementation.

Class inheritance is just reusing implementation, interface inheritance truly defines the relationship an interaction for the client. Creational patterns (e.g., AbstractFactory, Builder, Factory Method, Prototype, Singleton) ensure the system is written in terms of interfaces, not implementations.

## Class Inheritance vs. Object Composition vs. Parameterized Types (Generics)

The three most common ways to reuse functionality.

*Favor object composition over class inheritance.* However, heavy use of object composition can make designs harder to understand.

### Class Inheritance

Class inheritance is a more transparent reuse (white-box reuse), since the superclass' internals are often exposed to the subclass.

Pros:

- Defined statically at compile-time.
- Straightforward to use.
- Easier to modify implementation being reused (by overriding).

Cons:

- Cannot change implementation during run-time (inheritance defined at compile-time).
- Breaks encapsulation (superclass' detail exposed to subclass).
- Subclass implementation is bound up.
- Change in the superclass will force the subclass to change.
- If any part of the inherited implementation is not appropriate for new problem, superclass must be rewritten or replaced.
- *Designers often overuse inheritance as a reuse technique.*

### Object Composition

Object composition are black-box reuse, since it requires clear definition of objects' interfaces and no internal detail is visible (objects are black boxes).

Pros:

- Defined dynamically at run-time.
- Requires objects to respect each others' interfaces.
- Does not break encapsulation.
- Substantially fewer implementation dependencies.
- Forces each class encapsulated and focused on one task.

Cons:

- Interfaces must be carefully designed.
- Dynamic, highly parameterized software is harder to understand than more static software.
- Run-time inefficiencies (*but the human inefficiencies are more costly in the long run*).

### Parameterized Types

Pros:

- Defined statically at compile-time.

Cons:

- Cannot change implementation during run-time (inheritance defined at compile-time).

## Delegation

Two objects handle a request: receiver delegates operations to its **delegate**.

In class inheritance, the operation can access parent class object by using `this` or `self`, in delegation, this is achieved by the receiving object passing itself to the delegation as a reference.

In delegation pattern, the object which owns the delegation object can be thought of as the "subclass" in class inheritance. For example, in class inheritance the class `Dog` would be a subclass of `Animal`; in delegation the class `Dog` would *have* an `Animal`.

Common design patterns that use delegation are: State, Strategy, Visitor, Mediator, Chain of Responsibility, Bridge.

## Designing for Change

Common design mistakes that limits the system's ability to change and adapt, and design patterns that are potential solutions:

1. Creating an object by specifying a class explicitly: Abstract Factory, Factory method, Prototype.
2. Dependence on specific operations: Chain of Responsibility, Command.
3. Dependence on hardware or software platform: Abstract Factory, Bridge.
4. Dependence on object representations or implementations: Abstract Factory, Bridge, Memento, Proxy.
5. Algorithmic dependencies: Builder, Iterator, Strategy, Template Method, Visitor.
6. Tight coupling: Abstract Factory, Bridge, Chain of Responsibility, Command, Facade, Mediator, Observer.
7. Extending functionality by subclassing: Bridge, Chain of Responsibility, Composite, Decorator, Observer, Strategy.
8. Inability to alter class conveniently: Adapter, Decorator, Visitor.

### Design Priorities for Different Types of Programs

A toolkit specifies utilities can be used to build software for a specific application domain, the implementer needs to design the application and call code in the toolkit they want to reuse; a framework designs the application, and calls code written by the implementer, thus the implementer has less design choices to make.

Loose coupling, flexibility, extensibility and good documentation are extremely important in the design of a framework.

| Program Type | Main Focus | Difficulty (Video Game Metric) |
| - | - | - |
| Application | internal reuse, maintainability, extension | Hard |
| Toolkit | code reuse | Nightmare |
| Framework | design reuse | Hell |
