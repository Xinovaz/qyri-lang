# What is this?
Qyri, after "qiri," the Albanian word for "candle," is an in-depth and intelligent programming language, that boasts a clean syntax and a limitless ecosystem. Qyri reinforces good programming habits while still being intuitive and friendly.

# Why Qyri?
Qyri offers an endless number of opportunities. Qyri is elegant and pure, meaning the highlight of your code is its beauty. Any error messages you recieve come packed with information, meaning you won't have to track down those pesky commas. Qyri's syntax is very clean. Simplicity is key when designing your projects, and Qyri unlocks that for you, which means saying goodbye to spaghetti code. Qyri's naming conventions are designed to allow anyone reading your code to be able to effectively understand it.

But that's not all. Qyri offers not only beauty in its code, but in its runtime as well. Qyri is fast and not at all performance-heavy. Upon compilation, Qyri minimises any code, only allocating what is needed, and is excellent at lightweight halting for asynchronous programming and the like.

# Why not Qyri?
Don't use Qyri if you aren't focused on lightweight shipping. As well as this, typed constants and numeric emulation are possible, but not suggested for a conventional Qyri program. There aren't many reasons to not use Qyri, but those that are, are based solely on purism and preference.

# Example
### Disclaimer: Syntax is subject to change.
```
// fizzbuzz.qi

using std.io.print;
using mod, PI from math;

fn fizzbuzz = (n: int) $ str {
	if n `mod` 15 == 0 {
		return "FizzBuzz";
	} else if n `mod` 3 == 0 {
		return "Fizz";
	} else if n `mod` 5 == 0 {
		return "Buzz";
	} else {
		return n;
	}
}

fn main = () {
	for (var i = 0; i < 128; i++) {
		print(fizzbuzz(i));
	}
	
	except fizzbuzz(PI) {
		print("Pi is a constant of type float and will raise an exception.");
	}
}
```
# Donations

Qyri's first version, _qyri-ocular-00_, is under development. Qyri Ocular is meant to provide a simple Qyri ecosystem to demo the language's features before its first real release. Designing a language is hard, and although this is something I'm passionate about, I have school and work to focus on. So, a little bit of money can help motivate me to prioritize Qyri Ocular's development to release it.

## BTC
[17h3aXfrvr2MNLARTVyeQpCGBzgw53VAiJ](https://btc.com/17h3aXfrvr2MNLARTVyeQpCGBzgw53VAiJ)

## ETH
0x4d0883f2a3DE14f3988A395B463DA1473a8f8f8f (No hyperlink, sorry)

