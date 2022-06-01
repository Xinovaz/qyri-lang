/* program test -- QLL 1.0 */

:assembly[package]
.public:
	:assembly[flags]
	.public.flags:
		addr 0
	  /*[~~~~~~~~~~~~~version number~~~~~~~~~~~~~]*/
		$$pushi 1; $$pushi 0; $$pushi 0; $$pushi 1
		$$pushi 0; $$pushi 1; $$pushi 1; $$pushi 0
	  /*[pshilgls]~[admnprms]~[rlswarns]~[wrnerrs]*/
		:transaction[read]
		.assoc.public.flags._read:
			addr 1
			$$pushi 8
			$$pushi 1
			return
		.assoc.public.flags._override:
			addr 2
			$$pushi 8
			$$pushi 0
			return
	return

:illegal[builtin]
.func.print:
	$$jmpz 29
	$out
	call .func.print
	return	


.struct.Car:
	.assoc.func._ctor:
		next int
		$push
		return


.func.main:
	call main
	return

main:
	addr 16
	
	$$pushi 0
	$$pushi 10
	$$pushi 97
	call .func.print
	return