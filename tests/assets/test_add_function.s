.global add_val
.type add_val,%function 

add_val:
    add x0, x0,x1, lsl #1
    ret
