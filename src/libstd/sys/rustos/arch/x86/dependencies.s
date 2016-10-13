# TODO(ryan): rust seems to use libc's implementation of these functions
# and I'm too lazy to strip them out of rust. An alternative might be to 
# just find the .so file for them and add it to the linker (and hope it
# doesn't pull any linux dependencies

.globl _Unwind_Resume
.globl EXHAUSTED
.globl __mulodi4
.globl __divdi3
.globl __umoddi3
#.globl rust_begin_unwind
.globl __udivdi3
.globl __moddi3
.globl __powisf2
.globl __powidf2
.globl __fixunssfdi
.globl __fixunsdfdi

.global abort

.globl write
.globl strcmp
.globl floorf
.globl ceilf
.globl roundf
.globl truncf
.globl fmaf
.globl powf
.globl expf
.globl exp2f
.globl logf
.globl log2f
.globl log10f
.globl floor
.globl ceil
.globl round
.globl trunc
.globl fma
.globl pow
.globl exp
.globl exp2
.globl log
.globl log2
.globl log10
.globl fdim
.globl fmodf

.globl fdimf
.globl fmod
.globl ldexpf
.globl frexpf
.globl nextafterf
.globl fmaxf
.globl fminf
.globl cbrtf
.globl hypotf
.globl sinf
.globl cosf
.globl tanf
.globl asinf
.globl acosf
.globl atanf
.globl atan2f
.globl expm1f
.globl log1pf
.globl sinhf
.globl coshf
.globl tanhf
.globl ldexp
.globl frexp
.globl nextafter
.globl fmax
.globl fmin
.globl cbrt
.globl hypot
.globl sin
.globl cos
.globl tan
.globl asin
.globl acos
.globl atan
.globl atan2
.globl expm1
.globl log1p
.globl sinh
.globl cosh
.globl tanh
.globl fmod

_Unwind_Resume:
EXHAUSTED:
__mulodi4:
__divdi3:
__umoddi3:
#rust_begin_unwind:
__udivdi3:
__moddi3:
__powisf2:
__powidf2:
__fixunssfdi:
__fixunsdfdi:
write:
strcmp:
floorf:
ceilf:
roundf:
truncf:
fmaf:
powf:
expf:
exp2f:
logf:
log2f:
log10f:
floor:
ceil:
round:
trunc:
fma:
pow:
exp:
exp2:
log:
log2:
log10:
fdim:
fmodf:
fdimf:
fmod:
ldexpf:
frexpf:
nextafterf:
fmaxf:
fminf:
cbrtf:
hypotf:
sinf:
cosf:
tanf:
asinf:
acosf:
atanf:
atan2f:
expm1f:
log1pf:
sinhf:
coshf:
tanhf:
ldexp:
frexp:
nextafter:
fmax:
fmin:
cbrt:
hypot:
sin:
cos:
tan:
asin:
acos:
atan:
atan2:
expm1:
log1p:
sinh:
cosh:
tanh:
fmod:
  call abort
  