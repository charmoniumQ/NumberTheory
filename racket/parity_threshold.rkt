(module bases racket/base
	(require racket/file)
	(require plot)
	(require srfi/1)
	(require memoize)
	(require anaphoric)
	(require "lisp_utils.rkt")
	(require "racket_specific.rkt")
	(require "functions_sets.rkt")
	(require "k-ary.rkt")

	(define (parity-line? i k security)
		(every
			(lambda (i*) (list= =
				(k-ary-divisors-pow i* k)
				(k-ary-divisors-pow i* (if (odd? k) 1 3))))
			(rangei i (+ security i))))

	; returns minimum i where the k-ary factors of p^i are the same as the biunitary (for even k) or unitary (for odd k) factors of p^i
	(define (first-parity-line k security)
		(case k
			[(1) 0]
			[(2) 0]
			[else (let loop
				([i (first-parity-line (- k 2) security)])
				(if (parity-line? i k security)
					i
					(loop (+ 1 i))))]))

	(define (plot-parity odds evens fname [security 200])
		(plot-height 800)
		(plot-width 1000)
		(plot (list (lines odds) (points odds) (lines evens) (points evens)) #:out-file fname #:x-label "k" #:y-label "parity threshold"))

	(module+ main
		(define maxk 3)
		(define security 10)

		(define odds (enumerate (map (curryr first-parity-line security) (rangei 1 maxk 2))))
		(define evens (enumerate (map (curryr first-parity-line security) (rangei 2 maxk 2))))
		(plot-parity odds evens "output/parity_threshold.png")
		(display-to-file (as-csv (nested-map number->string evens 2)) "output/evens.csv")
		(display-to-file (as-csv (nested-map number->string odds 2)) "output/odds.csv"))

	(provide parity-line? first-parity-line plot-parity))
