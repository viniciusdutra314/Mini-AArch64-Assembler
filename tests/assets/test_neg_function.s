.global negate_val //comments here should be ignored
.type negate_val,%function 

negate_val:
    neg x0,x0, lsl #1
    ret
