.section .text
.global lgdt
.global test
.global no_op
.global interrupt
.global unified_handler
.global register_all_callbacks

no_op:
  iret

test:
  pusha
  call callback
  popa
  iret
  
# args: (pointer to gtd)
lgdt:
   mov 4(%esp), %eax
   lgdt (%eax)
   
   #reload segment registers
   movw $0x10, %ax # 0x10 is data segment
   movw %ax, %ds
   movw %ax, %es
   movw %ax, %fs
   movw %ax, %gs
   movw %ax, %ss
   
   ljmp $0x8,$out # set the cs register to 0x8 (code segment)
out:
   ret

# u8 -> ()
interrupt:
  int $0x22
  ret


.altmacro
  
.macro make_callback num
  callback_\num\():
.endm

.macro make_all_callbacks, num=50
.if \num+1
   make_callback %num 
      pushal
      pushl $\num
      call unified_handler
      
      addl $4, %esp
      popal
      iret
  make_all_callbacks \num-1
.endif
.endm
make_all_callbacks

.macro push_callback num
  pushl $callback_\num\()
.endm

# args: &mut IDT
# the idea here is to use an as macro to fill in
# all of the interrupts
register_all_callbacks:
  pushl %ebp
  movl %esp, %ebp
  
  .macro make_register_all_callbacks, num=50
    .if \num+1
	  push_callback %num # arg3 (fn) to add_entry
	  pushl $\num # arg2 (index) to add_entry
	  movl 8(%ebp), %eax
	  pushl %eax # arg1 (&self) to add_entry
	  call add_entry
	  movl %ebp, %esp
      make_register_all_callbacks \num-1
    .endif
  .endm
  make_register_all_callbacks
    
  leave
  ret
