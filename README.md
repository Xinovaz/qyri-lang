# What is this?
Qyri, after "qiri," the Albanian word for "candle," is an in-depth and intelligent programming language, that boasts a clean syntax and a limitless ecosystem. Qyri reinforces good programming habits while still being intuitive and friendly.

# Why Qyri?
Qyri offers an endless number of opportunities. Qyri is elegant and pure, meaning the highlight of your code is its beauty. Any error messages you recieve come packed with information, meaning you won't have to track down those pesky commas. Qyri's syntax is very clean. Simplicity is key when designing your projects, and Qyri unlocks that for you, which means saying goodbye to spaghetti code. Qyri's naming conventions are designed to allow anyone reading your code to be able to effectively understand it.

But that's not all. Qyri offers not only beauty in its code, but in its runtime as well. Qyri is fast and not at all performance-heavy. Upon compilation, Qyri minimises any code, only allocating what is needed, and is excellent at lightweight halting for asynchronous programming and the like.

# Why not Qyri?
Don't use Qyri if you aren't focused on lightweight shipping. As well as this, typed constants and numeric emulation are possible, but not suggested for a conventional Qyri program. There aren't many reasons to not use Qyri, but those that are, are based solely on purism and preference.

# Example
```
// Dynamically-typed Quicksort
fn sort(list, low, high) %dyn {

	if (length(list) == 1) {
		return list;
	}

	if (low < high) {
		var i = low - 1;
		var pivot = list[high];

		for (#dvar [j, low], // a macro that doesn't act as a statement
			j++,
			j <= high)
		{
			if (list[j] <= pivot) {
				i++;
				%(#ref [list[i], list[j]]) = (list[j], list[i]);
				/*
				Weird syntax alert!

				Why are there all these weird symbols floating about?
				Let's break it down.

				"#ref [list[i], list[j]]" is a macro that gets the referenced values 
				while remembering the object that they came from.
				When we wrap it with "%()", what we're really doing is taking advantage
				of the fact that Qyri remembers the items' parent list, and turning those
				references into portals (pointers of the items' addresses, their values).

				Qyri's job is supposed to be standing around and looking pretty, though, right?

				Well, an alternative would be:

				pointer (list[i], list[j]) = (list[j], list[i]);

				However, this option clears part of the memory where the values of 
				list[i] and list[j] were stored, mandating a bit shift later on. However,
				this bit shift would require a statically-typed scope, and this example
				is dynamically typed. 
				*/
			}
			%(#ref [list[i + 1], list[high]]) = (list[high], list[i + 1]);
			var prt = i + 1;
		}
		sort(list, low, prt - 1);
		sort(list, prt + 1, high);
	}
}

/* Clearing the function. Not recommended in production, this is just for 
you to be able to run in the playground. These lines remove the sort function
from the stack. */
var ssize = length((#machine_index [sort])..(#machine_index [%sort])); // Hideous and shameless
generic s = fit(ssize); *ssize; /* Creates a generic type s that is equal to the to
the size of the sort function. */

*%sort; // Literally dereferences the actual value, replacing it with empty Operands.
(#memory_from [#machine_index [%sort]]) << 0.as(s); *s; // Fill in the blank.

// Statically-typed Quicksort
fn sort(list: vec<char>, low: char, high: char) : vec<char> {
	if (length(list) == 1) {
		return list;
	}

	if (low <= high) {
		var i: char = low - 1;
		var pivot: char = list[high];

		for (#var [j, char, low],
			j++,
			j <= high)
		{
			if (list[j] <= pivot) {
				i++;
				pointer (list[i], list[j]) = (list[i], list[j]);
			}
			pointer (list[i + 1], list[high]) = (list[high], list[i + 1]);
			var prt: char = i + 1;
		}
		sort(list, low, prt - 1);
		sort(list, prt + 1, high);
		}
	}
}

/* I know I said it would need a bit shift, but this is the end of the program.
Theoretically, a longer program would require a bit shift or some markers for the garbage 
collector, but the program is over now. We can go home. I can see my family again. */
```

# Donations

Qyri's first version, _qyri-ocular-00_, is under development. Qyri Ocular is meant to provide a simple Qyri ecosystem to demo the language's features before its first real release. Designing a language is hard, and although this is something I'm passionate about, I have school and work to focus on. So, a little bit of money can help motivate me to prioritize Qyri Ocular's development to release it.

## BTC
[17h3aXfrvr2MNLARTVyeQpCGBzgw53VAiJ](https://btc.com/17h3aXfrvr2MNLARTVyeQpCGBzgw53VAiJ)

## ETH
0x4d0883f2a3DE14f3988A395B463DA1473a8f8f8f (No hyperlink, sorry)

