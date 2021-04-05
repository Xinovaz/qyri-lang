.. _start:
The Basics!
===========
The what?
---------

I said, the *basics*! The building blocks of your learning experience! Every programmer has to learn something, at some point in their life. I think that goes for everyone. But nobody really *wants* to sit down and learn. But, you have to. So, stop reading this, and go to the next section.

Table of Contents:
------------------
#. :ref:`Variables <variables>`
#. :ref:`Arithmetic <arithmetic>`
#. :ref:`Functions <functions>`
#. :ref:`Structures <structs>`
#. :ref:`Types <types>`
#. :ref:`Enumerations <enums>`

.. _variables:

Declaring Variables
===================
**The naming ceremony...**


Qyri has a very simple syntax for declaring variables:

``var x = 1;``

Notice that semicolon at the end of the line. Yes, Qyri needs semicolons. No, you aren't an exception.

Variables in Qyri are mutable by default. An immutable variable is called a ``constant``. Constants are declared like this:

``const PI = 3.14;``

Note that the name of this constant is capitalized. Qyri's job is took be simple and intuitive, but also readable and extendable. Thus, Qyri holds a few style properties that keep your code understandable. Constants are in SCREAMING_SNAKE_CASE and variables are in sneaky_snake_case.


.. _arithmetic:

Arithmetic
==========
**They did the math, they did the monster math...**

Qyri supports arithmetic. Obviously. Check it
::
	var x = 2 + 2;
	var y = x * 4;
	var z = x * y - 4;


In case you skipped every grade, ``x`` would equal 4, ``y`` would equal 16, and ``z`` 60. Qyri also supports the order of operations, as you would hope. As in:

``var n = 12 - 4 * 2;``

``n`` in this case would equal 4, rather than 16. This brings me to my next point, which is that infix operators, such as `` `mod\` ``, are at the bottom of the order. So, if we do this:

``var x = 4 + 4 `mod` 3;``

``x`` would equal 2.


.. _functions:

Declaring and Using Functions
=============================
**We all live in a dysfunctional family in one way or another...**

Functions are exactly the same in Qyri as almost any other language:

``print("Hello, there.");``

And storing them as variables works the same way:

``var general_kenobi = a_bold(1);``

That variable name is longer than you expected. Notice I used snake case like I mentioned earlier.

Ok, you're like, "Yo, functions are supes dope, dawg. Can I, like, totes make my own?" and I'm like, "Yeah dawg, don't even trip."
::
	fn add = (x, y) {
		return x + y;
	}

F█cking delicious. It might look a little different than what you're used to, but I assure you it works the same way. Now, the example I just showed you is fairly simple, and Qyri is dynamically-typed, so this isn't an issue. Unless, for some reason, you're working in a statically-typed program. In this case, everything in your code should be statically-typed. This is how that'd look:
::
	fn add = (x: int, y: int) $ int {
		return x + y;
	}

Pretty straightforward. The ``$`` operator tells Qyri that whatever comes after it is the type that's being returned.

Qyri is multi-paradigm. Qyri is both imperative *and* functional. So, what? So, you can write functions that can be fixed between two expressions, such as ``mod``. Every function with two parameters is infixable by default, so we can very well do this with our ``add`` function:

``var five = 2 `add` 3;``

If you prefer that your two-parameter function not be infixed, you can define it with a tilda concatenated to the beginning of your function name, like so:
::
	fn ~add = (x: int, y: int) $ int {
		return x + y;
	}

The reason I chose such a rarely-used character is *because of the fact*! Infix operators rock, and this is **my** programming language, and I say that if you want to write a biparametral function you're forced to flex your pinky weird.

Functions can also be threaded into one-another using the good old pipe-forward operator, ``|>``, because f█ck you that's why.

Instead of this:

``print(math.cot(2 * PI * r));``

you can write this:
::
	2 * PI * r
	|> math.cot
	|> print;

which is slightly more readable and makes your wife love you more. Sike.

Functions come in different flavours, like ice cream, only codier. For example, this is valid, too:
::
	var add = (x, y) {
		return x + y;
	};

as is this:
::
	const add = (x, y) {
		return x + y;
	};


"（ミ￣ー￣ミ）what? ``add`` isn't a variable, it's a funct-- ohhhh." You get it now. This syntax:

``() {}``

is an anonymous function. So, why implement the ``fn`` keyword if ``var`` and ``const`` are available. Well, they all do different things. When you declare a function using ``fn``, what you're doing is telling Qyri that whatever function name you use is now permanently allocated to that function, and it is unusable for any variable or constant. 

Conversely, using ``const`` will do that, but prevent you from using the function's return value, and you *must* call the function as a keyword, like so:
::
	add 2, 3;

Lastly, the ``var`` method of declaration allows the function's name to be mutable, meaning you can later use that name for a variable, constant, or other function. This is rarely useful, but it's implemented and there's nothing you can do about it.

Also notice that there's a ``;`` at the end of both the ``var`` and ``const`` methods of defining a function. You're still declaring a variable/constant when you do those, remember that.

.. _structs:

Structures
=======
**Whactures?**

``struct`` ures! A ``struct`` is a composite data type that allows a programmer such as yourself to generate their own grouped list of variables that are allocated separately from other variables. They act as objects or classes, and their fields are private by default.

There are two types of ``struct``s in Qyri:
1. Generic struct, this serves as an abstraction.
2. Membered struct, which houses several fields in which to input values.

Here's an example of a generic ``struct``:
::
	struct EndNode;

And a membered ``struct``:
::
	struct Point = {
		x,
		y,
	}

And here's a membered ``struct`` in a statically-typed fashion:
::
	struct Point = {
		x: int,
		y: int,
	}

.. note::
	Notice the variables syntax in statically-typed programs. It always looks like this: ``variable: type``. This is how Qyri interprets a variable as being statically-typed.

	Also, a ``struct`` can be typed too. Check it:
	::
		struct Point: int = {
			x: typeof(self),
			y: typeof(self),
		}

	One last thing to note is the naming conventions here. Just as variables, constants, and functions have naming conventions, ``structs`` do too. Write ``struct`` names in CamelCase. Pretty please.

Structs aren't just records, though. Structs can be extended to advanced uses, and implemented with methods. Here's how you do that:
::
	// First, initalise your struct
	struct Point = {
		x: int
		y: int
	}

	// Next, do this thingy

	Point -> {
		// Write your functions in here

		// Every time a struct has a 'new' method, it acts as a constructor
		fn new = (x: int, y: int) $ Point {
			return Point {
				x -> x,
				y -> y,
			};
		}

		fn inverted = () $ Point{
			return Point {
				x -> self.y,
				y -> self.x,
			};
		}
	};

	// You can now do something like this:

	var arbitrary_point = Point(4, 5);
	var flipped = arbitrary_point.inverted();
	// This is the same:
	var again = inverted(arbitrary_point);

	// And don't forget about the pipe-forward operator
	var point = Point(2, 3) |> inverted; // Resolves to Point(3, 2)

If you're a keen-eyed person, you might have noticed a couple interesting things.

The first thing you'll note is the ``->`` operator. This is called the "structure frame operator," and it serves an important purpose when it comes to structs. First, when its left side is a ``struct``, it expects the right side to be a scope, which is a section of code whose memory differs from the base's. When its left side is an identifier, the operator expects to be in a canonical ``struct``. It then takes the right side, which could be any expression or value, and smushes it into the field pertaining to the identifier on the left side of the operator.

.. note::
	The "canonical" form of an object is its notation before being loaded into Qyri's memory, such as a raw expression ``2 + 2 - 3`` or the definition of a new struct:
	::
		Stack {
			memory -> []
		}

	What's important to note is that canonical forms that aren't instructed to load into memory (via storage in variable or other container) aren't. This means that you can write comments that are actually strings, like in Python. However, I suggest that you just use comments to prevent confusion and clutter.

The second thing you might notice is that there's a semicolon at the end of the
::
	Point -> {/*...*/}
section. This is simply because ``->`` is an operator, so we're really writing an expression.

.. _types:

Types
=====
**You just aren't mine...**

Most language's documentation include the "types" section much earlier. However, I wanted to introduce the more important information before listing types. This will play into our next section, too.

There are 10 different base types in Qyri, and they go as follows:
1. ``int``
2. ``bool``
3. ``null``
4. ``str``
5. ``float``
6. ``double``
7. ``byte``
8. ``word``
9. ``long``
10. ``type``

Yes, there's a type type. Allow me to explain each and every one of them, in excruciating detail.

* An ``int`` is a signed 32-bit integer.
* A ``bool`` is either true or false.
* ``null`` is a value that signifies nothing.
* A ``str`` is a string, which is an ASCII-ized array of ``byte``s.
* A ``float`` is a signed 16-bit floating point number.
* A ``double`` is a signed 32-bit floating point number.
* A ``byte`` is an unsigned 8-bit integer.
* A ``word`` is an unsigned 16-bit integer.
* A ``long`` is an unsigned 32-bit integer. 
* A ``type`` is a type that signifies a type. I know that's a bit confusing, but if, say, a function wanted to convert one type of integer to another, it would want the number and the type to convert to.

That's all you really need to know about types, they're just generic identifiers. You can always get the ``type`` of a variable or object by calling the ``typeof()`` function on it.

.. _enums:

Enumerations
============
**How many lists does it take to get to the center of a stack overflow?**

An enumeration, or ``enum`` is a type of type! That's right, you can make *more* types than the ones I just listed. But, not without hard work and effort.

What does an ``enum`` look like? Well...
::
	enum foo = {
		bar,
		baz,
		bat,
		buzz,
		fuzz,
		faz,
		fat,
		cat,
	}

Something like this. What does this mean? It means that ``foo`` can have different variants. It means that ``foo(bar)`` is a valid value, as is ``foo(fuzz)`` and ``foo(cat)``. But, what do each of these equate to? Well, themselves. They're generic types. And I really want to hammer this into your head, because it's important: they aren't functions. They look like functions, but they aren't. They're enumerations.

So, we know ``foo`` can be any of the list values, but can ``foo`` just be a generic type on its own? Well, you have two options:
::
	struct foo; // The struct-y way
	// or
	enum foo; // The enum-y way

What's the difference? Well, using ``struct`` will allow you to implement methods onto ``foo``, whereas using ``enum`` will allow you to create variants of ``foo``. If you don't need either, that's fine. You can choose whichever based on arbitrary preference, but I suggest that if you're going to be using ``foo`` to type values, use ``struct``, whereas needing a generic type should entail using ``enum``. This is just part of Qyri's conventions.
