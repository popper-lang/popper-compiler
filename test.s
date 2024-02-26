	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 14, 0
	.globl	_main                           ## -- Begin function main
	.p2align	4, 0x90
_main:                                  ## @main
	.cfi_startproc
## %bb.0:                               ## %entry
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movabsq	$17179869187, %rax              ## imm = 0x400000003
	movq	%rax, 12(%rsp)
	movl	$5, 20(%rsp)
	leaq	12(%rsp), %rsi
	movq	%rsi, 8(%rsp)
	leaq	L_.str(%rip), %rdi
                                        ## kill: def $esi killed $esi killed $rsi
	xorl	%eax, %eax
	callq	_printf
	xorl	%eax, %eax
	addq	$24, %rsp
	retq
	.cfi_endproc
                                        ## -- End function
	.section	__TEXT,__cstring,cstring_literals
L_.str:                                 ## @.str
	.asciz	"%d\\n\n"

.subsections_via_symbols
