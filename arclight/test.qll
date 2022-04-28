/* program test -- QLL 1.0 */

:illegal[builtin]
.func.print:
	$$jmpz 4
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