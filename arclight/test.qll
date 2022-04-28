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
			$$pushi 8
			$$pushi 1
			$$out 2
		.assoc.public.flags._override:
			$$pushi 8
			$$pushi 0
			$$out 2
	return

:illegal[builtin]
.func.print:
	$$jmpz 28
	$out
	call .func.print
	return

.struct.Foo:
	.assoc.Foo.Add:
		alloc int
		$pop									

		next int
		alloc int
		$pop									

		back int	

		$push		

		next int
		$push		

		$add		

		return		

.func.main:
	call main
	return

main:
	addr 16

	$$pushi 0; $$pushi 10;
	$$pushi 33; $$pushi 100; $$pushi 108; $$pushi 114; $$pushi 111; $$pushi 119;
	$$pushi 32; $$pushi 44; $$pushi 111; $$pushi 108; $$pushi 108; $$pushi 101;
	$$pushi 72;

	call .func.print
	return