.global save_context
.global restore_context

save_context:
  movl 4(%esp), %eax

  # start thread save ===>  
  movl %ebx, 4(%eax) # TODO(ryan): actually need to save these regs? eax, ebx is actually clobbered here
  movl %ecx, 8(%eax)
  movl %edx, 12(%eax)
  movl %ebp, 16(%eax)
  movl %esi, 20(%eax)
  movl %edi, 24(%eax)  
  movl %esp, 28(%eax)
  movl 0(%esp), %ebx
  movl %ebx, 32(%eax)
  # <=== end thread save

  movl $0, %eax # return false
  ret

restore_context:
  # start context switch ===>
  movl 4(%esp), %eax
  
  movl 4(%eax), %ebx
  movl 8(%eax), %ecx
  movl 12(%eax), %edx
  movl 16(%eax), %ebp
  movl 20(%eax), %esi
  movl 24(%eax), %edi
  movl 28(%eax), %esp
  
  movl 32(%eax), %eax #eip
  movl %eax, 0(%esp)
  movl $1, %eax # return true
  ret
  