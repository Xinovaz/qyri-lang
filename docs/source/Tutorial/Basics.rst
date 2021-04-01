.. _start:
The Basics!
===========
The what?
---------

I said, the *basics*! The building blocks of your learning experience! Every programmer has to learn something, at some point in their life. I think that goes for everyone. But nobody really *wants* to sit down and learn. But, you have to. So, stop reading this, and go to the next section.

Table of contents:
------------------
#. :ref:`Variables <Variables>`
#. :ref:`Arithmetic <Arithmetic>`
#. :ref:`Functions <Functions>`

.. _Variables:
Declaring variables
===================
The naming ceremony...
----------------------

Qyri has a very simple syntax for declaring variables:

``var x = 1;``

Notice that semicolon at the end of the line. Yes, Qyri needs semicolons. No, you aren't an exception.

Variables in Qyri are mutable by default. An immutable variable is called a ``constant``. Constants are declared like this:

``const PI = 3.14;``

Note that the name of this constant is capitalized. Qyri's job is took be simple and intuitive, but also readable and extendable. Thus, Qyri holds a few style properties that keep your code understandable. Constants are in SCREAMING_SNAKE_CASE and variables are in sneaky_snake_case.


.. _Arithmetic:
Arithmetic
==========
They did the math, they did the monster math...
-----------------------------------------------

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


.. _Functions:
Declaring and Using Functions
=============================
We all live in a dysfunctional family in one way or another...
--------------------------------------------------------------

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
	}

as is this:
::
	const add = (x, y) {
		return x + y;
	}


"（ミ￣ー￣ミ）what? ``add`` isn't a variable, it's a funct-- ohhhh." You get it now. This syntax:

``() {}``

is an anonymous function. So, why implement the ``fn`` keyword if ``var`` and ``const`` are available. Well, they all do different things. When you declare a function using ``fn``, what you're doing is telling Qyri that whatever function name you use is now permanently allocated to that function, and it is unusable for any variable or constant. Conversely, using ``const`` will do that, but prevent you from using the function's return value, and you must call the function as a keyword. It acts as a ``void`` function, except instead of passing ``null`` it passes an error. Don't use it unless you're more cleverer than I. Lastly, the ``var`` method of declaration allows the function's name to be mutable, meaning you can later use that name for a variable, constant, or other function. This is rarely useful, but it's implemented and there's nothing you can do about it.