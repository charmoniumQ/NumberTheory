(module patterns racket/base
; m = pow(2, ceil(log(b, 2)))
; y = a reduced mod m
; b = pow(2, k) -> x = m + (a - y) / m if a is odd else m - 2 + (a - y) / m
; linear with a or a - y
	)
