# Design Patterns

## Definitions

- An **object** packages both data and procedures (**methods** or **operations**) that operates on that data.
- An object performas an operation when it receives **request** (or **message**) from a **client**.
- **Encapsulation**: requests are the *only* way to get an object to execute an operation, operations are the *only* way to change an object's internal data.
- The operation's name, objects it takes as parameters and the objects return value combined is called the operation's **signature**. A set of defined signatures is called the **interface**.
- A **type** is the name of a particular interface, two objects with the same type need only share parts of their interfaces. A **subtype**'s interface is a superset of its **supertype**.
- **Dynamic binding**: a request is not bounded to a particular implementation, and the implementation can be substituted during run-time, this substitution is called **polymorphism**.
- A **class** is the implementation of an interface, and an object is the **instance** of that class, the object's internal data is made up of **instance variables**.
- **Abstract calss**es may defer some or all of its operation implementation to their **subclass**, methods defined but not implemented are called **abstract operation**s.

## Important Leanrings

The hard part about object-oriented design is decomposing a system into objects. Many factors influence the decomposition, often in conflicting ways. There will always be disagreement on which approach is best.

Strict modeling may reflect today's needs but not tomorrow, abstraction is key to making design flexible.

### Type vs. Class

*It is very important to understand the difference between type and class*. Types are only concerned of the object's interface, while classes are concerned of the object's internal state and implementation. Same goes with interface inheritance and class inheritance. Class inheritance defines an object's implementation in terms of another object's implementation, it is a mechanism for code and representation sharing; interface inheritance describes when an object can be used in place of another.

### Program to an interface, not an implementation.

Class inheritance is just reusing implementation, interface inheritance truely defines the relationship an interaction for the client. Creational patterns (e.g., AbstractFactory, Builder, Factory Method, Prototype, Singleton) ensure the system is written in terms of interfaces, not implementations.

### Class Inheritance vs. Object Composition

**Favor object composition over class inheritance.**

Class inheritance and **object composition** are two most common ways to reuse functionalities. Class inheritance is a more transparent reuse (white-box reuse), since the superclass' internals are often exposed to the subclass; object composition are black-box reuse, since it requires clear definition of objects' interfaces and no internal detail is visible (objects are black boxes).

#### Class Inheritance

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

#### Object Composition

Pros:

- Defined dynamically at run-time.
- Requires objects to respect each others' interfaces.
- Does not break encapsulation.
- Substantially fewer implementation dependencies.
- Forces each class encapsulated and focused on one task.

Cons:

- Interfaces must be carefullly designed (not really a con).
