/* program main -- QLL 1.0 */

.func.print:							// fn print str null
	$$jmpz 3
	$out
	call .func.print
	return

.struct.Car:							// struct Car

	.assoc.Car.new:						// wrap ctr Car int int Car
		call .assoc.Car.Car
		return

	.assoc.Car.Car:						// ctr Car
		// load param sn
		next int
		alloc int
		$pop
		// load param hp
		next int
		alloc int
		$pop

		$push
		next int
		$push

		return

	.method.Car.flip:
		// synsgr gcollect
		$push
		next int
		$push

		$over

		return

.enum.Foo:								// enum Foo 16
.variant.Foo.Bar:; $$pushi 0; return 	// variant Bar Foo 0
.variant.Foo.Baz:; $$pushi 1; return 	// variant Baz Foo 1
.variant.Foo.Bat:; $$pushi 2; return 	// variant Bat Foo 2

.func.main:								// fn main null
	call main
	return

main:									// main
	addr 16

	$$pushi 101
	$$pushi 100
	call .assoc.Car.new					// ct Car
	addr 20
	alloc int
	$pop
	addr 24
	alloc int
	$pop

	addr 20
	call .method.Car.flip
	$drop // gcollect
	addr 32
	$pop
	addr 36
	$pop

	addr 40
	call .variant.Foo.Bar
	$pop

	return